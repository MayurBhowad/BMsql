# BMsql — Database Engine Mission

> **Project:** BMsql  
> **Current Version:** v0.1.0  
> **Status:** Foundation  
> **Versioning Strategy:** Semantic Versioning (MAJOR.MINOR.PATCH)


## Versioning

BMsql follows Semantic Versioning:

```text
MAJOR.MINOR.PATCH
```

- **MAJOR** — Major architectural or compatibility milestone.
- **MINOR** — New database capability or completed feature milestone.
- **PATCH** — Bug fixes, internal improvements, and backward-compatible corrections.

Initial project version:

```text
v0.1.0 — Foundation
```

## Release Path

| Version | Milestone | Meaning |
|---|---|---|
| v0.1.0 | Foundation | Rust project, CLI, tests, project structure |
| v0.2.0 | Persistent Storage | Database file and persistent key-value records |
| v0.3.0 | Pager | Fixed-size pages and page allocation |
| v0.4.0 | Row Storage | Structured row encoding and scanning |
| v0.5.0 | Tables | Tables, schemas, and typed values |
| v0.6.0 | Indexes | Persistent indexing and tree-based lookup |
| v0.7.0 | SQL Frontend | Tokenizer, parser, and AST |
| v0.8.0 | Query Engine | SQL execution and query processing |
| v0.9.0 | Optimization | Basic planning and index selection |
| v0.10.0 | Transactions | BEGIN, COMMIT, ROLLBACK |
| v0.11.0 | Recovery | Write-ahead logging and crash recovery |
| v0.12.0 | Buffer Pool | In-memory page caching |
| v0.13.0 | Concurrency | Locks and concurrent operations |
| v0.14.0 | Server | TCP database server |
| v1.0.0 | First Stable Engine | Complete first stable BMsql database engine |

## Release Rules

A version is released only when:

1. The milestone exit criteria are complete.
2. Relevant tests pass.
3. The previous functionality still works.
4. The version number is updated in `Cargo.toml`.
5. The change is documented in the changelog.

## Changelog Format

Maintain a `CHANGELOG.md` using:

```text
## [v0.1.0]

### Added
- Initial BMsql project structure

### Changed
- ...

### Fixed
- ...
```

---

## Mission

Build **BMsql**, a relational database engine in Rust from first principles.

The goal is not to copy MySQL source code or reproduce every MySQL feature. The goal is to progressively build the core systems behind a modern relational database so the project evolves from simple persistent storage into a SQL-capable, indexed, transactional database engine.

## BMsql Identity

```text
BMsql
|
+-- BM = Builder / Mayur project identity
+-- sql = Relational query language and database focus
```

BMsql is a custom database engine inspired by the architecture and concepts behind systems such as MySQL, but it is not intended to be a source-code clone.

## Primary Objective

Build a database engine that can eventually support:

- Persistent database files
- Tables and schemas
- Rows and typed values
- SQL commands
- Query parsing and execution
- Indexes
- Transactions
- Write-ahead logging
- Concurrency control

## Core Rule

Do not jump ahead.

Every phase must be completed, tested, and understood before moving to the next phase.

When a new idea appears:

1. Compare it with the current phase.
2. If it is required for the current phase, implement it.
3. If it belongs to a later phase, record it in the backlog.
4. Do not interrupt the current milestone.

The project progresses through dependency, not excitement.

---

# Architecture Path

```text
Client
  |
  v
SQL Input
  |
  v
Tokenizer
  |
  v
Parser
  |
  v
AST
  |
  v
Query Planner / Executor
  |
  v
Storage Engine
  |
  +--> Tables
  |
  +--> Indexes
  |
  +--> Transactions
  |
  v
Pager / Buffer Pool
  |
  v
Database Files
```

---

# Project Principles

## 1. Build from first principles

Prefer understanding the underlying mechanism before using an abstraction.

Example:

```text
Understand pages
    before
Buffer pool
    before
B+ Tree storage
```

## 2. Keep the first implementation simple

The first version of every component should be intentionally small.

Correctness comes before optimization.

## 3. Test every layer

Every component should have tests before depending components are added.

```text
Storage tests
    ->
Table tests
    ->
Index tests
    ->
SQL tests
```

## 4. Measure before optimizing

Do not add complexity without evidence.

## 5. Prefer internal clarity

Readable internal design is more valuable than copying production complexity too early.

---

# Technology Decisions

## Language

Rust

## Initial Runtime Model

Single-process command-line database.

## Storage

Local files.

## Networking

Not part of the early phases.

## SQL Compatibility

A small custom SQL subset inspired by relational databases.

Do not attempt full MySQL compatibility.

---

# Milestone Roadmap

# Phase 0 — Foundation

## Objective

Create the Rust project and establish the engineering environment.

## Build

- Cargo project
- Project structure
- Error handling strategy
- Logging strategy if required
- Test setup
- Command-line entry point

## Rust concepts introduced

- Cargo
- Modules
- Structs
- Enums
- Result
- Option
- Ownership
- Borrowing
- Lifetimes only when necessary

