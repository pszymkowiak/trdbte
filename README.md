# trdbte

A from-scratch rewrite of SQLite in Rust — targeting **superior performance**.

## Goal

Build a SQL database engine in Rust that beats SQLite on core operations (inserts, selects, scans, updates, deletes) while maintaining correctness and crash safety.

## Rust Advantages to Exploit

- Zero-copy page deserialization
- Memory-mapped I/O (mmap)
- SIMD for table scans and comparisons
- Lock-free concurrent readers (no global mutex like SQLite)
- Arena allocators for query execution
- Better LLVM inlining/optimization

## Target Architecture

```
src/
├── lib.rs              — Public API (Database::open, execute, query)
├── types.rs            — Value types (Integer, Float, Text, Blob, Null)
├── storage/
│   ├── pager.rs        — Page-based I/O with mmap
│   ├── btree.rs        — B-tree implementation
│   └── wal.rs          — Write-Ahead Log for crash safety
├── sql/
│   ├── tokenizer.rs    — SQL tokenizer
│   ├── parser.rs       — SQL parser → AST
│   └── planner.rs      — AST → execution plan
└── executor/
    └── mod.rs          — Execute plans against storage

benches/
└── vs_sqlite.rs        — Criterion benchmarks vs rusqlite (SQLite)

tests/
└── correctness.rs      — Functional tests comparing results with rusqlite
```

## SQL Features to Support

- CREATE TABLE / DROP TABLE
- INSERT INTO ... VALUES (...)
- SELECT ... FROM ... WHERE ... ORDER BY ... LIMIT
- UPDATE ... SET ... WHERE ...
- DELETE FROM ... WHERE ...

## Validation

Every feature MUST include:
1. **Functional tests** — same queries on trdbte and rusqlite, same results
2. **Benchmarks** — criterion comparisons vs rusqlite, must show superior performance

## License

MIT
