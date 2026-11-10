# Security Policy

## Reporting a Vulnerability

If you discover a security vulnerability, buffer alignment flaw, constant-time comparison leak, TOCTOU bypass, or state-machine invalid transition affecting `arcstone-treasury-bind`, please **do not open a public issue**.

Submit reports through either of the following private channels:

1. **GitHub Private Vulnerability Reporting (Recommended):**
   * Go to the **Security** tab of this repository (`trencinodin-stack/arcstone-treasury-bind`).
   * Click **Report a vulnerability**.
   * Fill out the form to submit a private report directly to maintainers.

2. **Email:**
   * Contact us at **security@arcstoneos.com**.

---

## What to Include

Please include, where applicable:
- A concise description of the vulnerability or predicate logic defect.
- Minimal reproduction steps or adversarial unit test (`cargo test`) demonstrating the issue.
- The affected crate version, release tag (e.g., `v0.1.0`), or commit hash anchor (`A-77-DELTA-SHIELD-LOCKED`).
- Whether the issue involves:
  * Memory ceiling or unaligned snapshot access ($S_{\max} > 4096\text{ B}$ or non-64-byte alignment);
  * Non-constant-time payload digest verification (`verify_payload_binding`);
  * Packed struct alignment or undefined behavior (compiler warning/error `E0793`);
  * 291-byte transcript preimage mismatch or Ed25519 signature bypass;
  * Seven-state reconciliation automaton escape (e.g., premature transition to `COMMITTED` without `pacs.002 ACSC`);
  * Stale epoch lease or invalid timestamp interval admission ($current\_time \notin [t_{\text{not\_before}}, t_{\text{not\_after}}]$);
  * Violation of `#![no_std]` or zero-heap runtime guarantees ($M_{\mathrm{alloc}} = 0$).

---

## Normative Immutability & Errata Protocol

`arcstone-treasury-bind` implements **ARC-SPEC-2026-BIND-01 (REV-11)** (Permanent DOI `10.5281/zenodo.24119501`) under an operational posture of $C_{\mathrm{ops}} = 0$.

- **No Unilateral Invariant Mutation:** Reporting a defect does not authorize altering normative wire constants (`MAX_INTENT_BYTES = 4096`, `EXECUTION_PERMIT_LEN = 96`), currency code uppercase constraints, zero-padding rules, or domain separation tags (`ARC-BIND-V1`) without a formal errata issuance linked to Master Canonical Errata **ARC-ERR-2026-001** (DOI `10.5281/zenodo.23069559`).
- **Dual-Plane Invariant Preservation:** Fixes must preserve strict isolation between the out-of-band streaming Data Plane (CAS storage) and the in-band Decision Plane (bounded snapshot memory evaluation).

---

## Scope & Bounded Claims

This repository is the canonical reference crate for deterministic institutional admission control and TOCTOU elimination. Its security guarantees are bounded strictly to:
- Deterministic, bounded admission parsing of static intent frames ($\le 4096\text{ B}$);
- Bit-exact constant-time payload binding verification;
- Generation of the 96-byte packed binary `ExecutionPermit`;
- Enforcing the closed seven-state settlement automaton.

The repository explicitly does **not** claim:
- High-level banking orchestration or core network routing;
- Long-term storage persistence of multi-gigabyte ISO 20022 clearing documents (delegated to out-of-band Content-Addressable Storage);
- Key management lifecycle outside the deterministic registry lookup closure.

---

## Response Timeline

We will acknowledge receipt within **48 hours** and work with you on a coordinated resolution and disclosure timeline.