## Exit criteria

- Project builds successfully
- Test command works
- Basic CLI starts
- Project structure is stable enough for Phase 1

---

# Phase 1 — Persistent Key-Value Storage

## Objective

Persist data to disk and retrieve it after restart.

## Build

- Open database file
- Create database file if missing
- Write records
- Read records
- Restart process and recover stored data

## Concepts

- Files
- Bytes
- Serialization
- File offsets
- Persistent storage

## Example

```text
SET name Mayur
GET name
```

## Exit criteria

- Data survives process restart
- Basic corruption handling exists
- Tests cover read/write behavior

---

# Phase 2 — Pager and Fixed-Size Pages

## Objective

Stop thinking in terms of entire files and start managing database pages.

## Build

- Fixed-size page abstraction
- Page identifiers
- Read page by ID
- Write page by ID
- Allocate new pages

## Concepts

- Disk blocks
- Page layout
- Random access
- File offsets

## Target model

```text
Database File
|
+-- Page 0
+-- Page 1
+-- Page 2
+-- Page 3
```

## Exit criteria

- Pages can be allocated
- Pages can be read
- Pages can be modified and written back
- Data survives restart

---

# Phase 3 — Row Storage

## Objective

Store structured records inside pages.

## Build

- Row representation
- Encode rows into bytes
- Decode bytes into rows
- Insert rows
- Scan rows

## Concepts

- Binary encoding
- Record layout
- Variable and fixed-size data
- Offsets

## Exit criteria

- Rows are persisted
- Rows can be read back correctly
- Full table scan is possible

---

# Phase 4 — Tables and Schemas

## Objective

Introduce relational structure.

## Build

- Database metadata
- Table metadata
- Column definitions
- Data types
- Row validation
- Table creation

## Initial types

- INTEGER
- TEXT
- BOOLEAN

## Example

```text
users
+----+--------+------+
| id | name   | age  |
+----+--------+------+
| 1  | Mayur  | 30   |
+----+--------+------+
```

## Exit criteria

- Multiple tables are supported
- Schema is persisted
- Invalid rows are rejected

---

# Phase 5 — Indexes

## Objective

Avoid scanning every row for every lookup.

## Build order

1. Simple in-memory index
2. Persistent index
3. Tree-based index
4. B+ Tree

## Concepts

- Lookup complexity
- Tree traversal
- Node splitting
- Sorted keys
- Range queries

## Target

```text
             [50]
            /    \
        [20]      [80]
       /   \      /   \
    [10]  [30]  [60]  [90]
```

## Exit criteria

- Indexed lookup is faster than full scan for suitable queries
- Index survives restart
- Insert and lookup tests pass

---

# Phase 6 — SQL Tokenizer

## Objective

Convert SQL text into tokens.

## Example

```sql
SELECT name FROM users WHERE id = 1;
```

Becomes conceptually:

```text
SELECT
IDENT(name)
FROM
IDENT(users)
WHERE
IDENT(id)
EQUALS
NUMBER(1)
SEMICOLON
```

## Exit criteria

- Valid SQL is tokenized correctly
- Invalid tokens produce useful errors

---

# Phase 7 — SQL Parser and AST

## Objective

Convert tokens into a structured representation.

## Example

```sql
SELECT name FROM users WHERE id = 1;
```

Conceptually becomes:

```text
SelectStatement
|
+-- columns: [name]
+-- table: users
+-- filter:
      id = 1
```

## Initial SQL subset

- CREATE TABLE
- INSERT
- SELECT
- DELETE
- UPDATE

## Exit criteria

- Statements produce ASTs
- Invalid syntax produces useful errors
- Parser tests cover valid and invalid input

---

# Phase 8 — Query Executor

## Objective

Execute parsed SQL against storage.

## Build

- Table scan
- Filter
- Projection
- Insert execution
- Update execution
- Delete execution

## Example

```sql
SELECT name
FROM users
WHERE age > 25;
```

Execution pipeline:

```text
Scan
  |
Filter
  |
Projection
  |
Result
```

## Exit criteria

- SQL statements modify real persistent data
- SELECT returns correct results
- Errors do not corrupt database state

---

# Phase 9 — Query Planning and Optimization

## Objective

Choose better execution paths.

## Build

- Decide between table scan and index lookup
- Basic execution cost estimation
- Explain query plan

## Example

```text
SELECT ... WHERE id = 10

Possible paths:

1. Full table scan
2. Index lookup
```

Choose the appropriate path.

## Exit criteria

- Indexed queries can use indexes automatically
- Query plan can be inspected
- Optimizations are validated with benchmarks

---

# Phase 10 — Transactions

## Objective

Allow multiple operations to behave as one unit.

## Build

- BEGIN
- COMMIT
- ROLLBACK

## Example

```sql
BEGIN;

UPDATE accounts
SET balance = balance - 100
WHERE id = 1;

UPDATE accounts
SET balance = balance + 100
WHERE id = 2;

COMMIT;
```

