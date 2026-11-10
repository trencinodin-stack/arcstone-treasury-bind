# arcstone-treasury-bind

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.24119501.svg)](https://doi.org/10.5281/zenodo.24119501)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/)
[![CI](https://github.com/trencinodin-stack/arcstone-treasury-bind/actions/workflows/ci.yml/badge.svg)](https://github.com/trencinodin-stack/arcstone-treasury-bind/actions/workflows/ci.yml)

Canonical `#[no_std]` Rust reference implementation of the **Arcstone Treasury Binding Protocol**.

- **Specification Identifier:** `ARC-SPEC-2026-BIND-01 (REV-11)`
- **Target Canonical Master Anchor:** `A-77-DELTA-SHIELD-LOCKED`
- **Permanent DOI:** [10.5281/zenodo.24119501](https://doi.org/10.5281/zenodo.24119501)
- **Primary Suite Record:** [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852)
- **Canonical Roots:** [admissibilityscience.com](https://admissibilityscience.com) | [paradigm.admissibilityscience.com](https://paradigm.admissibilityscience.com)

---

## Architectural Invariants

- **Zero Runtime Heap Allocation:** $M_{\text{alloc}} = 0$ across all admission gates, frame parsers, and commit barriers.
- **Atomic Memory Snapshot:** Single-pass evaluation over an isolated $S_{\max} \le 4096\text{ B}$ buffer in Tightly Coupled Memory (TCM), eliminating double-fetch Time-of-Check to Time-of-Use (TOCTOU) payload divergence.
- **Timing Invariants:** Bounded evaluation window $\tau_{\text{eval}} \le 10.00\text{ ms}$; hardware bus clamp $\tau_{\text{override}} \le 11.99\text{ ms}$ forcing deterministic fault transitions.
- **Non-Model Rule-Based Posture ($C_{\text{ops}} = 0$):** Fully deterministic non-model control plane compliant with **Federal Reserve SR 26-2** and **OCC Bulletin 2026-13**.

---

## Protocol Wire Layouts

### 1. IntentFrame Envelope ($S_{\max} \le 4096\text{ B}$)
Evaluated in static memory aligned to 64 bytes. Admittance calculates a 291-byte transcript preimage binding the economic intent, target ledger ID, valid time window, and content-addressed payload digest prior to signature validation.

### 2. ExecutionPermit (96 Bytes Packed)
The carrier permit output upon successful admission evaluation:

| Offset | Field | Type | Description |
| :--- | :--- | :--- | :--- |
| `0x00..0x20` | `token_id` | `[u8; 32]` | HMAC-SHA256 over 291-byte canonical preimage |
| `0x20..0x40` | `payload_sha256_digest` | `[u8; 32]` | Content-addressable SHA-256 payload digest |
| `0x40..0x48` | `epoch_lease_id` | `u64` | Monotonic epoch lease identifier (LE) |
| `0x48..0x50` | `valid_not_after_ns` | `u64` | Nanosecond expiration timestamp (LE) |
| `0x50..0x60` | `permit_mac` | `[u8; 16]` | Truncated message authentication code |

---

## Closed Reconciliation Automaton

Financial states follow a closed seven-state lifecycle enforcing external settlement finality:

```text
[INIT] ──> [ADMITTED] ──> [DISPATCH_RESERVED] ──> [COMMITTED]
              │                  │
              │ (storage fault)  ├──(pacs.002 RJCT)───────────> [TERMINAL_REJECTED]
              ▼                  │
           [DENIED]              └──(transport timeout)───────> [UNKNOWN_RECONCILE]
                                                                      │
                                                                      │ (dwell > 120s)
                                                                      ▼
                                                                [LOCKED_MANUAL]
```

## Test & Verification Corpus

Run the full adversarial conformance suite (`TC-BIND-01` through `TC-BIND-10`):

```bash
cargo test --test adversarial_corpus
```

Target bare-metal embedded architectures (`thumbv7em-none-eabihf`):

```bash
cargo build --target thumbv7em-none-eabihf --no-default-features
```

---

## Governance & Machine Context

- **Agentic Invariants:** See [`AGENTS.md`](AGENTS.md) for automated delegate and agent operating rules.
- **LLM Context:** Canonical specification overview available at [`llms.txt`](llms.txt).
- **Security Policy:** Reporting instructions and invariant scope defined in [`SECURITY.md`](SECURITY.md).
- **Citation:** Formal citation metadata provided in [`CITATION.cff`](CITATION.cff).

---

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
