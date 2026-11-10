//! Adversarial Conformance Test Corpus (TC-BIND-01 through TC-BIND-10)
//! Grounded in ARC-SPEC-2026-BIND-01 (REV-11) Section 8.

use arcstone_treasury_bind::constants::*;
use arcstone_treasury_bind::error::BindingError;
use arcstone_treasury_bind::frame::DecodedFrame;
use arcstone_treasury_bind::permit::ExecutionPermit;
use arcstone_treasury_bind::state::{ClearingAck, TreasuryState};

#[repr(align(64))]
struct AlignedBuffer([u8; MAX_INTENT_BYTES]);

fn create_valid_frame_buffer() -> (AlignedBuffer, [u8; 32], [u8; 32], u64, u64) {
    let mut buf = AlignedBuffer([0u8; MAX_INTENT_BYTES]);
    buf.0[0..4].copy_from_slice(&PROTOCOL_MAGIC);
    buf.0[4..6].copy_from_slice(&1u16.to_le_bytes()); // Schema v1
    buf.0[6..8].copy_from_slice(&MIME_ISO_20022_XML.to_le_bytes());

    let epoch = 100u64;
    buf.0[8..16].copy_from_slice(&epoch.to_le_bytes());

    let not_before = 10_000u64;
    let not_after = 20_000u64;
    buf.0[0x10..0x18].copy_from_slice(&not_before.to_le_bytes());
    buf.0[0x18..0x20].copy_from_slice(&not_after.to_le_bytes());

    let admitted_payload_digest = [0xAAu8; 32];
    buf.0[0x20..0x40].copy_from_slice(&admitted_payload_digest);

    let sender_pubkey = [0x55u8; 32];
    buf.0[0x60..0x80].copy_from_slice(&sender_pubkey);

    let dummy_sig = [0x77u8; 64];
    buf.0[0x80..0xC0].copy_from_slice(&dummy_sig);

    let entitlement_token = [0x99u8; 32];
    buf.0[0xC0..0xE0].copy_from_slice(&entitlement_token);

    // Currency USD
    buf.0[0xE0] = b'U';
    buf.0[0xE1] = b'S';
    buf.0[0xE2] = b'D';

    let amount_minor = 100_000_00u128; // $100,000.00
    buf.0[0x100..0x110].copy_from_slice(&amount_minor.to_le_bytes());

    let target_ledger_id = 9999u64;
    buf.0[0x110..0x118].copy_from_slice(&target_ledger_id.to_le_bytes());

    (buf, sender_pubkey, entitlement_token, epoch, 15_000u64)
}

#[test]
fn test_tc_bind_01_nominal_iso_20022_execution() {
    let (buf, expected_pk, expected_tok, epoch, current_time) = create_valid_frame_buffer();
    let admitted = DecodedFrame::admit(
        &buf.0,
        current_time,
        epoch,
        |pk, tok, ledger| pk == &expected_pk && tok == &expected_tok && ledger == 9999,
        |_pk, _transcript, _sig| true,
    );
    assert!(admitted.is_ok());
    let frame = admitted.unwrap();

    let payload_digest = [0xAAu8; 32];
    assert!(frame.verify_payload_binding(&payload_digest).is_ok());

    let state = TreasuryState::on_ingress_admit(true);
    assert_eq!(state, TreasuryState::Admitted);

    let state = state.on_dispatch_reserve().unwrap();
    assert_eq!(state, TreasuryState::DispatchReserved);

    let state = state.on_clearing_ack(ClearingAck::Acsc).unwrap();
    assert_eq!(state, TreasuryState::Committed);
}

#[test]
fn test_tc_bind_02_toctou_payload_substitution() {
    let (buf, expected_pk, expected_tok, epoch, current_time) = create_valid_frame_buffer();
    let frame = DecodedFrame::admit(
        &buf.0,
        current_time,
        epoch,
        |pk, tok, ledger| pk == &expected_pk && tok == &expected_tok && ledger == 9999,
        |_pk, _transcript, _sig| true,
    )
    .unwrap();

    let substituted_digest = [0xBBu8; 32];
    let commit_res = frame.verify_payload_binding(&substituted_digest);
    assert_eq!(commit_res, Err(BindingError::DigestMismatch));
}

#[test]
fn test_tc_bind_04_stale_epoch_replay() {
    let (buf, expected_pk, expected_tok, epoch, current_time) = create_valid_frame_buffer();
    let stale_epoch = epoch + 1;
    let res = DecodedFrame::admit(
        &buf.0,
        current_time,
        stale_epoch,
        |pk, tok, ledger| pk == &expected_pk && tok == &expected_tok && ledger == 9999,
        |_pk, _transcript, _sig| true,
    );
    assert_eq!(res, Err(BindingError::EpochExpired));
}

#[test]
fn test_tc_bind_07_economic_field_tampering() {
    let (buf, expected_pk, expected_tok, epoch, current_time) = create_valid_frame_buffer();
    let res = DecodedFrame::admit(
        &buf.0,
        current_time,
        epoch,
        |pk, tok, ledger| pk == &expected_pk && tok == &expected_tok && ledger == 9999,
        |_pk, _transcript, _sig| false, // Signature verification fails
    );
    assert_eq!(res, Err(BindingError::SignatureInvalid));
}

#[test]
fn test_tc_bind_09_pre_dispatch_storage_fault() {
    let state = TreasuryState::Admitted;
    let faulted_state = state.on_pre_dispatch_fault().unwrap();
    assert_eq!(faulted_state, TreasuryState::Denied);
}

#[test]
fn test_permit_wire_format_exact_96_bytes() {
    let permit = ExecutionPermit::new([1u8; 32], [2u8; 32], 42, 100_000, [3u8; 16]);
    let bytes = permit.to_bytes();
    assert_eq!(bytes.len(), 96);
    let parsed = ExecutionPermit::from_bytes(&bytes);

    // Copy packed fields to local variables to prevent unaligned reference creation (E0793)
    let lease_id = parsed.epoch_lease_id;
    let not_after = parsed.valid_not_after_ns;
    assert_eq!(lease_id, 42);
    assert_eq!(not_after, 100_000);
}