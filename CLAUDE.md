## Context Database — Your Memory

Location: `docs/agent_context.db` (SQLite, schema v1). **Query before reasoning. Write as you learn.**

```sql
-- Bootstrap: run this first, then follow the protocol rows it returns.
SELECT title, content FROM knowledge WHERE category = 'protocol' ORDER BY title;
```

## Project: mx20022

Rust ISO 20022 financial message library. Cargo workspace with 7 crates.

## Hard Rules

- **Build:** `cargo check --workspace --all-features` — must pass before any commit
- **Test:** `cargo test --workspace --all-features`
- **Lint:** `cargo clippy --workspace --all-features -- -D warnings`
- **Format:** `cargo fmt --all -- --check`
- **Edition:** 2021, MSRV 1.75.0, License: Apache-2.0

## Workspace Layout

```
mx20022/
├── Cargo.toml              # Workspace root
├── rustfmt.toml            # imports_granularity = "Crate"
├── deny.toml               # cargo-deny: licenses, bans, advisories
├── schemas/                # ISO 20022 XSD source files (input data, not source)
│   └── {head,pacs,pain,camt}/
├── scripts/                # download-schemas.sh, regenerate.sh
├── testdata/               # Shared fixtures across crates
│   ├── xml/{head,pacs,pain,camt}/   # ISO 20022 XML messages
│   ├── mt/                 # SWIFT MT messages
│   ├── translation/        # Paired MT↔MX samples ({pair}/input.* + expected.*)
│   └── schemes/{fednow,sepa,cbpr}/  # Scheme-specific fixtures
├── examples/               # Runnable examples (cargo run --example)
├── benches/                # Workspace-level criterion benchmarks
├── docs/                   # Agent context DB, PRD, schema docs
└── crates/
    ├── mx20022-codegen/    # XSD → IR → Rust code generator
    │   └── src/{xsd/, ir/, emit/}
    ├── mx20022-model/      # Generated types + hand-written common
    │   └── src/{common/, generated/{head,pacs,pain,camt}/}
    ├── mx20022-parse/      # XML parsing and serialization (quick-xml + serde)
    ├── mx20022-validate/   # Schema + business rule + scheme validation
    │   └── src/{schema/, rules/, schemes/}
    ├── mx20022-translate/  # MT ↔ MX bidirectional translation
    │   └── src/{mt/{fields/}, mappings/}
    ├── mx20022-cli/        # CLI tool (clap)
    │   └── src/commands/
    └── mx20022/            # Umbrella re-export crate
```

### Key Boundaries
- **generated/ vs common/**: `mx20022-model/src/generated/` is machine-written (do not hand-edit). `src/common/` is hand-written shared types.
- **testdata/**: Shared across crates. Crate-specific fixtures go in each crate's `tests/fixtures/`.
- **schemas/**: XSD input files. Not Rust source. Used by codegen tool and scripts.

## Key Commands

```bash
# Database queries (use sqlite3)
sqlite3 docs/agent_context.db "SELECT id, title, status FROM items WHERE status = 'in_progress';"

# Find next available work
sqlite3 docs/agent_context.db "SELECT i.id, i.title, i.priority, it.name FROM items i JOIN iterations it ON i.iteration_id = it.id WHERE i.status = 'not_started' AND NOT EXISTS (SELECT 1 FROM item_dependencies d JOIN items dep ON d.depends_on = dep.id WHERE d.item_id = i.id AND dep.status != 'complete') ORDER BY it.id, i.priority, i.id LIMIT 10;"
```
