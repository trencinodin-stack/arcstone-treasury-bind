# AGENTIC OPERATIONAL POSTURE & REPOSITORY INVARIANTS

This repository implements **ARC-SPEC-2026-BIND-01 (REV-11)**: *Deterministic Financial Admission: Content-Addressable Binding, TOCTOU Elimination, and Operational Risk Invariants for Institutional Treasury Rails*.

Autonomous agentic producers, automated clearing delegates, and LLM-driven pipelines operating within or interfacing with this repository must adhere to the following normative invariants.

---

## 1. Zero-Heap Allocation Invariant ($M_{\mathrm{alloc}} = 0$)

- **Constraint:** All admission evaluation code, wire parsing, and constant-time commit barriers must operate strictly without runtime heap allocation.
- **Enforcement:** Crate is `#![no_std]`. Any PR or commit introducing dynamic allocation primitives (`alloc::vec`, `alloc::string`, `Box`, `Arc`, or system allocator linkage) will be rejected by CI.
- **Buffer Guarantees:** Intent frames must be evaluated within a fixed static memory snapshot of $S_{\max} \le 4096\text{ B}$, aligned to 64 bytes in TCM (Tightly Coupled Memory).

---

## 2. Double-Fetch Elimination (Atomic Isolation)

- **Vulnerability Class:** Time-of-Check to Time-of-Use (TOCTOU) payload divergence and parser asymmetry.
- **Rule:** Agentic actors must never read an intent field from untrusted shared memory, evaluate safety rules, and then re-read the field for execution dispatch.
- **Admissibility Barrier:** Single-pass wire parsing via `DecodedFrame::admit()` must execute against an immutable memory reference. State commitments require verification against the 32-byte SHA-256 payload digest in constant time (`verify_payload_binding`).

---

## 3. Strict Seven-State Reconciliation Automaton

- Financial state lifecycles are closed and discrete:
  $$\mathrm{INIT} \longrightarrow \mathrm{ADMITTED} \longrightarrow \mathrm{DISPATCH\_RESERVED} \longrightarrow \mathrm{COMMITTED}$$
  $$\mathrm{ADMITTED} \overset{\text{fault}}{\longrightarrow} \mathrm{DENIED}$$
  $$\mathrm{DISPATCH\_RESERVED} \overset{\mathrm{RJCT}}{\longrightarrow} \mathrm{TERMINAL\_REJECTED}$$
  $$\mathrm{DISPATCH\_RESERVED} \overset{\text{timeout}}{\longrightarrow} \mathrm{UNKNOWN\_RECONCILE} \overset{\tau > 120\text{s}}{\longrightarrow} \mathrm{LOCKED\_MANUAL}$$
- **Settlement Finality:** An operation commits if and only if a valid external clearing receipt confirms settlement (`pacs.002 ACSC`).
- **Intermediate States:** Intermediate statuses (`ACCP`, `PDNG`) must place the record in `UNKNOWN_RECONCILE`. Automatic retries are strictly forbidden without an active cryptographic lease ID.

---

## 4. Bounded Latency & Circuit Breaking

- **Evaluation Window:** $\tau_{\mathrm{eval}} \le 10.00\text{ ms}$.
- **Hardware Clamp:** Hard hardware override at $\tau_{\mathrm{override}} = 11.99\text{ ms}$. If execution exceeds $11.99\text{ ms}$, the memory gate asserts an immediate bus fault and forces state transition to `DENIED`.

---

## 5. Non-Model Statutory Posture ($C_{\mathrm{ops}} = 0$)

- Under **Federal Reserve SR 26-2** and **OCC Bulletin 2026-13**, admission decisions made by this codebase are classified as deterministic, non-model rules-based operations.
- Generative models or autonomous agents acting on behalf of institutional treasuries are strictly prohibited from modifying admission predicates, tolerance margins, or currency padding rules at runtime.
