# Architecture

Technical architecture document for the mx20022 Rust ISO 20022 financial message library.

---

## 1. Overview

mx20022 is a Cargo workspace of 7 crates that provides end-to-end ISO 20022 financial message processing: code generation from XSD schemas, strongly-typed message models, XML parsing/serialization, multi-layer validation, bidirectional SWIFT MT ↔ MX translation, and a CLI tool.

The library targets payment infrastructure — FedNow, SEPA, SWIFT CBPR+ — where correctness, type safety, and performance are non-negotiable. All code is `unsafe`-free, linted with clippy pedantic, and tested against 553+ assertions.

### Design Principles

- **Schema-driven**: Generated types are the source of truth, produced from official ISO 20022 XSD files
- **Layered validation**: Schema constraints → business rules → scheme-specific rules, composable and extensible
- **Minimal dependencies**: Each crate pulls only what it needs; the model crate has zero non-serde dependencies
- **Feature-gated compilation**: Users compile only the message families they use (pacs, pain, camt, head)

---

## 2. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                        mx20022 (umbrella)                           │
│               Re-exports: model, parse, validate, translate         │
└────┬──────────────┬───────────────┬───────────────┬─────────────────┘
     │              │               │               │
     ▼              ▼               ▼               ▼
┌─────────┐  ┌───────────┐  ┌────────────┐  ┌─────────────┐
│  model   │  │   parse    │  │  validate   │  │  translate   │
│          │  │            │  │             │  │              │
│ Generated│  │ quick-xml  │  │ Rules +     │  │ MT parser +  │
│ types +  │◄─┤ + serde    │  │ Schemas +   │  │ 6 bidirect.  │
│ common   │  │ wrappers   │  │ Schemes     │  │ mappings     │
└─────┬────┘  └──────┬─────┘  └──────┬──────┘  └──────┬───────┘
      │              │               │                │
      │              └───────┬───────┘                │
      │                      │                        │
      └──────────────────────┼────────────────────────┘
                             │
                    ┌────────▼────────┐
                    │   mx20022-cli    │
                    │ inspect/validate │
                    │ translate/codegen│
                    └────────┬────────┘
                             │
                    ┌────────▼────────┐
                    │  mx20022-codegen │
                    │  XSD → IR → Rust │
                    └─────────────────┘
```

### Dependency Graph (compile-time)

```
codegen ──(generates)──► model
                           │
                     ┌─────┴─────┐
                     ▼           ▼
                   parse     validate
                     │           │
                     └─────┬─────┘
                           ▼
                       translate
                           │
                           ▼
                     cli (all crates)
                           │
                           ▼
                   mx20022 (umbrella, sans codegen)
```

### Data Flow: Message Processing Pipeline

```
                    ┌──────────┐
                    │ Raw XML  │
                    └────┬─────┘
                         │
              ┌──────────▼──────────┐
              │  envelope::detect   │  Namespace scanning (no full parse)
              │  → MessageId        │  e.g. "pacs.008.001.13"
              └──────────┬──────────┘
                         │
              ┌──────────▼──────────┐
              │  de::from_str::<T>  │  Deserialize to generated Document type
              │  → Document         │  via quick-xml + serde
              └──────────┬──────────┘
                         │
          ┌──────────────┼──────────────┐
          ▼              ▼              ▼
   ┌────────────┐ ┌───────────┐ ┌────────────┐
   │ Validatable│ │ RuleReg.  │ │ Scheme     │
   │ trait      │ │ validate  │ │ validate   │
   │ (typed)    │ │ (field)   │ │ (FedNow…)  │
   └──────┬─────┘ └─────┬─────┘ └──────┬─────┘
          └──────────────┼──────────────┘
                         ▼
              ┌──────────────────────┐
              │  ValidationResult    │
              │  errors[], warnings  │
              └──────────────────────┘
```

### Data Flow: Code Generation Pipeline

```
┌──────────┐     ┌───────────┐     ┌──────────┐     ┌───────────┐
│ XSD File │────►│ XSD Parser│────►│ IR Lower │────►│ Emitter   │
│ (schema) │     │ → Schema  │     │ → TypeGraph│    │ → Rust src│
└──────────┘     └───────────┘     └──────────┘     └───────────┘
                  quick-xml          Flatten          proc-macro2
                  streaming          inheritance      + quote
                                    Resolve refs      + prettyplease
```

### Data Flow: MT ↔ MX Translation

```
MT Direction:                           MX Direction:
┌──────────┐                            ┌──────────┐
│ MT text  │                            │ MX XML   │
└────┬─────┘                            └────┬─────┘
     │                                       │
     ▼                                       ▼
┌─────────────┐                        ┌──────────────┐
│ Block parser│                        │ de::from_str │
│ → MtMessage │                        │ → Document   │
└────┬────────┘                        └────┬─────────┘
     │                                      │
     ▼                                      ▼
