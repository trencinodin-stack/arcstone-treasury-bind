//! 96-Byte Packed Binary Execution Permit layout and constructor.

use crate::constants::SHA256_DIGEST_LEN;

#[repr(C, packed)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExecutionPermit {
    pub token_id: [u8; 32],
    pub payload_sha256_digest: [u8; SHA256_DIGEST_LEN],
    pub epoch_lease_id: u64,
    pub valid_not_after_ns: u64,
    pub permit_mac: [u8; 16],
}

impl ExecutionPermit {
    pub fn new(
        token_id: [u8; 32],
        payload_sha256_digest: [u8; SHA256_DIGEST_LEN],
        epoch_lease_id: u64,
        valid_not_after_ns: u64,
        permit_mac: [u8; 16],
    ) -> Self {
        Self {
            token_id,
            payload_sha256_digest,
            epoch_lease_id,
            valid_not_after_ns,
            permit_mac,
        }
    }

    pub fn to_bytes(&self) -> [u8; 96] {
        let mut out = [0u8; 96];
        out[0x00..0x20].copy_from_slice(&self.token_id);
        out[0x20..0x40].copy_from_slice(&self.payload_sha256_digest);
        out[0x40..0x48].copy_from_slice(&self.epoch_lease_id.to_le_bytes());
        out[0x48..0x50].copy_from_slice(&self.valid_not_after_ns.to_le_bytes());
        out[0x50..0x60].copy_from_slice(&self.permit_mac);
        out
    }

    pub fn from_bytes(bytes: &[u8; 96]) -> Self {
        let mut token_id = [0u8; 32];
        let mut payload_sha256_digest = [0u8; 32];
        let mut permit_mac = [0u8; 16];

        token_id.copy_from_slice(&bytes[0x00..0x20]);
        payload_sha256_digest.copy_from_slice(&bytes[0x20..0x40]);
        let epoch_lease_id = u64::from_le_bytes(bytes[0x40..0x48].try_into().unwrap());
        let valid_not_after_ns = u64::from_le_bytes(bytes[0x48..0x50].try_into().unwrap());
        permit_mac.copy_from_slice(&bytes[0x50..0x60]);

        Self {
            token_id,
            payload_sha256_digest,
            epoch_lease_id,
            valid_not_after_ns,
            permit_mac,
        }
    }
}
