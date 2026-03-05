-- seed-context.sql — Seed data for mx20022 agent context database v1
-- All inserts run as a single transaction for atomicity.

BEGIN TRANSACTION;

--------------------------------------------------------------------------------
-- PROTOCOL KNOWLEDGE (6 rows) — teaches agents how to use this database
--------------------------------------------------------------------------------

INSERT INTO knowledge (category, title, content) VALUES
('protocol', 'session-start', 'At the start of every session, run these queries to restore context:

1. SELECT title, content FROM knowledge WHERE category = ''protocol'' ORDER BY title;
2. SELECT id, name, status, maps_to FROM iterations ORDER BY id;
3. SELECT id, title, status, priority, iteration_id FROM items WHERE status IN (''in_progress'', ''blocked'') ORDER BY priority, id;
4. SELECT id, started_at, summary FROM sessions ORDER BY id DESC LIMIT 3;
5. SELECT e.entry_type, e.title, e.content FROM entries e JOIN sessions s ON e.session_id = s.id ORDER BY e.id DESC LIMIT 10;

Then create a new session row and begin logging.'),

('protocol', 'writing-entries', 'Log entries as you work using these types:

- decision: Architectural or design choices made. Include rationale and alternatives considered.
- progress: Tasks completed, milestones reached. Reference item IDs when applicable.
- blocker: Issues preventing progress. Include what you tried and what you need.
- discovery: Unexpected findings about the codebase, domain, or tools.
- note: General observations, reminders, or context for future sessions.

INSERT INTO entries (session_id, entry_type, title, content, item_id) VALUES (?, ?, ?, ?, ?);'),

('protocol', 'managing-items', 'Task lifecycle and status flow:

not_started → in_progress → complete
not_started → blocked → in_progress → complete
not_started → deferred
in_progress → blocked → in_progress → complete

When starting work on an item:
  UPDATE items SET status = ''in_progress'', updated_at = datetime(''now'') WHERE id = ?;

When completing:
  UPDATE items SET status = ''complete'', updated_at = datetime(''now'') WHERE id = ?;

Check dependencies before starting:
  SELECT d.depends_on, i.title, i.status FROM item_dependencies d JOIN items i ON d.depends_on = i.id WHERE d.item_id = ? AND i.status != ''complete'';

Find next available work:
  SELECT i.id, i.title, i.priority, it.name as iteration FROM items i JOIN iterations it ON i.iteration_id = it.id WHERE i.status = ''not_started'' AND NOT EXISTS (SELECT 1 FROM item_dependencies d JOIN items dep ON d.depends_on = dep.id WHERE d.item_id = i.id AND dep.status != ''complete'') ORDER BY it.id, i.priority, i.id;'),

('protocol', 'session-end', 'Before ending a session:

1. Update all in_progress items to reflect actual status.
2. Log a summary entry: INSERT INTO entries (session_id, entry_type, title, content) VALUES (?, ''note'', ''session-summary'', ?);
3. Update session: UPDATE sessions SET ended_at = datetime(''now''), summary = ? WHERE id = ?;
4. If multi-session work is ongoing, ensure the summary captures: what was done, what is next, any blockers or decisions pending.'),

('protocol', 'knowledge-categories', 'Knowledge table categories:

- protocol: How to use this database (these rows). Read first every session.
- architecture: System design decisions, crate structure, data flow.
- api: API contracts, function signatures, type sketches.
- reference: Lists and lookups (message types, mappings, milestones).
- config: Dependencies, tooling, Rust edition, MSRV.
- testing: Test strategy, fixture locations, coverage approach.
- domain: ISO 20022 background, SWIFT MT format, financial concepts.'),

('protocol', 'schema-info', 'Database schema v1 has 11 tables:

- schema_meta: Key-value version tracking.
- iterations: Project phases/milestones. Status: planned/active/completed/blocked.
- sessions: Work sessions with timestamps and summaries.
- entries: Typed log entries linked to sessions and optionally items.
- items: Granular tasks with status, priority, iteration linkage.
- item_dependencies: DAG of task prerequisites (item_id depends_on).
- concepts: Domain concept tags (e.g., codegen, xml-parsing).
- item_concepts: Many-to-many linking items to concepts.
- patterns: Reusable code patterns with examples.
- pattern_concepts: Many-to-many linking patterns to concepts.
- knowledge: Queryable knowledge base (this table).

Full DDL is in docs/agent-context-schema.sql.');

--------------------------------------------------------------------------------
-- PRD-DERIVED KNOWLEDGE (16 rows)
--------------------------------------------------------------------------------

INSERT INTO knowledge (category, title, content) VALUES
('architecture', 'workspace-structure', 'Cargo workspace with 7 crates:

- mx20022-codegen: XSD → IR → Rust code generator (build-time tool)
- mx20022-model: Generated types per message family, feature-gated
- mx20022-parse: XML parsing (quick-xml) and serialization
- mx20022-validate: Schema + business rule + scheme-specific validation
- mx20022-translate: MT ↔ MX bidirectional translation
- mx20022-cli: Command-line tool (clap-based)
- mx20022: Umbrella crate that re-exports everything

Dependency flow: codegen → model → parse → validate → translate → cli → umbrella
Feature flags gate message families: pacs, pain, camt, head, sese.'),

('architecture', 'codegen-strategy', 'XSD → Rust code generation pipeline:

1. Parse XSD using quick-xml (not a full XSD validator — targeted parser)
2. Build intermediate representation (IR) type graph:
   - ComplexType → struct fields with types, optionality, cardinality
   - SimpleType with restriction → newtype with validation
   - xs:choice → enum variants
   - xs:sequence → struct
   - Optional → Option<T>, repeating → Vec<T>
3. Resolve type references, inheritance (extension/restriction), and imports
4. Emit Rust: structs, enums, newtypes, serde derives, doc comments
5. Post-process with rustfmt

Key challenge: XSD uses complex type inheritance. Rust has no inheritance.
Strategy: Flatten inheritance via composition. Extension adds fields to parent struct.
Restriction narrows parent — emit subtype with tighter validation.

Builder generation: Runtime-validated (.build() → Result), not typestate.
Rationale: ISO 20022 types have 50+ fields; typestate would be prohibitively complex to generate.'),

('architecture', 'validation-layers', 'Three-layer validation architecture:

Layer 1 — Schema Validation:
  XML well-formedness, XSD type conformance, required elements, cardinality.
  Implemented in mx20022-validate, runs against generated type constraints.

Layer 2 — Business Rule Validation:
  IBAN check digits, BIC format, ISO 4217 currency codes, ISO 3166 country codes,
  LEI format, date/time consistency, amount/control sum checks.
  Pluggable rule registry pattern in mx20022-validate.

Layer 3 — Scheme-Specific Rules:
  FedNow: size limits, field restrictions, allowed BICs, value limits.
  SEPA: character set restrictions, purpose codes.
  CBPR+: usage guidelines, market practice rules.
  Users can register custom rule sets.

Output: Structured errors with XPath location, severity (error/warning/info),
rule ID, and human-readable message.'),

('architecture', 'feature-flags', 'Per-family feature gating in mx20022-model:

[features]
default = ["pacs"]
pacs = []        # pacs.002, .004, .008, .009, .028
pain = []        # pain.001, .002, .013
camt = []        # camt.029, .053, .054, .056
head = []        # head.001
all = ["pacs", "pain", "camt", "head"]

Users compile only the message families they need.
The umbrella mx20022 crate enables "all" by default.
CLI enables "all" since it must handle any message type.'),

('api', 'parsing-api', 'Parse API contract (mx20022-parse):

// Generic parse function
pub fn parse<T: DeserializeOwned>(xml: &str) -> Result<T, ParseError>;

// Type-specific convenience (generated)
pub fn parse_pacs008(xml: &str) -> Result<Pacs008, ParseError>;

// Serialization
pub fn to_xml<T: Serialize>(msg: &T) -> Result<String, SerializeError>;
pub fn to_xml_pretty<T: Serialize>(msg: &T) -> Result<String, SerializeError>;

ParseError includes: position, expected type, actual content, XPath-like path.
Uses quick-xml with serde for deserialization.
Zero-copy where feasible (Cow<str> for string fields in hot paths).'),

('api', 'builder-api', 'Builder API contract (generated by codegen):

let msg = Pacs008Builder::new()
    .message_id("MSG-2026-001")
    .creation_date_time(Utc::now())
    .number_of_transactions(1)
    .settlement_method(SettlementMethod::ClearingSystem)
    .credit_transfer(|tx| {
        tx.end_to_end_id("E2E-001")
          .amount(Currency::USD, 1500.00)
          .debtor(|d| d.name("Alice Corp").bic("ABCOUS33"))
          .creditor(|c| c.name("Bob Ltd").iban("DE89370400440532013000"))
    })
    .build()?;  // Returns Result<Pacs008, BuilderError>

Design: Runtime-validated builders. .build() returns Result.
Nested builders for complex sub-structures use closure pattern.
BuilderError enumerates missing required fields and constraint violations.'),

('api', 'validation-api', 'Validation API contract (mx20022-validate):

let validator = Validator::new()
    .with_schema_validation()
    .with_business_rules()
    .with_scheme_rules(Scheme::FedNow);

let result = validator.validate(&msg);

match result {
    Ok(()) => { /* valid */ }
    Err(errors) => {
        for e in &errors {
            // e.severity: Error | Warning | Info
            // e.message: human-readable
            // e.path: XPath-like location
            // e.rule_id: machine-readable rule identifier
        }
    }
}

Validator is composable — layer on rules incrementally.
Custom rules via: validator.with_custom_rule(my_rule_fn);'),

('api', 'translation-api', 'Translation API contract (mx20022-translate):

// MT → MX
let mt103 = mx20022::mt::parse_mt103(&raw_mt)?;
let pacs008: Pacs008 = translate::mt103_to_pacs008(&mt103)?;

// MX → MT (lossy — MX has richer structure)
let mt103: Mt103 = translate::pacs008_to_mt103(&pacs008)?;

Each mapping is a standalone, testable function.
Handles: field truncation, character set conversion (SWIFT ↔ UTF-8), data enrichment.
Lossy translations are documented; configurable behavior for ambiguous mappings.
Users can register custom field mappings for institution-specific extensions.'),

('reference', 'phase1-message-types', 'Phase 1 priority message types (13 total):

head.001 — Business Application Header (BAH)
pacs.002 — Payment Status Report
pacs.004 — Payment Return
pacs.008 — FI to FI Customer Credit Transfer
pacs.009 — FI to FI Financial Institution Credit Transfer
pacs.028 — FI to FI Payment Status Request
pain.001 — Customer Credit Transfer Initiation
pain.002 — Customer Payment Status Report
pain.013 — Creditor Payment Activation Request
camt.029 — Resolution of Investigation
camt.053 — Bank to Customer Statement
camt.054 — Bank to Customer Debit/Credit Notification
camt.056 — FI to FI Payment Cancellation Request

Families: head (1), pacs (5), pain (3), camt (4)'),

('reference', 'mt-mx-mappings', 'Priority MT ↔ MX translation pairs:

MT103     ↔ pacs.008  — Customer credit transfer
MT103 STP ↔ pacs.008  — Straight-through processing variant
MT202     ↔ pacs.009  — FI transfer
MT202 COV ↔ pacs.009  — Cover payment
MT900     ↔ camt.054  — Confirmation of debit
MT910     ↔ camt.054  — Confirmation of credit
MT940     ↔ camt.053  — Account statement
MT199/299 ↔ camt.056/029 — Cancellation and investigation

MT→MX gains structure (enrichment possible).
MX→MT loses structure (truncation, flattening).'),

('reference', 'release-plan', 'Release milestones:

v0.1 — Foundation:
  Code generator, head.001 + pacs.008 + pacs.002, schema validation, CLI (validate, inspect)

v0.2 — Core Payment Messages:
  All 13 Phase 1 types, builder API, business rule validation (IBAN, BIC, currency)

v0.3 — Translation:
  MT103 ↔ pacs.008, MT202 ↔ pacs.009, MT940 ↔ camt.053

v0.4 — Scheme Rules:
  FedNow, SEPA, CBPR+ validation rules. Docs, benchmarks, publish.

v0.5+ — Community-driven:
  More message families, WASM target, language bindings (PyO3, napi-rs)'),

('config', 'dependencies', 'Key crate dependencies:

Parsing:      quick-xml (XML), serde + serde_derive (serialization)
Validation:   thiserror (error types), iban_validate or custom, iso_currency
Codegen:      quick-xml (XSD reading), proc-macro2 + quote + syn (code emission)
CLI:          clap (arg parsing), colored or owo-colors (terminal output)
Testing:      pretty_assertions, insta (snapshot testing), criterion (benchmarks)
Translation:  (internal types only — no extra deps beyond parse/model)

Minimize dependencies. Prefer well-maintained, audited crates.'),

('config', 'rust-edition', 'Rust project configuration:

Edition: 2021
MSRV: 1.75.0 (for stable async traits, C-string literals)
License: Apache-2.0
Linting: clippy::pedantic with targeted allows
Formatting: rustfmt with default settings + imports_granularity = "Crate"'),

('testing', 'test-strategy', 'Test approach per layer:

Unit tests: Per-crate, testing individual functions. Use #[cfg(test)] modules.
Integration tests: Cross-crate workflows (parse → validate → serialize roundtrip).
Snapshot tests: Generated code captured via insta. Prevents accidental regressions.
Fixture-based: Real-world XML samples in tests/fixtures/ directories.
Property tests: Optional proptest for parser robustness (malformed input).
Benchmarks: criterion for parse, validate, serialize hot paths.
CI: cargo test --all-features, clippy, fmt check, cargo deny.

Key invariant: parse(to_xml(msg)) == msg for all generated types (roundtrip).'),

('domain', 'iso20022-overview', 'ISO 20022 is an international standard for financial messaging.

Key concepts:
- Messages defined in XSD schemas, organized by business area (pacs, pain, camt, etc.)
- Each message type has a 3-letter family + 3-digit number (e.g., pacs.008.001.10)
- The .001.10 suffix = variant.version. We target latest versions.
- Business Application Header (BAH / head.001) wraps every message.
- Messages are XML documents conforming to strict XSD schemas.
- The ISO 20022 catalogue has 800+ message definitions; we cover ~20 priority ones.

Industry context:
- Replacing legacy SWIFT MT (field-tag based) with richer XML structure.
- FedNow, SEPA, SWIFT CBPR+, CHIPS, CHAPS all mandate ISO 20022.
- Coexistence period (MT + MX) requires translation capabilities.'),

('domain', 'swift-mt-format', 'SWIFT MT message structure (5 blocks):

Block 1 — Basic Header: sender BIC, service ID, session/sequence numbers
  Format: {1:F01BANKBEBBAXXX0000000000}

Block 2 — Application Header: message type, receiver BIC, priority
  Input:  {2:I103BANKDEFFXXXXN}
  Output: {2:O1031200010103BANKBEBBAXXX00000000000101031200N}

Block 3 — User Header: optional fields (103: service ID, 113: banking priority, etc.)
  Format: {3:{103:CAT}{108:REF1234}}

Block 4 — Text Block: The message body. Field tags like :20:, :32A:, :50K:, etc.
  Format: {4:\r\n:20:REFERENCE\r\n:32A:260301USD1500,00\r\n...\r\n-}

Block 5 — Trailer: checksums, authentication
  Format: {5:{CHK:123456789ABC}}

MT103 key fields: :20: (ref), :23B: (bank op code), :32A: (value date/amount),
:50K: (ordering customer), :59: (beneficiary), :71A: (charges).');

--------------------------------------------------------------------------------
-- CONCEPTS (16)
--------------------------------------------------------------------------------

INSERT INTO concepts (name, description) VALUES
('codegen',           'XSD-to-Rust code generation pipeline'),
('xsd-parsing',       'Parsing ISO 20022 XSD schema files'),
('model-types',       'Generated Rust structs/enums for ISO 20022 messages'),
('xml-parsing',       'Parsing ISO 20022 XML messages into Rust types'),
('xml-serialization', 'Serializing Rust types back to ISO 20022 XML'),
('schema-validation', 'XSD schema conformance validation (layer 1)'),
('business-validation','Business rule validation — IBAN, BIC, currency (layer 2)'),
('scheme-validation', 'Scheme-specific rules — FedNow, SEPA, CBPR+ (layer 3)'),
('builder-api',       'Ergonomic builder pattern for message construction'),
('mt-parsing',        'SWIFT MT legacy message parsing'),
('mt-mx-translation', 'Bidirectional MT ↔ MX message translation'),
('cli',               'Command-line interface for validation, inspection, translation'),
('testing',           'Test infrastructure, fixtures, CI'),
('documentation',     'API docs, examples, guides'),
('ci-cd',             'Continuous integration and deployment'),
('workspace',         'Cargo workspace structure and configuration');

--------------------------------------------------------------------------------
-- PATTERNS (6)
--------------------------------------------------------------------------------

INSERT INTO patterns (name, description, example) VALUES
('error-types', 'Thiserror-based error enums per crate with structured context',
'#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("XML error at position {position}: {message}")]
    Xml { position: usize, message: String },
    #[error("unexpected element ''{element}'' at {path}")]
    UnexpectedElement { element: String, path: String },
}'),

('feature-gating', 'Cargo feature flags to conditionally compile message families',
'// In mx20022-model/Cargo.toml
[features]
default = ["pacs"]
pacs = []
pain = []
camt = []
head = []
all = ["pacs", "pain", "camt", "head"]

// In code
#[cfg(feature = "pacs")]
pub mod pacs;'),

('builder-pattern', 'Runtime-validated builders generated by codegen',
'pub struct Pacs008Builder {
    message_id: Option<String>,
    // ...
}
impl Pacs008Builder {
    pub fn new() -> Self { Self { message_id: None, /* ... */ } }
    pub fn message_id(mut self, v: impl Into<String>) -> Self {
        self.message_id = Some(v.into()); self
    }
    pub fn build(self) -> Result<Pacs008, BuilderError> {
        let message_id = self.message_id.ok_or(BuilderError::missing("message_id"))?;
        // ...
    }
}'),

('serde-xml', 'Serde derive with rename attributes for XML element names',
'#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "FIToFICstmrCdtTrf")]
pub struct Pacs008 {
    #[serde(rename = "GrpHdr")]
    pub group_header: GroupHeader,
    #[serde(rename = "CdtTrfTxInf")]
    pub credit_transfer_tx_info: Vec<CreditTransferTransaction>,
}'),

('test-fixtures', 'XML/MT fixtures in tests/fixtures/ with naming convention',
'tests/fixtures/
  pacs008_valid_basic.xml        — minimal valid message
  pacs008_valid_full.xml         — all optional fields populated
  pacs008_invalid_missing_hdr.xml — missing required GroupHeader
  pacs008_invalid_bad_iban.xml   — IBAN check digit failure
  mt103_basic.txt                — minimal MT103 message
  mt103_pacs008_pair/            — paired MT/MX for translation tests'),

('newtype-validation', 'Newtype wrappers for restricted XSD simple types',
'/// IBAN identifier — validated on construction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IbanIdentifier(String);

impl IbanIdentifier {
    pub fn new(value: impl Into<String>) -> Result<Self, ValidationError> {
        let s = value.into();
        // validate format and check digit
        Ok(Self(s))
    }
    pub fn as_str(&self) -> &str { &self.0 }
}');

--------------------------------------------------------------------------------
-- PATTERN ↔ CONCEPT LINKS
--------------------------------------------------------------------------------

-- error-types (pattern 1) relates to: xml-parsing, schema-validation, business-validation
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(1, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(1, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(1, (SELECT id FROM concepts WHERE name = 'business-validation'));

-- feature-gating (pattern 2) relates to: model-types, workspace
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(2, (SELECT id FROM concepts WHERE name = 'model-types')),
(2, (SELECT id FROM concepts WHERE name = 'workspace'));

-- builder-pattern (pattern 3) relates to: builder-api, codegen
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(3, (SELECT id FROM concepts WHERE name = 'builder-api')),
(3, (SELECT id FROM concepts WHERE name = 'codegen'));

-- serde-xml (pattern 4) relates to: xml-parsing, xml-serialization, model-types
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(4, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(4, (SELECT id FROM concepts WHERE name = 'xml-serialization')),
(4, (SELECT id FROM concepts WHERE name = 'model-types'));

-- test-fixtures (pattern 5) relates to: testing
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(5, (SELECT id FROM concepts WHERE name = 'testing'));

-- newtype-validation (pattern 6) relates to: model-types, schema-validation, codegen
INSERT INTO pattern_concepts (pattern_id, concept_id) VALUES
(6, (SELECT id FROM concepts WHERE name = 'model-types')),
(6, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(6, (SELECT id FROM concepts WHERE name = 'codegen'));

--------------------------------------------------------------------------------
-- ITERATIONS (12)
--------------------------------------------------------------------------------

INSERT INTO iterations (id, name, status, maps_to, description) VALUES
(1,  'project-bootstrap',       'planned', 'v0.1', 'Cargo workspace, CI, tooling, project infrastructure'),
(2,  'xsd-schema-management',   'planned', 'v0.1', 'Download XSD schemas, build XSD parser for IR generation'),
(3,  'code-generation-engine',  'planned', 'v0.1', 'IR-to-Rust code emission with serde derives and doc comments'),
(4,  'v01-parse-validate-cli',  'planned', 'v0.1', 'Parse/serialize/validate head.001, pacs.008, pacs.002 + CLI'),
(5,  'remaining-phase1-types',  'planned', 'v0.2', 'Generate remaining 10 Phase 1 message types with feature gates'),
(6,  'builder-api',             'planned', 'v0.2', 'Ergonomic builder code generation for all Phase 1 types'),
(7,  'business-rule-validation','planned', 'v0.2', 'IBAN, BIC, currency, cross-field business rule validation'),
(8,  'mt-parser',               'planned', 'v0.3', 'SWIFT MT message parser (blocks and field-level)'),
(9,  'mt-mx-translation',       'planned', 'v0.3', 'Bidirectional MT ↔ MX translation engine'),
(10, 'scheme-fednow',           'planned', 'v0.4', 'FedNow-specific validation rules and constraints'),
(11, 'scheme-sepa-cbpr',        'planned', 'v0.4', 'SEPA and CBPR+ scheme-specific validation rules'),
(12, 'release-polish',          'planned', 'v0.4', 'Documentation, benchmarks, CI hardening, crates.io publish');

--------------------------------------------------------------------------------
-- ITEMS (~120 tasks) organized by iteration
--------------------------------------------------------------------------------

-- ============================================================================
-- ITERATION 1: project-bootstrap (items 1–15)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(1,  1, 'Initialize Cargo workspace root',
    'Create root Cargo.toml with [workspace] members listing all 7 crates. Set resolver = "2", define workspace-level package metadata (edition, license, repository, rust-version).', 'critical'),
(2,  1, 'Create mx20022-codegen crate skeleton',
    'cargo init --lib crates/mx20022-codegen. Add quick-xml, proc-macro2, quote, syn deps. Stub lib.rs with module declarations.', 'critical'),
(3,  1, 'Create mx20022-model crate skeleton',
    'cargo init --lib crates/mx20022-model. Set up feature flags (pacs, pain, camt, head, all). Stub lib.rs with conditional module declarations.', 'critical'),
(4,  1, 'Create mx20022-parse crate skeleton',
    'cargo init --lib crates/mx20022-parse. Add quick-xml, serde deps. Depend on mx20022-model. Stub parse/serialize functions.', 'critical'),
(5,  1, 'Create mx20022-validate crate skeleton',
    'cargo init --lib crates/mx20022-validate. Add thiserror dep. Depend on mx20022-model. Stub Validator struct and error types.', 'high'),
(6,  1, 'Create mx20022-translate crate skeleton',
    'cargo init --lib crates/mx20022-translate. Depend on mx20022-model. Stub module structure for MT parsing and translation.', 'medium'),
(7,  1, 'Create mx20022-cli crate skeleton',
    'cargo init --bin crates/mx20022-cli. Add clap dep. Depend on parse, validate, translate. Stub main.rs with clap App definition.', 'medium'),
(8,  1, 'Create mx20022 umbrella crate',
    'cargo init --lib crates/mx20022. Re-export all sub-crates. Enable "all" features by default. Add crate-level documentation.', 'medium'),
(9,  1, 'Configure workspace-level dependencies',
    'Use [workspace.dependencies] in root Cargo.toml to centralize versions: quick-xml, serde, thiserror, clap, etc. Crates inherit via workspace = true.', 'high'),
(10, 1, 'Set up GitHub Actions CI',
    'Create .github/workflows/ci.yml: cargo check, cargo test --all-features, cargo clippy -- -D warnings, cargo fmt --check. Matrix: stable + MSRV.', 'high'),
(11, 1, 'Add rustfmt.toml configuration',
    'Create rustfmt.toml with imports_granularity = "Crate", edition = "2021". Ensure consistent formatting across all crates.', 'medium'),
(12, 1, 'Add clippy.toml configuration',
    'Create clippy.toml or workspace-level Cargo.toml [lints] section. Enable pedantic with targeted allows for ISO 20022 naming conventions.', 'medium'),
(13, 1, 'Set up cargo-deny configuration',
    'Create deny.toml: license allow list (Apache-2.0, MIT, BSD-*), ban duplicate versions, advisories check. Add to CI.', 'medium'),
(14, 1, 'Configure MSRV and edition',
    'Set rust-version = "1.75.0" and edition = "2021" in workspace Cargo.toml. Verify CI tests against MSRV.', 'high'),
(15, 1, 'Add workspace test infrastructure',
    'Create tests/ directory structure. Add pretty_assertions and insta as dev-dependencies. Set up test fixture directory conventions.', 'medium');

-- ============================================================================
-- ITERATION 2: xsd-schema-management (items 16–27)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(16, 2, 'Create XSD download and management script',
    'Script to download official ISO 20022 XSD schemas from iso20022.org. Store in schemas/ directory. Track schema versions. Handle head, pacs, pain, camt families.', 'high'),
(17, 2, 'Implement XSD top-level element parsing',
    'Parse xs:schema root element, extract targetNamespace, xmlns declarations. Parse top-level xs:element declarations with name and type attributes.', 'critical'),
(18, 2, 'Implement XSD simple type parsing',
    'Parse xs:simpleType definitions including xs:restriction (pattern, minLength, maxLength, enumeration, minInclusive, maxInclusive, totalDigits, fractionDigits).', 'critical'),
(19, 2, 'Implement XSD complex type parsing',
    'Parse xs:complexType with xs:sequence, xs:all. Extract element declarations with name, type, minOccurs, maxOccurs. Handle mixed content.', 'critical'),
(20, 2, 'Implement XSD choice group parsing',
    'Parse xs:choice groups with their child elements. Track minOccurs/maxOccurs on the choice itself and its members. Map to enum variant candidates in IR.', 'critical'),
(21, 2, 'Implement XSD attribute parsing',
    'Parse xs:attribute declarations (name, type, use=required/optional). Less common in ISO 20022 but needed for completeness.', 'medium'),
(22, 2, 'Implement XSD import and include handling',
    'Parse xs:import and xs:include elements. Resolve cross-schema type references. Build a unified type namespace from multiple XSD files.', 'high'),
(23, 2, 'Build intermediate representation (IR) type graph',
    'Design IR data structures: TypeNode enum (Struct, Enum, Newtype, Alias), Field (name, type_ref, optional, repeated), TypeRef (resolved/unresolved). Build graph from parsed XSD.', 'critical'),
(24, 2, 'Resolve type references and inheritance in IR',
    'Walk IR graph, resolve all type references to concrete definitions. Handle xs:extension (add fields to parent) and xs:restriction (narrow parent constraints). Detect and handle circular references.', 'critical'),
(25, 2, 'Handle XSD restriction/extension hierarchy',
    'Flatten inheritance chains: if B extends A extends Base, B''s IR struct has all fields from Base + A + B. Restrictions narrow validation constraints on inherited fields.', 'high'),
(26, 2, 'Add XSD parser unit tests',
    'Test each XSD parsing function against isolated XSD fragments. Cover: simple types with various restrictions, complex types with sequences and choices, imports, edge cases.', 'high'),
(27, 2, 'Create XSD test fixtures',
    'Create minimal XSD files exercising each pattern: simple restriction, complex sequence, choice group, extension, restriction, multi-file import. Store in tests/fixtures/xsd/.', 'high');

-- ============================================================================
-- ITERATION 3: code-generation-engine (items 28–39)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(28, 3, 'Design code emission module structure',
    'Create modules in mx20022-codegen: emitter.rs (main entry), types.rs (struct/enum emission), newtypes.rs (restricted types), serde_attrs.rs (XML rename attrs), docs.rs (doc comments).', 'high'),
(29, 3, 'Emit struct definitions from IR complex types',
    'Generate Rust struct with pub fields for each IR Struct node. Apply correct field types, Option<T> wrapping, Vec<T> for repeated. Add #[derive(Debug, Clone, Serialize, Deserialize)].', 'critical'),
(30, 3, 'Emit enum definitions from IR choice types',
    'Generate Rust enum with variants for each IR Enum node. Each variant wraps the choice element type. Add serde rename attributes for XML round-tripping.', 'critical'),
(31, 3, 'Emit newtype wrappers from IR restricted simple types',
    'Generate newtype structs (pub struct IbanId(String)) for restricted simple types. Add validation in new() constructor based on XSD pattern, length, enumeration constraints.', 'high'),
(32, 3, 'Generate serde rename attributes',
    'Add #[serde(rename = "XmlElementName")] to each struct and field. Handle namespace prefixes. Ensure XML element names match XSD exactly for round-trip fidelity.', 'critical'),
(33, 3, 'Generate Option<T> for optional elements',
    'Elements with minOccurs=0 emit as Option<T>. Add #[serde(skip_serializing_if = "Option::is_none")] to prevent empty tags in output XML.', 'high'),
(34, 3, 'Generate Vec<T> for repeating elements',
    'Elements with maxOccurs > 1 or maxOccurs="unbounded" emit as Vec<T>. Add appropriate serde attributes for sequence handling.', 'high'),
(35, 3, 'Generate doc comments from XSD annotations',
    'Extract xs:annotation/xs:documentation content from XSD. Emit as /// doc comments on generated structs, enums, and fields. Clean up HTML entities and formatting.', 'medium'),
(36, 3, 'Generate Display and Debug implementations',
    'Add Display impl for key types (message IDs, amounts, dates) showing human-friendly output. Debug is covered by derive. Add PartialEq, Eq where appropriate.', 'medium'),
(37, 3, 'Integrate rustfmt post-processing',
    'After code emission, run rustfmt on generated output. Handle rustfmt failures gracefully (emit unformatted with warning). Make rustfmt path configurable.', 'medium'),
(38, 3, 'End-to-end test: XSD → IR → Rust → compile',
    'Integration test that takes a real ISO 20022 XSD, runs full pipeline, writes Rust source, and compiles it. Verifies no panics, no compile errors. This is the critical confidence gate.', 'critical'),
(39, 3, 'Add snapshot tests for generated code',
    'Use insta to snapshot generated Rust code for key message types. Prevents accidental regressions in code generation. Review snapshots on any codegen changes.', 'high');

-- ============================================================================
-- ITERATION 4: v01-parse-validate-cli (items 40–54)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(40, 4, 'Generate model types for head.001',
    'Run codegen on head.001 XSD (Business Application Header). Verify generated types compile and have correct field structure. Place in mx20022-model/src/head/.', 'critical'),
(41, 4, 'Generate model types for pacs.008',
    'Run codegen on pacs.008 XSD (FI to FI Customer Credit Transfer). Most complex Phase 1 type. Verify field count, nested types, choice handling. Place in mx20022-model/src/pacs/.', 'critical'),
(42, 4, 'Generate model types for pacs.002',
    'Run codegen on pacs.002 XSD (Payment Status Report). Verify status code enums, grouping structures. Place in mx20022-model/src/pacs/.', 'critical'),
(43, 4, 'Implement XML parsing with quick-xml',
    'Implement parse<T>(xml: &str) -> Result<T, ParseError> using quick-xml serde deserialization. Handle namespaces, default namespace declarations, and ISO 20022 envelope structure.', 'critical'),
(44, 4, 'Implement XML serialization',
    'Implement to_xml<T>(msg: &T) -> Result<String, SerializeError> and to_xml_pretty variant. Output well-formed XML with correct namespace declarations matching ISO 20022 requirements.', 'critical'),
(45, 4, 'Add roundtrip parse/serialize tests',
    'For each generated type (head.001, pacs.008, pacs.002): parse fixture XML → serialize back → parse again → assert equal. This is the core correctness invariant.', 'critical'),
(46, 4, 'Implement schema validation layer',
    'Build Layer 1 validator in mx20022-validate: check required fields present, string length constraints, pattern matching, enumeration values. Use generated type metadata.', 'high'),
(47, 4, 'Add validation error types',
    'Define ValidationError with: path (XPath-like), severity (Error/Warning/Info), rule_id, message. Define ValidationResult as Vec<ValidationError>. Implement Display/Error.', 'high'),
(48, 4, 'Create XML fixtures for v0.1 types',
    'Create valid and invalid XML samples for head.001, pacs.008, pacs.002. Include: minimal valid, fully populated, missing required field, invalid field value, malformed XML.', 'high'),
(49, 4, 'Implement CLI skeleton with clap',
    'Set up clap-based CLI in mx20022-cli. Define subcommands: validate, inspect, translate (stubbed). Add global options: --format (json/text), --verbose.', 'high'),
(50, 4, 'Implement mx validate command',
    'CLI command: mx validate <file> [--schema] [--business-rules] [--scheme <name>]. Parse XML, run validators, output results. Exit code 0 = valid, 1 = errors found.', 'high'),
(51, 4, 'Implement mx inspect command',
    'CLI command: mx inspect <file>. Parse XML and display structured summary: message type, key fields (sender, receiver, amount, date), field count, nested structure overview.', 'high'),
(52, 4, 'Add CLI integration tests',
    'Test CLI commands against fixture files using assert_cmd or similar. Verify correct output format, exit codes, error handling for missing/invalid files.', 'medium'),
(53, 4, 'Write parse crate API documentation',
    'Add crate-level docs, module docs, and function docs to mx20022-parse. Include usage examples that compile (doctests). Cover error handling patterns.', 'medium'),
(54, 4, 'Write validate crate API documentation',
    'Add crate-level docs to mx20022-validate. Document Validator builder pattern, error types, how to add custom rules. Include doctests.', 'medium');

-- ============================================================================
-- ITERATION 5: remaining-phase1-types (items 55–68)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(55, 5, 'Generate model types for pacs.004',
    'Run codegen on pacs.004 XSD (Payment Return). Verify return reason codes, original transaction references. Place in mx20022-model/src/pacs/.', 'high'),
(56, 5, 'Generate model types for pacs.009',
    'Run codegen on pacs.009 XSD (FI to FI Financial Institution Credit Transfer). Verify interbank-specific fields. Place in mx20022-model/src/pacs/.', 'high'),
(57, 5, 'Generate model types for pacs.028',
    'Run codegen on pacs.028 XSD (Payment Status Request). Verify request structure and references. Place in mx20022-model/src/pacs/.', 'high'),
(58, 5, 'Generate model types for pain.001',
    'Run codegen on pain.001 XSD (Customer Credit Transfer Initiation). Complex type with batch payment support. Place in mx20022-model/src/pain/.', 'high'),
(59, 5, 'Generate model types for pain.002',
    'Run codegen on pain.002 XSD (Customer Payment Status Report). Verify status tracking types. Place in mx20022-model/src/pain/.', 'high'),
(60, 5, 'Generate model types for pain.013',
    'Run codegen on pain.013 XSD (Creditor Payment Activation Request). Request-to-pay message. Place in mx20022-model/src/pain/.', 'high'),
(61, 5, 'Generate model types for camt.053',
    'Run codegen on camt.053 XSD (Bank to Customer Statement). Complex reporting structure with balance and entries. Place in mx20022-model/src/camt/.', 'high'),
(62, 5, 'Generate model types for camt.054',
    'Run codegen on camt.054 XSD (Debit/Credit Notification). Event-driven notification type. Place in mx20022-model/src/camt/.', 'high'),
(63, 5, 'Generate model types for camt.056',
    'Run codegen on camt.056 XSD (Payment Cancellation Request). Cancellation workflow types. Place in mx20022-model/src/camt/.', 'high'),
(64, 5, 'Generate model types for camt.029',
    'Run codegen on camt.029 XSD (Resolution of Investigation). Investigation response types. Place in mx20022-model/src/camt/.', 'high'),
(65, 5, 'Add feature flags per message family',
    'Ensure mx20022-model Cargo.toml has feature flags: pacs, pain, camt, head. Each module guarded by #[cfg(feature = "...")]. Test compilation with individual features.', 'high'),
(66, 5, 'Add parse/serialize tests for all new types',
    'Roundtrip tests for all 10 new message types. Fixture-based: parse XML → serialize → parse → assert equal. Cover minimal and full variants.', 'high'),
(67, 5, 'Create XML fixtures for all new message types',
    'Valid and invalid XML samples for each of the 10 new types. Minimum: one valid minimal, one valid full, one with missing required field per type.', 'high'),
(68, 5, 'Update CLI to support all Phase 1 types',
    'Extend mx validate and mx inspect to detect and handle all 13 Phase 1 message types. Auto-detect message type from XML root element or BAH.', 'medium');

-- ============================================================================
-- ITERATION 6: builder-api (items 69–78)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(69, 6, 'Design builder code generation strategy',
    'Define how codegen produces builders: one Builder struct per message type, setter methods for each field, nested closure builders for complex sub-structures. .build() → Result<T, BuilderError>.', 'high'),
(70, 6, 'Generate builder structs for head.001',
    'Codegen emits Pacs008Builder etc. Start with head.001 as simplest case. Verify setter API, required field tracking, build validation.', 'high'),
(71, 6, 'Generate builders for pacs message types',
    'Generate builders for pacs.002, .004, .008, .009, .028. Handle complex nested structures (GroupHeader, CreditTransferTransaction, etc.) with nested builders.', 'high'),
(72, 6, 'Generate builders for pain message types',
    'Generate builders for pain.001, .002, .013. Handle batch payment structures in pain.001 with collection builders.', 'high'),
(73, 6, 'Generate builders for camt message types',
    'Generate builders for camt.029, .053, .054, .056. Handle statement entry collections and balance structures.', 'high'),
(74, 6, 'Add nested builder support',
    'Implement closure-based nested builders: .credit_transfer(|tx| tx.amount(...).debtor(|d| d.name(...))). Each nested builder validates its own required fields.', 'high'),
(75, 6, 'Add runtime validation in .build()',
    '.build() checks all required fields are set, validates constraints (string length, pattern, enumeration). Returns BuilderError listing all violations, not just the first.', 'high'),
(76, 6, 'Add builder usage examples',
    'Create examples/build_pacs008.rs and similar showing end-to-end message construction. Include in crate docs and README.', 'medium'),
(77, 6, 'Add builder unit tests',
    'Test each builder: successful build, missing required field, invalid field value, nested builder errors. Test error messages are descriptive.', 'high'),
(78, 6, 'Integration test: build → serialize → parse roundtrip',
    'Build a message with the builder API, serialize to XML, parse it back, compare with original. Proves builder and parser agree on structure.', 'critical');

-- ============================================================================
-- ITERATION 7: business-rule-validation (items 79–89)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(79, 7, 'Implement IBAN validation',
    'Validate IBAN format: country code (2 alpha) + check digits (2 numeric) + BBAN. Verify check digit using mod-97 algorithm. Support all ISO 13616 countries.', 'critical'),
(80, 7, 'Implement BIC validation',
    'Validate BIC/SWIFT code format: 4 alpha (bank) + 2 alpha (country) + 2 alphanum (location) + optional 3 alphanum (branch). 8 or 11 characters.', 'critical'),
(81, 7, 'Implement ISO 4217 currency code validation',
    'Validate 3-letter currency codes against ISO 4217 list. Include active currencies. Optionally validate numeric codes and decimal places.', 'high'),
(82, 7, 'Implement ISO 3166 country code validation',
    'Validate 2-letter country codes against ISO 3166-1 alpha-2 list. Include all currently assigned codes.', 'high'),
(83, 7, 'Implement LEI format validation',
    'Validate Legal Entity Identifier: 20 alphanumeric characters. First 4 = LOU, next 2 = reserved (00), next 12 = entity, last 2 = check digits (mod-97-10).', 'medium'),
(84, 7, 'Implement amount/control sum consistency',
    'Validate that sum of individual transaction amounts equals the control sum in the group header. Handle currency precision correctly (2 or 3 decimal places).', 'high'),
(85, 7, 'Implement date/time consistency rules',
    'Validate: creation date not in future, settlement date not before creation date, dates within reasonable range. Handle timezone-aware and naive datetimes.', 'medium'),
(86, 7, 'Create validation rule registry',
    'Design pluggable rule system: trait ValidationRule { fn validate(&self, msg: &dyn Any) -> Vec<ValidationError>; }. Rules register by message type. Users add custom rules.', 'high'),
(87, 7, 'Add structured validation error output',
    'Ensure all business rules produce ValidationError with: path (field location), severity, rule_id (e.g., "BR-IBAN-01"), message. Support JSON output format for CLI.', 'high'),
(88, 7, 'Add business rule validation tests',
    'Test each rule with valid inputs, boundary cases, and known-invalid inputs. Use real-world IBAN/BIC examples. Test rule registry composition.', 'high'),
(89, 7, 'Integration test: parse → validate pipeline',
    'Parse real XML fixture, run full validation stack (schema + business rules), verify expected errors/warnings are produced with correct paths and rule IDs.', 'high');

-- ============================================================================
-- ITERATION 8: mt-parser (items 90–100)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(90, 8, 'Define MT message block structure types',
    'Create Rust types for MT message blocks: Block1 (BasicHeader), Block2 (ApplicationHeader, Input/Output variants), Block3 (UserHeader), Block4 (TextBlock), Block5 (Trailer).', 'critical'),
(91, 8, 'Implement Block 1 (basic header) parser',
    'Parse {1:F01BANKBEBBAXXX0000000000} format. Extract: application ID, service ID, LT address (BIC + branch), session number, sequence number.', 'high'),
(92, 8, 'Implement Block 2 (application header) parser',
    'Parse Input {2:I103BANKDEFFXXXXN} and Output {2:O103...} formats. Extract: message type, destination, priority, delivery monitoring, obsolescence period.', 'high'),
(93, 8, 'Implement Block 3 (user header) parser',
    'Parse {3:{103:CAT}{108:REF1234}{121:uuid}} format. Extract tag-value pairs. Handle known tags: 103 (service ID), 108 (MUR), 111 (service type), 121 (UETR).', 'high'),
(94, 8, 'Implement Block 4 (text block) parser',
    'Parse message body between {4:\r\n and \r\n-}. Split into fields by :tag: markers. Handle multi-line field values. Core of MT parsing.', 'critical'),
(95, 8, 'Implement Block 5 (trailer) parser',
    'Parse {5:{CHK:...}{TNG:}{PDE:...}} format. Extract checksum, training flag, possible duplicate emission, delayed message indicators.', 'medium'),
(96, 8, 'Implement MT103 field-level parser',
    'Parse Block 4 fields specific to MT103: :20: (ref), :23B: (bank op code), :32A: (value date + currency + amount), :50K:/:50F: (ordering customer), :59:/:59F: (beneficiary), :71A: (charges), :70: (remittance info). Map to typed MT103 struct.', 'critical'),
(97, 8, 'Implement MT202 field-level parser',
    'Parse Block 4 fields for MT202: :20: (ref), :21: (related ref), :32A: (value date + amount), :52A: (ordering institution), :58A: (beneficiary institution). Map to typed MT202 struct.', 'high'),
(98, 8, 'Implement MT940 field-level parser',
    'Parse Block 4 fields for MT940: :20: (ref), :25: (account), :28C: (statement number), :60F: (opening balance), :61: (statement line), :62F: (closing balance), :86: (info to account owner). Handle multi-line :86: fields.', 'high'),
(99, 8, 'Add MT parser test fixtures',
    'Create MT103, MT202, MT940 test messages covering: minimal valid, full fields, multi-transaction MT940, edge cases (special characters, max length fields). Store in tests/fixtures/mt/.', 'high'),
(100, 8, 'Add MT parser unit tests',
    'Test each block parser and field parser independently. Test full message parsing end-to-end. Cover error cases: malformed blocks, unknown fields, truncated messages.', 'high');

-- ============================================================================
-- ITERATION 9: mt-mx-translation (items 101–111)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(101, 9, 'Design translation mapping framework',
    'Create trait TranslationMapping<From, To> with translate(&self, from: &From) -> Result<To, TranslationError>. Support field-level mapping configuration and custom overrides.', 'high'),
(102, 9, 'Implement MT103 → pacs.008 translation',
    'Map MT103 fields to pacs.008 structure: :50K: → Debtor, :59: → Creditor, :32A: → InterbankSettlementAmount + Date, :20: → EndToEndIdentification. Handle field enrichment.', 'critical'),
(103, 9, 'Implement pacs.008 → MT103 translation',
    'Reverse mapping. Handle data loss: MX has richer structure than MT. Truncate long fields to MT limits (16x for references, 4*35x for names). Document lossy fields.', 'critical'),
(104, 9, 'Implement MT202 → pacs.009 translation',
    'Map MT202 fields to pacs.009. Simpler than MT103 mapping (fewer fields). Handle COV (cover payment) variant with underlying transaction info.', 'high'),
(105, 9, 'Implement pacs.009 → MT202 translation',
    'Reverse mapping for pacs.009 → MT202. Handle cover payment detection and appropriate MT202COV generation.', 'high'),
(106, 9, 'Implement MT940 → camt.053 translation',
    'Map MT940 statement structure to camt.053. Complex: :61: lines → Entry elements, :60F:/:62F: balances → Balance elements, :86: → EntryDetails. Multi-entry handling.', 'high'),
(107, 9, 'Implement camt.053 → MT940 translation',
    'Reverse mapping. Significant data loss — camt.053 is far richer. Truncate entry details, flatten structured remittance info. Document limitations.', 'high'),
(108, 9, 'Handle character set conversion',
    'SWIFT MT uses a restricted character set (SWIFT-X, SWIFT-Y, SWIFT-Z). ISO 20022 XML uses UTF-8. Implement bidirectional conversion with transliteration for unsupported chars.', 'high'),
(109, 9, 'Handle field truncation and data loss',
    'Define TranslationWarning type for documenting data loss. Track which fields were truncated, what data was dropped, suggest manual review. Include in translation result.', 'medium'),
(110, 9, 'Add translation tests with paired samples',
    'Create paired MT/MX samples in tests/fixtures/translation/. For each pair: translate MT→MX, compare with expected MX. Translate MX→MT, compare with expected MT. Roundtrip where possible.', 'critical'),
(111, 9, 'Add mx translate CLI command',
    'CLI command: mx translate --from mt103 --to pacs008 <file>. Auto-detect input format. Output translated message. Show warnings for data loss. Support --format for output.', 'high');

-- ============================================================================
-- ITERATION 10: scheme-fednow (items 112–117)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(112, 10, 'Research FedNow-specific rules',
    'Study FedNow ISO 20022 Implementation Guide. Document message size limits, restricted fields, value limits, allowed message types, BIC requirements, character set restrictions.', 'critical'),
(113, 10, 'Implement FedNow message size limits',
    'FedNow has XML message size limits. Validate serialized message size against limit. Report clear error with current vs. max size.', 'high'),
(114, 10, 'Implement FedNow field restrictions',
    'Certain fields are mandatory in FedNow that are optional in base ISO 20022. Others are restricted to specific values. Implement as Layer 3 validation rules.', 'high'),
(115, 10, 'Implement FedNow value limits',
    'FedNow has per-transaction and daily value limits. Validate amounts against current limits. Make limits configurable for testing and future changes.', 'high'),
(116, 10, 'Add FedNow test fixtures',
    'Create XML fixtures conforming to FedNow requirements. Include: valid FedNow pacs.008, invalid (exceeds size), invalid (missing required FedNow field), invalid (amount too high).', 'high'),
(117, 10, 'Integration test: full FedNow validation',
    'End-to-end test: parse FedNow message → schema validation → business rules → FedNow scheme rules. Verify all layers work together correctly.', 'high');

-- ============================================================================
-- ITERATION 11: scheme-sepa-cbpr (items 118–124)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(118, 11, 'Research SEPA-specific rules',
    'Study EPC SEPA implementation guides. Document: allowed character set (Latin + limited special chars), purpose code requirements, SEPA-specific mandatory fields, amount limits.', 'critical'),
(119, 11, 'Implement SEPA character set restrictions',
    'SEPA restricts characters to a subset of UTF-8 (Basic Latin, no special chars beyond limited set). Validate all text fields. Report specific character violations.', 'high'),
(120, 11, 'Implement SEPA purpose code requirements',
    'SEPA requires CategoryPurpose or Purpose codes for certain transaction types. Validate presence and values against SEPA-allowed code lists.', 'high'),
(121, 11, 'Research CBPR+ usage guidelines',
    'Study SWIFT CBPR+ Implementation Guide. Document: mandatory usage rules, restricted elements, translation rules from MT, header requirements.', 'critical'),
(122, 11, 'Implement CBPR+ validation rules',
    'CBPR+ adds constraints on top of base ISO 20022: mandatory fields, restricted code values, structured address requirements. Implement as Layer 3 rules.', 'high'),
(123, 11, 'Add SEPA and CBPR+ test fixtures',
    'Create XML fixtures for SEPA and CBPR+ variants. Include: valid SEPA pacs.008, invalid char set, missing purpose code, valid CBPR+ message, CBPR+ violations.', 'high'),
(124, 11, 'Integration test: SEPA and CBPR+ validation',
    'End-to-end tests for both scheme validators. Verify correct error reporting, no false positives on valid messages, proper scheme identification.', 'high');

-- ============================================================================
-- ITERATION 12: release-polish (items 125–131)
-- ============================================================================

INSERT INTO items (id, iteration_id, title, description, priority) VALUES
(125, 12, 'Write comprehensive crate-level documentation',
    'Each crate gets thorough top-level docs: purpose, usage examples, feature flags, error handling. The umbrella crate docs serve as the main entry point with getting-started guide.', 'high'),
(126, 12, 'Create examples directory',
    'Create examples/ with runnable programs: parse_message.rs, build_message.rs, validate_message.rs, translate_mt_to_mx.rs. Each demonstrates a core workflow end-to-end.', 'high'),
(127, 12, 'Add performance benchmarks',
    'Use criterion to benchmark: XML parse, XML serialize, validation (all layers), translation. Target: < 100μs for pacs.008 parse+validate. Benchmark across message sizes.', 'medium'),
(128, 12, 'Set up cargo-publish workflow',
    'GitHub Actions workflow to publish to crates.io on tag push. Publish crates in dependency order. Verify all crate metadata (description, keywords, categories, readme).', 'medium'),
(129, 12, 'Write CHANGELOG',
    'Create CHANGELOG.md following Keep a Changelog format. Document all changes for v0.1 through v0.4. Include breaking changes, new features, bug fixes.', 'medium'),
(130, 12, 'Final CI/CD hardening',
    'Add cargo deny check, cargo audit, MSRV verification, doc build test, example compilation test. Ensure CI catches all regressions before merge.', 'high'),
(131, 12, 'Create GitHub release automation',
    'Automate GitHub Releases with changelogs from tagged commits. Include pre-built CLI binaries for major platforms (Linux, macOS, Windows) via cross-compilation.', 'medium');

--------------------------------------------------------------------------------
-- ITEM DEPENDENCIES (critical path and logical ordering)
--------------------------------------------------------------------------------

-- Iteration 1: crate skeletons depend on workspace root
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(2, 1), (3, 1), (4, 1), (5, 1), (6, 1), (7, 1), (8, 1);
-- Workspace deps depend on all skeletons
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(9, 2), (9, 3), (9, 4), (9, 5), (9, 6), (9, 7), (9, 8);
-- CI and test infra depend on workspace compiling
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(10, 9), (15, 9);
-- Lint/fmt configs depend on workspace root
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(11, 1), (12, 1), (13, 1), (14, 1);

-- Iteration 2: XSD parsing depends on codegen crate
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(16, 2), (17, 2), (18, 2), (19, 2), (20, 2), (21, 2), (22, 2);
-- IR graph depends on all XSD parsers
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(23, 17), (23, 18), (23, 19), (23, 20), (23, 21), (23, 22);
-- Type resolution → inheritance handling chain
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(24, 23), (25, 24);
-- Tests depend on parser implementations + test infra
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(26, 17), (26, 18), (26, 19), (26, 20);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(27, 15);

-- Iteration 3: code emission depends on IR being complete
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(28, 25);
-- All emission modules depend on design
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(29, 28), (30, 28), (31, 28), (32, 28), (33, 28), (34, 28), (35, 28), (36, 28);
-- rustfmt depends on emission existing
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(37, 29);
-- E2E test depends on all emission + model crate skeleton
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(38, 29), (38, 30), (38, 31), (38, 32), (38, 33), (38, 34), (38, 37), (38, 3);
-- Snapshot tests depend on E2E passing
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(39, 38);

-- Iteration 4: model generation depends on codegen E2E
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(40, 38), (41, 38), (42, 38);
-- Parse depends on model types + parse crate skeleton
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(43, 40), (43, 41), (43, 42), (43, 4);
-- Serialize depends on parse
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(44, 43);
-- Roundtrip tests depend on both
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(45, 43), (45, 44);
-- Validation error types depend on validate crate
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(47, 5);
-- Schema validation depends on model types + validate crate + error types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(46, 40), (46, 41), (46, 42), (46, 5), (46, 47);
-- Fixtures depend on model types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(48, 40), (48, 41), (48, 42);
-- CLI skeleton depends on cli crate + parse
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(49, 7), (49, 43);
-- CLI commands depend on skeleton + relevant features
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(50, 49), (50, 46);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(51, 49), (51, 43);
-- CLI tests depend on commands
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(52, 50), (52, 51);
-- Docs depend on implementations
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(53, 43), (53, 44);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(54, 46);

-- Iteration 5: all remaining types depend on codegen E2E
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(55, 38), (56, 38), (57, 38), (58, 38), (59, 38), (60, 38),
(61, 38), (62, 38), (63, 38), (64, 38);
-- Feature flags depend on all types existing
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(65, 55), (65, 56), (65, 57), (65, 58), (65, 59), (65, 60),
(65, 61), (65, 62), (65, 63), (65, 64);
-- Tests depend on feature flags + parse
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(66, 65), (66, 43);
-- Fixtures depend on all new types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(67, 55), (67, 56), (67, 57), (67, 58), (67, 59), (67, 60),
(67, 61), (67, 62), (67, 63), (67, 64);
-- CLI update depends on feature flags + existing CLI validate
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(68, 65), (68, 50);

-- Iteration 6: builder design depends on codegen + all types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(69, 38), (69, 65);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(70, 69);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(71, 69), (71, 65);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(72, 69), (72, 65);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(73, 69), (73, 65);
-- Nested builders depend on basic builders
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(74, 70);
-- Build validation + docs + tests depend on nested builders
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(75, 74);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(76, 75);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(77, 75);
-- Roundtrip integration test depends on builders + serialization
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(78, 75), (78, 43), (78, 44);

-- Iteration 7: business rules depend on schema validation layer
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(79, 46), (80, 46), (81, 46), (82, 46), (83, 46), (84, 46), (85, 46);
-- Rule registry depends on basic rules existing
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(86, 79), (86, 80);
-- Structured output depends on registry
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(87, 86);
-- Tests depend on rules
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(88, 79), (88, 80), (88, 81), (88, 82);
-- Integration depends on all rules + parse
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(89, 87), (89, 88), (89, 43);

-- Iteration 8: MT block types depend on translate crate
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(90, 6);
-- Block parsers depend on types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(91, 90), (92, 90), (93, 90), (94, 90), (95, 90);
-- Field parsers depend on block 4 (text block) parser
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(96, 94), (97, 94), (98, 94);
-- Tests and fixtures depend on parsers
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(99, 96), (99, 97), (99, 98);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(100, 96), (100, 97), (100, 98);

-- Iteration 9: translation framework depends on MT parser + MX parse
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(101, 96), (101, 43);
-- Individual translations depend on framework + relevant parsers + MX types
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(102, 101), (102, 96), (102, 41);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(103, 101), (103, 41);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(104, 101), (104, 97), (104, 56);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(105, 101), (105, 56);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(106, 101), (106, 98), (106, 61);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(107, 101), (107, 61);
-- Char set conversion + data loss tracking
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(108, 101);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(109, 102);
-- Translation tests depend on all translations
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(110, 102), (110, 103), (110, 104), (110, 105), (110, 106), (110, 107);
-- CLI translate depends on translations + CLI skeleton
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(111, 102), (111, 49);

-- Iteration 10: FedNow depends on validation framework
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(112, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(113, 112), (113, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(114, 112), (114, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(115, 112), (115, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(116, 113), (116, 114), (116, 115);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(117, 116);

-- Iteration 11: SEPA/CBPR+ depends on validation framework
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(118, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(119, 118), (119, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(120, 118), (120, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(121, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(122, 121), (122, 86);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(123, 119), (123, 120), (123, 122);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(124, 123);

-- Iteration 12: polish depends on major features complete
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(125, 54), (125, 78), (125, 89);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(126, 78), (126, 89), (126, 111);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(127, 45), (127, 89);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(128, 125);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(129, 125);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(130, 10), (130, 13);
INSERT INTO item_dependencies (item_id, depends_on) VALUES
(131, 128);

--------------------------------------------------------------------------------
-- ITEM ↔ CONCEPT LINKS
--------------------------------------------------------------------------------

-- Iteration 1: workspace setup
INSERT INTO item_concepts (item_id, concept_id) VALUES
(1,  (SELECT id FROM concepts WHERE name = 'workspace')),
(2,  (SELECT id FROM concepts WHERE name = 'codegen')),
(2,  (SELECT id FROM concepts WHERE name = 'workspace')),
(3,  (SELECT id FROM concepts WHERE name = 'model-types')),
(3,  (SELECT id FROM concepts WHERE name = 'workspace')),
(4,  (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(4,  (SELECT id FROM concepts WHERE name = 'workspace')),
(5,  (SELECT id FROM concepts WHERE name = 'schema-validation')),
(5,  (SELECT id FROM concepts WHERE name = 'workspace')),
(6,  (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(6,  (SELECT id FROM concepts WHERE name = 'workspace')),
(7,  (SELECT id FROM concepts WHERE name = 'cli')),
(7,  (SELECT id FROM concepts WHERE name = 'workspace')),
(8,  (SELECT id FROM concepts WHERE name = 'workspace')),
(9,  (SELECT id FROM concepts WHERE name = 'workspace')),
(10, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(11, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(12, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(13, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(14, (SELECT id FROM concepts WHERE name = 'workspace')),
(15, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 2: XSD parsing
INSERT INTO item_concepts (item_id, concept_id) VALUES
(16, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(17, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(18, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(19, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(20, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(21, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(22, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(23, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(23, (SELECT id FROM concepts WHERE name = 'codegen')),
(24, (SELECT id FROM concepts WHERE name = 'codegen')),
(25, (SELECT id FROM concepts WHERE name = 'codegen')),
(26, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(26, (SELECT id FROM concepts WHERE name = 'testing')),
(27, (SELECT id FROM concepts WHERE name = 'xsd-parsing')),
(27, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 3: code generation
INSERT INTO item_concepts (item_id, concept_id) VALUES
(28, (SELECT id FROM concepts WHERE name = 'codegen')),
(29, (SELECT id FROM concepts WHERE name = 'codegen')),
(29, (SELECT id FROM concepts WHERE name = 'model-types')),
(30, (SELECT id FROM concepts WHERE name = 'codegen')),
(30, (SELECT id FROM concepts WHERE name = 'model-types')),
(31, (SELECT id FROM concepts WHERE name = 'codegen')),
(31, (SELECT id FROM concepts WHERE name = 'model-types')),
(32, (SELECT id FROM concepts WHERE name = 'codegen')),
(32, (SELECT id FROM concepts WHERE name = 'xml-serialization')),
(33, (SELECT id FROM concepts WHERE name = 'codegen')),
(34, (SELECT id FROM concepts WHERE name = 'codegen')),
(35, (SELECT id FROM concepts WHERE name = 'codegen')),
(35, (SELECT id FROM concepts WHERE name = 'documentation')),
(36, (SELECT id FROM concepts WHERE name = 'codegen')),
(37, (SELECT id FROM concepts WHERE name = 'codegen')),
(38, (SELECT id FROM concepts WHERE name = 'codegen')),
(38, (SELECT id FROM concepts WHERE name = 'testing')),
(39, (SELECT id FROM concepts WHERE name = 'codegen')),
(39, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 4: parse/validate/CLI
INSERT INTO item_concepts (item_id, concept_id) VALUES
(40, (SELECT id FROM concepts WHERE name = 'model-types')),
(40, (SELECT id FROM concepts WHERE name = 'codegen')),
(41, (SELECT id FROM concepts WHERE name = 'model-types')),
(41, (SELECT id FROM concepts WHERE name = 'codegen')),
(42, (SELECT id FROM concepts WHERE name = 'model-types')),
(42, (SELECT id FROM concepts WHERE name = 'codegen')),
(43, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(44, (SELECT id FROM concepts WHERE name = 'xml-serialization')),
(45, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(45, (SELECT id FROM concepts WHERE name = 'xml-serialization')),
(45, (SELECT id FROM concepts WHERE name = 'testing')),
(46, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(47, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(48, (SELECT id FROM concepts WHERE name = 'testing')),
(49, (SELECT id FROM concepts WHERE name = 'cli')),
(50, (SELECT id FROM concepts WHERE name = 'cli')),
(50, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(51, (SELECT id FROM concepts WHERE name = 'cli')),
(51, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(52, (SELECT id FROM concepts WHERE name = 'cli')),
(52, (SELECT id FROM concepts WHERE name = 'testing')),
(53, (SELECT id FROM concepts WHERE name = 'documentation')),
(53, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(54, (SELECT id FROM concepts WHERE name = 'documentation')),
(54, (SELECT id FROM concepts WHERE name = 'schema-validation'));

-- Iteration 5: remaining types
INSERT INTO item_concepts (item_id, concept_id) VALUES
(55, (SELECT id FROM concepts WHERE name = 'model-types')),
(55, (SELECT id FROM concepts WHERE name = 'codegen')),
(56, (SELECT id FROM concepts WHERE name = 'model-types')),
(56, (SELECT id FROM concepts WHERE name = 'codegen')),
(57, (SELECT id FROM concepts WHERE name = 'model-types')),
(57, (SELECT id FROM concepts WHERE name = 'codegen')),
(58, (SELECT id FROM concepts WHERE name = 'model-types')),
(58, (SELECT id FROM concepts WHERE name = 'codegen')),
(59, (SELECT id FROM concepts WHERE name = 'model-types')),
(59, (SELECT id FROM concepts WHERE name = 'codegen')),
(60, (SELECT id FROM concepts WHERE name = 'model-types')),
(60, (SELECT id FROM concepts WHERE name = 'codegen')),
(61, (SELECT id FROM concepts WHERE name = 'model-types')),
(61, (SELECT id FROM concepts WHERE name = 'codegen')),
(62, (SELECT id FROM concepts WHERE name = 'model-types')),
(62, (SELECT id FROM concepts WHERE name = 'codegen')),
(63, (SELECT id FROM concepts WHERE name = 'model-types')),
(63, (SELECT id FROM concepts WHERE name = 'codegen')),
(64, (SELECT id FROM concepts WHERE name = 'model-types')),
(64, (SELECT id FROM concepts WHERE name = 'codegen')),
(65, (SELECT id FROM concepts WHERE name = 'model-types')),
(65, (SELECT id FROM concepts WHERE name = 'workspace')),
(66, (SELECT id FROM concepts WHERE name = 'testing')),
(66, (SELECT id FROM concepts WHERE name = 'xml-parsing')),
(67, (SELECT id FROM concepts WHERE name = 'testing')),
(68, (SELECT id FROM concepts WHERE name = 'cli'));

-- Iteration 6: builder API
INSERT INTO item_concepts (item_id, concept_id) VALUES
(69, (SELECT id FROM concepts WHERE name = 'builder-api')),
(69, (SELECT id FROM concepts WHERE name = 'codegen')),
(70, (SELECT id FROM concepts WHERE name = 'builder-api')),
(71, (SELECT id FROM concepts WHERE name = 'builder-api')),
(72, (SELECT id FROM concepts WHERE name = 'builder-api')),
(73, (SELECT id FROM concepts WHERE name = 'builder-api')),
(74, (SELECT id FROM concepts WHERE name = 'builder-api')),
(75, (SELECT id FROM concepts WHERE name = 'builder-api')),
(76, (SELECT id FROM concepts WHERE name = 'builder-api')),
(76, (SELECT id FROM concepts WHERE name = 'documentation')),
(77, (SELECT id FROM concepts WHERE name = 'builder-api')),
(77, (SELECT id FROM concepts WHERE name = 'testing')),
(78, (SELECT id FROM concepts WHERE name = 'builder-api')),
(78, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 7: business validation
INSERT INTO item_concepts (item_id, concept_id) VALUES
(79, (SELECT id FROM concepts WHERE name = 'business-validation')),
(80, (SELECT id FROM concepts WHERE name = 'business-validation')),
(81, (SELECT id FROM concepts WHERE name = 'business-validation')),
(82, (SELECT id FROM concepts WHERE name = 'business-validation')),
(83, (SELECT id FROM concepts WHERE name = 'business-validation')),
(84, (SELECT id FROM concepts WHERE name = 'business-validation')),
(85, (SELECT id FROM concepts WHERE name = 'business-validation')),
(86, (SELECT id FROM concepts WHERE name = 'business-validation')),
(86, (SELECT id FROM concepts WHERE name = 'schema-validation')),
(87, (SELECT id FROM concepts WHERE name = 'business-validation')),
(88, (SELECT id FROM concepts WHERE name = 'business-validation')),
(88, (SELECT id FROM concepts WHERE name = 'testing')),
(89, (SELECT id FROM concepts WHERE name = 'business-validation')),
(89, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 8: MT parser
INSERT INTO item_concepts (item_id, concept_id) VALUES
(90,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(91,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(92,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(93,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(94,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(95,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(96,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(97,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(98,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(99,  (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(99,  (SELECT id FROM concepts WHERE name = 'testing')),
(100, (SELECT id FROM concepts WHERE name = 'mt-parsing')),
(100, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 9: translation
INSERT INTO item_concepts (item_id, concept_id) VALUES
(101, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(102, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(103, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(104, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(105, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(106, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(107, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(108, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(109, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(110, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(110, (SELECT id FROM concepts WHERE name = 'testing')),
(111, (SELECT id FROM concepts WHERE name = 'mt-mx-translation')),
(111, (SELECT id FROM concepts WHERE name = 'cli'));

-- Iteration 10: FedNow
INSERT INTO item_concepts (item_id, concept_id) VALUES
(112, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(113, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(114, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(115, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(116, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(116, (SELECT id FROM concepts WHERE name = 'testing')),
(117, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(117, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 11: SEPA/CBPR+
INSERT INTO item_concepts (item_id, concept_id) VALUES
(118, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(119, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(120, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(121, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(122, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(123, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(123, (SELECT id FROM concepts WHERE name = 'testing')),
(124, (SELECT id FROM concepts WHERE name = 'scheme-validation')),
(124, (SELECT id FROM concepts WHERE name = 'testing'));

-- Iteration 12: polish
INSERT INTO item_concepts (item_id, concept_id) VALUES
(125, (SELECT id FROM concepts WHERE name = 'documentation')),
(126, (SELECT id FROM concepts WHERE name = 'documentation')),
(127, (SELECT id FROM concepts WHERE name = 'testing')),
(128, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(129, (SELECT id FROM concepts WHERE name = 'documentation')),
(130, (SELECT id FROM concepts WHERE name = 'ci-cd')),
(131, (SELECT id FROM concepts WHERE name = 'ci-cd'));

COMMIT;
