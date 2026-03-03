//! Translate a SWIFT MT103 message to an ISO 20022 pacs.008.
//!
//! Run with: `cargo run -p mx20022 --example translate_mt103`

use mx20022::parse::ser;
use mx20022::translate::mappings::mt103_to_pacs008::mt103_to_pacs008;
use mx20022::translate::mt;
use mx20022::translate::mt::fields::mt103::parse_mt103;

fn main() {
    let mt_text = include_str!("../../../testdata/mt/mt103.txt");

    // -----------------------------------------------------------------------
    // Step 1: parse the raw MT message into block structure
    // -----------------------------------------------------------------------
    let msg = mt::parse(mt_text).expect("failed to parse MT message");

    let msg_type = msg.message_type().unwrap_or("<unknown>");
    println!("Parsed MT{} message", msg_type);

    // -----------------------------------------------------------------------
    // Step 2: parse Block 4 into MT103 field structure
    // -----------------------------------------------------------------------
    let mt103 = parse_mt103(&msg.block4).expect("failed to parse MT103 fields");

    println!("\n--- MT103 fields ---");
    println!("  Sender's reference  : {}", mt103.senders_reference);
    println!("  Bank operation code : {}", mt103.bank_operation_code);
    println!("  Value date          : {}", mt103.value_date);
    println!(
        "  Amount              : {} {}",
        mt103.currency, mt103.interbank_settled_amount
    );
    println!("  Details of charges  : {}", mt103.details_of_charges);
    if let Some(nm) = &mt103.ordering_customer.name {
        println!("  Ordering customer   : {}", nm);
    }
    if let Some(nm) = &mt103.beneficiary.name {
        println!("  Beneficiary         : {}", nm);
    }

    // -----------------------------------------------------------------------
    // Step 3: translate MT103 → pacs.008.001.13
    // -----------------------------------------------------------------------
    let result = mt103_to_pacs008(&mt103, "EXAMPLE-MSG-001", "2023-06-15T12:00:00")
        .expect("translation failed");

    if !result.warnings.is_empty() {
        println!("\n--- Translation warnings ---");
        for w in &result.warnings.warnings {
            println!("  {}: {}", w.field, w.message);
        }
    } else {
        println!("\nTranslation completed with no warnings");
    }

    // -----------------------------------------------------------------------
    // Step 4: serialize the pacs.008 document back to XML
    // -----------------------------------------------------------------------
    let xml = ser::to_string(&result.message).expect("serialization failed");

    println!("\n--- Generated pacs.008 XML ({} bytes) ---", xml.len());
    // Print a truncated preview so output stays readable.
    let preview_len = xml.len().min(600);
    println!("{}", &xml[..preview_len]);
    if xml.len() > preview_len {
        println!("... ({} bytes total)", xml.len());
    }
}
