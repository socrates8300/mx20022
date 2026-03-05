# mx20022 — Executive Summary

**A Rust library for ISO 20022 financial messaging**

---

## What It Does

mx20022 is an open-source Rust library for parsing, validating, translating, and generating ISO 20022 (MX) financial messages. It provides strongly-typed, machine-generated data models for the most widely used payment message formats, along with bidirectional translation between legacy SWIFT MT and modern ISO 20022 MX formats.

The library ships as a modular Cargo workspace of seven crates, a command-line tool, and comprehensive documentation — everything a development team needs to integrate ISO 20022 into payment infrastructure.

## Why It Matters

The global financial industry is in the midst of a multi-year migration from legacy SWIFT MT messages to ISO 20022. Major payment rails — FedNow, SEPA, SWIFT CBPR+, CHIPS, CHAPS, TARGET2, and others — either already mandate ISO 20022 or are actively migrating. Despite this, no dominant open-source toolkit exists outside the Java ecosystem, and existing options are incomplete or poorly maintained.

mx20022 fills that gap with a high-performance, memory-safe implementation in Rust — purpose-built for the throughput demands of payment switches, message routers, and real-time settlement systems.

## Key Capabilities

- **13 message schemas** across four ISO 20022 families: payments clearing (pacs), payment initiation (pain), cash management (camt), and business application headers (head)
- **Strongly-typed models** generated directly from official ISO 20022 XSD schemas, with serde serialization and builder-pattern construction
- **XML round-trip fidelity** — parse and serialize ISO 20022 XML with namespace-aware handling
- **Three-layer validation** — XSD schema conformance, business rules (IBAN, BIC, LEI, currency, amounts), and scheme-specific rules for FedNow, SEPA, and CBPR+
- **Bidirectional MT/MX translation** — MT103, MT202, and MT940 mapped to their ISO 20022 equivalents with data-loss warnings and character set handling
- **XSD code generator** — new message types can be added by pointing the generator at an official schema file, keeping the library current as standards evolve

## Target Applications

| Use Case | Example |
|---|---|
| Payment integration | Constructing and parsing pacs.008/pacs.002 messages for FedNow or SEPA Instant |
| Legacy migration | Translating incoming MT103 traffic to pacs.008 during the SWIFT coexistence period |
| Compliance & QA | Validating outbound messages against scheme-specific rules before network submission |
| Platform building | Embedding as the message layer in payment orchestration engines, ledgers, or reconciliation systems |

## Technical Profile

| Attribute | Detail |
|---|---|
| Language | Rust (Edition 2021, MSRV 1.75.0) |
| License | Apache-2.0 |
| Safety | `unsafe` code forbidden workspace-wide; clippy pedantic lints enforced |
| CI | Automated checks on stable + MSRV: build, test, lint, format, dependency audit |
| Modularity | Feature-flagged message families — compile only what you use |

## Current Status

**Version 0.1.0** (released March 2026) delivers the complete Phase 1 scope: all 13 priority message schemas, the full validation stack, bidirectional MT/MX translation for the three most common message pairs, the CLI tool, and the XSD code generator. The library is functional and tested, with benchmark infrastructure in place.

Planned next phases include expanded message coverage driven by community demand, performance optimizations (zero-copy parsing, SIMD), WASM targets for browser tooling, and language bindings via PyO3 and napi-rs.

## Competitive Position

mx20022 is the first full-featured ISO 20022 library in Rust and the first open-source toolkit in any language to combine code-generated message models, built-in MT/MX translation, and scheme-specific validation (FedNow, SEPA, CBPR+) in a single package. Comparable open-source alternatives are limited to Java (prowide-iso20022) and are either commercially licensed for translation features or incomplete.
