## Project Name: `mx20022` (working title)

**Version:** 0.1 (Draft)  
**Date:** March 2026

---

## 1. Vision

A comprehensive, high-performance, open-source Rust library for parsing, generating, validating, and transforming ISO 20022 (MX) financial messages. Think of it as the **jPOS of the ISO 20022 world** — a foundational toolkit that payment developers reach for first when building anything that touches modern payment rails.

The global financial industry is in the middle of a multi-year migration from legacy SWIFT MT messages to ISO 20022 MX. FedNow, SEPA, SWIFT CBPR+, CHIPS, CHAPS, LYNX, TARGET2, and PIX either already mandate ISO 20022 or are actively migrating. Despite this, there is no dominant, well-maintained open-source library for working with these messages — especially outside the Java ecosystem.

`mx20022` fills that gap.

---

## 2. Problem Statement

### Who feels this pain?

- **Fintech engineers** building payment integrations against FedNow, SEPA Instant, SWIFT gpi, or domestic real-time payment schemes.
- **Bank integration teams** migrating from MT to MX and needing to parse, validate, enrich, and route messages.
- **Payment processors and PSPs** handling multi-rail, multi-currency message flows.
- **Compliance and operations teams** who need tooling to inspect, search, and audit payment messages.
- **Open-source developers** building payment orchestration, ledger, or reconciliation systems who need a solid message layer underneath.

### What's wrong today?

- **No dominant open-source ISO 20022 toolkit exists.** Most implementations are proprietary (Volante, Bottomline, Finastra) or internal to large banks.
- **Java-heavy ecosystem.** The few open-source options (prowide-iso20022, iso20022-Java) are Java-only and often incomplete or poorly maintained.
- **XSD complexity is brutal.** The ISO 20022 message catalogue contains hundreds of XSD schemas. Developers end up hand-rolling partial parsers for the 2-3 message types they need, leading to fragile, non-reusable code.
- **Validation is an afterthought.** Schema validation alone isn't enough — business rules (e.g., BIC format, IBAN check digits, currency code validity, conditional field requirements) are where real bugs live, and they're rarely implemented in OSS tooling.
- **No good MT↔MX translation layer.** The coexistence period means many systems need to convert between MT and MX, which is a non-trivial mapping exercise.

---

## 3. Goals & Non-Goals

### Goals

1. **Parse** any ISO 20022 XML message into strongly-typed Rust structs.
2. **Generate** valid ISO 20022 XML from Rust structs with builder APIs.
3. **Validate** messages against XSD schemas _and_ business rules.
4. **Cover the most-used message families first:** `pacs` (payments clearing & settlement), `pain` (payment initiation), `camt` (cash management), `head` (business application header).
5. **Provide a clean, ergonomic Rust API** that feels native — not like a Java port.
6. **MT↔MX translation** for the most common SWIFT message pairs (MT103↔pacs.008, MT202↔pacs.009, MT940↔camt.053, etc.).
7. **Extensibility** — make it straightforward for users to add support for additional message types or custom variants.
8. **Zero-copy parsing where feasible** for high-throughput use cases (payment switches, message routers).
9. **Comprehensive documentation and examples** targeting developers who are new to ISO 20022.

### Non-Goals (for v1)

- Full coverage of all 800+ ISO 20022 message definitions. We prioritize the ~20-30 most commonly used.
- A standalone runtime or server. This is a library, not a platform.
- GUI tooling (though we may provide a CLI for message inspection/validation).
- Direct network I/O or protocol handling (MQSeries, SWIFT Alliance, etc.).
- Cryptographic signing or PKI — out of scope for the core library.

---

## 4. Target Users & Use Cases

### Persona 1: The Fintech Payment Engineer

> "I'm integrating with FedNow and I need to construct a `pacs.008` message, validate it, and parse the `pacs.002` response. I don't want to write an XML parser from scratch."

**Use case:** Programmatic message construction, serialization, and parsing within a Rust payment service.

### Persona 2: The Bank Migration Developer

> "We're migrating from SWIFT MT to MX. I need to take incoming MT103 messages and translate them to pacs.008 for our new core banking system."

**Use case:** MT→MX and MX→MT translation during the coexistence period.

### Persona 3: The Compliance / QA Engineer

> "I need to validate that outbound messages conform to scheme-specific rules before they hit the network. I also need to inspect and search large batches of messages for audit."

**Use case:** Batch validation, field-level inspection, and compliance rule checking.

