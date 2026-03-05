# mx20022

Rust library for ISO 20022 financial messages. Parse, validate, and translate
between SWIFT MT and ISO 20022 MX formats.

[![CI](https://github.com/socrates8300/mx20022/actions/workflows/ci.yml/badge.svg)](https://github.com/socrates8300/mx20022/actions/workflows/ci.yml)
[![License: Apache-2.0 OR MIT](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](#license)
[![MSRV: 1.75.0](https://img.shields.io/badge/MSRV-1.75.0-orange.svg)](https://releases.rs/docs/1.75.0/)

## Features

- **13 message schemas** — pacs.008, pacs.002, pacs.004, pacs.009, pacs.028,
  pain.001, pain.002, pain.013, camt.053, camt.054, camt.056, camt.029,
  head.001
- **Strongly typed** — every element, attribute, and enum variant is a named
  Rust type with serde Serialize/Deserialize
- **Builder pattern** — all struct types expose a builder with required-field
  validation
- **XML round-trip** — parse and serialize ISO 20022 XML via quick-xml + serde
- **MT parser** — SWIFT FIN message parser for MT103, MT202, MT940
- **Bidirectional translation** — MT103 ↔ pacs.008, MT202 ↔ pacs.009,
  MT940 ↔ camt.053
- **Validation** — IBAN, BIC, LEI, currency, amount format, plus scheme-level
  rules for FedNow, SEPA, and CBPR+
- **XSD constraint validation** — every generated newtype validates pattern,
  length, and range constraints at construction (`TryFrom<String>`) and on
  full message trees via the `Validatable` trait
- **Code generation** — XSD → intermediate representation → Rust source

## Workspace Crates

| Crate | Description |
|-------|-------------|
| `mx20022` | Umbrella re-export (use this as your dependency) |
| `mx20022-model` | Generated types + hand-written common types |
| `mx20022-parse` | XML deserialization and serialization |
| `mx20022-validate` | Schema rules, business rules, scheme validators |
| `mx20022-translate` | MT ↔ MX bidirectional translation |
| `mx20022-codegen` | XSD → Rust code generator |
| `mx20022-cli` | Command-line tool |

## Quick Start

### Requirements

- Rust 1.75.0 or later (stable)
- [just](https://github.com/casey/just) (optional, for task runner shortcuts)

### Add the dependency

```toml
[dependencies]
mx20022 = "0.1"
```

Feature flags on `mx20022-model` control which message families are compiled.
The umbrella crate enables all of them.

| Feature | Message families | Default |
|---------|-----------------|---------|
| `pacs` | pacs.002, pacs.004, pacs.008, pacs.009, pacs.028 | yes |
| `pain` | pain.001, pain.002, pain.013 | no |
| `camt` | camt.029, camt.053, camt.054, camt.056 | no |
| `head` | head.001 | no |
| `all` | all of the above | no |

### Parse an ISO 20022 message

```rust
use mx20022::model::generated::pacs::pacs_008_001_13::Document;
use mx20022::parse::{de, envelope};

let xml = std::fs::read_to_string("message.xml")?;

// Detect the message type from the XML namespace
let msg_id = envelope::detect_message_type(&xml)?;
println!("Message type: {}", msg_id.dotted()); // e.g. "pacs.008.001.13"

// Deserialize to a strongly-typed struct
let doc: Document = de::from_str(&xml)?;
let hdr = &doc.fi_to_fi_cstmr_cdt_trf.grp_hdr;
println!("Message ID: {}", hdr.msg_id.0);
println!("Transactions: {}", hdr.nb_of_txs.0);
```

### Validate against FedNow rules

```rust
use mx20022::validate::schemes::{fednow::FedNowValidator, SchemeValidator};

let validator = FedNowValidator::new(); // $500K default limit
let result = validator.validate(&xml, "pacs.008.001.13");

if result.is_valid() {
    println!("Valid ({} warnings)", result.warning_count());
} else {
    for err in &result.errors {
        println!("[{}] {} — {}", err.rule_id, err.path, err.message);
    }
}
```

### Validate XSD constraints on a message

```rust
use mx20022::model::common::validate::IsoMessage;
use mx20022::model::generated::pacs::pacs_008_001_13::Document;
use mx20022::parse::de;

let xml = std::fs::read_to_string("message.xml")?;
let doc: Document = de::from_str(&xml)?;
let violations = doc.validate_message();

if violations.is_empty() {
    println!("All XSD constraints satisfied");
} else {
    for v in &violations {
        println!("[{:?}] {} — {}", v.kind, v.path, v.message);
    }
}
```

### Translate MT103 → pacs.008

```rust
use mx20022::translate::mt;
use mx20022::translate::mt::fields::mt103::parse_mt103;
use mx20022::translate::mappings::mt103_to_pacs008::mt103_to_pacs008;
use mx20022::parse::ser;

let mt_text = std::fs::read_to_string("mt103.txt")?;

// Parse raw SWIFT FIN blocks → MT103 fields
let msg = mt::parse(&mt_text)?;
let mt103 = parse_mt103(&msg.block4)?;

// Translate to pacs.008 document
let result = mt103_to_pacs008(&mt103, "MSG-001", "2024-01-15T10:00:00")?;

// Serialize back to XML
let xml = ser::to_string(&result.message)?;
println!("{xml}");
```

## CLI

Build and install the CLI:

```bash
cargo install --path crates/mx20022-cli
```

### Commands

```bash
# Inspect message structure
mx20022-cli inspect testdata/xml/pacs/pacs_008_001_13_minimal.xml

# Validate with optional scheme rules
mx20022-cli validate testdata/xml/pacs/pacs_008_001_13_minimal.xml
mx20022-cli validate message.xml --scheme fednow

# Translate between MT and MX
mx20022-cli translate testdata/mt/mt103.txt --to pacs008
mx20022-cli translate message.xml --to mt103

# Generate Rust types from an XSD
mx20022-cli codegen schemas/pacs/pacs.008.001.13.xsd -o output.rs
```

Supported `--to` targets: `pacs008`, `mt103`, `pacs009`, `mt202`, `camt053`, `mt940`.

Supported `--scheme` values: `fednow`, `sepa`, `cbpr`.

The CLI rejects input files larger than 10 MB to prevent out-of-memory conditions.

## Development

```bash
# Run all checks (check + test + clippy + fmt)
just

# Individual commands
just check          # cargo check --workspace --all-features
just test           # cargo test --workspace --all-features
just clippy         # clippy with -D warnings
just fmt            # auto-format
just fmt-check      # format check only
just deny           # cargo-deny license/advisory audit
just doc            # build documentation
just bench          # run criterion benchmarks
just coverage       # HTML coverage report via llvm-cov
just msrv           # verify MSRV 1.75.0 compatibility

# Full CI suite locally
just ci
```

### Project Layout

```
mx20022/
├── crates/
│   ├── mx20022/            # Umbrella re-export + examples
│   ├── mx20022-model/      # Generated types (src/generated/) + common types (src/common/)
│   ├── mx20022-parse/      # XML parsing and serialization
│   ├── mx20022-validate/   # Schema, business rule, and scheme validation
│   ├── mx20022-translate/  # MT ↔ MX translation + MT parser
│   ├── mx20022-codegen/    # XSD → Rust code generator
│   └── mx20022-cli/        # Command-line tool
├── schemas/                # ISO 20022 XSD source files
├── testdata/               # Shared test fixtures (XML, MT, translation pairs)
├── docs/                   # Project documentation
├── justfile                # Task runner
├── deny.toml               # cargo-deny configuration
└── rust-toolchain.toml     # Rust stable + components
```

**Key boundary:** Files under `crates/mx20022-model/src/generated/` are
machine-written by the codegen tool. Do not hand-edit them — modify the code
generator instead.

See [ARCHITECTURE.md](ARCHITECTURE.md) for a detailed design walkthrough of
each crate, the code generation pipeline, and key architectural decisions.

### Running Examples

```bash
cargo run -p mx20022 --example parse_pacs008
cargo run -p mx20022 --example validate_message
cargo run -p mx20022 --example translate_mt103
cargo run -p mx20022 --example roundtrip
```

## Supported Messages

| Family | Message | Version | Description |
|--------|---------|---------|-------------|
| head | 001.001 | 04 | Business Application Header |
| pacs | 008.001 | 13 | FI to FI Customer Credit Transfer |
| pacs | 002.001 | 14 | Payment Status Report |
| pacs | 004.001 | 11 | Return of Funds |
| pacs | 009.001 | 10 | FI to FI Credit Transfer |
| pacs | 028.001 | 05 | FI Status Request |
| pain | 001.001 | 11 | Customer Credit Transfer Initiation |
| pain | 002.001 | 13 | Customer Payment Status Report |
| pain | 013.001 | 09 | Creditor Payment Activation Request |
| camt | 053.001 | 11 | Bank to Customer Statement |
| camt | 054.001 | 11 | Bank to Customer Debit/Credit Notification |
| camt | 056.001 | 11 | Payment Cancellation Request |
| camt | 029.001 | 12 | Resolution of Investigation |

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feat/my-change`)
3. Ensure all checks pass: `just ci`
4. Open a pull request against `main`

All code must pass `cargo check`, `cargo test`, `cargo clippy -- -D warnings`,
and `cargo fmt --check` before merge. CI runs these on both stable and MSRV
1.75.0.

`unsafe` code is forbidden workspace-wide.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or
  <https://opensource.org/licenses/MIT>)

at your option.
