//! Snapshot tests for the emit module using `insta`.
//!
//! Run `cargo test -p mx20022-codegen -- --test snapshots` to execute.
//! Run `cargo insta review` to accept/reject new snapshots.

use mx20022_codegen::{emit, ir, xsd};

/// Parse the head.001.001.04 XSD and snapshot the emitted Rust source.
#[test]
fn snapshot_head_001_001_04() {
    let xsd_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/head/head.001.001.04.xsd"
    );

    let file = std::fs::File::open(xsd_path).expect("head.001.001.04.xsd not found");
    let schema = xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");
    let graph = ir::lower(&schema).unwrap();
    let code = emit::emit(&graph);

    // Verify it is valid Rust.
    syn::parse_file(&code).expect("generated code must be valid Rust");

    insta::assert_snapshot!("head_001_001_04_emit", code);
}

/// Parse the pacs.008.001.13 XSD and snapshot the emitted Rust source.
#[test]
fn snapshot_pacs_008_001_13() {
    let xsd_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/pacs/pacs.008.001.13.xsd"
    );

    let file = std::fs::File::open(xsd_path).expect("pacs.008.001.13.xsd not found");
    let schema = xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");
    let graph = ir::lower(&schema).unwrap();
    let code = emit::emit(&graph);

    // Verify it is valid Rust.
    syn::parse_file(&code).expect("generated code must be valid Rust");

    insta::assert_snapshot!("pacs_008_001_13_emit", code);
}

/// Parse the camt.053.001.11 XSD and snapshot the emitted Rust source.
#[test]
fn snapshot_camt_053_001_11() {
    let xsd_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/camt/camt.053.001.11.xsd"
    );

    let file = std::fs::File::open(xsd_path).expect("camt.053.001.11.xsd not found");
    let schema = xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");
    let graph = ir::lower(&schema).unwrap();
    let code = emit::emit(&graph);

    // Verify it is valid Rust.
    syn::parse_file(&code).expect("generated code must be valid Rust");

    insta::assert_snapshot!("camt_053_001_11_emit", code);
}

/// Verify specific structural properties of the head.001.001.04 emission.
#[test]
fn head_001_emits_expected_types() {
    let xsd_path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../schemas/head/head.001.001.04.xsd"
    );

    let file = std::fs::File::open(xsd_path).expect("head.001.001.04.xsd not found");
    let schema = xsd::parse(std::io::BufReader::new(file)).expect("XSD parse failed");
    let graph = ir::lower(&schema).unwrap();
    let code = emit::emit(&graph);

    // Root struct.
    assert!(
        code.contains("pub struct BusinessApplicationHeaderV04"),
        "missing root struct"
    );

    // Newtype with length constraints in doc comments.
    assert!(code.contains("pub struct Max35Text"), "missing Max35Text");
    assert!(
        code.contains("Maximum length"),
        "missing max-length constraint doc"
    );

    // Choice enum.
    assert!(
        code.contains("pub enum AddressType3Choice"),
        "missing AddressType3Choice"
    );

    // Code enum.
    assert!(
        code.contains("pub enum AddressType2Code"),
        "missing AddressType2Code"
    );

    // Opaque struct (SignatureEnvelope uses xs:any, emitted with $value field).
    assert!(code.contains("\"$value\""), "missing $value field");

    // Opaque struct.
    assert!(
        code.contains("pub struct SignatureEnvelope"),
        "missing SignatureEnvelope"
    );

    // Serde annotations present.
    assert!(
        code.contains("serde::Serialize"),
        "missing serde::Serialize derive"
    );
    assert!(
        code.contains("serde::Deserialize"),
        "missing serde::Deserialize derive"
    );
}
