//! Version-neutral pacs.008 facts used by scheme field rules.

use mx20022_model::generated::pacs::pacs_008_001_13::{
    AccountIdentification4Choice, ChargeBearerType1Code, Document, SettlementMethod1Code,
};
use mx20022_parse::ParseError;
use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;

#[derive(Debug, Default)]
pub(crate) struct Facts {
    pub(crate) nb_of_txs: Option<String>,
    pub(crate) settlement_method: Option<String>,
    pub(crate) instg_agent_bic: Option<String>,
    pub(crate) instd_agent_bic: Option<String>,
    pub(crate) transactions: Vec<Transaction>,
}

#[derive(Debug, Default)]
pub(crate) struct Transaction {
    pub(crate) charge_bearer: Option<String>,
    pub(crate) currency: Option<String>,
    pub(crate) amount: Option<String>,
    pub(crate) uetr: Option<String>,
    pub(crate) end_to_end_id: Option<String>,
    pub(crate) debtor_name: Option<String>,
    pub(crate) creditor_name: Option<String>,
    pub(crate) debtor_agent_bic: Option<String>,
    pub(crate) creditor_agent_bic: Option<String>,
    pub(crate) has_settlement_date: bool,
    pub(crate) has_debtor_iban: bool,
    pub(crate) has_creditor_iban: bool,
    pub(crate) unstructured_remittance: Vec<String>,
}

#[derive(Debug)]
struct Node {
    name: Vec<u8>,
    text: String,
    currency: Option<String>,
}

impl Facts {
    pub(crate) fn from_xml(document: &str) -> Result<Self, ParseError> {
        let mut reader = Reader::from_str(document);
        let mut facts = Self::default();
        let mut stack = Vec::<Node>::new();
        let mut current_transaction = None;

        loop {
            let event = reader.read_event().map_err(|error| {
                ParseError::InvalidEnvelope(format!("malformed pacs.008 Document: {error}"))
            })?;
            match event {
                Event::Start(element) => {
                    let name = element.local_name().as_ref().to_vec();
                    if name == b"CdtTrfTxInf" && !inside_supplementary_data(&stack) {
                        facts.transactions.push(Transaction::default());
                        current_transaction = facts.transactions.len().checked_sub(1);
                    }
                    stack.push(Node {
                        currency: currency_attribute(&element)?,
                        name,
                        text: String::new(),
                    });
                }
                Event::Empty(element) => {
                    let name = element.local_name().as_ref().to_vec();
                    if name == b"CdtTrfTxInf" && !inside_supplementary_data(&stack) {
                        facts.transactions.push(Transaction::default());
                    }
                }
                Event::Text(text) => {
                    if let Some(node) = stack.last_mut() {
                        let decoded = text.decode().map_err(|error| {
                            ParseError::InvalidEnvelope(format!(
                                "pacs.008 text is not decodable: {error}"
                            ))
                        })?;
                        let unescaped = quick_xml::escape::unescape(&decoded).map_err(|error| {
                            ParseError::InvalidEnvelope(format!(
                                "pacs.008 text contains an invalid entity: {error}"
                            ))
                        })?;
                        node.text.push_str(&unescaped);
                    }
                }
                Event::CData(text) => {
                    if let Some(node) = stack.last_mut() {
                        let decoded = text.decode().map_err(|error| {
                            ParseError::InvalidEnvelope(format!(
                                "pacs.008 CDATA is not decodable: {error}"
                            ))
                        })?;
                        node.text.push_str(&decoded);
                    }
                }
                Event::End(_) => {
                    let node = stack.pop().ok_or_else(|| {
                        ParseError::InvalidEnvelope(
                            "pacs.008 Document contains an unmatched closing element".to_owned(),
                        )
                    })?;
                    let ends_transaction = node.name == b"CdtTrfTxInf";
                    if !inside_supplementary_data(&stack) {
                        process_node(&mut facts, current_transaction, &stack, node);
                    }
                    if ends_transaction {
                        current_transaction = None;
                    }
                }
                Event::Eof => break,
                _ => {}
            }
        }

        Ok(facts)
    }
}