┌──────────────┐                       ┌──────────────┐
│ Field parser │                       │ Mapping fn   │
│ → Mt103/etc  │                       │ → String (MT)│
└────┬─────────┘                       └────┬─────────┘
     │                                      │
     ▼                                      ▼
┌──────────────┐                       ┌──────────────┐
│ Mapping fn   │                       │ MT text out  │
│ → Document   │                       │ + warnings   │
└────┬─────────┘                       └──────────────┘
     │
     ▼
┌──────────────┐
│ ser::to_xml  │
│ + warnings   │
└──────────────┘
```

---

## 3. Module Breakdown

### 3.1 mx20022-codegen

**Purpose**: Compile ISO 20022 XSD schemas into idiomatic Rust source code.

**Module structure**:
```
src/
├── lib.rs                      # Public API: xsd, ir, emit
├── main.rs                     # CLI entry point
├── xsd/
│   ├── types.rs                # XSD AST: Schema, Element, SimpleType, ComplexType,
│   │                           #   Restriction, Facet, ComplexContent, Attribute
│   └── parser.rs               # Streaming quick-xml parser with Context stack
├── ir/
│   ├── types.rs                # IR types: TypeGraph, TypeDef variants, Constraint
│   └── lower.rs                # XSD AST → IR lowering (flatten inheritance, resolve refs)
└── emit/
    ├── mod.rs                  # Orchestrator: dispatch TypeDef → emitter
    ├── structs.rs              # xs:sequence → Rust struct + serde renames
    ├── enums.rs                # xs:choice → Rust enum (newtype variants)
    ├── code_enums.rs           # xs:enumeration → Rust enum (unit variants)
    ├── newtypes.rs             # xs:simpleType → newtype + TryFrom validation
    ├── value_with_attr.rs      # xs:simpleContent → struct with $value + @attrs
    ├── opaque.rs               # xs:any → String wrapper
    ├── builders.rs             # Builder pattern generation
    ├── validate.rs             # Validatable + IsoMessage trait impls
    ├── pattern_codegen.rs      # XSD regex → inline Rust checks (no regex dep)
    └── util.rs                 # Name conversion, cardinality helpers
```

**Three-stage pipeline**:

| Stage | Input | Output | Key Logic |
|-------|-------|--------|-----------|
| **Parse** | XSD XML bytes | `xsd::Schema` | Streaming quick-xml; state machine with Context stack; handles nested types |
| **Lower** | `xsd::Schema` | `ir::TypeGraph` | Flattens XSD inheritance to composition; resolves type refs; maps xs:choice → enum, xs:sequence → struct, xs:simpleType → newtype or code enum |
| **Emit** | `ir::TypeGraph` | Formatted Rust source | proc-macro2 + quote token streams; syn parse validation; prettyplease formatting |

**XSD → Rust type mappings**:

| XSD Construct | IR Type | Rust Output |
|---|---|---|
| `xs:sequence` | `StructDef` | `struct` with serde field renames |
| `xs:choice` | `EnumDef` | `enum` with newtype tuple variants |
| `xs:simpleType` (enumeration only) | `CodeEnumDef` | `enum` with unit variants |
| `xs:simpleType` (other facets) | `NewtypeDef` | Newtype struct with `TryFrom<String>` validation |
| `xs:simpleContent` + `xs:extension` | `ValueWithAttrDef` | Struct with `$value` + `@attr` fields |
| `xs:any` | `OpaqueDef` | String wrapper |

**Cardinality mapping**:

| XSD | IR | Rust |
|---|---|---|
| `minOccurs=1, maxOccurs=1` | `Required` | `T` |
| `minOccurs=0, maxOccurs=1` | `Optional` | `Option<T>` |
| `maxOccurs=unbounded` | `Vec` | `Vec<T>` |
| `maxOccurs=N` (N > 1) | `BoundedVec(N)` | `Vec<T>` + doc comment |

**Name conversion**: XML PascalCase element names → `snake_case` Rust fields via `xml_to_snake_case()`. Enum code values (e.g., `"ADDR"`) → `PascalCase` variants.

### 3.2 mx20022-model

**Purpose**: Strongly-typed ISO 20022 message definitions — generated types plus hand-written common abstractions.

**Module structure**:
```
src/
├── lib.rs
├── common/
│   ├── choice.rs       # ChoiceWrapper<T> — $value wrapper for quick-xml enum compat
│   ├── builder.rs      # BuilderError { type_name, missing_fields }
│   └── validate.rs     # Validatable trait, ConstraintViolation, ConstraintKind, IsoMessage
└── generated/          # Machine-written — do not hand-edit
    ├── head/           # head.001.001.02, head.001.001.04
    ├── pacs/           # pacs.002.001.14, pacs.004.001.11, pacs.008.001.{10,13},
    │                   #   pacs.009.001.10, pacs.028.001.05
    ├── pain/           # pain.001.001.11, pain.002.001.13, pain.013.001.09
    └── camt/           # camt.029.001.12, camt.053.001.11, camt.054.001.11, camt.056.001.11
