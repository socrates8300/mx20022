//! Parse an ISO 20022 pacs.008 message and print key fields.
//!
//! Run with: `cargo run -p mx20022 --example parse_pacs008`

use mx20022::model::generated::pacs::pacs_008_001_13::Document;
use mx20022::parse::{de, envelope};

fn main() {
    let xml = include_str!("../../../testdata/xml/pacs/pacs_008_001_13_minimal.xml");

    // Detect the message type from the XML namespace before full parsing.
    let msg_id = envelope::detect_message_type(xml).expect("failed to detect message type");
    println!(
        "Detected message type: {}.{}.{}",
        msg_id.family, msg_id.msg_id, msg_id.version
    );
    println!("Canonical id: {}", msg_id.dotted());

    // Deserialize to the strongly-typed generated struct.
    let doc: Document = de::from_str(xml).expect("failed to parse pacs.008 XML");

    let hdr = &doc.fi_to_fi_cstmr_cdt_trf.grp_hdr;
    println!("\n--- Group Header ---");
    println!("  Message ID        : {}", hdr.msg_id.0);
    println!("  Creation datetime : {}", hdr.cre_dt_tm.0);
    println!("  Number of txns    : {}", hdr.nb_of_txs.0);

    let txns = &doc.fi_to_fi_cstmr_cdt_trf.cdt_trf_tx_inf;
    println!("\n--- Credit Transfer Transactions ({}) ---", txns.len());
    for (i, tx) in txns.iter().enumerate() {
        println!("  Transaction {}:", i + 1);
        println!("    End-to-end ID : {}", tx.pmt_id.end_to_end_id.0);
        println!(
            "    Amount        : {} {}",
            tx.intr_bk_sttlm_amt.value.0, tx.intr_bk_sttlm_amt.ccy.0
        );
        if let Some(dt) = &tx.intr_bk_sttlm_dt {
            println!("    Settlement dt : {}", dt.0);
        }
        if let Some(nm) = &tx.dbtr.nm {
            println!("    Debtor        : {}", nm.0);
        }
        if let Some(nm) = &tx.cdtr.nm {
            println!("    Creditor      : {}", nm.0);
        }
    }
}