impl From<&Document> for Facts {
    fn from(document: &Document) -> Self {
        let message = &document.fi_to_fi_cstmr_cdt_trf;
        Self {
            nb_of_txs: Some(message.grp_hdr.nb_of_txs.0.clone()),
            settlement_method: Some(settlement_method(&message.grp_hdr.sttlm_inf.sttlm_mtd)),
            instg_agent_bic: message
                .grp_hdr
                .instg_agt
                .as_ref()
                .and_then(|agent| agent.fin_instn_id.bicfi.as_ref())
                .map(|bic| bic.0.clone()),
            instd_agent_bic: message
                .grp_hdr
                .instd_agt
                .as_ref()
                .and_then(|agent| agent.fin_instn_id.bicfi.as_ref())
                .map(|bic| bic.0.clone()),
            transactions: message
                .cdt_trf_tx_inf
                .iter()
                .map(|transaction| Transaction {
                    charge_bearer: Some(charge_bearer(&transaction.chrg_br)),
                    currency: Some(transaction.intr_bk_sttlm_amt.ccy.0.clone()),
                    amount: Some(transaction.intr_bk_sttlm_amt.value.0.clone()),
                    uetr: transaction
                        .pmt_id
                        .uetr
                        .as_ref()
                        .map(|value| value.0.clone()),
                    end_to_end_id: Some(transaction.pmt_id.end_to_end_id.0.clone()),
                    debtor_name: transaction.dbtr.nm.as_ref().map(|name| name.0.clone()),
                    creditor_name: transaction.cdtr.nm.as_ref().map(|name| name.0.clone()),
                    debtor_agent_bic: transaction
                        .dbtr_agt
                        .fin_instn_id
                        .bicfi
                        .as_ref()
                        .map(|bic| bic.0.clone()),
                    creditor_agent_bic: transaction
                        .cdtr_agt
                        .fin_instn_id
                        .bicfi
                        .as_ref()
                        .map(|bic| bic.0.clone()),
                    has_settlement_date: transaction.intr_bk_sttlm_dt.is_some(),
                    has_debtor_iban: has_iban(transaction.dbtr_acct.as_ref()),
                    has_creditor_iban: has_iban(transaction.cdtr_acct.as_ref()),
                    unstructured_remittance: transaction.rmt_inf.as_ref().map_or_else(
                        Vec::new,
                        |remittance| {
                            remittance
                                .ustrd
                                .iter()
                                .map(|value| value.0.clone())
                                .collect()
                        },
                    ),
                })
                .collect(),
        }
    }
}

fn inside_supplementary_data(stack: &[Node]) -> bool {
    stack.iter().any(|node| node.name == b"SplmtryData")
}

fn currency_attribute(element: &BytesStart<'_>) -> Result<Option<String>, ParseError> {
    for attribute in element.attributes() {
        let attribute = attribute.map_err(|error| {
            ParseError::InvalidEnvelope(format!("invalid pacs.008 attribute: {error}"))
        })?;
        if attribute.key.local_name().as_ref() == b"Ccy" {
            let value = attribute
                .decoded_and_normalized_value(quick_xml::XmlVersion::Implicit1_0, element.decoder())
                .map_err(|error| {
                    ParseError::InvalidEnvelope(format!("invalid pacs.008 currency: {error}"))
                })?;
            return Ok(Some(value.into_owned()));
        }
    }
    Ok(None)
}