```

**Feature flags** (Cargo.toml):

| Feature | Families | Default |
|---|---|---|
| `pacs` | pacs.002, .004, .008, .009, .028 | **yes** |
| `pain` | pain.001, .002, .013 | no |
| `camt` | camt.029, .053, .054, .056 | no |
| `head` | head.001 | no |
| `all` | all of the above | no |

**Key common types**:

```rust
// Wraps xs:choice enums for quick-xml serde compatibility
pub struct ChoiceWrapper<T> {
    #[serde(rename = "$value")]
    pub inner: T,
}

// Builder validation error
pub struct BuilderError {
    pub type_name: String,
    pub missing_fields: Vec<String>,
}

// Self-validation trait (impls generated by codegen)
pub trait Validatable {
    fn validate_constraints(&self, path: &str, violations: &mut Vec<ConstraintViolation>);
}

// Document-level message marker
pub trait IsoMessage: Validatable {
    fn message_type(&self) -> &'static str;  // "pacs.008.001.13"
    fn root_path(&self) -> &'static str;     // "/Document"
    fn validate_message(&self) -> Vec<ConstraintViolation>;
}

pub struct ConstraintViolation {
    pub path: String,         // XPath-like location
    pub message: String,
    pub kind: ConstraintKind, // MinLength, MaxLength, Pattern, etc.
}
```

**Generated type patterns** (per message module):

1. **Newtypes** with constraint validation:
   ```rust
   #[serde(transparent)]
   pub struct ActiveCurrencyCode(pub String);

   impl TryFrom<String> for ActiveCurrencyCode { ... }  // Pattern/length checks
   impl ActiveCurrencyCode { pub fn new(v: impl Into<String>) -> Result<Self, ConstraintError> { ... } }
   ```

2. **Enums** for xs:choice (newtype variants) and xs:enumeration (unit variants):
   ```rust
   pub enum AccountIdentification4Choice {
       #[serde(rename = "IBAN")] IBAN(IBAN2007Identifier),
       #[serde(rename = "Othr")] Othr(GenericAccountIdentification1),
   }
   ```

3. **Structs** with builder pattern:
   ```rust
   pub struct GroupHeader131 { /* fields with #[serde(rename)] */ }
   pub struct GroupHeader131Builder { /* Option<T> per field */ }
   impl GroupHeader131 { pub fn builder() -> GroupHeader131Builder { ... } }
   ```

4. **Root Document** type wrapping the top-level message element:
   ```rust
   pub struct Document {
       #[serde(rename = "FIToFICstmrCdtTrf")]
       pub fi_to_fi_cstmr_cdt_trf: FIToFICustomerCreditTransferV10,
   }
   ```

**Serde attribute conventions**:

| Attribute | Purpose |
|---|---|
| `#[serde(rename = "XmlName")]` | XML element name |
| `#[serde(rename = "@AttrName")]` | XML attribute |
| `#[serde(rename = "$value")]` | XML text content |
| `#[serde(transparent)]` | Newtype passthrough |
| `#[serde(skip_serializing_if = "Option::is_none")]` | Omit absent optional fields |
| `#[serde(skip_serializing_if = "Vec::is_empty")]` + `#[serde(default)]` | Omit empty collections |

### 3.3 mx20022-parse

**Purpose**: XML deserialization, serialization, and message type detection. Thin adapter over quick-xml + serde.

**Module structure**:
```
src/
├── lib.rs          # Re-exports: de, ser, envelope, error
├── de.rs           # from_str<T>, from_reader<T>
├── ser.rs          # to_string<T>, to_string_with_declaration<T>
├── envelope.rs     # detect_message_type, parse_namespace, MessageId
└── error.rs        # ParseError enum
```

**Public API**:

| Function | Signature | Description |
|---|---|---|
| `de::from_str` | `<T: DeserializeOwned>(xml: &str) → Result<T, ParseError>` | Deserialize XML to typed struct |
| `de::from_reader` | `<R: BufRead, T: DeserializeOwned>(r: R) → Result<T, ParseError>` | Streaming deserialization |
| `ser::to_string` | `<T: Serialize>(v: &T) → Result<String, ParseError>` | Serialize to XML |
| `ser::to_string_with_declaration` | `<T: Serialize>(v: &T) → Result<String, ParseError>` | Serialize with `<?xml?>` header |
| `envelope::detect_message_type` | `(xml: &str) → Result<MessageId, ParseError>` | Extract message type from xmlns |

