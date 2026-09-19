# BMsql

**v0.4.0 — Phase 3: Row Storage**

A relational database engine built in Rust from first principles. BMsql is developed in phased milestones — each release adds one layer of capability on top of a tested foundation.

> This README and the [User Guide](./USER_GUIDE.md) will be updated as the project progresses through each phase.

---

## Status

| | |
|---|---|
| **Version** | v0.4.0 |
| **Phase** | 3 — Row Storage |
| **Language** | Rust (2024 edition) |
| **Dependencies** | None |

At v0.4.0, BMsql builds on the pager with structured page layout and slotted record insertion: a 7-byte `PageHeader`, a `Slot` type (serialize/deserialize), and `Page::insert_record` that writes record bytes and a slot directory entry. The `Pager` still provides a bounded FIFO page cache, page I/O, size, page count, and allocation. The CLI prints the database name. There is no record read-by-slot API, tables/schemas, or SQL yet.

---

## Quick Start

**Prerequisites:** [Rust](https://rustup.rs/) 1.85+ (stable)

```bash
git clone <repository-url>
cd BMsql
cargo run
```

Expected output:

```text
BMsql
```

Run tests:

```bash
cargo test
```

For full installation, build options, and troubleshooting, see the [User Guide](./USER_GUIDE.md).

---

## What Is BMsql?

BMsql (**BM** = Builder / Mayur project identity, **sql** = relational query language) is a custom database engine built in Rust from first principles.

---

## Documentation

| Document | Description |
|---|---|
| [USER_GUIDE.md](./USER_GUIDE.md) | Install, build, run, test, and troubleshoot the current release |
| [DATABASE_ENGINE_MISSION.md](./DATABASE_ENGINE_MISSION.md) | Architecture, principles, and development protocol |

---

## Project Layout

```text
BMsql/
├── Cargo.toml                  # Package manifest (version 0.4.0, crate name: bmsql)
├── src/
│   ├── lib.rs                  # Library root (exports database, error, page, pager, storage)
│   ├── database.rs             # Database type; create/open database file helpers
│   ├── error.rs                # BmsqlError type
│   ├── page.rs                 # Page, PageHeader, Slot; insert_record + slot directory
│   ├── storage.rs              # DatabaseFile (open, write_page, read_page, size)
│   ├── pager.rs                # Pager (FIFO cache, read/write, size, page_count, allocate_page)
│   └── main.rs                 # CLI entry point
├── tests/
│   └── smoke_test.rs           # Integration test (database name)
├── README.md                   # This file
├── USER_GUIDE.md               # User-facing guide for the current release
└── DATABASE_ENGINE_MISSION.md  # Mission and development protocol
```

---

## License

License not yet specified.