### Persona 4: The Platform Builder

> "I'm building an open-source payment orchestration engine and I need a solid message layer underneath. I don't want to own the parsing and validation code."

**Use case:** Dependency / building block for higher-level payment systems.

---

## 5. Architecture Overview

```
┌─────────────────────────────────────────────────────┐
│                    mx20022 crate                     │
├─────────────────────────────────────────────────────┤
│                                                     │
│  ┌─────────────┐  ┌──────────────┐  ┌────────────┐ │
│  │   Models     │  │   Parser     │  │  Builder   │ │
│  │ (generated   │  │ (XML → Rust) │  │ (Rust →    │ │
│  │  from XSD)   │  │              │  │   XML)     │ │
│  └──────┬───────┘  └──────┬───────┘  └─────┬──────┘ │
│         │                 │                │        │
│  ┌──────▼─────────────────▼────────────────▼──────┐ │
│  │              Validation Engine                  │ │
│  │  • XSD schema validation                       │ │
│  │  • Business rule validation                    │ │
│  │  • Scheme-specific rules (FedNow, SEPA, etc.)  │ │
│  └────────────────────┬───────────────────────────┘ │
│                       │                             │
│  ┌────────────────────▼───────────────────────────┐ │
│  │           Translation Engine                    │ │
│  │  • MT ↔ MX mapping                            │ │
│  │  • Enrichment & transformation                 │ │
│  └────────────────────────────────────────────────┘ │
│                                                     │
│  ┌────────────────────────────────────────────────┐ │
│  │           Codegen (build-time)                  │ │
│  │  • XSD → Rust struct generation                │ │
│  │  • Serde annotations                           │ │
│  │  • Validation annotations                      │ │
│  └────────────────────────────────────────────────┘ │
│                                                     │
├─────────────────────────────────────────────────────┤
│  CLI Tool (optional)                                │
│  • mx validate <file>                               │
│  • mx inspect <file>                                │
│  • mx translate --from mt103 --to pacs008 <file>    │
│  • mx generate --type pacs.008 --template <file>    │
└─────────────────────────────────────────────────────┘
```

### Key Architectural Decisions

**Code generation from XSD schemas.** The ISO 20022 message catalogue is defined in XSD. Rather than hand-writing Rust structs for hundreds of message types, we build a code generator that reads the official XSD files and emits Rust structs with `serde` derive macros. This is similar to how protobuf/gRPC works — the schemas are the source of truth. This is the single most important architectural decision in the project.

**Workspace of crates.** The project should be organized as a Cargo workspace:

- `mx20022-model` — Generated types for each message family (feature-gated so you only compile what you use)
- `mx20022-parse` — XML parsing and serialization
- `mx20022-validate` — Schema + business rule validation
- `mx20022-translate` — MT↔MX translation
- `mx20022-codegen` — The XSD→Rust code generator (build dependency)
- `mx20022-cli` — Command-line tool
- `mx20022` — Umbrella crate that re-exports everything

**Feature flags for message families.** Users who only need `pacs` shouldn't have to compile `pain`, `camt`, `sese`, etc. Cargo feature flags make this clean.

---

## 6. Priority Message Families & Types

### Phase 1 — Core (v0.1–v0.3)

These are the messages you will encounter on virtually every real-time payment integration:

|Family|Message|Description|
|---|---|---|
|`head`|head.001|Business Application Header (BAH)|
|`pacs`|pacs.008|FI to FI Customer Credit Transfer|
|`pacs`|pacs.002|Payment Status Report|
|`pacs`|pacs.004|Payment Return|
|`pacs`|pacs.009|FI to FI Financial Institution Credit Transfer|
|`pacs`|pacs.028|FI to FI Payment Status Request|
|`pain`|pain.001|Customer Credit Transfer Initiation|
|`pain`|pain.002|Customer Payment Status Report|
|`pain`|pain.013|Creditor Payment Activation Request|
|`camt`|camt.053|Bank to Customer Statement|
|`camt`|camt.054|Bank to Customer Debit/Credit Notification|
|`camt`|camt.056|FI to FI Payment Cancellation Request|
|`camt`|camt.029|Resolution of Investigation|

### Phase 2 — Extended (v0.4–v0.6)

Securities, trade finance, and additional cash management messages based on community demand.

### Phase 3 — Full Catalogue

Community-driven expansion. The code generator should make adding new message types straightforward.

---

## 7. API Design Sketch

### Parsing

