//! Deterministic error taxonomy for admission and verification gates.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BindingError {
    FrameSizeInvalid,
    BufferMisaligned,
    MagicBytesMismatch,
    EpochExpired,
    DigestMismatch,
    SignatureInvalid,
    KeyUnauthorized,
    InvalidTimestampRange,
    InvalidCurrencyPadding,
    InvalidCurrencyCode,
    ReservedFieldNonZero,
    SemanticMismatch,
    PreDispatchFault,
    InvalidStateTransition,
}
