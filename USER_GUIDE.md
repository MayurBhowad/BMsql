# BMsql User Guide

**Version:** v4.0.0 — Phase 2: Pager

> This guide will be updated as BMsql progresses through each release phase.

This guide covers how to install, build, run, and verify BMsql at its current release. At v4.0.0, BMsql includes an in-memory `Database` type, a shared `BmsqlError` type, a fixed-size `Page` abstraction, page offset calculation, database file create/open with byte and page writes, and seeking to a page offset. It does not yet provide a dedicated page-by-ID read/write API, row storage, or SQL.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Building](#building)
5. [Running](#running)
6. [Testing](#testing)
7. [Release Builds](#release-builds)
8. [What Works in v4.0.0](#what-works-in-v400)
9. [Troubleshooting](#troubleshooting)
10. [Further Reading](#further-reading)

---

## Overview

BMsql v4.0.0 adds pager primitives on top of the foundation:

- A working Rust/Cargo project (crate name: `bmsql`, version `4.0.0`)
- A library crate with `Database`, `BmsqlError`, and `Page` types
- A `Page` type: 4096-byte blocks identified by `PageId` (`u64`), zero-initialized on creation
- `Page::page_offset` maps a page ID to a byte offset (`page_id * 4096`)
- `create_database_file` and `open_database_file` for a database file on disk
- Seeking to a page offset (`Page::page_offset`) and writing bytes there
- Tests that write and read bytes, write a full page (4096 bytes), and seek to a page offset
- A CLI entry point (`cargo run`) that prints the database name
- Stable project layout

There is no dedicated page-by-ID read/write API, no row encoding, and no query language yet.

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

No additional dependencies are required at v4.0.0. The project has zero external crate dependencies.

---

## Building

### Debug build (default)

```bash
cargo build
```

The compiled binary is placed at:

```text
target/debug/bmsql
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
./target/debug/bmsql
```

**Expected output:**

```text
BMsql
```

This confirms the project builds and the `Database` type is wired into the CLI.

---

## Testing

Run the test suite:

```bash
cargo test
```

At v4.0.0, the library tests cover `Database`, `Page` (size, id, zero bytes, offset), database file create/open, byte write/read, writing a page to a file, and seeking to a page offset. One integration test checks the database name. A successful run looks like:

```text
running 11 tests
test database::tests::database_can_be_created ... ok
test page::tests::page_has_correct_size ... ok
test page::tests::new_page_contains_zeroes ... ok
test page::tests::page_has_correct_id ... ok
test page::tests::page_id_has_correct_offset ... ok
test page::tests::database_file_can_be_created ... ok
test page::tests::existing_database_file_can_be_opened ... ok
test page::tests::database_file_can_store_bytes ... ok
test page::tests::database_file_can_read_bytes ... ok
test page::tests::page_can_be_written_to_database_file ... ok
test page::tests::file_can_seek_to_page_offset ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 1 test
test database_has_name ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

---

## Release Builds

For an optimized production binary:

```bash
cargo build --release
```

The release binary is placed at:

```text
target/release/bmsql
```

Release builds are faster at runtime but take longer to compile. For development, the debug profile is sufficient.

---

## What Works in v4.0.0

| Capability | Status |
|---|---|
| Project compiles with `cargo build` | Yes |
| CLI starts with `cargo run` | Yes |
| Test command runs with `cargo test` | Yes |
| `Database` type (in-memory, name only) | Yes |
| `BmsqlError` type (`Io`, `InvalidInput`) | Yes |
| `Page` type (4096 bytes, `PageId`, zero-init) | Yes |
| Page offset from page ID (`page_id * 4096`) | Yes |
| Create and open a database file | Yes |
| Write and read bytes from a database file | Yes |
| Write a full page (4096 bytes) to a database file | Yes |
| Seek to a page offset and write bytes | Yes |
| Dedicated page-by-ID read/write API | No |
| Row storage or SQL | No |
| Version set to 4.0.0 in `Cargo.toml` | Yes |

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

### `BMsql` does not appear

Confirm you are in the project root (the directory containing `Cargo.toml`):

```bash
ls Cargo.toml
cargo run
```

You should see `BMsql` printed to stdout.

---

## Further Reading

- [README.md](./README.md) — project overview and quick start
- [DATABASE_ENGINE_MISSION.md](./DATABASE_ENGINE_MISSION.md) — architecture and development protocol