```rust
use mx20022::pacs::Pacs008;

let xml = std::fs::read_to_string("payment.xml")?;
let msg: Pacs008 = mx20022::parse(&xml)?;

println!("Debtor: {}", msg.credit_transfer.debtor.name);
println!("Amount: {} {}",
    msg.credit_transfer.amount.value,
    msg.credit_transfer.amount.currency);
```

### Building

```rust
use mx20022::pacs::Pacs008Builder;

let msg = Pacs008Builder::new()
    .message_id("MSG-2026-001")
    .creation_date_time(Utc::now())
    .number_of_transactions(1)
    .settlement_method(SettlementMethod::ClearingSystem)
    .credit_transfer(|tx| {
        tx.end_to_end_id("E2E-001")
          .amount(Currency::USD, 1500.00)
          .debtor(|d| d.name("Alice Corp").bic("ABCOUS33"))
          .creditor(|c| c.name("Bob Ltd")
                        .iban("DE89370400440532013000"))
    })
    .build()?;

let xml = mx20022::to_xml(&msg)?;
```

### Validation

```rust
use mx20022::validate::{Validator, RuleSet};

let validator = Validator::new()
    .with_schema_validation()
    .with_business_rules()
    .with_scheme_rules(Scheme::FedNow);

let result = validator.validate(&msg);

match result {
    Ok(()) => println!("Message is valid"),
    Err(errors) => {
        for e in errors {
            println!("[{}] {} at {}",
                e.severity, e.message, e.path);
        }
    }
}
```

### Translation

```rust
use mx20022::translate;

let mt103_raw = std::fs::read_to_string("incoming.mt103")?;
let mt103 = mx20022::mt::parse_mt103(&mt103_raw)?;
let pacs008: Pacs008 = translate::mt103_to_pacs008(&mt103)?;
```

---

## 8. Validation Strategy

Validation happens in three layers:

### Layer 1: Schema Validation

- XML well-formedness
- XSD type conformance (string lengths, patterns, enumerations)
- Required vs. optional element presence
- Cardinality constraints

### Layer 2: Business Rule Validation

- IBAN check digit verification
- BIC format and existence checking (optional, with registry data)
- Currency code validity (ISO 4217)
- Country code validity (ISO 3166)
- LEI format validation
- Date/time consistency (e.g., creation date not in the future)
- Amount consistency (sum of individual transactions = control sum)

### Layer 3: Scheme-Specific Rules

- FedNow: message size limits, field restrictions, allowed BICs, value limits
- SEPA: SEPA-specific character set restrictions, purpose code requirements
- SWIFT CBPR+: usage guidelines and market practice rules
- These are pluggable — users can register custom rule sets

Each layer produces structured error output with XPath-like location, severity (error/warning/info), rule identifier, and human-readable message.

---

## 9. Code Generation Strategy

This is the engine room of the project.

### Input

- Official ISO 20022 XSD schemas from iso20022.org
- Supplementary metadata for field-level documentation

### Output

- One Rust module per message type
- Strongly-typed structs with `serde::Serialize` and `serde::Deserialize`
- Builder structs for ergonomic construction
- `Display` and `Debug` implementations
- Comprehensive doc comments pulled from XSD annotations

### Approach

1. Parse XSD using `quick-xml` or a dedicated XSD parser
2. Build an intermediate representation of the type graph
3. Resolve complex type inheritance and restriction
4. Emit Rust code via a template engine or `proc_macro`
5. Run `rustfmt` on generated output

### Key Challenges

- ISO 20022 XSDs use complex type inheritance heavily — Rust doesn't have inheritance, so we need a flattening strategy (likely composition + enums for choice types)
- `xs:choice` maps to Rust enums
- `xs:sequence` maps to Rust structs
- Optional elements map to `Option<T>`
- Repeating elements map to `Vec<T>`
- Restricted simple types map to newtypes with validation

---

## 10. MT↔MX Translation

The coexistence of SWIFT MT and ISO 20022 MX will last for years. Translation is a high-value feature.

### Priority Mappings

|MT|MX|Description|
|---|---|---|
|MT103|pacs.008|Customer credit transfer|
|MT103 STP|pacs.008|Straight-through processing variant|
|MT202|pacs.009|FI transfer|
|MT202 COV|pacs.009|Cover payment|
|MT900|camt.054|Confirmation of debit|
|MT910|camt.054|Confirmation of credit|
|MT940|camt.053|Account statement|
|MT199/299|camt.056/029|Cancellation and investigation|

