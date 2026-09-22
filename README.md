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

At v0.4.0, BMsql builds on the pager with slotted row storage: a 7-byte `PageHeader`, a `Slot` type, `Page::insert_record` / `Page::read_record`, and slot directory rebuild on `from_data`. Storage layers are `DatabaseFile` → `PageManager` (allocate/read/write to disk) → `Pager` (FIFO cache over `PageManager`). Records survive page serialize/deserialize and database reopen. The CLI prints the database name. There are no tables/schemas or SQL yet.

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
│   ├── page.rs                 # Page, PageHeader, Slot; insert_record / read_record
│   ├── storage.rs              # DatabaseFile + PageManager (allocate/read/write/page_count)
│   ├── pager.rs                # Pager (FIFO cache over PageManager)
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