**MessageId**:
```rust
pub struct MessageId {
    pub family: String,   // "pacs"
    pub msg_id: String,   // "008"
    pub variant: String,  // "001"
    pub version: String,  // "13"
}

impl MessageId {
    pub fn dotted(&self) -> String  // "pacs.008.001.13"
}
```

**Envelope detection** scans for `xmlns` attributes containing `urn:iso:std:iso:20022:tech:xsd:` without full XML parsing — fast pre-flight identification.

**Error type**:
```rust
pub enum ParseError {
    Deserialize(quick_xml::DeError),
    Serialize(quick_xml::SeError),
    Io(std::io::Error),
    InvalidEnvelope(String),
}
```

### 3.4 mx20022-validate

**Purpose**: Three-layer validation: schema constraints, business rules, and payment scheme rules.

**Module structure**:
```
src/
├── lib.rs
├── error.rs              # ValidationError, ValidationResult, Severity
├── rules/
│   ├── mod.rs            # Rule trait, RuleRegistry
│   ├── iban.rs           # IBAN check digits (ISO 13616, mod-97)
│   ├── bic.rs            # BIC format (ISO 9362, 8 or 11 chars)
│   ├── currency.rs       # Currency codes (ISO 4217, 175 codes)
│   ├── country.rs        # Country codes (ISO 3166-1, 249 + XK)
│   ├── lei.rs            # LEI format (ISO 17442, 20 chars, mod-97)
│   ├── amount.rs         # Decimal format (positive, max 18.5 digits)
│   ├── datetime.rs       # ISO 8601 date and datetime
│   ├── length.rs         # Min/max length (Unicode codepoints)
│   └── pattern.rs        # Regex pattern matching (XSD facets)
├── schema/
│   ├── mod.rs            # SchemaValidator orchestrator
│   └── constraints.rs    # FieldConstraint, ConstraintSet
└── schemes/
    ├── mod.rs            # SchemeValidator trait
    ├── fednow.rs         # FedNow rules (USD, $500K limit, UETR, etc.)
    ├── sepa.rs           # SEPA SCT rules (EUR, IBAN required, charset)
    ├── cbpr.rs           # CBPR+ rules (BAH required, 4 BICs mandatory)
    └── xml_scan.rs       # Lightweight XML string extraction helpers
```

**Validation architecture**:

```
Layer 1 — Schema Constraints (type-level)
├── Validatable trait (on generated types)
├── XSD facets: minLength, maxLength, pattern, min/maxInclusive, totalDigits
└── ConstraintViolation with XPath location

Layer 2 — Business Rules (field-level)
├── RuleRegistry (HashMap<String, Box<dyn Rule>>)
├── 8 built-in rules: IBAN, BIC, Currency, Country, LEI, Amount, Date, DateTime
├── Configurable: MinLength, MaxLength, LengthRange, Pattern
└── Extensible: implement Rule trait, register with registry

Layer 3 — Scheme Rules (message-level)
├── SchemeValidator trait with validate() and validate_typed()
├── FedNow: 15+ rules (currency, settlement, amounts, UETR, size limits)
├── SEPA: 12+ rules (EUR, IBAN both sides, charset, amount bounds)
├── CBPR+: 12+ rules (BAH, 4 BICs, UETR, charge bearer, UTF-8)
└── Pluggable: implement SchemeValidator for custom schemes
```

**Core trait**:
```rust
pub trait Rule: Send + Sync {
    fn id(&self) -> &str;
    fn validate(&self, value: &str, path: &str) -> Vec<ValidationError>;
}
```

**Scheme validator trait** (dual-path):
```rust
pub trait SchemeValidator: Send + Sync {
    fn name(&self) -> &'static str;
    fn supported_messages(&self) -> &[&str];
    fn validate(&self, xml: &str, message_type: &str) -> ValidationResult;       // XML string path
    fn validate_typed(&self, msg: &dyn Any, message_type: &str) -> Option<ValidationResult>; // Typed path
}
```

**Validation output**:
```rust
pub struct ValidationResult { pub errors: Vec<ValidationError> }
pub struct ValidationError {
    pub path: String,        // XPath-like location
    pub severity: Severity,  // Error | Warning | Info
    pub rule_id: String,     // e.g. "IBAN_CHECK", "FEDNOW_CURRENCY"
    pub message: String,     // Human-readable
}
```

`ValidationResult::is_valid()` returns true only when zero `Severity::Error` entries exist. Warnings and info do not fail validation.

### 3.5 mx20022-translate

**Purpose**: Bidirectional translation between SWIFT MT (FIN) messages and ISO 20022 MX (XML) messages.

