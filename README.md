# Stream X-Machines in Rust
[<img alt="github" src="https://img.shields.io/badge/github-ruben1729/sxm-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/Ruben1729/sxm)
[<img alt="crates.io" src="https://img.shields.io/crates/v/sxm.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/sxm)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-sxm-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/sxm)

# Introduction
`sxm` is a Rust framework for modeling, visualizing, and testing systems using **Stream X-Machines (SXM)** and **Communicating Stream X-Machine Systems (CSXMS)**.

Unlike traditional Finite State Machines (FSMs) that only model control flow, Stream X-Machines integrate dynamic data structures (memory) with the state transition logic. This library provides a type-safe, trait-based approach to implementing these formal models in embedded systems and high-reliability software.

## Features

* **Formal Specification**: Define machines with typed Inputs ($\Sigma$), Outputs ($\Gamma$), States ($Q$), Memory ($M$), and Processing Functions ($\Phi$).
* **Model-Based Testing (MBT)**: automatically generate test suites to verify your implementation against the model.
   * **Logic Verification**: Implements the **W-Method** to generate conformance tests that prove the control logic is correct.
   * **Robustness Testing**: Generates **Input-Completeness** tests to ensure the system handles invalid or unexpected inputs gracefully without crashing.
   * **Symbolic Execution**: Uses memory-aware Breadth-First Search to discover data-dependent transitions (e.g., cracking a PIN code to test a locked state).
* **System Visualization**:
   * Generate **Graphviz (DOT)** diagrams of the internal state machine logic (the associated finite automaton).
   * Generate **System Context** diagrams for black-box integration views.
   * Visualize connected **CSXM** systems with routing logic (Matrix $E$).
* **Communicating Systems**: Support for connecting multiple SXMs (e.g., a Keypad and a Door) via adapter patterns to model complex, distributed behaviors.

## Reference

This library is an implementation of the formal verification and testing concepts presented in:

> **Testing Communicating Stream X-machines**
> *F. Ipate, T. Bălănescu, and G. Eleftherakis*
> Department of Computer Science and Mathematics, University of Pitesti, Romania & CITY College, Greece.

Specifically, it implements the testing conditions and methodologies described for deterministic stream X-machines and addresses the "Design for Test" conditions such as **Input-Completeness** and **Output-Distinguishability**.

## License

Licensed under either of

* Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.