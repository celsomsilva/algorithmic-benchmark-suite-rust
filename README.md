# Algorithmic Benchmark Suite (Rust)

> Java version of this repository: [Algorithmic Benchmark Suite (Java)](https://github.com/celsomsilva/algorithmic-benchmark-suite-java)
> R language version of this repository: [Algorithmic Benchmark Suite (R)](https://github.com/celsomsilva/algorithmic-benchmark-suite-R)



A **systems-level framework** for implementing and benchmarking classical algorithms in **Rust**, with emphasis on **explicit control, memory safety, correctness, and
predictable performance**.

This repository contains modern Rust implementations of classical algorithms, designed to be **testable, benchmarkable, and reproducible**, while making data ownership, mutation, and execution costs explicit.

The implementations are conceptually inspired by legacy Pascal and C code written in the early 2000s, preserved separately as historical references.

The idea is simple and consistent across languages:

* study classical algorithms through explicit implementations,
* reimplement them without hiding logic behind abstractions,
* validate correctness with automated tests,
* measure performance with reproducible benchmarks.

This Rust version represents the **systems-level layer** of the Algorithmic Benchmark Suite.

---

## Motivation

After implementing and benchmarking classical algorithms in Java (engineering focus) and R (statistical analysis), Rust naturally emerged as the next step.

Rust provides:

* explicit control without a garbage collector,
* strong compile-time guarantees,
* predictable performance characteristics,
* memory safety enforced by the type system.

Rather than treating Rust as a performance-oriented novelty, this project uses it as a **clarifying language**, where ownership, borrowing, and mutation are explicit design decisions rather than hidden runtime behavior.

---

## What’s Inside

* Explicit Rust implementations of classical algorithms
* Ownership-aware data structures
* Automated correctness tests
* Simple, reproducible benchmarks
* Technical notes documenting design and performance trade-offs

---

## Legacy Code Reference

This repository includes a minimal `selected_algorithms/` directory containing only the Pascal and C source files that serve as direct conceptual references.

The complete, unmodified and cleaned legacy archive is preserved separately at:

https://github.com/celsomsilva/algorithmic-legacy-code

All legacy files are included strictly for historical and conceptual reference. No algorithmic logic was altered.

---

## Repository Layout

## Repository Structure

```
algorithmic-benchmark-suite-rust/
  Cargo.toml
  README.md
  LICENSE
  .gitignore

  selected_algorithms/        # Original Pascal / C source files

  modern_rust/
    src/
      lib.rs
      search/
      sort/
      structures/

    tests/

    benchmarks/

  docs/
```

---

## Getting Started

### Requirements

* Rust (stable toolchain)
* Cargo

Install Rust via [https://rustup.rs](https://rustup.rs) if needed.

---

### Building the Project

To compile the library and run all automated tests:

```bash
cargo build
cargo test
```

---

### Running Benchmarks


To run benchmarks:

```bash
cargo run --release --bin <benchmark_name>
```

---

## Author

This project was developed by an engineer and data scientist with a background in:

* Postgraduate degree in **Data Science and Analytics (USP)**
* Bachelor's degree in **Computer Engineering (UERJ)**
* Special interest in statistical models, interpretability, and applied AI
* Strong interest in algorithmic reasoning, correctness, and performance evaluation

---

## Contact

* [LinkedIn](https://linkedin.com/in/celso-m-silva)
* Or open an [issue](https://github.com/celsomsilva/algorithmic-benchmark-suite-rust/issues)