**Module structure**:
```
src/
├── lib.rs
├── mt/
│   ├── types.rs          # MtMessage, Block1–5, TagField
│   ├── parser.rs         # 5-block FIN parser (brace-depth counting)
│   ├── error.rs          # MtError variants
│   └── fields/
│       ├── common.rs     # Amount, Account, PartyInfo, date parsers
│       ├── mt103.rs      # MT103 fields → Mt103 struct
│       ├── mt202.rs      # MT202 fields → Mt202 struct
│       └── mt940.rs      # MT940 fields → Mt940 struct (Balance, StatementLine)
└── mappings/
    ├── error.rs          # TranslationError, TranslationResult<T>, TranslationWarnings
    ├── charset.rs        # SWIFT FIN charset validation + diacritic approximation
    ├── helpers.rs        # Shared conversion: party→FI, account→CashAccount, etc.
    ├── mt103_to_pacs008.rs
    ├── pacs008_to_mt103.rs
    ├── mt202_to_pacs009.rs
    ├── pacs009_to_mt202.rs
    ├── mt940_to_camt053.rs
    └── camt053_to_mt940.rs
```

**Supported translation pairs**:

| MT Format | MX Format | Direction |
|---|---|---|
| MT103 (Customer Credit Transfer) | pacs.008.001.13 | ↔ |
| MT202 (FI Credit Transfer) | pacs.009.001.10 | ↔ |
| MT940 (Customer Statement) | camt.053.001.11 | ↔ |

**MT parsing pipeline**:

1. **Block extraction** (`parser.rs`): Brace-depth counting splits raw FIN text into 5 blocks. No regex for block boundaries.
2. **Field extraction** (`parser.rs`): Regex `^:(\d{2}[A-Z]?):(.*)$` identifies tag fields; continuation lines joined with `\n`.
3. **Field parsing** (`fields/*.rs`): Tag dispatching (`match tag.as_str()`) populates typed MT structs (Mt103, Mt202, Mt940).

**MT field types**:

```rust
pub struct Mt103 {
    pub senders_reference: String,        // :20:
    pub bank_operation_code: String,      // :23B:
    pub value_date: String,               // :32A: (date portion)
    pub currency: String,                 // :32A: (currency portion)
    pub interbank_settled_amount: String,  // :32A: (amount portion)
    pub ordering_customer: PartyInfo,     // :50A/F/K:
    pub beneficiary: PartyInfo,           // :59/59A/59F:
    pub details_of_charges: String,       // :71A:
    // ... 14 optional fields
}

pub struct Amount { pub currency: String, pub value: String }
pub struct Account { pub iban: Option<String>, pub bic: Option<String>, pub account: Option<String> }
pub struct PartyInfo { pub account: Option<Account>, pub name: Option<String>, pub address_lines: Vec<String> }
```

**Translation function signatures**:

```rust
// MT → MX (requires caller-provided msg_id and timestamp)
pub fn mt103_to_pacs008(mt103: &Mt103, msg_id: &str, creation_time: &str)
    -> Result<TranslationResult<pacs008::Document>, TranslationError>

// MX → MT
pub fn pacs008_to_mt103(doc: &pacs008::Document)
    -> Result<TranslationResult<String>, TranslationError>
```

**Translation result**:
```rust
pub struct TranslationResult<T: Debug> {
    pub message: T,                      // Document or String
    pub warnings: TranslationWarnings,   // Data loss / unmapped field notices
}
```

**Charset handling** (`charset.rs`): Converts UTF-8 → SWIFT FIN safe ASCII with diacritic approximation (ü→u, é→e, €→EUR) and reports whether replacements occurred.

**Data loss semantics**: MT→MX can gain structure (enrichment). MX→MT can lose structure (truncation, flattening). Warnings are emitted for unmapped fields (e.g., `:53:`, `:54:`, `:72:`, charges).

### 3.6 mx20022-cli

**Purpose**: Command-line interface for message inspection, validation, translation, and code generation.

**Commands** (clap derive):

| Command | Input | Output |
|---|---|---|
| `inspect <file>` | XML file | Message type, family, version, namespace, file size |
| `validate <file> [--scheme fednow\|sepa\|cbpr]` | XML file | Validation report (errors, warnings, rule IDs) |
| `translate <file> --to <target>` | MT text or MX XML | Translated message (XML or MT text) |
| `codegen <xsd> [-o output.rs]` | XSD schema file | Generated Rust source |

**Supported --to targets**: `pacs008`, `mt103`, `pacs009`, `mt202`, `camt053`, `mt940`.

### 3.7 mx20022 (Umbrella)

**Purpose**: Single-crate dependency for library consumers.

```rust
pub use mx20022_model as model;
pub use mx20022_parse as parse;
pub use mx20022_translate as translate;
pub use mx20022_validate as validate;
```

Enables `all` features on the model crate. Does **not** re-export codegen (build-time tool only).

---

## 4. Data Models

### Core Type Hierarchy

