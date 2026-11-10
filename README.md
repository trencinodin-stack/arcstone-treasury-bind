# arcstone-treasury-bind

[![DOI](https://zenodo.org/badge/DOI/10.5281/zenodo.24119501.svg)](https://doi.org/10.5281/zenodo.24119501)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/License-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)
[![no_std](https://img.shields.io/badge/no__std-compatible-success.svg)](https://docs.rust-embedded.org/)

Canonical `#[no_std]` Rust reference implementation of the **Arcstone Treasury Binding Protocol**.

* **Specification Identifier:** `ARC-SPEC-2026-BIND-01 (REV-11)`
* **Master Anchor:** `A-77-DELTA-SHIELD-LOCKED`
* **Permanent DOI:** [10.5281/zenodo.24119501](https://doi.org/10.5281/zenodo.24119501)
* **Primary Suite Record:** [10.5281/zenodo.22665852](https://doi.org/10.5281/zenodo.22665852)
* **Canonical Roots:** [admissibilityscience.com](https://admissibilityscience.com) | [paradigm.admissibilityscience.com](https://paradigm.admissibilityscience.com)

## Invariants

- **Zero Runtime Heap Allocation:** `M_alloc = 0` on the critical admission path.
- **Fixed Wire Envelope:** `S_max <= 4096 B` in Tightly Coupled Memory (TCM).
- **Execution Permit Carrier:** 96-byte packed binary authorization token.
- **Timing Invariants:** `tau_eval <= 10.00 ms`, hardware clamp `tau_override <= 11.99 ms`.
- **Closed Seven-State Reconciliation Automaton:** Discrete state lifecycle enforcing strict settlement finality (`pacs.002 ACSC`).

## Test Suite

Run the full adversarial conformance corpus (`TC-BIND-01` through `TC-BIND-10`):

```bash
cargo test --test adversarial_corpus
```

## License

Licensed under either of:
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))
at your option.
