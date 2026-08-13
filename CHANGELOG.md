# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Added `mx20022_parse::de::document_xml`, which returns the complete payload
  `Document` as a zero-copy XML slice whether it is bare or nested below an
  arbitrary transport wrapper. Descendant `Document` elements inside that
  payload are not treated as additional candidates.

### Changed

- Bumped all seven workspace crates and their internal dependency requirements
  to 0.4.0. This source change is not a published release.
- Scheme validation now unwraps `Document`, routes `pacs.008` by its
  authoritative namespace, deserializes `pacs.008.001.13`, and delegates all
  field rules to `validate_typed`. Other `pacs.008` versions retain raw checks,
  apply the same version-neutral field rules, and report
  `SCHEME_UNTYPED_VERSION`.
- FedNow and SEPA EndToEndId length checks now operate on typed values and count
  Unicode scalar values. Parse or required-field failures report one
  `SCHEME_PARSE` finding at `/Document`; independent raw findings are preserved
  alongside extraction, namespace-routing, and deserialization failures.
- CLI generic IBAN, BIC, LEI, currency, and date checks use command-private XML
  extraction helpers; scheme validation invokes `SchemeValidator::validate`
  once.

### Fixed

- `mx20022-cli validate` now reads the complete XML stream before reporting a
  valid message type, so malformed or truncated XML without `--scheme` prints
  a parse diagnostic and exits non-zero instead of reporting success.
- `FEDNOW_SINGLE_TX` now checks both the declared `NbOfTxs` and the actual
  number of `CdtTrfTxInf` elements. A message declaring one transaction while
  carrying multiple transactions is rejected on typed and older-version
  validation paths.
- Nested `CdtTrfTxInf` elements inside `SplmtryData` no longer terminate the
  enclosing transaction during older-version pacs.008 fact extraction, so
  subsequent scheme value rules cannot be bypassed.
- `SEPA_SINGLE_TX` now checks both declared and actual transaction counts, and
  SEPA restricted-Latin checks again cover all `Nm`, `StrtNm`, and `TwnNm`
  fields in addition to `Ustrd` on typed and older-version validation paths.

### Removed

- Removed the public `mx20022_validate::schemes::xml_scan` module and the
  scanner-based scheme field-rule path. This is a deliberate breaking change
  for 0.4.0.
- On typed `pacs.008.001.13` parse failures, required or invalid generated
  fields now consolidate into `SCHEME_PARSE` rather than the scanner-era
  `CBPR_CHRGBR_REQUIRED`, `CBPR_CHRGBR_VALUE`, `CBPR_E2E_REQUIRED`, or
  `SEPA_CHRGBR_REQUIRED` IDs. The version-neutral path for older `pacs.008`
  versions still emits these scheme IDs when applicable.

## [0.3.2] - 2026-08-12

### Changed

- Raised the declared minimum supported Rust version from 1.75 to 1.79, the
  actual compiler floor of the patched `quick-xml` 0.41 dependency.
- Updated `proc-macro2` to 1.0.107, `quote` to 1.0.47, `serde` to 1.0.229,
  `thiserror` to 2.0.20, `syn` to 3.0.3, and `prettyplease` to 0.3.0.
- Kept MSRV-sensitive dependencies on compatible release lines: `clap`
  remains below 4.6, `indexmap` below 2.12, and Criterion on 0.5.

### Fixed

- CI now invokes each matrix toolchain explicitly, preventing the repository's
  stable toolchain override from producing a false-green MSRV check.
- Local commands and devcontainer setup now consistently install and verify
  Rust 1.79.

## [0.3.1] - 2026-08-12

### Security

