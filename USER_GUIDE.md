# BMsql User Guide

**Version:** v0.1.0 — Phase 0: Foundation

> This guide will be updated as BMsql progresses through each release phase.

This guide covers how to install, build, run, and verify BMsql at its current release. At v0.1.0, BMsql is a foundation project — it does not yet provide database commands, persistent storage, or SQL.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Building](#building)
5. [Running](#running)
6. [Testing](#testing)
7. [Release Builds](#release-builds)
8. [What Works in v0.1.0](#what-works-in-v010)
9. [Troubleshooting](#troubleshooting)
10. [Further Reading](#further-reading)

---

## Overview

BMsql v0.1.0 is the first release in a phased database engine project. This release focuses entirely on **Phase 0 — Foundation**:

- A working Rust/Cargo project
- A CLI entry point (`cargo run`)
- A test harness (`cargo test`)
- Stable project layout

There is no database file, no query language, and no persistent data in this version.

---

## Prerequisites

Install Rust using [rustup](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify your installation:

```bash
rustc --version
cargo --version
```

BMsql uses the **2024 edition** of Rust. Use a recent stable toolchain (1.85 or newer recommended).

---

## Installation

### From source

```bash
git clone <repository-url>
cd BMsql
```

No additional dependencies are required at v0.1.0. The project has zero external crate dependencies.

---

## Building

### Debug build (default)

```bash
cargo build
```

The compiled binary is placed at:

```text
target/debug/BMsql
```

### Check without producing a binary

Useful for quick compile verification:

```bash
cargo check
```

---

## Running

Start the CLI entry point:

```bash
cargo run
```

Or run the compiled binary directly:

```bash
./target/debug/BMsql
```

**Expected output:**

```text
Hello, world!
```

This confirms the project builds and the main entry point executes correctly.

---

## Testing

Run the test suite:

```bash
cargo test
```

At v0.1.0, the test harness is set up but no unit tests are defined yet. A successful run looks like:

```text
running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```


---

## Release Builds

For an optimized production binary:

```bash
cargo build --release
```

The release binary is placed at:

```text
target/release/BMsql
```

Release builds are faster at runtime but take longer to compile. For development, the debug profile is sufficient.

---

## What Works in v0.1.0

| Capability | Status |
|---|---|
| Project compiles with `cargo build` | Yes |
| CLI starts with `cargo run` | Yes |
| Test command runs with `cargo test` | Yes |
| Project structure defined | Yes |
| Version set to 0.1.0 in `Cargo.toml` | Yes |

---

## Troubleshooting

### `error: edition 2024 is unstable` or similar

Update your Rust toolchain:

```bash
rustup update stable
```

Ensure you are on Rust 1.85 or newer:

```bash
rustc --version
```

### `cargo: command not found`

Rust/Cargo is not installed or not on your `PATH`. Re-run the [rustup installer](https://rustup.rs/) and restart your shell.

### Build fails with permission errors on `target/`

Ensure you have write access to the project directory. Do not run `cargo` as root unless necessary.

### `Hello, world!` does not appear

Confirm you are in the project root (the directory containing `Cargo.toml`):

```bash
ls Cargo.toml
cargo run
```

---

## Further Reading

- [README.md](./README.md) — project overview and quick start
- [DATABASE_ENGINE_MISSION.md](./DATABASE_ENGINE_MISSION.md) — architecture and development protocol
