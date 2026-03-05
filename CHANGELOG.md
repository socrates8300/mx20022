# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-02

### Added

#### Workspace & Infrastructure
- Cargo workspace with 7 crates and shared configuration
- CI pipeline: check, test, clippy, fmt, cargo-deny on stable + MSRV 1.75.0
- Development tooling: justfile task runner, pre-commit hooks, devcontainer
- License: Apache-2.0
- `rust-toolchain.toml`, `deny.toml`, `.cargo/config.toml`
- Dependabot for Cargo and GitHub Actions

#### Code Generation (`mx20022-codegen`)
- XSD parser with full ISO 20022 schema support
- IR type graph with structs, enums, newtypes, choice types, value-with-attribute types
- Rust code emitter with serde Serialize/Deserialize derives
- Builder pattern generation for all struct types
- ChoiceWrapper for XML choice element serialization
- 13 message schemas supported

#### Message Types (`mx20022-model`)
- Generated types for 13 ISO 20022 message versions:
  - `head.001.001.04` — Business Application Header
  - `pacs.008.001.13` — FI to FI Customer Credit Transfer
  - `pacs.002.001.14` — Payment Status Report
  - `pacs.004.001.11` — Return of Funds
  - `pacs.009.001.10` — FI to FI Credit Transfer
  - `pacs.028.001.05` — FI Status Request
  - `pain.001.001.11` — Customer Credit Transfer Initiation
  - `pain.002.001.13` — Customer Payment Status Report
  - `pain.013.001.09` — Creditor Payment Activation Request
  - `camt.053.001.11` — Bank to Customer Statement
  - `camt.054.001.11` — Bank to Customer Debit/Credit Notification
  - `camt.056.001.11` — Payment Cancellation Request
  - `camt.029.001.12` — Resolution of Investigation
- Feature flags: `head`, `pacs`, `pain`, `camt`, `all` (default: `pacs`)
- Builder pattern with required-field validation on all struct types
- Common types: `ChoiceWrapper<T>`, `BuilderError`

#### XML Parsing (`mx20022-parse`)
- XML deserialization via quick-xml + serde
- XML serialization with namespace declarations
- Message type detection from XML namespace URIs
- Envelope parsing with `MessageId` extraction

#### Validation (`mx20022-validate`)
- Rule-based validation engine with `Rule` trait and `RuleRegistry`
- Built-in validators: IBAN, BIC, currency (ISO 4217), country (ISO 3166-1), LEI, amount format, datetime, date
- XSD facet validators: min/max length, regex pattern
- Schema constraint validation from XSD metadata
- Scheme-specific validators:
  - **FedNow**: USD currency, CLRG settlement, SLEV charges, amount limits ($0.01–$500K), UETR requirement, message size limits
  - **SEPA**: EUR currency, SEPA character set, IBAN requirements, amount limits, field length restrictions
  - **CBPR+**: BAH requirement, BIC requirements (4 mandatory), UETR/E2E ID, charge bearer validation

#### MT Parsing & Translation (`mx20022-translate`)
- SWIFT MT message parser with block-level extraction (Blocks 1–5)
- Field-level parsers for MT103, MT202, MT940
- MT940 packed `:61:` statement line parser
- Bidirectional translation:
  - MT103 <-> pacs.008.001.13
  - MT202 <-> pacs.009.001.10
  - MT940 <-> camt.053.001.11
- SWIFT character set handling with diacritic approximation
- Translation warnings for data loss and truncation

#### CLI (`mx20022-cli`)
- `parse` command: detect and display ISO 20022 message structure
- `validate` command: validate with rules + optional `--scheme` flag
- `translate` command: MT<->MX conversion with 6 direction options

#### Examples & Documentation
- Runnable examples: parse, validate, translate, roundtrip
- Performance benchmarks: parse, serialize, validate, translate
- XSD schema download script

[Unreleased]: https://github.com/jamesray/mx20022/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/jamesray/mx20022/releases/tag/v0.1.0