## Concepts

- Atomicity
- Consistency
- Isolation
- Durability

## Exit criteria

- Commit makes changes durable
- Rollback restores previous state
- Failed transactions do not leave partial changes

---

# Phase 11 — Write-Ahead Logging

## Objective

Recover from crashes.

## Rule

```text
Write log
    before
Write database pages
```

## Build

- WAL file
- Transaction log records
- Recovery process
- Redo / undo strategy as appropriate for the implementation

## Exit criteria

- Simulated crash can recover consistent state
- WAL tests validate recovery scenarios

---

# Phase 12 — Buffer Pool

## Objective

Reduce unnecessary disk access.

## Build

- In-memory page cache
- Page replacement strategy
- Dirty pages
- Flush mechanism

## Concepts

- Cache hits
- Cache misses
- LRU or similar replacement policy
- Memory pressure

## Exit criteria

- Frequently accessed pages avoid repeated disk reads
- Dirty pages are persisted correctly

---

# Phase 13 — Concurrency

## Objective

Support multiple operations safely.

## Build progressively

1. Database-level locking
2. Table-level locking
3. Finer-grained locking
4. Deadlock handling
5. Isolation behavior

## Concepts

- Race conditions
- Locks
- Deadlocks
- Isolation levels

## Exit criteria

- Concurrent operations do not corrupt data
- Conflicting writes are controlled
- Deadlocks are detected or prevented

---

# Phase 14 — Networking

## Objective

Turn the database engine into a server.

## Build

```text
Client
  |
TCP
  |
Database Server
```

## Build

- TCP listener
- Request protocol
- Session handling
- Multiple clients

## Exit criteria

- External clients can connect
- Commands are executed through the server

---

# Current Release Scope

```text
BMsql v0.1.0
Phase: Foundation
```

# Current Scope

## Now

Only work on:

# Phase 0 — Foundation

Current objective:

```text
Create a clean Rust database project
that can become the foundation for
the storage engine.
```

Do not implement:

- SQL
- B+ Trees
- Transactions
- WAL
- Networking
- Concurrency

Those belong to later phases.

---

# Decision Log

Record important architecture decisions here.

## Decision 001

**Language:** Rust

**Reason:** Low-level systems control, strong performance, memory safety, and suitable concurrency primitives.

## Decision 002

**Initial target:** A custom relational database engine.

**Reason:** Full MySQL compatibility would add unnecessary protocol and feature complexity before the core database internals are built.

## Decision 003

**Development strategy:** Build vertically in small milestones.

**Reason:** Each layer should remain testable and understandable.

---

# Backlog

Ideas that should not interrupt the current phase.

- Secondary indexes
- JOIN
- Aggregations
- Query optimizer improvements
- MVCC
- Replication
- Authentication
- MySQL wire protocol compatibility
- Storage compression
- Full-text search
- Query cache

---

# Working Protocol for Cursor

Before writing code:

1. Read this file.
2. Identify the current phase.
3. Check the exit criteria.
4. Implement only the smallest required next step.
5. Add tests.
6. Run tests.
7. Update progress.

Do not introduce future-phase architecture unless it is required by the current milestone.

When blocked:

```text
Understand the current layer
before
adding another layer.
```

---

# Version Progress

- [x] v0.1.0 — Mission and versioning defined
- [ ] v0.1.0 — Foundation implementation complete
- [ ] v0.2.0 — Persistent storage complete
- [ ] v0.3.0 — Pager complete
- [ ] v0.4.0 — Row storage complete
- [ ] v0.5.0 — Tables complete
- [ ] v0.6.0 — Indexes complete
- [ ] v0.7.0 — SQL frontend complete
- [ ] v0.8.0 — Query engine complete
- [ ] v0.9.0 — Optimization complete
- [ ] v0.10.0 — Transactions complete
- [ ] v0.11.0 — Recovery complete
- [ ] v0.12.0 — Buffer pool complete
- [ ] v0.13.0 — Concurrency complete
- [ ] v0.14.0 — Server complete
- [ ] v1.0.0 — First stable BMsql engine

---

# Progress Tracker

- [x] Project mission defined
- [x] Language selected: Rust
- [ ] Phase 0 complete
- [ ] Phase 1 complete
- [ ] Phase 2 complete
- [ ] Phase 3 complete
- [ ] Phase 4 complete
- [ ] Phase 5 complete
- [ ] Phase 6 complete
- [ ] Phase 7 complete
- [ ] Phase 8 complete
- [ ] Phase 9 complete
- [ ] Phase 10 complete
- [ ] Phase 11 complete
- [ ] Phase 12 complete
- [ ] Phase 13 complete
- [ ] Phase 14 complete

---

# Definition of Done

The mission is complete when the database can:

```text
Create a database
    ->
Create tables
    ->
Persist rows
    ->
Restart safely
    ->
Execute SQL
    ->
Use indexes
    ->
Run transactions
    ->
Recover after crashes
    ->
Handle concurrent operations
```

At that point, the project is a real database engine.
