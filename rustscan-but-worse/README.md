# rustscan-but-worse

**rustscan-but-worse** is an experimental TCP port scanner written in Rust.

The project is designed as a hands-on introduction to Rust development, with a focus on networking, error handling, concurrency, and systems programming.

Inspired by tools such as RustScan and Nmap, the objective is to build a simple port scanner from scratch while understanding the underlying concepts and implementation decisions.

This is an educational project, not an attempt to compete with established network scanning tools.

## Project Goals

The scanner will be developed incrementally, introducing new functionality as the underlying Rust concepts are learned.

Planned development stages include:

1. **Fundamentals** — Variables, types, functions, and program structure.
2. **TCP Networking** — Establishing connections and identifying open ports.
3. **Port Ranges** — Scanning multiple ports on a specified target.
4. **Error Handling** — Managing connection failures, timeouts, and invalid input.
5. **Command-Line Interface** — Accepting targets and scanning options through CLI arguments.
6. **Concurrency** — Exploring parallel scanning to improve performance.
7. **Optimization** — Improving code quality, efficiency, and reliability.

## Getting Started

### Prerequisites

- Rust
- Cargo

Install the Rust toolchain using [rustup](https://rustup.rs/) if necessary.

### Building

From the project directory, run:

`cargo build`

### Running

Run the current development build using:

`cargo run`

The scanner is in early development. Command-line options and scanning functionality will be introduced as the project progresses.

## Scope

The initial implementation will focus on TCP connect scanning using Rust's standard library.

More advanced functionality may be explored later, but the primary objective is to understand the implementation rather than rely on existing scanning libraries.

## Disclaimer

This project is intended for educational purposes and authorized network testing only.

Only scan systems and networks you own or have explicit permission to assess.

## Status

**Early Development**

Functionality will be added incrementally as new Rust concepts are introduced and understood.
