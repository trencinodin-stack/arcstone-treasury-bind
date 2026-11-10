//! Normative single-pass IntentFrame parser and constant-time commit barrier.

use crate::constants::*;
use crate::error::BindingError;

#[repr(C)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecodedFrame {
    pub magic: [u8; 4],
    pub schema_version: u16,
    pub mime_type: u16,
    pub epoch_id: u64,
    pub valid_not_before: u64,
    pub valid_not_after: u64,
    pub payload_digest: [u8; SHA256_DIGEST_LEN],
    pub merkle_root: [u8; 32],
    pub sender_pubkey: [u8; 32],
    pub signature: [u8; 64],
    pub entitlement_token: [u8; 32],
    pub currency_code: [u8; 3],
    pub amount_minor_units: u128,
    pub target_ledger_id: u64,
}

impl DecodedFrame {
    /// Safe single-pass admission evaluation from an isolated, quiesced snapshot memory buffer.
    /// Eliminates double-fetch by parsing and verifying exclusively from a single immutable 4KB reference.
    pub fn admit<F, V>(
        buffer: &[u8; MAX_INTENT_BYTES],
        current_time_ns: u64,
        current_epoch: u64,
        key_registry_lookup: F,
        ed25519_verifier: V,
    ) -> Result<Self, BindingError>
    where
        F: Fn(&[u8; 32], &[u8; 32], u64) -> bool,
        V: Fn(&[u8; 32], &[u8; SIGNED_TRANSCRIPT_LEN], &[u8; 64]) -> bool,
    {
        if (buffer.as_ptr() as usize) % 64 != 0 {
            return Err(BindingError::BufferMisaligned);
        }

        let magic: [u8; 4] = [buffer[0], buffer[1], buffer[2], buffer[3]];
        if magic != PROTOCOL_MAGIC {
            return Err(BindingError::MagicBytesMismatch);
        }

        let not_before = u64::from_le_bytes(buffer[0x10..0x18].try_into().unwrap_or([0; 8]));
        let not_after = u64::from_le_bytes(buffer[0x18..0x20].try_into().unwrap_or([0; 8]));
        if not_before > not_after {
            return Err(BindingError::InvalidTimestampRange);
        }

        if current_time_ns < not_before || current_time_ns > not_after {
            return Err(BindingError::EpochExpired);
        }

        let epoch_id = u64::from_le_bytes(buffer[8..16].try_into().unwrap_or([0; 8]));
        if epoch_id != current_epoch {
            return Err(BindingError::EpochExpired);
        }

        // Validate 3-byte currency code is ASCII uppercase [A-Z]
        let currency_code: [u8; 3] = [buffer[0xE0], buffer[0xE1], buffer[0xE2]];
        for &c in &currency_code {
            if !(b'A'..=b'Z').contains(&c) {
                return Err(BindingError::InvalidCurrencyCode);
            }
        }

        // Validate 29-byte zero padding on currency field (0x00E3..0x0100)
        for &pad_byte in &buffer[0xE3..0x100] {
            if pad_byte != 0 {
                return Err(BindingError::InvalidCurrencyPadding);
            }
        }

        // Validate reserved scratchpad is strictly zeroed (0x0118..0x1000)
        for &scratch_byte in &buffer[0x118..0x1000] {
            if scratch_byte != 0 {
                return Err(BindingError::ReservedFieldNonZero);
            }
        }

        let mut sender_pubkey = [0u8; 32];
        sender_pubkey.copy_from_slice(&buffer[0x60..0x80]);

        let mut signature = [0u8; 64];
        signature.copy_from_slice(&buffer[0x80..0xC0]);

        let mut entitlement_token = [0u8; 32];
        entitlement_token.copy_from_slice(&buffer[0xC0..0xE0]);

        let target_ledger_id = u64::from_le_bytes(buffer[0x110..0x118].try_into().unwrap_or([0; 8]));

        // Verify sender key authority: KeyAuthorized(pubkey, entitlement_token, target_ledger_id)
        if !key_registry_lookup(&sender_pubkey, &entitlement_token, target_ledger_id) {
            return Err(BindingError::KeyUnauthorized);
        }

        // Construct 291-Byte Preimage: DOMAIN_TAG || Frame[0..0x80] || [0; 64] || Frame[0xC0..0x118]
        let mut transcript = [0u8; SIGNED_TRANSCRIPT_LEN];
        transcript[0..11].copy_from_slice(DOMAIN_TAG);
        transcript[11..139].copy_from_slice(&buffer[0..0x80]);
        // Bytes 139..203 represent the zeroed 64-byte signature slot
        transcript[203..291].copy_from_slice(&buffer[0xC0..0x118]);

        // Strict RFC 8032 Ed25519 Signature Verification
        if !ed25519_verifier(&sender_pubkey, &transcript, &signature) {
            return Err(BindingError::SignatureInvalid);
        }

        let mut payload_digest = [0u8; 32];
        payload_digest.copy_from_slice(&buffer[0x20..0x40]);

        let mut merkle_root = [0u8; 32];
        merkle_root.copy_from_slice(&buffer[0x40..0x60]);

        Ok(DecodedFrame {
            magic,
            schema_version: u16::from_le_bytes(buffer[4..6].try_into().unwrap_or([0; 2])),
            mime_type: u16::from_le_bytes(buffer[6..8].try_into().unwrap_or([0; 2])),
            epoch_id,
            valid_not_before: not_before,
            valid_not_after: not_after,
            payload_digest,
            merkle_root,
            sender_pubkey,
            signature,
            entitlement_token,
            currency_code,
            amount_minor_units: u128::from_le_bytes(buffer[0x100..0x110].try_into().unwrap_or([0; 16])),
            target_ledger_id,
        })
    }

    /// Commit barrier: Verifies bit-exact payload hash match in constant time.
    pub fn verify_payload_binding(
        &self,
        computed_payload_digest: &[u8; SHA256_DIGEST_LEN],
    ) -> Result<(), BindingError> {
        let mut diff: u8 = 0;
        for i in 0..SHA256_DIGEST_LEN {
            diff |= core::hint::black_box(self.payload_digest[i] ^ computed_payload_digest[i]);
        }

        if diff != 0 {
            return Err(BindingError::DigestMismatch);
        }

        Ok(())
    }

    /// Build the canonical 291-byte transcript preimage used for signature verification and TokenID calculation.
    pub fn build_preimage(buffer: &[u8; MAX_INTENT_BYTES]) -> [u8; SIGNED_TRANSCRIPT_LEN] {
        let mut transcript = [0u8; SIGNED_TRANSCRIPT_LEN];
        transcript[0..11].copy_from_slice(DOMAIN_TAG);
        transcript[11..139].copy_from_slice(&buffer[0..0x80]);
        transcript[203..291].copy_from_slice(&buffer[0xC0..0x118]);
        transcript
    }
}