```
Document (root, per message type)
└── Message body struct (e.g., FIToFICustomerCreditTransferV10)
    ├── GroupHeader (metadata: msg_id, creation_dt, nb_of_txs, settlement_info)
    └── CreditTransferTransaction[] (repeatable)
        ├── PaymentId (InstructionId, EndToEndId, UETR)
        ├── Amount (currency + value via ValueWithAttr pattern)
        ├── Debtor / Creditor (PartyIdentification → name, address, id)
        ├── DebtorAgent / CreditorAgent (BranchAndFinancialInstitutionId)
        ├── RemittanceInformation (structured or unstructured)
        └── ... (message-specific fields)
```

### IR Type Graph (codegen internal)

```rust
pub struct TypeGraph {
    pub namespace: String,
    pub root_elements: Vec<RootElement>,
    pub types: IndexMap<String, TypeDef>,  // Insertion-ordered
}

pub enum TypeDef {
    Struct(StructDef),        // xs:sequence → fields with cardinality
    Enum(EnumDef),            // xs:choice → newtype variants
    CodeEnum(CodeEnumDef),    // xs:enumeration → unit variants
    Newtype(NewtypeDef),      // xs:simpleType → constrained wrapper
    ValueWithAttr(ValueWithAttrDef),  // xs:simpleContent → $value + @attrs
    Opaque(OpaqueDef),        // xs:any → String wrapper
}

pub enum Constraint {
    MinLength(u32), MaxLength(u32), Pattern(String),
    MinInclusive(String), MaxInclusive(String),
    TotalDigits(u32), FractionDigits(u32),
}
```

### Validation Domain Model

```
RuleRegistry ─── HashMap<id, Box<dyn Rule>>
     │
     ▼
ConstraintSet ─── Vec<FieldConstraint { path, rule_ids[] }>
     │
     ▼
SchemaValidator ─── orchestrates field-level validation
     │
     ▼
SchemeValidator ─── message-level scheme rules (FedNow, SEPA, CBPR+)
     │
     ▼
ValidationResult ─── Vec<ValidationError { path, severity, rule_id, message }>
```

### MT Message Model

```
MtMessage
├── Block1 { app_id, service_id, lt_address, session, sequence }
├── Block2::Input { msg_type, destination, priority, ... }
│       or Block2::Output { msg_type, input_time, mir, ... }
├── Block3 { fields: Vec<(tag, value)> }  — optional
├── Block4 { fields: Vec<TagField { tag, value }> }  — required
└── Block5 { fields: Vec<(tag, value)> }  — optional
```

---

## 5. API Surface

### Parsing

```rust
use mx20022::parse::{de, ser, envelope};

// Detect message type (fast, no full parse)
let msg_id = envelope::detect_message_type(&xml)?;
println!("{}", msg_id.dotted()); // "pacs.008.001.13"

// Deserialize
let doc: pacs008::Document = de::from_str(&xml)?;

// Serialize
let xml_out = ser::to_string_with_declaration(&doc)?;

// Round-trip invariant: de::from_str(ser::to_string(&doc)) == doc
```

### Building Messages

```rust
use mx20022::model::generated::pacs::pacs_008_001_13::*;

let hdr = GroupHeader131::builder()
    .msg_id(Max35Text("MSG-001".into()))
    .cre_dt_tm(ISODateTime("2026-03-01T10:00:00".into()))
    .nb_of_txs(Max15NumericText("1".into()))
    .sttlm_inf(settlement)
    .build()?;  // Result<GroupHeader131, BuilderError>
```

Builders are flat (not closure-nested). Nested types are constructed separately and passed in. All required fields must be set or `build()` returns `BuilderError`.

### Validation

```rust
use mx20022::validate::rules::RuleRegistry;
use mx20022::validate::schemes::{fednow::FedNowValidator, SchemeValidator};

// Field-level rule validation
let registry = RuleRegistry::with_defaults();
let errors = registry.validate_field("GB82WEST12345698765432", "/iban", &["IBAN_CHECK"]);

// Scheme validation (XML path)
let fednow = FedNowValidator::new();
let result = fednow.validate(&xml, "pacs.008.001.13");

// Scheme validation (typed path)
let result = fednow.validate_typed(&doc as &dyn std::any::Any, "pacs.008.001.13");

// Typed constraint validation
use mx20022::model::common::validate::IsoMessage;
let violations = doc.validate_message();
```

### Translation

```rust
use mx20022::translate::mt;
use mx20022::translate::mappings::mt103_to_pacs008::mt103_to_pacs008;

// MT → MX
let msg = mt::parser::parse(&mt_text)?;
let mt103 = mt::fields::mt103::parse_mt103(&msg.block4)?;
let result = mt103_to_pacs008(&mt103, "MSG-001", "2026-03-01T10:00:00")?;
let doc = result.message;
for w in &result.warnings.warnings {
    eprintln!("Warning: {} — {}", w.field, w.message);
}

// MX → MT
use mx20022::translate::mappings::pacs008_to_mt103::pacs008_to_mt103;
let result = pacs008_to_mt103(&doc)?;
let mt_text = result.message;
```