- `crossbeam-epoch` bumped 0.9.18 → 0.9.20 to close RUSTSEC-2026-0204.
  Dev-only transitive dependency via `criterion`; no runtime exposure. (#37)

### Fixed

- `validate_iban` no longer panics on multibyte UTF-8 input. Non-ASCII is
  rejected after whitespace stripping, before the fixed-offset `&str`
  slices, keeping the original ISO 13616 field checks. Regression tests
  assert the ASCII invariant for CJK, mixed ASCII+CJK, and 4-byte emoji
  input. (#37)

### Added

- `# Caution` documentation on `mx20022_parse::de::from_reader`: the read
  is unbounded, so callers on untrusted streams should wrap the reader
  with `Read::take` or pre-size into a buffer and use `from_str`. (#37)

### Removed

- Unused dev-dependencies: `pretty_assertions` (codegen, model, parse,
  validate), `insta` (parse), and the duplicate `mx20022-model` dev-dep
  in validate. (#37)

## [0.3.0] - 2026-05-05

### Added

- New `legacy-pacs` feature flag in `mx20022-model` exposing four predecessor
  message versions still in active use: `pacs.002.001.10`, `pacs.002.001.12`,
  `pacs.008.001.08`, `pacs.008.001.10`. The `all` feature now includes
  `legacy-pacs`. (#20)
- `mx20022::prelude` curated re-exports for typical workflows (errors, the
  `mt` module, envelope detection, validation entry points, the six MT ↔ MX
  translation functions). The umbrella crate now ships with three runnable
  quickstart doctests for envelope detection, MT103 → pacs.008 translation,
  and IBAN validation. (#21)
- `mx20022_translate::mappings::charset::wrap_lines` plus `WrapError` for
  SWIFT MT line-budget wrapping (word-wrap with hard-cut and overflow
  reporting). (#22)
- `mx20022_parse::de::from_str_in_envelope` for deserialization that tags
  failures with the detected ISO 20022 message identifier. New error
  variant `ParseError::DeserializeIn { context, source }` carries the
  detected dotted message ID alongside the underlying `quick_xml`
  diagnostic. (#23)
- Crate-level quickstart and supported-pair matrix on
  `mx20022-translate`. (#24)

### Changed

- **Behavior change:** `pacs008_to_mt103` and `pacs009_to_mt202` no longer
  substitute the literal `"UNKNOWNXXXXX"` placeholder when `DbtrAgt` /
  `CdtrAgt` (pacs.008) or `Dbtr` / `Cdtr` (pacs.009) lacks a BICFI / Nm.
  These callers now return `TranslationError::MissingField`. The previous
  output was a schema-invalid 12-character placeholder; consumers that
  relied on the silent fallback need to attach a BIC before translating.
  (#18)
- **Behavior change:** `mt940_to_camt053` no longer emits an empty
  `Ccy` on each `ReportEntry13` amount. The entry currency is now
  inherited from the `:60F:` opening balance; entries are rejected
  with a `:61:`-tagged warning when the opening balance carries no
  currency. (#19)
- `pacs008_to_mt103` enforces the SWIFT 4 × 35 line budget on `:50K:`,
  `:59:`, and `:70:`. The party account line is hard-truncated and the
  party name is word-wrapped onto remaining lines, with truncation
  warnings tagged by field. (#22)

### Fixed

- MT940 → camt.053 round-trips now produce schema-valid currency on
  every entry instead of an empty `Ccy` placeholder. (#19)

## [0.2.0] - 2026-04-25

### Changed

- **Breaking:** reduced the public API surface for validation and translation internals. These removals are intentional: the deleted items exposed customization hooks and helpers that were either unused, redundant with narrower APIs, or too low-level to support as stable public contracts.
- `RuleRegistry::validate_all` was removed. Call `RuleRegistry::validate_field(value, path, rule_ids)` with the explicit rule IDs that should apply to the field. This keeps validation deterministic and avoids running unrelated rules against arbitrary values.
- `SchemaValidator::registry_mut` and `SchemaValidator::constraints_mut` were removed. Build validation through the supported constructors and field-level validation methods instead of mutating internal state after construction.
- `ConstraintSet::for_path` is now crate-private. Use `ConstraintSet::validate_field` or `SchemaValidator::validate_field` when checking values; downstream code should not depend on raw constraint lookup.
- `mappings::helpers::bic_to_fi_id` was removed. Use the party-based mapping helpers when translating MT party data, or construct `BranchAndFinancialInstitutionIdentification8` directly when a caller already has a raw BIC.
- `MtError::InvalidFieldTag` was removed. Field parser errors should use the existing parse and invalid-value variants that carry concrete field context.

### Fixed

- Release workflow can now call the CI workflow because `.github/workflows/ci.yml` exposes `workflow_call`.

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

[Unreleased]: https://github.com/socrates8300/mx20022/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/socrates8300/mx20022/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/socrates8300/mx20022/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/socrates8300/mx20022/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/socrates8300/mx20022/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/socrates8300/mx20022/releases/tag/v0.1.0
