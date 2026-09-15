# BMsql

**v0.3.0 — Phase 2: Pager**

A relational database engine built in Rust from first principles. BMsql is developed in phased milestones — each release adds one layer of capability on top of a tested foundation.

> This README and the [User Guide](./USER_GUIDE.md) will be updated as the project progresses through each phase.

---

## Status

| | |
|---|---|
| **Version** | v0.3.0 |
| **Phase** | 2 — Pager |
| **Language** | Rust (2024 edition) |
| **Dependencies** | None |

At v0.3.0, BMsql provides a library crate with an in-memory `Database` type, a `BmsqlError` type, a fixed-size `Page` abstraction (4096 bytes), a `DatabaseFile` storage layer, and a `Pager` that wraps the file for page read/write, an in-memory page cache, file size, page count, and page allocation. Page data persists across reopen; reading a missing page returns `BmsqlError::Io`. The CLI prints the database name. There is no row storage or SQL yet.

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
├── Cargo.toml                  # Package manifest (version 0.3.0, crate name: bmsql)
├── src/
│   ├── lib.rs                  # Library root (exports database, error, page, pager, storage)
│   ├── database.rs             # Database type; create/open database file helpers
│   ├── error.rs                # BmsqlError type
│   ├── page.rs                 # Page type (PAGE_SIZE, Clone, page_offset, from_data)
│   ├── storage.rs              # DatabaseFile (open, write_page, read_page, size)
│   ├── pager.rs                # Pager (cache, read/write, size, page_count, allocate_page)
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