### Newtype Construction (with validation)

```rust
// Validated construction
let currency = ActiveCurrencyCode::new("USD")?;  // Ok
let bad = ActiveCurrencyCode::new("US");          // Err(ConstraintError)

// Also via TryFrom
let code: ActiveCurrencyCode = "EUR".to_string().try_from()?;

// Direct field access (for serde compat)
let raw = ActiveCurrencyCode(arbitrary_string);  // No validation — use with caution
```

---

## 6. Design Patterns

### Code Generation over Hand-Writing

The single most important architectural decision. ISO 20022 defines 800+ message types. Hand-writing Rust types would be unmaintainable. Instead, a three-stage pipeline (XSD → IR → Rust) auto-generates all types from the official schemas. Adding a new message type means pointing the generator at an XSD file.

**Trade-off**: Generated code is committed to the repository (not build-time). This keeps builds fast and avoids proc-macro complexity, at the cost of repository size (~103K generated LoC).

### Newtype Pattern for Domain Primitives

Every XSD simple type becomes a Rust newtype: `Max35Text(String)`, `ISODateTime(String)`, `ActiveCurrencyCode(String)`. This prevents mixing semantically different string values at the type level. Constrained newtypes validate via `TryFrom<String>` / `new()` using inline checks compiled from XSD patterns — no runtime regex dependency.

### Builder Pattern (Runtime-Validated)

All generated structs expose `.builder()` returning a `{Name}Builder` with fluent setters and a `build() → Result<T, BuilderError>` that validates required fields.

**Why not typestate builders?** ISO 20022 types frequently have 50+ fields. Typestate would generate `2^N` intermediate types per struct — prohibitively complex for codegen. Runtime validation with clear error messages is the pragmatic choice.

### Trait-Based Validation

Validation uses trait objects throughout:
- `Rule` trait for individual field validators (IBAN, BIC, etc.)
- `SchemeValidator` trait for message-level scheme rules
- `Validatable` trait for self-validation of generated types

This enables:
- User-defined rules registered at runtime
- Custom scheme validators
- Third-party validation plugins

### ChoiceWrapper for XML Enum Compatibility

`xs:choice` elements (mutually exclusive alternatives) map naturally to Rust enums. However, quick-xml's serde integration requires a `$value`-renamed wrapper for enum fields in structs. `ChoiceWrapper<T>` provides this transparently with `Deref`/`DerefMut` for ergonomic access.

### Dual Validation Paths

Scheme validators implement both:
- `validate(&str, &str) → ValidationResult` — operates on raw XML via string scanning
- `validate_typed(&dyn Any, &str) → Option<ValidationResult>` — operates on deserialized types

The typed path is preferred (compile-time field access, no fragile string scanning). The XML path exists for backward compatibility and for validating messages that haven't been deserialized.

### Error Composition

Error types use `thiserror` derives consistently. Each crate defines its own error enum. Cross-crate errors use `#[from]` for transparent propagation:

```
TranslationError
├── MtParse(MtError)
├── MxParse(ParseError)
├── Builder(BuilderError)
├── MissingField { field, context }
├── InvalidFieldValue { field, detail }
└── UnsupportedMessageType(String)
```

### Layered Architecture

Each crate has a single responsibility and minimal coupling:

| Crate | Knows About | Does Not Know About |
|---|---|---|
| model | serde | XML parsing, validation rules, MT format |
| parse | model types, quick-xml | validation, translation |
| validate | model types (optional) | parsing details, translation |
| translate | model types, parse, MT format | validation rules |
| cli | all crates | (end consumer) |

---

## 7. Dependencies

### Production Dependencies

| Crate | Version | Used By | Purpose |
|---|---|---|---|
| `serde` | 1.x | model, parse, validate | Serialization framework |
| `quick-xml` | 0.37 | parse, codegen | XML parsing and serialization |
| `thiserror` | 2.x | parse, validate, translate | Error derive macros |
| `regex` | 1.x | validate | XSD pattern matching in validation rules |
| `indexmap` | 2.x | codegen | Ordered type maps in IR |
| `clap` | 4.x | cli | CLI argument parsing |
| `proc-macro2` | 1.x | codegen | Token stream manipulation |
| `quote` | 1.x | codegen | Rust code generation |
| `syn` | 2.x | codegen | Generated code validation |
| `prettyplease` | 0.2 | codegen | Code formatting |

### Development Dependencies

| Crate | Version | Purpose |
|---|---|---|
| `pretty_assertions` | 1.x | Readable test diffs |
| `insta` | 1.x (yaml) | Snapshot testing for generated code |
| `criterion` | 0.5 | Performance benchmarks |

### Dependency Minimization

