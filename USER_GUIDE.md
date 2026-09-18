# BMsql User Guide

**Version:** v0.4.0 — Phase 3: Row Storage

> This guide will be updated as BMsql progresses through each release phase.

This guide covers how to install, build, run, and verify BMsql at its current release. At v0.4.0, BMsql adds structured page layout and basic record insertion on top of the pager: a 7-byte `PageHeader`, a `Slot` type (`SLOT_SIZE` = 4), and `Page::insert_record`. It does not yet provide a record scan/read-by-slot API, tables/schemas, or SQL.

---

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Building](#building)
5. [Running](#running)
6. [Testing](#testing)
7. [Release Builds](#release-builds)
8. [What Works in v0.4.0](#what-works-in-v040)
9. [Troubleshooting](#troubleshooting)
10. [Further Reading](#further-reading)

---

## Overview

BMsql v0.4.0 is the **Row Storage** milestone:

- A working Rust/Cargo project (crate name: `bmsql`, version `0.4.0`)
- A library crate with `Database`, `BmsqlError`, `Page`, `PageHeader`, `Slot`, `DatabaseFile`, and `Pager` types
- A `Page` type: fixed-size blocks (`PAGE_SIZE` = 4096) with a 7-byte header (`PAGE_HEADER_SIZE`) and `PAGE_DATA_SIZE` payload bytes
- `PageHeader`: `page_type` (u8), `record_count` (u16 LE), `free_space_offset` (u16 LE), `slot_directory_offset` (u16 LE)
- `Slot`: `offset` and `length` (each u16); `SLOT_SIZE` = 4
- `Page::insert_record` appends record bytes into free space, increments `record_count`, and advances `free_space_offset`; returns `BmsqlError::InvalidInput` if the record does not fit
- `Page::to_bytes` / `from_data` round-trip the on-disk layout (header + data)
- `page_offset(page_id)` maps a page ID to a byte offset (`page_id * PAGE_SIZE`)
- `DatabaseFile` and `Pager` from v0.3.0 (page I/O, FIFO cache, size, page count, allocate)
- A CLI entry point (`cargo run`) that prints the database name

There is no record read-by-slot or scan API, no tables/schemas, and no query language yet.

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

No additional dependencies are required at v0.4.0. The project has zero external crate dependencies.

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

At v0.4.0, the library tests cover pager/storage behavior plus page header layout, slots, and record insertion. One integration test checks the database name. A successful run looks like:

```text
running 50 tests
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
test page::tests::page_header_has_correct_values ... ok
test page::tests::page_header_has_correct_size ... ok
test page::tests::page_from_data_preserves_data_after_header ... ok
test page::tests::page_total_size_is_4096_bytes ... ok
test page::tests::page_header_can_be_serialized ... ok
test page::tests::page_can_be_serialized_to_4096_bytes ... ok
test page::tests::page_header_can_be_deserialized ... ok
test page::tests::page_from_data_preserves_header ... ok
test page::tests::page_can_insert_record ... ok
test page::tests::page_can_insert_multiple_records ... ok
test page::tests::page_rejects_record_when_not_enough_space ... ok
test page::tests::page_inserts_records_after_existing_data ... ok
test page::tests::slot_has_correct_values ... ok
test storage::tests::... ... ok
test pager::tests::... ... ok

test result: ok. 50 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 1 test
test database_has_name ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

(Storage and pager tests from v0.3.0 remain; names abbreviated above as `...`.)

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

## What Works in v0.4.0

| Capability | Status |
|---|---|
| Project compiles with `cargo build` | Yes |
| CLI starts with `cargo run` | Yes |
| Test command runs with `cargo test` | Yes |
| Pager / `DatabaseFile` (I/O, FIFO cache, size, allocate) | Yes |
| `Page` (`PAGE_SIZE` = 4096, `to_bytes` / `from_data`) | Yes |
| `PageHeader` (7 bytes, including `slot_directory_offset`) | Yes |
| `Slot` (offset + length, `SLOT_SIZE` = 4) | Yes |
| `Page::insert_record` (append; reject if no space) | Yes |
| Record read-by-slot / scan API | No |
| Tables, schemas, or SQL | No |
| Version set to 0.4.0 in `Cargo.toml` | Yes |

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