### Design

- Each mapping is a standalone, testable function
- Mappings handle field truncation, character set conversion (SWIFT → UTF-8), and data enrichment
- Lossy translations (MT→MX can gain structure; MX→MT can lose it) are documented and configurable
- Users can register custom field mappings for institution-specific extensions

---

## 11. Competitive Landscape

|Project|Language|Status|Notes|
|---|---|---|---|
|prowide-iso20022|Java|Active|Most complete OSS option. Java-only. Complex API.|
|iso20022-Java|Java|Stale|Incomplete, not actively maintained|
|py20022|Python|Minimal|Thin wrapper, limited message support|
|Moov ISO 20022|Go|Early|Part of Moov financial toolkit, incomplete|
|**(This project)**|**Rust**|**Planned**|**Full-featured, code-generated, high-performance**|

### Key differentiators for `mx20022`

- Rust's performance and safety — relevant for payment switches processing millions of messages
- Code generation from XSD means staying current with schema updates is semi-automated
- Ergonomic, idiomatic Rust API (not a Java port)
- Built-in MT↔MX translation (prowide has this, but it's commercial/licensed)
- Scheme-specific validation (FedNow, SEPA, CBPR+)
- CLI tooling for operations and debugging

---

## 12. Release Plan

### v0.1 — Foundation

- Code generator producing Rust structs from XSD
- `head.001`, `pacs.008`, `pacs.002` parsing and generation
- Basic schema validation
- CLI: `mx validate` and `mx inspect`
- Published to crates.io

### v0.2 — Core Payment Messages

- Remaining Phase 1 message types
- Builder API
- Business rule validation (IBAN, BIC, currency)
- Comprehensive test suite with real-world message samples

### v0.3 — Translation

- MT103↔pacs.008 translation
- MT202↔pacs.009 translation
- MT940↔camt.053 translation

### v0.4 — Scheme Rules

- FedNow-specific validation rules
- SEPA-specific validation rules
- SWIFT CBPR+ usage guidelines

### v0.5+ — Community-Driven

- Additional message families based on demand
- Performance optimization (zero-copy parsing, SIMD)
- WASM target for browser-based tooling
- Language bindings (Python via PyO3, Node via napi-rs)

---

## 13. Success Metrics

- **Adoption:** 500+ stars on GitHub within 12 months; 1,000+ monthly downloads on crates.io within 6 months of first publish
- **Correctness:** 100% pass rate on ISO 20022 conformance test suites (where available)
- **Coverage:** Phase 1 message types fully supported within 6 months
- **Community:** 10+ external contributors within 12 months
- **Performance:** Parse and validate a `pacs.008` message in < 100μs on commodity hardware

---

## 14. Open Questions

1. **Naming:** `mx20022`? `iso20022-rs`? `rust-iso20022`? Naming matters for discoverability.
2. **XSD versioning:** ISO 20022 schemas are versioned. Do we support multiple versions simultaneously, or track latest? (Recommendation: default to latest, with feature flags for older versions where needed.)
3. **Async support:** Should parsing/validation be async-aware, or is this purely synchronous? (Recommendation: sync core with optional async wrappers — parsing is CPU-bound, not I/O-bound.)
4. **CBPR+ vs. domestic variants:** SWIFT's CBPR+ usage guidelines restrict the base ISO 20022 schemas. Same is true for FedNow, SEPA, etc. How deep do we go on variant-specific types? (Recommendation: validation rules, not separate types.)
5. **Licensing:** Apache 2.0? MIT? Dual license? (Recommendation: Apache 2.0 OR MIT, consistent with the Rust ecosystem convention.)
6. **Governance:** Solo maintainer to start, or set up a GitHub org with a contribution framework from day one?

---

## 15. References

- [ISO 20022 Message Catalogue](https://www.iso20022.org/catalogue-messages)
- [SWIFT ISO 20022 Programme](https://www.swift.com/standards/iso-20022-programme)
- [FedNow ISO 20022 Implementation Guide](https://www.frbservices.org/financial-services/fednow)
- [SEPA Message Implementation Guides](https://www.europeanpaymentscouncil.eu/)
- [jPOS Project](http://jpos.org/) — spiritual ancestor for ISO 8583
- [prowide-iso20022](https://github.com/prowide/prowide-iso20022) — Java reference
- [TigerBeetle](https://tigerbeetle.com/) — inspiration for financial-grade systems engineering