- **model crate**: Zero production dependencies beyond serde. Pure data types.
- **parse crate**: Only quick-xml + serde + thiserror.
- **translate crate**: No additional dependencies — uses parse + model types internally.
- **validate crate**: regex is the only addition beyond thiserror.
- `unsafe` is forbidden workspace-wide (`unsafe_code = "forbid"`).

---

## 8. Error Handling

### Error Types by Crate

| Crate | Error Type | Variants |
|---|---|---|
| **codegen** | `xsd::ParseError` | `Xml`, `MissingAttribute`, `InvalidAttributeValue`, `MissingSchemaRoot`, `Utf8` |
| **codegen** | `ir::LowerError` | `UnknownBase { base, context }` |
| **model** | `BuilderError` | Struct: `{ type_name, missing_fields }` |
| **model** | `ConstraintError` | Struct: `{ kind: ConstraintKind, message }` |
| **parse** | `ParseError` | `Deserialize`, `Serialize`, `Io`, `InvalidEnvelope` |
| **validate** | `ValidationError` | Struct: `{ path, severity, rule_id, message }` |
| **translate** | `MtError` | `InvalidBlockStructure`, `InvalidBlockContent`, `MissingBlock`, `InvalidFieldTag`, `MissingField`, `InvalidFieldValue` |
| **translate** | `TranslationError` | `UnsupportedMessageType`, `MissingField`, `InvalidFieldValue`, `MtParse`, `MxParse`, `Builder` |

### Error Propagation Pattern

All error types implement `std::error::Error` + `Display` via `thiserror`. Cross-crate errors use `#[from]` for `?` propagation. The CLI catches all errors at the top level and formats them for human consumption with exit codes.

### Validation vs. Hard Errors

The library distinguishes between:
- **Hard errors** (`Result<T, E>`): Parse failures, missing blocks, invalid structure — the operation cannot complete
- **Soft findings** (`ValidationResult`): Rule violations with severity levels — the message exists but may not conform to rules

This separation is intentional: a syntactically valid XML message that violates FedNow rules should still parse successfully. Validation is a separate, opt-in step.

---

## 9. Testing Architecture

### Strategy

| Layer | Technique | Location |
|---|---|---|
| Unit tests | `#[cfg(test)]` modules | Per-crate `src/**/*.rs` |
| Integration tests | Cross-crate workflows | Per-crate `tests/` directories |
| Snapshot tests | insta YAML snapshots | codegen (generated code regression) |
| Fixture-based | Real XML/MT samples | `testdata/` (shared), `tests/fixtures/` (per-crate) |
| Benchmarks | criterion suites | `benches/` (parse, validate, translate) |

### Key Invariant

```
parse(serialize(msg)) == msg    // Round-trip identity for all generated types
```

### Test Fixtures

```
testdata/
├── xml/{head,pacs,pain,camt}/   # ISO 20022 XML messages
├── mt/                          # SWIFT MT messages (MT103, MT202, MT940)
├── translation/                 # Paired MT↔MX samples (input + expected)
└── schemes/{fednow,sepa,cbpr}/  # Scheme-specific valid/invalid samples
```

### Metrics

- **553 tests** (547 pass, 3 ignored, 0 fail as of v0.4)
- **Zero TODO/FIXME/unimplemented** in codebase
- CI runs on both stable and MSRV 1.75.0

---

## 10. Future Considerations

### Planned (v0.5–v0.6)

| Item | Description |
|---|---|
| **Ergonomic API layer** | Prelude module with type aliases, convenience `parse<T>()`/`to_xml<T>()` at top level |
| **Typed validation pipeline** | Migrate scheme validators from XML string scanning to `Validatable` trait path |
| **WASM bindings** | Separate `mx20022-wasm` crate via wasm-bindgen, target < 2MB bundle |
| **Python bindings** | Separate `mx20022-python` crate via PyO3/maturin, type stubs for mypy |

### Extensibility Points

- **New message types**: Point codegen at an XSD file → generates full type + builder + validation
- **Custom validation rules**: Implement `Rule` trait, register with `RuleRegistry`
- **Custom scheme validators**: Implement `SchemeValidator` trait
- **Custom MT mappings**: Follow the `helpers.rs` patterns for party/account/FI conversion

### Known Gaps

| Gap | Status | Resolution |
|---|---|---|
| Scheme validators use XML string scanning (fragile) | Typed path added, XML path retained for compat | v0.5: migrate to typed-only |
| Builder API is flat, not nested-closure style | Acceptable trade-off for codegen complexity | Possible `with_defaults()` convenience |
| Expert module paths required (no prelude) | DX friction | v0.5: prelude with type aliases |
| SEPA validator uses f64 for amount comparison | Precision risk | v0.5: integer-cent arithmetic |
| No Display for ValidationResult | Usability gap | v0.5: Display impls |
