//! Demonstrate a full MT103 -> pacs.008 -> MT103 roundtrip.
//!
//! Shows that field content survives both translation legs without loss of
//! the core payment data (reference, amount, currency, parties).
//!
//! Run with: `cargo run -p mx20022 --example roundtrip`

use mx20022::parse::ser;
use mx20022::translate::mappings::mt103_to_pacs008::mt103_to_pacs008;
use mx20022::translate::mappings::pacs008_to_mt103::pacs008_to_mt103;
use mx20022::translate::mt;
use mx20022::translate::mt::fields::mt103::parse_mt103;

fn main() {
    let original_mt = include_str!("../../../testdata/mt/mt103.txt");

    println!("=== Original MT103 ===");
    println!("{}", original_mt.trim());

    // -----------------------------------------------------------------------
    // Leg 1: MT103 -> pacs.008.001.13
    // -----------------------------------------------------------------------
    let msg = mt::parse(original_mt).expect("failed to parse MT103");
    let mt103 = parse_mt103(&msg.block4).expect("failed to parse MT103 fields");

    let fwd = mt103_to_pacs008(&mt103, "ROUNDTRIP-001", "2023-06-15T12:00:00")
        .expect("MT103 -> pacs.008 translation failed");

    if !fwd.warnings.is_empty() {
        println!("\n--- Forward translation warnings ---");
        for w in &fwd.warnings.warnings {
            println!("  {}: {}", w.field, w.message);
        }
    }

    // Show the intermediate XML.
    let xml = ser::to_string(&fwd.message).expect("serialization failed");
    println!("\n=== Intermediate pacs.008 XML ({} bytes) ===", xml.len());
    let preview_len = xml.len().min(600);
    println!("{}", &xml[..preview_len]);
    if xml.len() > preview_len {
        println!("... ({} bytes total)", xml.len());
    }

    // -----------------------------------------------------------------------
    // Leg 2: pacs.008 -> MT103
    // -----------------------------------------------------------------------
    let rev = pacs008_to_mt103(&fwd.message).expect("pacs.008 -> MT103 translation failed");

    if !rev.warnings.is_empty() {
        println!("\n--- Reverse translation warnings ---");
        for w in &rev.warnings.warnings {
            println!("  {}: {}", w.field, w.message);
        }
    }

    println!("\n=== Roundtripped MT103 ===");
    println!("{}", rev.message.trim());

    // -----------------------------------------------------------------------
    // Spot-check: key fields must survive the roundtrip
    // -----------------------------------------------------------------------
    println!("\n=== Roundtrip field check ===");

    let rt_msg = mt::parse(&rev.message).expect("failed to parse roundtripped MT103");
    let rt_mt103 = parse_mt103(&rt_msg.block4).expect("failed to parse roundtripped MT103 fields");

    check_field(
        "Sender's reference",
        &mt103.senders_reference,
        &rt_mt103.senders_reference,
    );
    check_field("Currency", &mt103.currency, &rt_mt103.currency);
    check_field(
        "Amount",
        &mt103.interbank_settled_amount,
        &rt_mt103.interbank_settled_amount,
    );
    check_field(
        "Details of charges",
        &mt103.details_of_charges,
        &rt_mt103.details_of_charges,
    );

    println!("\nRoundtrip complete.");
}

fn check_field(label: &str, original: &str, roundtripped: &str) {
    if original == roundtripped {
        println!("  [OK] {}: {:?}", label, original);
    } else {
        println!(
            "  [DIFF] {}: original={:?} roundtripped={:?}",
            label, original, roundtripped
        );
    }
}
