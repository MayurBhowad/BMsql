# BMsql User Guide

**Version:** v0.4.0 — Phase 3: Row Storage

> This guide will be updated as BMsql progresses through each release phase.

This guide covers how to install, build, run, and verify BMsql at its current release. At v0.4.0, BMsql adds slotted row storage on top of the pager: a 7-byte `PageHeader`, a `Slot` type, `Page::insert_record` / `Page::read_record`, and slot directory rebuild when loading a page via `from_data`. Records survive page serialize/deserialize and database reopen. It does not yet provide tables/schemas or SQL.

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
- A `Page` type: fixed-size blocks (`PAGE_SIZE` = 4096) with a 7-byte header (`PAGE_HEADER_SIZE`) and `PAGE_DATA_SIZE` payload bytes; maintains an in-memory `slots` list
- `PageHeader`: `page_type` (u8), `record_count` (u16 LE), `free_space_offset` (u16 LE), `slot_directory_offset` (u16 LE; starts at `PAGE_SIZE` and grows down)
- `Slot`: `offset` and `length` (each u16); `SLOT_SIZE` = 4; `to_bytes` / `from_bytes`
- `Page::insert_record` appends record bytes, writes a slot directory entry, and updates header fields; rejects inserts that would collide with the slot directory (`BmsqlError::InvalidInput`) without mutating the page
- `Page::read_record(index)` returns the record bytes for a slot index (`None` if out of range)
- `Page::from_data` rebuilds the in-memory slot list from the on-disk slot directory using `record_count`
- `Page::slot_count` and `Page::slots(index)` for inspecting slots
- `Page::to_bytes` / `from_data` round-trip header, records, and slots
- `page_offset(page_id)` maps a page ID to a byte offset (`page_id * PAGE_SIZE`)
- `DatabaseFile` and `Pager` (page I/O, FIFO cache, size, page count, allocate); records persist after reopen
- A CLI entry point (`cargo run`) that prints the database name

There are no tables/schemas and no query language yet.

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

At v0.4.0, the library tests cover pager/storage plus slotted insert/read, space checks against the slot directory, and records/slots surviving serialize and reopen. One integration test checks the database name. A successful run looks like:

```text
running 67 tests
...
test page::tests::page_can_read_record ... ok
test page::tests::page_can_read_multiple_records ... ok
test page::tests::page_returns_none_for_invalid_record_index ... ok
test page::tests::page_slots_can_survive_serialization ... ok
test page::tests::page_records_survive_serialization ... ok
test page::tests::page_rejects_record_when_slot_does_not_fit ... ok
test page::tests::rejected_record_does_not_change_page ... ok
test storage::tests::page_records_persist_after_reopening_database ... ok
...

test result: ok. 67 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 1 test
test database_has_name ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

(Other page, storage, and pager tests omitted above as `...`.)

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
| `Slot` (offset + length; `to_bytes` / `from_bytes`) | Yes |
| `Page::insert_record` (append + slot directory; reject if no space) | Yes |
| `Page::read_record(index)` (bytes by slot index) | Yes |
| `Page::slot_count` / `Page::slots(index)` | Yes |
| Slots/records survive `to_bytes` / `from_data` and file reopen | Yes |
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