fn process_node(
    facts: &mut Facts,
    current_transaction: Option<usize>,
    ancestors: &[Node],
    node: Node,
) {
    let text = node.text.trim().to_owned();
    let name = node.name.as_slice();

    if current_transaction.is_none() {
        match name {
            b"NbOfTxs" if has_ancestor(ancestors, b"GrpHdr") => {
                facts.nb_of_txs.get_or_insert(text);
            }
            b"SttlmMtd" if has_ancestor(ancestors, b"GrpHdr") => {
                facts.settlement_method.get_or_insert(text);
            }
            b"BICFI" if has_ancestor(ancestors, b"InstgAgt") => {
                facts.instg_agent_bic.get_or_insert(text);
            }
            b"BICFI" if has_ancestor(ancestors, b"InstdAgt") => {
                facts.instd_agent_bic.get_or_insert(text);
            }
            _ => {}
        }
        return;
    }

    let transaction = &mut facts.transactions[current_transaction.expect("checked above")];
    match name {
        b"ChrgBr" => {
            transaction.charge_bearer.get_or_insert(text);
        }
        b"IntrBkSttlmAmt" => {
            transaction.currency = node.currency;
            transaction.amount.get_or_insert(text);
        }
        b"UETR" => {
            transaction.uetr.get_or_insert(text);
        }
        b"EndToEndId" => {
            transaction.end_to_end_id.get_or_insert(text);
        }
        b"Nm" if nearest_party(ancestors) == Some(b"Dbtr".as_slice()) => {
            transaction.debtor_name.get_or_insert(text);
        }
        b"Nm" if nearest_party(ancestors) == Some(b"Cdtr".as_slice()) => {
            transaction.creditor_name.get_or_insert(text);
        }
        b"BICFI" if has_ancestor(ancestors, b"DbtrAgt") => {
            transaction.debtor_agent_bic.get_or_insert(text);
        }
        b"BICFI" if has_ancestor(ancestors, b"CdtrAgt") => {
            transaction.creditor_agent_bic.get_or_insert(text);
        }
        b"IntrBkSttlmDt" => {
            transaction.has_settlement_date = true;
        }
        b"IBAN" if has_ancestor(ancestors, b"DbtrAcct") => {
            transaction.has_debtor_iban = true;
        }
        b"IBAN" if has_ancestor(ancestors, b"CdtrAcct") => {
            transaction.has_creditor_iban = true;
        }
        b"Ustrd" => {
            transaction.unstructured_remittance.push(text);
        }
        _ => {}
    }
}

fn has_ancestor(ancestors: &[Node], name: &[u8]) -> bool {
    ancestors.iter().any(|node| node.name == name)
}

fn nearest_party(ancestors: &[Node]) -> Option<&[u8]> {
    ancestors
        .iter()
        .rev()
        .map(|node| node.name.as_slice())
        .find(|name| matches!(*name, b"Dbtr" | b"Cdtr"))
}

fn has_iban(
    account: Option<&mx20022_model::generated::pacs::pacs_008_001_13::CashAccount40>,
) -> bool {
    account.as_ref().is_some_and(|account| {
        account
            .id
            .as_ref()
            .is_some_and(|choice| matches!(choice.inner, AccountIdentification4Choice::IBAN(_)))
    })
}

fn settlement_method(value: &SettlementMethod1Code) -> String {
    match value {
        SettlementMethod1Code::Inda => "INDA",
        SettlementMethod1Code::Inga => "INGA",
        SettlementMethod1Code::Cove => "COVE",
        SettlementMethod1Code::Clrg => "CLRG",
    }
    .to_owned()
}

fn charge_bearer(value: &ChargeBearerType1Code) -> String {
    match value {
        ChargeBearerType1Code::Debt => "DEBT",
        ChargeBearerType1Code::Cred => "CRED",
        ChargeBearerType1Code::Shar => "SHAR",
        ChargeBearerType1Code::Slev => "SLEV",
    }
    .to_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_each_transaction_without_reading_supplementary_payloads() {
        let xml = r#"<Document><FIToFICstmrCdtTrf><GrpHdr><NbOfTxs>1</NbOfTxs><SttlmInf><SttlmMtd>CLRG</SttlmMtd></SttlmInf></GrpHdr><CdtTrfTxInf><PmtId><EndToEndId>e2e</EndToEndId></PmtId><IntrBkSttlmAmt Ccy="EUR">1.00</IntrBkSttlmAmt><Dbtr><Nm>Alice</Nm></Dbtr><SplmtryData><Envlp><Document><CdtTrfTxInf><IntrBkSttlmAmt Ccy="USD">9.00</IntrBkSttlmAmt></CdtTrfTxInf></Document></Envlp></SplmtryData></CdtTrfTxInf></FIToFICstmrCdtTrf></Document>"#;
        let facts = Facts::from_xml(xml).unwrap();
        assert_eq!(facts.nb_of_txs.as_deref(), Some("1"));
        assert_eq!(facts.transactions.len(), 1);
        assert_eq!(facts.transactions[0].currency.as_deref(), Some("EUR"));
        assert_eq!(facts.transactions[0].debtor_name.as_deref(), Some("Alice"));
    }
}
