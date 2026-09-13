# BMsql User Guide

**Version:** v4.0.0 — Phase 2: Pager

> This guide will be updated as BMsql progresses through each release phase.

This guide covers how to install, build, run, and verify BMsql at its current release. At v4.0.0, BMsql includes an in-memory `Database` type, a shared `BmsqlError` type, a fixed-size `Page` abstraction (`PAGE_SIZE` = 4096), page offset calculation, and a `DatabaseFile` storage type that reads and writes pages by ID at their file offset (returning `Result<..., BmsqlError>`). Page data persists across reopen; reading a missing page returns `BmsqlError::Io`. It does not yet provide row storage or SQL.

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
- A library crate with `Database`, `BmsqlError`, `Page`, and `DatabaseFile` types
- A `Page` type: fixed-size blocks (`PAGE_SIZE` = 4096) identified by `PageId` (`u64`), zero-initialized on creation, with `data` / `data_mut` and `from_data`
- `page_offset(page_id)` maps a page ID to a byte offset (`page_id * PAGE_SIZE`)
- `create_database_file` and `open_database_file` helpers for a database file on disk
- `DatabaseFile::open`, `write_page`, and `read_page` — seek to the page offset and read or write the full page (uses `PAGE_SIZE`; returns `BmsqlError` on failure)
- Page data persists after closing and reopening the database file
- Reading a page that is not present in the file returns `BmsqlError::Io`
- Tests for page layout, file I/O, seek, multi-page writes, page reads, persistence, and missing-page `BmsqlError::Io`
- A CLI entry point (`cargo run`) that prints the database name
- Stable project layout

There is no row encoding and no query language yet.

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

At v4.0.0, the library tests cover `Database`, `Page`, file create/open/read/write, seek to page offset, and `DatabaseFile` open/write/read (including a second page, persistence after reopen, and `BmsqlError::Io` when reading a missing page). One integration test checks the database name. A successful run looks like:

```text
running 19 tests
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
test storage::tests::database_file_can_be_opened ... ok
test storage::tests::database_file_can_write_page ... ok
test storage::tests::database_file_can_write_multiple_pages ... ok
test storage::tests::database_file_can_read_page ... ok
test storage::tests::database_file_can_read_second_page ... ok
test storage::tests::page_data_persists_after_reopening_database ... ok
test storage::tests::reading_missing_page_returns_error ... ok
test storage::tests::reading_missing_page_returns_io_error ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

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
| `Page` type (`PAGE_SIZE` = 4096, `PageId`, zero-init, `data_mut`, `from_data`) | Yes |
| Page offset from page ID (`page_offset`) | Yes |
| Create and open a database file (helpers) | Yes |
| Write and read bytes from a database file | Yes |
| Seek to a page offset and write bytes | Yes |
| `DatabaseFile::open` / `write_page` / `read_page` (return `BmsqlError`) | Yes |
| Write multiple pages to a database file | Yes |
| Page data persists after reopen | Yes |
| Reading a missing page returns `BmsqlError::Io` | Yes |
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
