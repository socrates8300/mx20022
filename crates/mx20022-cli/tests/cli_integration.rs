//! Integration tests for `mx20022-cli`.
//!
//! These tests compile and invoke the binary directly via `std::process::Command`
//! so that real argument parsing, I/O, and exit-code behaviour is exercised.

use std::path::PathBuf;
use std::process::Command;

fn bin_path() -> PathBuf {
    // The integration-test binary is always compiled to the same target tree as
    // the crate under test.  `CARGO_BIN_EXE_mx20022-cli` is set by Cargo when
    // running integration tests.
    PathBuf::from(env!("CARGO_BIN_EXE_mx20022-cli"))
}

fn testdata(rel: &str) -> PathBuf {
    // Cargo sets CARGO_MANIFEST_DIR to the crate root.
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_root
        .join("../../testdata")
        .join(rel)
        .canonicalize()
        .unwrap_or_else(|_| panic!("testdata path not found: {rel}"))
}

// ---------------------------------------------------------------------------
// inspect
// ---------------------------------------------------------------------------

#[test]
fn inspect_head_xml_prints_message_type() {
    let output = Command::new(bin_path())
        .args([
            "inspect",
            &testdata("xml/head/head_001_001_04_minimal.xml").to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        output.status.success(),
        "inspect should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("head.001.001.04"),
        "expected message type in output, got:\n{stdout}"
    );
    assert!(stdout.contains("head"), "expected family in output");
}

#[test]
fn inspect_pacs_008_xml_prints_message_type() {
    let output = Command::new(bin_path())
        .args([
            "inspect",
            &testdata("xml/pacs/pacs_008_001_13_minimal.xml").to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("pacs.008.001.13"), "stdout: {stdout}");
}

#[test]
fn inspect_nonexistent_file_exits_nonzero() {
    let output = Command::new(bin_path())
        .args(["inspect", "/nonexistent/path/message.xml"])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        !output.status.success(),
        "inspect on missing file should exit non-zero"
    );
}

// ---------------------------------------------------------------------------
// validate
// ---------------------------------------------------------------------------

#[test]
fn validate_valid_pacs_exits_zero() {
    let output = Command::new(bin_path())
        .args([
            "validate",
            &testdata("xml/pacs/pacs_008_001_13_minimal.xml").to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        output.status.success(),
        "validate on valid message should exit 0, stderr: {}\nstdout: {}",
        String::from_utf8_lossy(&output.stderr),
        String::from_utf8_lossy(&output.stdout)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("OK"),
        "expected OK in output, got:\n{stdout}"
    );
}

#[test]
fn validate_invalid_bic_exits_nonzero() {
    let output = Command::new(bin_path())
        .args([
            "validate",
            &testdata("xml/pacs/pacs_008_invalid_bic.xml").to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        !output.status.success(),
        "validate on invalid message should exit non-zero"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("ERROR"),
        "expected ERROR findings in output, got:\n{stdout}"
    );
    assert!(
        stdout.contains("BIC_CHECK"),
        "expected BIC_CHECK rule ID in output, got:\n{stdout}"
    );
}

#[test]
fn validate_head_xml_exits_zero() {
    let output = Command::new(bin_path())
        .args([
            "validate",
            &testdata("xml/head/head_001_001_04_minimal.xml").to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        output.status.success(),
        "validate on valid head message should exit 0, stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

// ---------------------------------------------------------------------------
// codegen
// ---------------------------------------------------------------------------

#[test]
fn codegen_head_xsd_produces_rust() {
    let xsd_path = {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root
            .join("../../schemas/head/head.001.001.04.xsd")
            .canonicalize()
            .expect("head.001.001.04.xsd not found")
    };

    let output = Command::new(bin_path())
        .args(["codegen", &xsd_path.to_string_lossy()])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        output.status.success(),
        "codegen should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("pub struct"),
        "expected generated structs in output, got:\n{stdout}"
    );
    assert!(
        stdout.contains("BusinessApplicationHeaderV04"),
        "expected BusinessApplicationHeaderV04 in generated code"
    );
}

#[test]
fn codegen_writes_to_output_file() {
    let xsd_path = {
        let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        crate_root
            .join("../../schemas/head/head.001.001.04.xsd")
            .canonicalize()
            .expect("head.001.001.04.xsd not found")
    };

    let out_file = std::env::temp_dir().join("mx20022_cli_codegen_test.rs");

    let output = Command::new(bin_path())
        .args([
            "codegen",
            &xsd_path.to_string_lossy(),
            "--output",
            &out_file.to_string_lossy(),
        ])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(
        output.status.success(),
        "codegen with --output should exit 0, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let written = std::fs::read_to_string(&out_file).expect("output file not written");
    assert!(written.contains("pub struct BusinessApplicationHeaderV04"));

    // Clean up.
    let _ = std::fs::remove_file(&out_file);
}

#[test]
fn codegen_nonexistent_xsd_exits_nonzero() {
    let output = Command::new(bin_path())
        .args(["codegen", "/nonexistent/schema.xsd"])
        .output()
        .expect("failed to run mx20022-cli");

    assert!(!output.status.success());
}
