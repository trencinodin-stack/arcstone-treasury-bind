//! Normative wire protocol constants for ARC-SPEC-2026-BIND-01 (REV-11).

pub const MAX_INTENT_BYTES: usize = 4096;
pub const SHA256_DIGEST_LEN: usize = 32;
pub const PROTOCOL_MAGIC: [u8; 4] = *b"ARCS";
pub const DOMAIN_TAG: &[u8; 11] = b"ARC-BIND-V1";
pub const SIGNED_TRANSCRIPT_LEN: usize = 11 + 128 + 64 + 88; // Exactly 291 Bytes
pub const EXECUTION_PERMIT_LEN: usize = 96;                   // Exactly 96 Bytes

pub const MIME_ISO_20022_XML: u16 = 0x0001;
pub const MIME_RFC_8785_JSON: u16 = 0x0002;
