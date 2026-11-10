//! Closed Seven-State Reconciliation Automaton per ARC-SPEC-2026-BIND-01 (REV-11).

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreasuryState {
    Init,
    Admitted,
    DispatchReserved,
    Committed,
    Denied,
    TerminalRejected,
    UnknownReconcile { locked_manual: bool },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClearingAck {
    Acsc, // Accepted Settlement Completed (Final)
    Accp, // Accepted Technical / Intermediate
    Pdng, // Pending
    Rjct, // Rejected (Final)
}

impl TreasuryState {
    /// Ingress evaluation transition.
    pub fn on_ingress_admit(allowed: bool) -> Self {
        if allowed {
            TreasuryState::Admitted
        } else {
            TreasuryState::Denied
        }
    }

    /// Commit initiation transition (Atomic CAS on WAL: TokenID -> RESERVED).
    pub fn on_dispatch_reserve(&self) -> Result<Self, crate::error::BindingError> {
        match self {
            TreasuryState::Admitted => Ok(TreasuryState::DispatchReserved),
            _ => Err(crate::error::BindingError::InvalidStateTransition),
        }
    }

    /// Pre-dispatch fault (e.g., CAS storage retrieval timeout before network dispatch).
    pub fn on_pre_dispatch_fault(&self) -> Result<Self, crate::error::BindingError> {
        match self {
            TreasuryState::Admitted => Ok(TreasuryState::Denied),
            _ => Err(crate::error::BindingError::InvalidStateTransition),
        }
    }

    /// External clearing rail response transition.
    pub fn on_clearing_ack(&self, ack: ClearingAck) -> Result<Self, crate::error::BindingError> {
        match self {
            TreasuryState::DispatchReserved | TreasuryState::UnknownReconcile { .. } => match ack {
                ClearingAck::Acsc => Ok(TreasuryState::Committed),
                ClearingAck::Accp | ClearingAck::Pdng => {
                    Ok(TreasuryState::UnknownReconcile { locked_manual: false })
                }
                ClearingAck::Rjct => Ok(TreasuryState::TerminalRejected),
            },
            _ => Err(crate::error::BindingError::InvalidStateTransition),
        }
    }

    /// Transport partition / timeout handler.
    pub fn on_transport_timeout(&self) -> Result<Self, crate::error::BindingError> {
        match self {
            TreasuryState::DispatchReserved => {
                Ok(TreasuryState::UnknownReconcile { locked_manual: false })
            }
            _ => Err(crate::error::BindingError::InvalidStateTransition),
        }
    }

    /// Dwell timeout (>120s) or clearing cutoff proximity reached.
    pub fn on_manual_lock_engage(&self) -> Result<Self, crate::error::BindingError> {
        match self {
            TreasuryState::UnknownReconcile { .. } => {
                Ok(TreasuryState::UnknownReconcile { locked_manual: true })
            }
            _ => Err(crate::error::BindingError::InvalidStateTransition),
        }
    }
}
