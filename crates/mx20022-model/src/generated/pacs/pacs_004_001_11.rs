/*! Generated from ISO 20022 XSD schema.
Namespace: `urn:iso:std:iso:20022:tech:xsd:pacs.004.001.11`*/
/// Fraction digits: 5
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyAndAmountSimpleType(pub String);
/// Pattern: `[A-Z]{3,3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveCurrencyCode(pub String);
/// Fraction digits: 5
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType(pub String);
/// Pattern: `[A-Z]{3,3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddressType2Code {
    #[serde(rename = "ADDR")]
    Addr,
    #[serde(rename = "PBOX")]
    Pbox,
    #[serde(rename = "HOME")]
    Home,
    #[serde(rename = "BIZZ")]
    Bizz,
    #[serde(rename = "MLTO")]
    Mlto,
    #[serde(rename = "DLVY")]
    Dlvy,
}
/// Pattern: `[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Authorisation1Code {
    #[serde(rename = "AUTH")]
    Auth,
    #[serde(rename = "FDET")]
    Fdet,
    #[serde(rename = "FSUM")]
    Fsum,
    #[serde(rename = "ILEV")]
    Ilev,
}
/// Pattern: `[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BICFIDec2014Identifier(pub String);
/// Fraction digits: 10
/// Total digits: 11
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BatchBookingIndicator(pub bool);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargeBearerType1Code {
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "SHAR")]
    Shar,
    #[serde(rename = "SLEV")]
    Slev,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearingChannel2Code {
    #[serde(rename = "RTGS")]
    Rtgs,
    #[serde(rename = "RTNS")]
    Rtns,
    #[serde(rename = "MPNS")]
    Mpns,
    #[serde(rename = "BOOK")]
    Book,
}
/// Pattern: `[A-Z]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CountryCode(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreditDebitCode {
    #[serde(rename = "CRDT")]
    Crdt,
    #[serde(rename = "DBIT")]
    Dbit,
}
/// Fraction digits: 17
/// Total digits: 18
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct DecimalNumber(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocumentType3Code {
    #[serde(rename = "RADM")]
    Radm,
    #[serde(rename = "RPIN")]
    Rpin,
    #[serde(rename = "FXDR")]
    Fxdr,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "PUOR")]
    Puor,
    #[serde(rename = "SCOR")]
    Scor,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocumentType6Code {
    #[serde(rename = "MSIN")]
    Msin,
    #[serde(rename = "CNFA")]
    Cnfa,
    #[serde(rename = "DNFA")]
    Dnfa,
    #[serde(rename = "CINV")]
    Cinv,
    #[serde(rename = "CREN")]
    Cren,
    #[serde(rename = "DEBN")]
    Debn,
    #[serde(rename = "HIRI")]
    Hiri,
    #[serde(rename = "SBIN")]
    Sbin,
    #[serde(rename = "CMCN")]
    Cmcn,
    #[serde(rename = "SOAC")]
    Soac,
    #[serde(rename = "DISP")]
    Disp,
    #[serde(rename = "BOLD")]
    Bold,
    #[serde(rename = "VCHR")]
    Vchr,
    #[serde(rename = "AROI")]
    Aroi,
    #[serde(rename = "TSUT")]
    Tsut,
    #[serde(rename = "PUOR")]
    Puor,
}
/// Pattern: `[0-9]{2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Exact2NumericText(pub String);
/// Pattern: `[a-zA-Z0-9]{4}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 3
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCashClearingSystem1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCategoryPurpose1Code(pub String);
/// Minimum length: 1
/// Maximum length: 5
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCreditorAgentInstruction1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDiscountAmountType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalGarnishmentType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 35
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalLocalInstrument1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalMandateSetupReason1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalPurpose1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalReturnReason1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalServiceLevel1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalTaxAmountType1Code(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Frequency6Code {
    #[serde(rename = "YEAR")]
    Year,
    #[serde(rename = "MNTH")]
    Mnth,
    #[serde(rename = "QURT")]
    Qurt,
    #[serde(rename = "MIAN")]
    Mian,
    #[serde(rename = "WEEK")]
    Week,
    #[serde(rename = "DAIL")]
    Dail,
    #[serde(rename = "ADHO")]
    Adho,
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "FRTN")]
    Frtn,
}
/// Pattern: `[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODate(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODateTime(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISOTime(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISOYear(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Instruction4Code {
    #[serde(rename = "PHOA")]
    Phoa,
    #[serde(rename = "TELA")]
    Tela,
}
/// Pattern: `[A-Z0-9]{18,18}[0-9]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MandateClassification1Code {
    #[serde(rename = "FIXE")]
    Fixe,
    #[serde(rename = "USGB")]
    Usgb,
    #[serde(rename = "VARI")]
    Vari,
}
/// Minimum length: 1
/// Maximum length: 1025
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max1025Text(pub String);
/// Minimum length: 1
/// Maximum length: 105
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max105Text(pub String);
/// Minimum length: 1
/// Maximum length: 10240
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max10KBinary(pub String);
/// Minimum length: 1
/// Maximum length: 128
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max128Text(pub String);
/// Minimum length: 1
/// Maximum length: 140
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max140Text(pub String);
/// Pattern: `[0-9]{1,15}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText(pub String);
/// Minimum length: 1
/// Maximum length: 16
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max16Text(pub String);
/// Minimum length: 1
/// Maximum length: 2048
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max2048Text(pub String);
/// Minimum length: 1
/// Maximum length: 34
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max34Text(pub String);
/// Minimum length: 1
/// Maximum length: 350
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max350Text(pub String);
/// Minimum length: 1
/// Maximum length: 35
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max35Text(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max4Text(pub String);
/// Minimum length: 1
/// Maximum length: 70
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max70Text(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum NamePrefix2Code {
    #[serde(rename = "DOCT")]
    Doct,
    #[serde(rename = "MADM")]
    Madm,
    #[serde(rename = "MISS")]
    Miss,
    #[serde(rename = "MIST")]
    Mist,
    #[serde(rename = "MIKS")]
    Miks,
}
/// Fraction digits: 0
/// Total digits: 18
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Number(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PaymentMethod4Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "DD")]
    Dd,
    #[serde(rename = "TRA")]
    Tra,
}
/// Fraction digits: 10
/// Total digits: 11
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PercentageRate(pub String);
/// Pattern: `\+[0-9]{1,3}-[0-9()+\-]{1,30}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PreferredContactMethod1Code {
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "PHON")]
    Phon,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "CELL")]
    Cell,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Priority3Code {
    #[serde(rename = "URGT")]
    Urgt,
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SequenceType3Code {
    #[serde(rename = "FRST")]
    Frst,
    #[serde(rename = "RCUR")]
    Rcur,
    #[serde(rename = "FNAL")]
    Fnal,
    #[serde(rename = "OOFF")]
    Ooff,
    #[serde(rename = "RPRE")]
    Rpre,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SettlementMethod1Code {
    #[serde(rename = "INDA")]
    Inda,
    #[serde(rename = "INGA")]
    Inga,
    #[serde(rename = "COVE")]
    Cove,
    #[serde(rename = "CLRG")]
    Clrg,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TaxRecordPeriod1Code {
    #[serde(rename = "MM01")]
    Mm01,
    #[serde(rename = "MM02")]
    Mm02,
    #[serde(rename = "MM03")]
    Mm03,
    #[serde(rename = "MM04")]
    Mm04,
    #[serde(rename = "MM05")]
    Mm05,
    #[serde(rename = "MM06")]
    Mm06,
    #[serde(rename = "MM07")]
    Mm07,
    #[serde(rename = "MM08")]
    Mm08,
    #[serde(rename = "MM09")]
    Mm09,
    #[serde(rename = "MM10")]
    Mm10,
    #[serde(rename = "MM11")]
    Mm11,
    #[serde(rename = "MM12")]
    Mm12,
    #[serde(rename = "QTR1")]
    Qtr1,
    #[serde(rename = "QTR2")]
    Qtr2,
    #[serde(rename = "QTR3")]
    Qtr3,
    #[serde(rename = "QTR4")]
    Qtr4,
    #[serde(rename = "HLF1")]
    Hlf1,
    #[serde(rename = "HLF2")]
    Hlf2,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator(pub bool);
/// Pattern: `[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UUIDv4Identifier(pub String);
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AccountIdentification4Choice {
    #[serde(rename = "IBAN")]
    IBAN(IBAN2007Identifier),
    #[serde(rename = "Othr")]
    Othr(GenericAccountIdentification1),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AccountSchemeName1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalAccountIdentification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActiveCurrencyAndAmount {
    #[serde(rename = "$value")]
    pub value: ActiveCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveCurrencyCode,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "$value")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddressType3Choice {
    #[serde(rename = "Cd")]
    Cd(AddressType2Code),
    #[serde(rename = "Prtry")]
    Prtry(GenericIdentification30),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendmentInformationDetails14 {
    #[serde(rename = "OrgnlMndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_mndt_id: Option<Max35Text>,
    #[serde(rename = "OrgnlCdtrSchmeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_schme_id: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlCdtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlCdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cdtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "OrgnlDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "OrgnlDbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "OrgnlDbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlDbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "OrgnlFnlColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_fnl_colltn_dt: Option<ISODate>,
    #[serde(rename = "OrgnlFrqcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_frqcy: Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    #[serde(rename = "OrgnlRsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_rsn: Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
    #[serde(rename = "OrgnlTrckgDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_trckg_days: Option<Exact2NumericText>,
}
/// Builder for [`AmendmentInformationDetails14`]. Construct via [`AmendmentInformationDetails14::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmendmentInformationDetails14Builder {
    orgnl_mndt_id: ::std::option::Option<Max35Text>,
    orgnl_cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    orgnl_cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    orgnl_cdtr_agt_acct: ::std::option::Option<CashAccount40>,
    orgnl_dbtr: ::std::option::Option<PartyIdentification135>,
    orgnl_dbtr_acct: ::std::option::Option<CashAccount40>,
    orgnl_dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    orgnl_dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    orgnl_fnl_colltn_dt: ::std::option::Option<ISODate>,
    orgnl_frqcy: ::std::option::Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    orgnl_rsn: ::std::option::Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
    orgnl_trckg_days: ::std::option::Option<Exact2NumericText>,
}
impl AmendmentInformationDetails14Builder {
    /// Set the `orgnl_mndt_id` field.
    #[must_use]
    pub fn orgnl_mndt_id(mut self, value: Max35Text) -> AmendmentInformationDetails14Builder {
        self.orgnl_mndt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cdtr_schme_id` field.
    #[must_use]
    pub fn orgnl_cdtr_schme_id(
        mut self,
        value: PartyIdentification135,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_cdtr_schme_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cdtr_agt` field.
    #[must_use]
    pub fn orgnl_cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cdtr_agt_acct` field.
    #[must_use]
    pub fn orgnl_cdtr_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_dbtr` field.
    #[must_use]
    pub fn orgnl_dbtr(
        mut self,
        value: PartyIdentification135,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_dbtr_acct` field.
    #[must_use]
    pub fn orgnl_dbtr_acct(mut self, value: CashAccount40) -> AmendmentInformationDetails14Builder {
        self.orgnl_dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_dbtr_agt` field.
    #[must_use]
    pub fn orgnl_dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_dbtr_agt_acct` field.
    #[must_use]
    pub fn orgnl_dbtr_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_fnl_colltn_dt` field.
    #[must_use]
    pub fn orgnl_fnl_colltn_dt(mut self, value: ISODate) -> AmendmentInformationDetails14Builder {
        self.orgnl_fnl_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_frqcy` field.
    #[must_use]
    pub fn orgnl_frqcy(
        mut self,
        value: crate::common::ChoiceWrapper<Frequency36Choice>,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_frqcy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_rsn` field.
    #[must_use]
    pub fn orgnl_rsn(
        mut self,
        value: crate::common::ChoiceWrapper<MandateSetupReason1Choice>,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_trckg_days` field.
    #[must_use]
    pub fn orgnl_trckg_days(
        mut self,
        value: Exact2NumericText,
    ) -> AmendmentInformationDetails14Builder {
        self.orgnl_trckg_days = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<AmendmentInformationDetails14, crate::common::BuilderError> {
        ::std::result::Result::Ok(AmendmentInformationDetails14 {
            orgnl_mndt_id: self.orgnl_mndt_id,
            orgnl_cdtr_schme_id: self.orgnl_cdtr_schme_id,
            orgnl_cdtr_agt: self.orgnl_cdtr_agt,
            orgnl_cdtr_agt_acct: self.orgnl_cdtr_agt_acct,
            orgnl_dbtr: self.orgnl_dbtr,
            orgnl_dbtr_acct: self.orgnl_dbtr_acct,
            orgnl_dbtr_agt: self.orgnl_dbtr_agt,
            orgnl_dbtr_agt_acct: self.orgnl_dbtr_agt_acct,
            orgnl_fnl_colltn_dt: self.orgnl_fnl_colltn_dt,
            orgnl_frqcy: self.orgnl_frqcy,
            orgnl_rsn: self.orgnl_rsn,
            orgnl_trckg_days: self.orgnl_trckg_days,
        })
    }
}
impl AmendmentInformationDetails14 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmendmentInformationDetails14Builder {
        AmendmentInformationDetails14Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AmountType4Choice {
    #[serde(rename = "InstdAmt")]
    InstdAmt(ActiveOrHistoricCurrencyAndAmount),
    #[serde(rename = "EqvtAmt")]
    EqvtAmt(EquivalentAmount2),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Authorisation1Choice {
    #[serde(rename = "Cd")]
    Cd(Authorisation1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max128Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchAndFinancialInstitutionIdentification6 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification18,
    #[serde(rename = "BrnchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData3>,
}
/// Builder for [`BranchAndFinancialInstitutionIdentification6`]. Construct via [`BranchAndFinancialInstitutionIdentification6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BranchAndFinancialInstitutionIdentification6Builder {
    fin_instn_id: ::std::option::Option<FinancialInstitutionIdentification18>,
    brnch_id: ::std::option::Option<BranchData3>,
}
impl BranchAndFinancialInstitutionIdentification6Builder {
    /// Set the `fin_instn_id` field.
    #[must_use]
    pub fn fin_instn_id(
        mut self,
        value: FinancialInstitutionIdentification18,
    ) -> BranchAndFinancialInstitutionIdentification6Builder {
        self.fin_instn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `brnch_id` field.
    #[must_use]
    pub fn brnch_id(
        mut self,
        value: BranchData3,
    ) -> BranchAndFinancialInstitutionIdentification6Builder {
        self.brnch_id = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<
        BranchAndFinancialInstitutionIdentification6,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fin_instn_id.is_none() {
            missing.push("fin_instn_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BranchAndFinancialInstitutionIdentification6".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BranchAndFinancialInstitutionIdentification6 {
            fin_instn_id: self.fin_instn_id.unwrap(),
            brnch_id: self.brnch_id,
        })
    }
}
impl BranchAndFinancialInstitutionIdentification6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BranchAndFinancialInstitutionIdentification6Builder {
        BranchAndFinancialInstitutionIdentification6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchData3 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<LEIIdentifier>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
}
/// Builder for [`BranchData3`]. Construct via [`BranchData3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BranchData3Builder {
    id: ::std::option::Option<Max35Text>,
    lei: ::std::option::Option<LEIIdentifier>,
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress24>,
}
impl BranchData3Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> BranchData3Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> BranchData3Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> BranchData3Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: PostalAddress24) -> BranchData3Builder {
        self.pstl_adr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<BranchData3, crate::common::BuilderError> {
        ::std::result::Result::Ok(BranchData3 {
            id: self.id,
            lei: self.lei,
            nm: self.nm,
            pstl_adr: self.pstl_adr,
        })
    }
}
impl BranchData3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BranchData3Builder {
        BranchData3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashAccount40 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::common::ChoiceWrapper<AccountIdentification4Choice>>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<CashAccountType2Choice>>,
    #[serde(rename = "Ccy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
    #[serde(rename = "Prxy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prxy: Option<ProxyAccountIdentification1>,
}
/// Builder for [`CashAccount40`]. Construct via [`CashAccount40::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CashAccount40Builder {
    id: ::std::option::Option<crate::common::ChoiceWrapper<AccountIdentification4Choice>>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<CashAccountType2Choice>>,
    ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    nm: ::std::option::Option<Max70Text>,
    prxy: ::std::option::Option<ProxyAccountIdentification1>,
}
impl CashAccount40Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(
        mut self,
        value: crate::common::ChoiceWrapper<AccountIdentification4Choice>,
    ) -> CashAccount40Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<CashAccountType2Choice>,
    ) -> CashAccount40Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy` field.
    #[must_use]
    pub fn ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CashAccount40Builder {
        self.ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max70Text) -> CashAccount40Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prxy` field.
    #[must_use]
    pub fn prxy(mut self, value: ProxyAccountIdentification1) -> CashAccount40Builder {
        self.prxy = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<CashAccount40, crate::common::BuilderError> {
        ::std::result::Result::Ok(CashAccount40 {
            id: self.id,
            tp: self.tp,
            ccy: self.ccy,
            nm: self.nm,
            prxy: self.prxy,
        })
    }
}
impl CashAccount40 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CashAccount40Builder {
        CashAccount40Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CashAccountType2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalCashAccountType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CategoryPurpose1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalCategoryPurpose1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Charges7 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification6,
}
/// Builder for [`Charges7`]. Construct via [`Charges7::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Charges7Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
}
impl Charges7Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> Charges7Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `agt` field.
    #[must_use]
    pub fn agt(mut self, value: BranchAndFinancialInstitutionIdentification6) -> Charges7Builder {
        self.agt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<Charges7, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.agt.is_none() {
            missing.push("agt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Charges7".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Charges7 {
            amt: self.amt.unwrap(),
            agt: self.agt.unwrap(),
        })
    }
}
impl Charges7 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Charges7Builder {
        Charges7Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearingSystemIdentification2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalClearingSystemIdentification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ClearingSystemIdentification3Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalCashClearingSystem1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ClearingSystemMemberIdentification2 {
    #[serde(rename = "ClrSysId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_id: Option<crate::common::ChoiceWrapper<ClearingSystemIdentification2Choice>>,
    #[serde(rename = "MmbId")]
    pub mmb_id: Max35Text,
}
/// Builder for [`ClearingSystemMemberIdentification2`]. Construct via [`ClearingSystemMemberIdentification2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ClearingSystemMemberIdentification2Builder {
    clr_sys_id:
        ::std::option::Option<crate::common::ChoiceWrapper<ClearingSystemIdentification2Choice>>,
    mmb_id: ::std::option::Option<Max35Text>,
}
impl ClearingSystemMemberIdentification2Builder {
    /// Set the `clr_sys_id` field.
    #[must_use]
    pub fn clr_sys_id(
        mut self,
        value: crate::common::ChoiceWrapper<ClearingSystemIdentification2Choice>,
    ) -> ClearingSystemMemberIdentification2Builder {
        self.clr_sys_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mmb_id` field.
    #[must_use]
    pub fn mmb_id(mut self, value: Max35Text) -> ClearingSystemMemberIdentification2Builder {
        self.mmb_id = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<ClearingSystemMemberIdentification2, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.mmb_id.is_none() {
            missing.push("mmb_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ClearingSystemMemberIdentification2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ClearingSystemMemberIdentification2 {
            clr_sys_id: self.clr_sys_id,
            mmb_id: self.mmb_id.unwrap(),
        })
    }
}
impl ClearingSystemMemberIdentification2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ClearingSystemMemberIdentification2Builder {
        ClearingSystemMemberIdentification2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Contact4 {
    #[serde(rename = "NmPrfx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm_prfx: Option<NamePrefix2Code>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PhneNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phne_nb: Option<PhoneNumber>,
    #[serde(rename = "MobNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mob_nb: Option<PhoneNumber>,
    #[serde(rename = "FaxNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fax_nb: Option<PhoneNumber>,
    #[serde(rename = "EmailAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max2048Text>,
    #[serde(rename = "EmailPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_purp: Option<Max35Text>,
    #[serde(rename = "JobTitl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_titl: Option<Max35Text>,
    #[serde(rename = "Rspnsblty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rspnsblty: Option<Max35Text>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "Othr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr: Vec<OtherContact1>,
    #[serde(rename = "PrefrdMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefrd_mtd: Option<PreferredContactMethod1Code>,
}
/// Builder for [`Contact4`]. Construct via [`Contact4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Contact4Builder {
    nm_prfx: ::std::option::Option<NamePrefix2Code>,
    nm: ::std::option::Option<Max140Text>,
    phne_nb: ::std::option::Option<PhoneNumber>,
    mob_nb: ::std::option::Option<PhoneNumber>,
    fax_nb: ::std::option::Option<PhoneNumber>,
    email_adr: ::std::option::Option<Max2048Text>,
    email_purp: ::std::option::Option<Max35Text>,
    job_titl: ::std::option::Option<Max35Text>,
    rspnsblty: ::std::option::Option<Max35Text>,
    dept: ::std::option::Option<Max70Text>,
    othr: ::std::vec::Vec<OtherContact1>,
    prefrd_mtd: ::std::option::Option<PreferredContactMethod1Code>,
}
impl Contact4Builder {
    /// Set the `nm_prfx` field.
    #[must_use]
    pub fn nm_prfx(mut self, value: NamePrefix2Code) -> Contact4Builder {
        self.nm_prfx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> Contact4Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `phne_nb` field.
    #[must_use]
    pub fn phne_nb(mut self, value: PhoneNumber) -> Contact4Builder {
        self.phne_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mob_nb` field.
    #[must_use]
    pub fn mob_nb(mut self, value: PhoneNumber) -> Contact4Builder {
        self.mob_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fax_nb` field.
    #[must_use]
    pub fn fax_nb(mut self, value: PhoneNumber) -> Contact4Builder {
        self.fax_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `email_adr` field.
    #[must_use]
    pub fn email_adr(mut self, value: Max2048Text) -> Contact4Builder {
        self.email_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `email_purp` field.
    #[must_use]
    pub fn email_purp(mut self, value: Max35Text) -> Contact4Builder {
        self.email_purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `job_titl` field.
    #[must_use]
    pub fn job_titl(mut self, value: Max35Text) -> Contact4Builder {
        self.job_titl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rspnsblty` field.
    #[must_use]
    pub fn rspnsblty(mut self, value: Max35Text) -> Contact4Builder {
        self.rspnsblty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dept` field.
    #[must_use]
    pub fn dept(mut self, value: Max70Text) -> Contact4Builder {
        self.dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(mut self, value: ::std::vec::Vec<OtherContact1>) -> Contact4Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(mut self, value: OtherContact1) -> Contact4Builder {
        self.othr.push(value);
        self
    }
    /// Set the `prefrd_mtd` field.
    #[must_use]
    pub fn prefrd_mtd(mut self, value: PreferredContactMethod1Code) -> Contact4Builder {
        self.prefrd_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<Contact4, crate::common::BuilderError> {
        ::std::result::Result::Ok(Contact4 {
            nm_prfx: self.nm_prfx,
            nm: self.nm,
            phne_nb: self.phne_nb,
            mob_nb: self.mob_nb,
            fax_nb: self.fax_nb,
            email_adr: self.email_adr,
            email_purp: self.email_purp,
            job_titl: self.job_titl,
            rspnsblty: self.rspnsblty,
            dept: self.dept,
            othr: self.othr,
            prefrd_mtd: self.prefrd_mtd,
        })
    }
}
impl Contact4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Contact4Builder {
        Contact4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditTransferMandateData1 {
    #[serde(rename = "MndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<Max35Text>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<MandateTypeInformation2>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<ISODate>,
    #[serde(rename = "DtOfVrfctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_vrfctn: Option<ISODateTime>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<Max10KBinary>,
    #[serde(rename = "FrstPmtDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frst_pmt_dt: Option<ISODate>,
    #[serde(rename = "FnlPmtDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnl_pmt_dt: Option<ISODate>,
    #[serde(rename = "Frqcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
}
/// Builder for [`CreditTransferMandateData1`]. Construct via [`CreditTransferMandateData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditTransferMandateData1Builder {
    mndt_id: ::std::option::Option<Max35Text>,
    tp: ::std::option::Option<MandateTypeInformation2>,
    dt_of_sgntr: ::std::option::Option<ISODate>,
    dt_of_vrfctn: ::std::option::Option<ISODateTime>,
    elctrnc_sgntr: ::std::option::Option<Max10KBinary>,
    frst_pmt_dt: ::std::option::Option<ISODate>,
    fnl_pmt_dt: ::std::option::Option<ISODate>,
    frqcy: ::std::option::Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    rsn: ::std::option::Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
}
impl CreditTransferMandateData1Builder {
    /// Set the `mndt_id` field.
    #[must_use]
    pub fn mndt_id(mut self, value: Max35Text) -> CreditTransferMandateData1Builder {
        self.mndt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: MandateTypeInformation2) -> CreditTransferMandateData1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt_of_sgntr` field.
    #[must_use]
    pub fn dt_of_sgntr(mut self, value: ISODate) -> CreditTransferMandateData1Builder {
        self.dt_of_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt_of_vrfctn` field.
    #[must_use]
    pub fn dt_of_vrfctn(mut self, value: ISODateTime) -> CreditTransferMandateData1Builder {
        self.dt_of_vrfctn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_sgntr` field.
    #[must_use]
    pub fn elctrnc_sgntr(mut self, value: Max10KBinary) -> CreditTransferMandateData1Builder {
        self.elctrnc_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frst_pmt_dt` field.
    #[must_use]
    pub fn frst_pmt_dt(mut self, value: ISODate) -> CreditTransferMandateData1Builder {
        self.frst_pmt_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fnl_pmt_dt` field.
    #[must_use]
    pub fn fnl_pmt_dt(mut self, value: ISODate) -> CreditTransferMandateData1Builder {
        self.fnl_pmt_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frqcy` field.
    #[must_use]
    pub fn frqcy(
        mut self,
        value: crate::common::ChoiceWrapper<Frequency36Choice>,
    ) -> CreditTransferMandateData1Builder {
        self.frqcy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(
        mut self,
        value: crate::common::ChoiceWrapper<MandateSetupReason1Choice>,
    ) -> CreditTransferMandateData1Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<CreditTransferMandateData1, crate::common::BuilderError> {
        ::std::result::Result::Ok(CreditTransferMandateData1 {
            mndt_id: self.mndt_id,
            tp: self.tp,
            dt_of_sgntr: self.dt_of_sgntr,
            dt_of_vrfctn: self.dt_of_vrfctn,
            elctrnc_sgntr: self.elctrnc_sgntr,
            frst_pmt_dt: self.frst_pmt_dt,
            fnl_pmt_dt: self.fnl_pmt_dt,
            frqcy: self.frqcy,
            rsn: self.rsn,
        })
    }
}
impl CreditTransferMandateData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditTransferMandateData1Builder {
        CreditTransferMandateData1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditTransferTransaction52 {
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<PartyIdentification135>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification135,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3acct: Option<CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification135,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification135>,
    #[serde(rename = "InstrForCdtrAgt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instr_for_cdtr_agt: Vec<InstructionForCreditorAgent3>,
    #[serde(rename = "InstrForNxtAgt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instr_for_nxt_agt: Vec<InstructionForNextAgent1>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation10>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation21>,
    #[serde(rename = "InstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
/// Builder for [`CreditTransferTransaction52`]. Construct via [`CreditTransferTransaction52::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditTransferTransaction52Builder {
    ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
    initg_pty: ::std::option::Option<PartyIdentification135>,
    dbtr: ::std::option::Option<PartyIdentification135>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt1acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt2acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt3acct: ::std::option::Option<CashAccount40>,
    intrmy_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt1acct: ::std::option::Option<CashAccount40>,
    intrmy_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt2acct: ::std::option::Option<CashAccount40>,
    intrmy_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt3acct: ::std::option::Option<CashAccount40>,
    cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    cdtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr: ::std::option::Option<PartyIdentification135>,
    cdtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_cdtr: ::std::option::Option<PartyIdentification135>,
    instr_for_cdtr_agt: ::std::vec::Vec<InstructionForCreditorAgent3>,
    instr_for_nxt_agt: ::std::vec::Vec<InstructionForNextAgent1>,
    tax: ::std::option::Option<TaxInformation10>,
    rmt_inf: ::std::option::Option<RemittanceInformation21>,
    instd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl CreditTransferTransaction52Builder {
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: PartyIdentification135,
    ) -> CreditTransferTransaction52Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `initg_pty` field.
    #[must_use]
    pub fn initg_pty(
        mut self,
        value: PartyIdentification135,
    ) -> CreditTransferTransaction52Builder {
        self.initg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(mut self, value: PartyIdentification135) -> CreditTransferTransaction52Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1` field.
    #[must_use]
    pub fn prvs_instg_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1acct` field.
    #[must_use]
    pub fn prvs_instg_agt1acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2` field.
    #[must_use]
    pub fn prvs_instg_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2acct` field.
    #[must_use]
    pub fn prvs_instg_agt2acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3` field.
    #[must_use]
    pub fn prvs_instg_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3acct` field.
    #[must_use]
    pub fn prvs_instg_agt3acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction52Builder {
        self.prvs_instg_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1` field.
    #[must_use]
    pub fn intrmy_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.intrmy_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1acct` field.
    #[must_use]
    pub fn intrmy_agt1acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.intrmy_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2` field.
    #[must_use]
    pub fn intrmy_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.intrmy_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2acct` field.
    #[must_use]
    pub fn intrmy_agt2acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.intrmy_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3` field.
    #[must_use]
    pub fn intrmy_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.intrmy_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3acct` field.
    #[must_use]
    pub fn intrmy_agt3acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.intrmy_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction52Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(mut self, value: PartyIdentification135) -> CreditTransferTransaction52Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> CreditTransferTransaction52Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: PartyIdentification135,
    ) -> CreditTransferTransaction52Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_for_cdtr_agt` field (replaces any previously added items).
    #[must_use]
    pub fn instr_for_cdtr_agt(
        mut self,
        value: ::std::vec::Vec<InstructionForCreditorAgent3>,
    ) -> CreditTransferTransaction52Builder {
        self.instr_for_cdtr_agt = value;
        self
    }
    /// Append one item to the `instr_for_cdtr_agt` field.
    #[must_use]
    pub fn add_instr_for_cdtr_agt(
        mut self,
        value: InstructionForCreditorAgent3,
    ) -> CreditTransferTransaction52Builder {
        self.instr_for_cdtr_agt.push(value);
        self
    }
    /// Set the `instr_for_nxt_agt` field (replaces any previously added items).
    #[must_use]
    pub fn instr_for_nxt_agt(
        mut self,
        value: ::std::vec::Vec<InstructionForNextAgent1>,
    ) -> CreditTransferTransaction52Builder {
        self.instr_for_nxt_agt = value;
        self
    }
    /// Append one item to the `instr_for_nxt_agt` field.
    #[must_use]
    pub fn add_instr_for_nxt_agt(
        mut self,
        value: InstructionForNextAgent1,
    ) -> CreditTransferTransaction52Builder {
        self.instr_for_nxt_agt.push(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxInformation10) -> CreditTransferTransaction52Builder {
        self.tax = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(mut self, value: RemittanceInformation21) -> CreditTransferTransaction52Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_amt` field.
    #[must_use]
    pub fn instd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> CreditTransferTransaction52Builder {
        self.instd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<CreditTransferTransaction52, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.dbtr.is_none() {
            missing.push("dbtr".to_owned());
        }
        if self.dbtr_agt.is_none() {
            missing.push("dbtr_agt".to_owned());
        }
        if self.cdtr_agt.is_none() {
            missing.push("cdtr_agt".to_owned());
        }
        if self.cdtr.is_none() {
            missing.push("cdtr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CreditTransferTransaction52".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CreditTransferTransaction52 {
            ultmt_dbtr: self.ultmt_dbtr,
            initg_pty: self.initg_pty,
            dbtr: self.dbtr.unwrap(),
            dbtr_acct: self.dbtr_acct,
            dbtr_agt: self.dbtr_agt.unwrap(),
            dbtr_agt_acct: self.dbtr_agt_acct,
            prvs_instg_agt1: self.prvs_instg_agt1,
            prvs_instg_agt1acct: self.prvs_instg_agt1acct,
            prvs_instg_agt2: self.prvs_instg_agt2,
            prvs_instg_agt2acct: self.prvs_instg_agt2acct,
            prvs_instg_agt3: self.prvs_instg_agt3,
            prvs_instg_agt3acct: self.prvs_instg_agt3acct,
            intrmy_agt1: self.intrmy_agt1,
            intrmy_agt1acct: self.intrmy_agt1acct,
            intrmy_agt2: self.intrmy_agt2,
            intrmy_agt2acct: self.intrmy_agt2acct,
            intrmy_agt3: self.intrmy_agt3,
            intrmy_agt3acct: self.intrmy_agt3acct,
            cdtr_agt: self.cdtr_agt.unwrap(),
            cdtr_agt_acct: self.cdtr_agt_acct,
            cdtr: self.cdtr.unwrap(),
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
            instr_for_cdtr_agt: self.instr_for_cdtr_agt,
            instr_for_nxt_agt: self.instr_for_nxt_agt,
            tax: self.tax,
            rmt_inf: self.rmt_inf,
            instd_amt: self.instd_amt,
        })
    }
}
impl CreditTransferTransaction52 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditTransferTransaction52Builder {
        CreditTransferTransaction52Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditorReferenceInformation2 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditorReferenceType2>,
    #[serde(rename = "Ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
}
/// Builder for [`CreditorReferenceInformation2`]. Construct via [`CreditorReferenceInformation2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditorReferenceInformation2Builder {
    tp: ::std::option::Option<CreditorReferenceType2>,
    r#ref: ::std::option::Option<Max35Text>,
}
impl CreditorReferenceInformation2Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: CreditorReferenceType2) -> CreditorReferenceInformation2Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref` field.
    #[must_use]
    pub fn r#ref(mut self, value: Max35Text) -> CreditorReferenceInformation2Builder {
        self.r#ref = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<CreditorReferenceInformation2, crate::common::BuilderError> {
        ::std::result::Result::Ok(CreditorReferenceInformation2 {
            tp: self.tp,
            r#ref: self.r#ref,
        })
    }
}
impl CreditorReferenceInformation2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditorReferenceInformation2Builder {
        CreditorReferenceInformation2Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreditorReferenceType1Choice {
    #[serde(rename = "Cd")]
    Cd(DocumentType3Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditorReferenceType2 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<CreditorReferenceType1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`CreditorReferenceType2`]. Construct via [`CreditorReferenceType2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditorReferenceType2Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<CreditorReferenceType1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl CreditorReferenceType2Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<CreditorReferenceType1Choice>,
    ) -> CreditorReferenceType2Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> CreditorReferenceType2Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<CreditorReferenceType2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CreditorReferenceType2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CreditorReferenceType2 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl CreditorReferenceType2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditorReferenceType2Builder {
        CreditorReferenceType2Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DateAndDateTime2Choice {
    #[serde(rename = "Dt")]
    Dt(ISODate),
    #[serde(rename = "DtTm")]
    DtTm(ISODateTime),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DateAndPlaceOfBirth1 {
    #[serde(rename = "BirthDt")]
    pub birth_dt: ISODate,
    #[serde(rename = "PrvcOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvc_of_birth: Option<Max35Text>,
    #[serde(rename = "CityOfBirth")]
    pub city_of_birth: Max35Text,
    #[serde(rename = "CtryOfBirth")]
    pub ctry_of_birth: CountryCode,
}
/// Builder for [`DateAndPlaceOfBirth1`]. Construct via [`DateAndPlaceOfBirth1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DateAndPlaceOfBirth1Builder {
    birth_dt: ::std::option::Option<ISODate>,
    prvc_of_birth: ::std::option::Option<Max35Text>,
    city_of_birth: ::std::option::Option<Max35Text>,
    ctry_of_birth: ::std::option::Option<CountryCode>,
}
impl DateAndPlaceOfBirth1Builder {
    /// Set the `birth_dt` field.
    #[must_use]
    pub fn birth_dt(mut self, value: ISODate) -> DateAndPlaceOfBirth1Builder {
        self.birth_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvc_of_birth` field.
    #[must_use]
    pub fn prvc_of_birth(mut self, value: Max35Text) -> DateAndPlaceOfBirth1Builder {
        self.prvc_of_birth = ::std::option::Option::Some(value);
        self
    }
    /// Set the `city_of_birth` field.
    #[must_use]
    pub fn city_of_birth(mut self, value: Max35Text) -> DateAndPlaceOfBirth1Builder {
        self.city_of_birth = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry_of_birth` field.
    #[must_use]
    pub fn ctry_of_birth(mut self, value: CountryCode) -> DateAndPlaceOfBirth1Builder {
        self.ctry_of_birth = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<DateAndPlaceOfBirth1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.birth_dt.is_none() {
            missing.push("birth_dt".to_owned());
        }
        if self.city_of_birth.is_none() {
            missing.push("city_of_birth".to_owned());
        }
        if self.ctry_of_birth.is_none() {
            missing.push("ctry_of_birth".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DateAndPlaceOfBirth1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DateAndPlaceOfBirth1 {
            birth_dt: self.birth_dt.unwrap(),
            prvc_of_birth: self.prvc_of_birth,
            city_of_birth: self.city_of_birth.unwrap(),
            ctry_of_birth: self.ctry_of_birth.unwrap(),
        })
    }
}
impl DateAndPlaceOfBirth1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DateAndPlaceOfBirth1Builder {
        DateAndPlaceOfBirth1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DatePeriod2 {
    #[serde(rename = "FrDt")]
    pub fr_dt: ISODate,
    #[serde(rename = "ToDt")]
    pub to_dt: ISODate,
}
/// Builder for [`DatePeriod2`]. Construct via [`DatePeriod2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DatePeriod2Builder {
    fr_dt: ::std::option::Option<ISODate>,
    to_dt: ::std::option::Option<ISODate>,
}
impl DatePeriod2Builder {
    /// Set the `fr_dt` field.
    #[must_use]
    pub fn fr_dt(mut self, value: ISODate) -> DatePeriod2Builder {
        self.fr_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to_dt` field.
    #[must_use]
    pub fn to_dt(mut self, value: ISODate) -> DatePeriod2Builder {
        self.to_dt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<DatePeriod2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr_dt.is_none() {
            missing.push("fr_dt".to_owned());
        }
        if self.to_dt.is_none() {
            missing.push("to_dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DatePeriod2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DatePeriod2 {
            fr_dt: self.fr_dt.unwrap(),
            to_dt: self.to_dt.unwrap(),
        })
    }
}
impl DatePeriod2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DatePeriod2Builder {
        DatePeriod2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DiscountAmountAndType1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<DiscountAmountType1Choice>>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}
/// Builder for [`DiscountAmountAndType1`]. Construct via [`DiscountAmountAndType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DiscountAmountAndType1Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<DiscountAmountType1Choice>>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl DiscountAmountAndType1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<DiscountAmountType1Choice>,
    ) -> DiscountAmountAndType1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> DiscountAmountAndType1Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<DiscountAmountAndType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DiscountAmountAndType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DiscountAmountAndType1 {
            tp: self.tp,
            amt: self.amt.unwrap(),
        })
    }
}
impl DiscountAmountAndType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DiscountAmountAndType1Builder {
        DiscountAmountAndType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DiscountAmountType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalDiscountAmountType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    #[serde(rename = "PmtRtr")]
    pub pmt_rtr: PaymentReturnV11,
}
/// Builder for [`Document`]. Construct via [`Document::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentBuilder {
    pmt_rtr: ::std::option::Option<PaymentReturnV11>,
}
impl DocumentBuilder {
    /// Set the `pmt_rtr` field.
    #[must_use]
    pub fn pmt_rtr(mut self, value: PaymentReturnV11) -> DocumentBuilder {
        self.pmt_rtr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<Document, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pmt_rtr.is_none() {
            missing.push("pmt_rtr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Document".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Document {
            pmt_rtr: self.pmt_rtr.unwrap(),
        })
    }
}
impl Document {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentBuilder {
        DocumentBuilder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentAdjustment1 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max4Text>,
    #[serde(rename = "AddtlInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max140Text>,
}
/// Builder for [`DocumentAdjustment1`]. Construct via [`DocumentAdjustment1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentAdjustment1Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    rsn: ::std::option::Option<Max4Text>,
    addtl_inf: ::std::option::Option<Max140Text>,
}
impl DocumentAdjustment1Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> DocumentAdjustment1Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> DocumentAdjustment1Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(mut self, value: Max4Text) -> DocumentAdjustment1Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field.
    #[must_use]
    pub fn addtl_inf(mut self, value: Max140Text) -> DocumentAdjustment1Builder {
        self.addtl_inf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<DocumentAdjustment1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DocumentAdjustment1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DocumentAdjustment1 {
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind,
            rsn: self.rsn,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl DocumentAdjustment1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentAdjustment1Builder {
        DocumentAdjustment1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentLineIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<DocumentLineType1>,
    #[serde(rename = "Nb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
    #[serde(rename = "RltdDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<ISODate>,
}
/// Builder for [`DocumentLineIdentification1`]. Construct via [`DocumentLineIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentLineIdentification1Builder {
    tp: ::std::option::Option<DocumentLineType1>,
    nb: ::std::option::Option<Max35Text>,
    rltd_dt: ::std::option::Option<ISODate>,
}
impl DocumentLineIdentification1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: DocumentLineType1) -> DocumentLineIdentification1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb` field.
    #[must_use]
    pub fn nb(mut self, value: Max35Text) -> DocumentLineIdentification1Builder {
        self.nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_dt` field.
    #[must_use]
    pub fn rltd_dt(mut self, value: ISODate) -> DocumentLineIdentification1Builder {
        self.rltd_dt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<DocumentLineIdentification1, crate::common::BuilderError> {
        ::std::result::Result::Ok(DocumentLineIdentification1 {
            tp: self.tp,
            nb: self.nb,
            rltd_dt: self.rltd_dt,
        })
    }
}
impl DocumentLineIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentLineIdentification1Builder {
        DocumentLineIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentLineInformation1 {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub id: Vec<DocumentLineIdentification1>,
    #[serde(rename = "Desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max2048Text>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<RemittanceAmount3>,
}
/// Builder for [`DocumentLineInformation1`]. Construct via [`DocumentLineInformation1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentLineInformation1Builder {
    id: ::std::vec::Vec<DocumentLineIdentification1>,
    desc: ::std::option::Option<Max2048Text>,
    amt: ::std::option::Option<RemittanceAmount3>,
}
impl DocumentLineInformation1Builder {
    /// Set the `id` field (replaces any previously added items).
    #[must_use]
    pub fn id(
        mut self,
        value: ::std::vec::Vec<DocumentLineIdentification1>,
    ) -> DocumentLineInformation1Builder {
        self.id = value;
        self
    }
    /// Append one item to the `id` field.
    #[must_use]
    pub fn add_id(mut self, value: DocumentLineIdentification1) -> DocumentLineInformation1Builder {
        self.id.push(value);
        self
    }
    /// Set the `desc` field.
    #[must_use]
    pub fn desc(mut self, value: Max2048Text) -> DocumentLineInformation1Builder {
        self.desc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: RemittanceAmount3) -> DocumentLineInformation1Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<DocumentLineInformation1, crate::common::BuilderError> {
        ::std::result::Result::Ok(DocumentLineInformation1 {
            id: self.id,
            desc: self.desc,
            amt: self.amt,
        })
    }
}
impl DocumentLineInformation1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentLineInformation1Builder {
        DocumentLineInformation1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DocumentLineType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<DocumentLineType1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`DocumentLineType1`]. Construct via [`DocumentLineType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentLineType1Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<DocumentLineType1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl DocumentLineType1Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<DocumentLineType1Choice>,
    ) -> DocumentLineType1Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> DocumentLineType1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<DocumentLineType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DocumentLineType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DocumentLineType1 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl DocumentLineType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentLineType1Builder {
        DocumentLineType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocumentLineType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalDocumentLineType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EquivalentAmount2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyOfTrf")]
    pub ccy_of_trf: ActiveOrHistoricCurrencyCode,
}
/// Builder for [`EquivalentAmount2`]. Construct via [`EquivalentAmount2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct EquivalentAmount2Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ccy_of_trf: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
}
impl EquivalentAmount2Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> EquivalentAmount2Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy_of_trf` field.
    #[must_use]
    pub fn ccy_of_trf(mut self, value: ActiveOrHistoricCurrencyCode) -> EquivalentAmount2Builder {
        self.ccy_of_trf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<EquivalentAmount2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.ccy_of_trf.is_none() {
            missing.push("ccy_of_trf".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "EquivalentAmount2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(EquivalentAmount2 {
            amt: self.amt.unwrap(),
            ccy_of_trf: self.ccy_of_trf.unwrap(),
        })
    }
}
impl EquivalentAmount2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> EquivalentAmount2Builder {
        EquivalentAmount2Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum FinancialIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalFinancialInstitutionIdentification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FinancialInstitutionIdentification18 {
    #[serde(rename = "BICFI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bicfi: Option<BICFIDec2014Identifier>,
    #[serde(rename = "ClrSysMmbId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_mmb_id: Option<ClearingSystemMemberIdentification2>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<LEIIdentifier>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Othr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
/// Builder for [`FinancialInstitutionIdentification18`]. Construct via [`FinancialInstitutionIdentification18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FinancialInstitutionIdentification18Builder {
    bicfi: ::std::option::Option<BICFIDec2014Identifier>,
    clr_sys_mmb_id: ::std::option::Option<ClearingSystemMemberIdentification2>,
    lei: ::std::option::Option<LEIIdentifier>,
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress24>,
    othr: ::std::option::Option<GenericFinancialIdentification1>,
}
impl FinancialInstitutionIdentification18Builder {
    /// Set the `bicfi` field.
    #[must_use]
    pub fn bicfi(
        mut self,
        value: BICFIDec2014Identifier,
    ) -> FinancialInstitutionIdentification18Builder {
        self.bicfi = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys_mmb_id` field.
    #[must_use]
    pub fn clr_sys_mmb_id(
        mut self,
        value: ClearingSystemMemberIdentification2,
    ) -> FinancialInstitutionIdentification18Builder {
        self.clr_sys_mmb_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> FinancialInstitutionIdentification18Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> FinancialInstitutionIdentification18Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(
        mut self,
        value: PostalAddress24,
    ) -> FinancialInstitutionIdentification18Builder {
        self.pstl_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field.
    #[must_use]
    pub fn othr(
        mut self,
        value: GenericFinancialIdentification1,
    ) -> FinancialInstitutionIdentification18Builder {
        self.othr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<FinancialInstitutionIdentification18, crate::common::BuilderError>
    {
        ::std::result::Result::Ok(FinancialInstitutionIdentification18 {
            bicfi: self.bicfi,
            clr_sys_mmb_id: self.clr_sys_mmb_id,
            lei: self.lei,
            nm: self.nm,
            pstl_adr: self.pstl_adr,
            othr: self.othr,
        })
    }
}
impl FinancialInstitutionIdentification18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FinancialInstitutionIdentification18Builder {
        FinancialInstitutionIdentification18Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Frequency36Choice {
    #[serde(rename = "Tp")]
    Tp(Frequency6Code),
    #[serde(rename = "Prd")]
    Prd(FrequencyPeriod1),
    #[serde(rename = "PtInTm")]
    PtInTm(FrequencyAndMoment1),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrequencyAndMoment1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "PtInTm")]
    pub pt_in_tm: Exact2NumericText,
}
/// Builder for [`FrequencyAndMoment1`]. Construct via [`FrequencyAndMoment1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FrequencyAndMoment1Builder {
    tp: ::std::option::Option<Frequency6Code>,
    pt_in_tm: ::std::option::Option<Exact2NumericText>,
}
impl FrequencyAndMoment1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Frequency6Code) -> FrequencyAndMoment1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pt_in_tm` field.
    #[must_use]
    pub fn pt_in_tm(mut self, value: Exact2NumericText) -> FrequencyAndMoment1Builder {
        self.pt_in_tm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<FrequencyAndMoment1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.pt_in_tm.is_none() {
            missing.push("pt_in_tm".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "FrequencyAndMoment1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(FrequencyAndMoment1 {
            tp: self.tp.unwrap(),
            pt_in_tm: self.pt_in_tm.unwrap(),
        })
    }
}
impl FrequencyAndMoment1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FrequencyAndMoment1Builder {
        FrequencyAndMoment1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FrequencyPeriod1 {
    #[serde(rename = "Tp")]
    pub tp: Frequency6Code,
    #[serde(rename = "CntPerPrd")]
    pub cnt_per_prd: DecimalNumber,
}
/// Builder for [`FrequencyPeriod1`]. Construct via [`FrequencyPeriod1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FrequencyPeriod1Builder {
    tp: ::std::option::Option<Frequency6Code>,
    cnt_per_prd: ::std::option::Option<DecimalNumber>,
}
impl FrequencyPeriod1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Frequency6Code) -> FrequencyPeriod1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cnt_per_prd` field.
    #[must_use]
    pub fn cnt_per_prd(mut self, value: DecimalNumber) -> FrequencyPeriod1Builder {
        self.cnt_per_prd = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<FrequencyPeriod1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.cnt_per_prd.is_none() {
            missing.push("cnt_per_prd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "FrequencyPeriod1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(FrequencyPeriod1 {
            tp: self.tp.unwrap(),
            cnt_per_prd: self.cnt_per_prd.unwrap(),
        })
    }
}
impl FrequencyPeriod1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FrequencyPeriod1Builder {
        FrequencyPeriod1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Garnishment3 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType1,
    #[serde(rename = "Grnshee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification135>,
    #[serde(rename = "GrnshmtAdmstr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification135>,
    #[serde(rename = "RefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<Max140Text>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<ISODate>,
    #[serde(rename = "RmtdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "FmlyMdclInsrncInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fmly_mdcl_insrnc_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "MplyeeTermntnInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mplyee_termntn_ind: Option<TrueFalseIndicator>,
}
/// Builder for [`Garnishment3`]. Construct via [`Garnishment3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Garnishment3Builder {
    tp: ::std::option::Option<GarnishmentType1>,
    grnshee: ::std::option::Option<PartyIdentification135>,
    grnshmt_admstr: ::std::option::Option<PartyIdentification135>,
    ref_nb: ::std::option::Option<Max140Text>,
    dt: ::std::option::Option<ISODate>,
    rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    fmly_mdcl_insrnc_ind: ::std::option::Option<TrueFalseIndicator>,
    mplyee_termntn_ind: ::std::option::Option<TrueFalseIndicator>,
}
impl Garnishment3Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: GarnishmentType1) -> Garnishment3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshee` field.
    #[must_use]
    pub fn grnshee(mut self, value: PartyIdentification135) -> Garnishment3Builder {
        self.grnshee = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshmt_admstr` field.
    #[must_use]
    pub fn grnshmt_admstr(mut self, value: PartyIdentification135) -> Garnishment3Builder {
        self.grnshmt_admstr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref_nb` field.
    #[must_use]
    pub fn ref_nb(mut self, value: Max140Text) -> Garnishment3Builder {
        self.ref_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> Garnishment3Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmtd_amt` field.
    #[must_use]
    pub fn rmtd_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> Garnishment3Builder {
        self.rmtd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fmly_mdcl_insrnc_ind` field.
    #[must_use]
    pub fn fmly_mdcl_insrnc_ind(mut self, value: TrueFalseIndicator) -> Garnishment3Builder {
        self.fmly_mdcl_insrnc_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mplyee_termntn_ind` field.
    #[must_use]
    pub fn mplyee_termntn_ind(mut self, value: TrueFalseIndicator) -> Garnishment3Builder {
        self.mplyee_termntn_ind = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<Garnishment3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Garnishment3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Garnishment3 {
            tp: self.tp.unwrap(),
            grnshee: self.grnshee,
            grnshmt_admstr: self.grnshmt_admstr,
            ref_nb: self.ref_nb,
            dt: self.dt,
            rmtd_amt: self.rmtd_amt,
            fmly_mdcl_insrnc_ind: self.fmly_mdcl_insrnc_ind,
            mplyee_termntn_ind: self.mplyee_termntn_ind,
        })
    }
}
impl Garnishment3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Garnishment3Builder {
        Garnishment3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GarnishmentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<GarnishmentType1Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GarnishmentType1`]. Construct via [`GarnishmentType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GarnishmentType1Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<GarnishmentType1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl GarnishmentType1Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<GarnishmentType1Choice>,
    ) -> GarnishmentType1Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GarnishmentType1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<GarnishmentType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GarnishmentType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GarnishmentType1 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl GarnishmentType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GarnishmentType1Builder {
        GarnishmentType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum GarnishmentType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalGarnishmentType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericAccountIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max34Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<AccountSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericAccountIdentification1`]. Construct via [`GenericAccountIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericAccountIdentification1Builder {
    id: ::std::option::Option<Max34Text>,
    schme_nm: ::std::option::Option<crate::common::ChoiceWrapper<AccountSchemeName1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericAccountIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max34Text) -> GenericAccountIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<AccountSchemeName1Choice>,
    ) -> GenericAccountIdentification1Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericAccountIdentification1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<GenericAccountIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericAccountIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericAccountIdentification1 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericAccountIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericAccountIdentification1Builder {
        GenericAccountIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericFinancialIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<FinancialIdentificationSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericFinancialIdentification1`]. Construct via [`GenericFinancialIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericFinancialIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    schme_nm: ::std::option::Option<
        crate::common::ChoiceWrapper<FinancialIdentificationSchemeName1Choice>,
    >,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericFinancialIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericFinancialIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<FinancialIdentificationSchemeName1Choice>,
    ) -> GenericFinancialIdentification1Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericFinancialIdentification1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<GenericFinancialIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericFinancialIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericFinancialIdentification1 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericFinancialIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericFinancialIdentification1Builder {
        GenericFinancialIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericIdentification30 {
    #[serde(rename = "Id")]
    pub id: Exact4AlphaNumericText,
    #[serde(rename = "Issr")]
    pub issr: Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
}
/// Builder for [`GenericIdentification30`]. Construct via [`GenericIdentification30::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericIdentification30Builder {
    id: ::std::option::Option<Exact4AlphaNumericText>,
    issr: ::std::option::Option<Max35Text>,
    schme_nm: ::std::option::Option<Max35Text>,
}
impl GenericIdentification30Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Exact4AlphaNumericText) -> GenericIdentification30Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericIdentification30Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(mut self, value: Max35Text) -> GenericIdentification30Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<GenericIdentification30, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if self.issr.is_none() {
            missing.push("issr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericIdentification30".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericIdentification30 {
            id: self.id.unwrap(),
            issr: self.issr.unwrap(),
            schme_nm: self.schme_nm,
        })
    }
}
impl GenericIdentification30 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericIdentification30Builder {
        GenericIdentification30Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericOrganisationIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericOrganisationIdentification1`]. Construct via [`GenericOrganisationIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericOrganisationIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    schme_nm: ::std::option::Option<
        crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>,
    >,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericOrganisationIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericOrganisationIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>,
    ) -> GenericOrganisationIdentification1Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericOrganisationIdentification1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<GenericOrganisationIdentification1, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericOrganisationIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericOrganisationIdentification1 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericOrganisationIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericOrganisationIdentification1Builder {
        GenericOrganisationIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericPersonIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericPersonIdentification1`]. Construct via [`GenericPersonIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericPersonIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    schme_nm:
        ::std::option::Option<crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericPersonIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericPersonIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>,
    ) -> GenericPersonIdentification1Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericPersonIdentification1Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<GenericPersonIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericPersonIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericPersonIdentification1 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericPersonIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericPersonIdentification1Builder {
        GenericPersonIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupHeader99 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: ISODateTime,
    #[serde(rename = "Authstn")]
    /// Maximum 2 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authstn: Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    #[serde(rename = "BtchBookg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<BatchBookingIndicator>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericText,
    #[serde(rename = "CtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[serde(rename = "GrpRtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_rtr: Option<TrueFalseIndicator>,
    #[serde(rename = "TtlRtrdIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_rtrd_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "SttlmInf")]
    pub sttlm_inf: SettlementInstruction11,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
}
/// Builder for [`GroupHeader99`]. Construct via [`GroupHeader99::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GroupHeader99Builder {
    msg_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    authstn: ::std::vec::Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    btch_bookg: ::std::option::Option<BatchBookingIndicator>,
    nb_of_txs: ::std::option::Option<Max15NumericText>,
    ctrl_sum: ::std::option::Option<DecimalNumber>,
    grp_rtr: ::std::option::Option<TrueFalseIndicator>,
    ttl_rtrd_intr_bk_sttlm_amt: ::std::option::Option<ActiveCurrencyAndAmount>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    sttlm_inf: ::std::option::Option<SettlementInstruction11>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation28>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
}
impl GroupHeader99Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> GroupHeader99Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> GroupHeader99Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authstn` field (replaces any previously added items).
    #[must_use]
    pub fn authstn(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    ) -> GroupHeader99Builder {
        self.authstn = value;
        self
    }
    /// Append one item to the `authstn` field.
    #[must_use]
    pub fn add_authstn(
        mut self,
        value: crate::common::ChoiceWrapper<Authorisation1Choice>,
    ) -> GroupHeader99Builder {
        self.authstn.push(value);
        self
    }
    /// Set the `btch_bookg` field.
    #[must_use]
    pub fn btch_bookg(mut self, value: BatchBookingIndicator) -> GroupHeader99Builder {
        self.btch_bookg = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_txs` field.
    #[must_use]
    pub fn nb_of_txs(mut self, value: Max15NumericText) -> GroupHeader99Builder {
        self.nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrl_sum` field.
    #[must_use]
    pub fn ctrl_sum(mut self, value: DecimalNumber) -> GroupHeader99Builder {
        self.ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grp_rtr` field.
    #[must_use]
    pub fn grp_rtr(mut self, value: TrueFalseIndicator) -> GroupHeader99Builder {
        self.grp_rtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_rtrd_intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn ttl_rtrd_intr_bk_sttlm_amt(
        mut self,
        value: ActiveCurrencyAndAmount,
    ) -> GroupHeader99Builder {
        self.ttl_rtrd_intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> GroupHeader99Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_inf` field.
    #[must_use]
    pub fn sttlm_inf(mut self, value: SettlementInstruction11) -> GroupHeader99Builder {
        self.sttlm_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(mut self, value: PaymentTypeInformation28) -> GroupHeader99Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> GroupHeader99Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> GroupHeader99Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<GroupHeader99, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.msg_id.is_none() {
            missing.push("msg_id".to_owned());
        }
        if self.cre_dt_tm.is_none() {
            missing.push("cre_dt_tm".to_owned());
        }
        if self.nb_of_txs.is_none() {
            missing.push("nb_of_txs".to_owned());
        }
        if self.sttlm_inf.is_none() {
            missing.push("sttlm_inf".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GroupHeader99".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GroupHeader99 {
            msg_id: self.msg_id.unwrap(),
            cre_dt_tm: self.cre_dt_tm.unwrap(),
            authstn: self.authstn,
            btch_bookg: self.btch_bookg,
            nb_of_txs: self.nb_of_txs.unwrap(),
            ctrl_sum: self.ctrl_sum,
            grp_rtr: self.grp_rtr,
            ttl_rtrd_intr_bk_sttlm_amt: self.ttl_rtrd_intr_bk_sttlm_amt,
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            sttlm_inf: self.sttlm_inf.unwrap(),
            pmt_tp_inf: self.pmt_tp_inf,
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
        })
    }
}
impl GroupHeader99 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GroupHeader99Builder {
        GroupHeader99Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstructionForCreditorAgent3 {
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalCreditorAgentInstruction1Code>,
    #[serde(rename = "InstrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<Max140Text>,
}
/// Builder for [`InstructionForCreditorAgent3`]. Construct via [`InstructionForCreditorAgent3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct InstructionForCreditorAgent3Builder {
    cd: ::std::option::Option<ExternalCreditorAgentInstruction1Code>,
    instr_inf: ::std::option::Option<Max140Text>,
}
impl InstructionForCreditorAgent3Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(
        mut self,
        value: ExternalCreditorAgentInstruction1Code,
    ) -> InstructionForCreditorAgent3Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_inf` field.
    #[must_use]
    pub fn instr_inf(mut self, value: Max140Text) -> InstructionForCreditorAgent3Builder {
        self.instr_inf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<InstructionForCreditorAgent3, crate::common::BuilderError> {
        ::std::result::Result::Ok(InstructionForCreditorAgent3 {
            cd: self.cd,
            instr_inf: self.instr_inf,
        })
    }
}
impl InstructionForCreditorAgent3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> InstructionForCreditorAgent3Builder {
        InstructionForCreditorAgent3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InstructionForNextAgent1 {
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<Instruction4Code>,
    #[serde(rename = "InstrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<Max140Text>,
}
/// Builder for [`InstructionForNextAgent1`]. Construct via [`InstructionForNextAgent1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct InstructionForNextAgent1Builder {
    cd: ::std::option::Option<Instruction4Code>,
    instr_inf: ::std::option::Option<Max140Text>,
}
impl InstructionForNextAgent1Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(mut self, value: Instruction4Code) -> InstructionForNextAgent1Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_inf` field.
    #[must_use]
    pub fn instr_inf(mut self, value: Max140Text) -> InstructionForNextAgent1Builder {
        self.instr_inf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<InstructionForNextAgent1, crate::common::BuilderError> {
        ::std::result::Result::Ok(InstructionForNextAgent1 {
            cd: self.cd,
            instr_inf: self.instr_inf,
        })
    }
}
impl InstructionForNextAgent1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> InstructionForNextAgent1Builder {
        InstructionForNextAgent1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocalInstrument2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalLocalInstrument1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MandateClassification1Choice {
    #[serde(rename = "Cd")]
    Cd(MandateClassification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MandateRelatedData2Choice {
    #[serde(rename = "DrctDbtMndt")]
    DrctDbtMndt(MandateRelatedInformation15),
    #[serde(rename = "CdtTrfMndt")]
    CdtTrfMndt(CreditTransferMandateData1),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MandateRelatedInformation15 {
    #[serde(rename = "MndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<Max35Text>,
    #[serde(rename = "DtOfSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_of_sgntr: Option<ISODate>,
    #[serde(rename = "AmdmntInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amdmnt_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "AmdmntInfDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amdmnt_inf_dtls: Option<AmendmentInformationDetails14>,
    #[serde(rename = "ElctrncSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_sgntr: Option<Max1025Text>,
    #[serde(rename = "FrstColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frst_colltn_dt: Option<ISODate>,
    #[serde(rename = "FnlColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fnl_colltn_dt: Option<ISODate>,
    #[serde(rename = "Frqcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frqcy: Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
    #[serde(rename = "TrckgDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trckg_days: Option<Exact2NumericText>,
}
/// Builder for [`MandateRelatedInformation15`]. Construct via [`MandateRelatedInformation15::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct MandateRelatedInformation15Builder {
    mndt_id: ::std::option::Option<Max35Text>,
    dt_of_sgntr: ::std::option::Option<ISODate>,
    amdmnt_ind: ::std::option::Option<TrueFalseIndicator>,
    amdmnt_inf_dtls: ::std::option::Option<AmendmentInformationDetails14>,
    elctrnc_sgntr: ::std::option::Option<Max1025Text>,
    frst_colltn_dt: ::std::option::Option<ISODate>,
    fnl_colltn_dt: ::std::option::Option<ISODate>,
    frqcy: ::std::option::Option<crate::common::ChoiceWrapper<Frequency36Choice>>,
    rsn: ::std::option::Option<crate::common::ChoiceWrapper<MandateSetupReason1Choice>>,
    trckg_days: ::std::option::Option<Exact2NumericText>,
}
impl MandateRelatedInformation15Builder {
    /// Set the `mndt_id` field.
    #[must_use]
    pub fn mndt_id(mut self, value: Max35Text) -> MandateRelatedInformation15Builder {
        self.mndt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt_of_sgntr` field.
    #[must_use]
    pub fn dt_of_sgntr(mut self, value: ISODate) -> MandateRelatedInformation15Builder {
        self.dt_of_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amdmnt_ind` field.
    #[must_use]
    pub fn amdmnt_ind(mut self, value: TrueFalseIndicator) -> MandateRelatedInformation15Builder {
        self.amdmnt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amdmnt_inf_dtls` field.
    #[must_use]
    pub fn amdmnt_inf_dtls(
        mut self,
        value: AmendmentInformationDetails14,
    ) -> MandateRelatedInformation15Builder {
        self.amdmnt_inf_dtls = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_sgntr` field.
    #[must_use]
    pub fn elctrnc_sgntr(mut self, value: Max1025Text) -> MandateRelatedInformation15Builder {
        self.elctrnc_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frst_colltn_dt` field.
    #[must_use]
    pub fn frst_colltn_dt(mut self, value: ISODate) -> MandateRelatedInformation15Builder {
        self.frst_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fnl_colltn_dt` field.
    #[must_use]
    pub fn fnl_colltn_dt(mut self, value: ISODate) -> MandateRelatedInformation15Builder {
        self.fnl_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frqcy` field.
    #[must_use]
    pub fn frqcy(
        mut self,
        value: crate::common::ChoiceWrapper<Frequency36Choice>,
    ) -> MandateRelatedInformation15Builder {
        self.frqcy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(
        mut self,
        value: crate::common::ChoiceWrapper<MandateSetupReason1Choice>,
    ) -> MandateRelatedInformation15Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trckg_days` field.
    #[must_use]
    pub fn trckg_days(mut self, value: Exact2NumericText) -> MandateRelatedInformation15Builder {
        self.trckg_days = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<MandateRelatedInformation15, crate::common::BuilderError> {
        ::std::result::Result::Ok(MandateRelatedInformation15 {
            mndt_id: self.mndt_id,
            dt_of_sgntr: self.dt_of_sgntr,
            amdmnt_ind: self.amdmnt_ind,
            amdmnt_inf_dtls: self.amdmnt_inf_dtls,
            elctrnc_sgntr: self.elctrnc_sgntr,
            frst_colltn_dt: self.frst_colltn_dt,
            fnl_colltn_dt: self.fnl_colltn_dt,
            frqcy: self.frqcy,
            rsn: self.rsn,
            trckg_days: self.trckg_days,
        })
    }
}
impl MandateRelatedInformation15 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> MandateRelatedInformation15Builder {
        MandateRelatedInformation15Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum MandateSetupReason1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalMandateSetupReason1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max70Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MandateTypeInformation2 {
    #[serde(rename = "SvcLvl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svc_lvl: Option<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    #[serde(rename = "CtgyPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
    #[serde(rename = "Clssfctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clssfctn: Option<crate::common::ChoiceWrapper<MandateClassification1Choice>>,
}
/// Builder for [`MandateTypeInformation2`]. Construct via [`MandateTypeInformation2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct MandateTypeInformation2Builder {
    svc_lvl: ::std::option::Option<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    lcl_instrm: ::std::option::Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    ctgy_purp: ::std::option::Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
    clssfctn: ::std::option::Option<crate::common::ChoiceWrapper<MandateClassification1Choice>>,
}
impl MandateTypeInformation2Builder {
    /// Set the `svc_lvl` field.
    #[must_use]
    pub fn svc_lvl(
        mut self,
        value: crate::common::ChoiceWrapper<ServiceLevel8Choice>,
    ) -> MandateTypeInformation2Builder {
        self.svc_lvl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lcl_instrm` field.
    #[must_use]
    pub fn lcl_instrm(
        mut self,
        value: crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    ) -> MandateTypeInformation2Builder {
        self.lcl_instrm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy_purp` field.
    #[must_use]
    pub fn ctgy_purp(
        mut self,
        value: crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    ) -> MandateTypeInformation2Builder {
        self.ctgy_purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clssfctn` field.
    #[must_use]
    pub fn clssfctn(
        mut self,
        value: crate::common::ChoiceWrapper<MandateClassification1Choice>,
    ) -> MandateTypeInformation2Builder {
        self.clssfctn = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<MandateTypeInformation2, crate::common::BuilderError> {
        ::std::result::Result::Ok(MandateTypeInformation2 {
            svc_lvl: self.svc_lvl,
            lcl_instrm: self.lcl_instrm,
            ctgy_purp: self.ctgy_purp,
            clssfctn: self.clssfctn,
        })
    }
}
impl MandateTypeInformation2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> MandateTypeInformation2Builder {
        MandateTypeInformation2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrganisationIdentification29 {
    #[serde(rename = "AnyBIC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBICDec2014Identifier>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<LEIIdentifier>,
    #[serde(rename = "Othr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr: Vec<GenericOrganisationIdentification1>,
}
/// Builder for [`OrganisationIdentification29`]. Construct via [`OrganisationIdentification29::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OrganisationIdentification29Builder {
    any_bic: ::std::option::Option<AnyBICDec2014Identifier>,
    lei: ::std::option::Option<LEIIdentifier>,
    othr: ::std::vec::Vec<GenericOrganisationIdentification1>,
}
impl OrganisationIdentification29Builder {
    /// Set the `any_bic` field.
    #[must_use]
    pub fn any_bic(
        mut self,
        value: AnyBICDec2014Identifier,
    ) -> OrganisationIdentification29Builder {
        self.any_bic = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> OrganisationIdentification29Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(
        mut self,
        value: ::std::vec::Vec<GenericOrganisationIdentification1>,
    ) -> OrganisationIdentification29Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(
        mut self,
        value: GenericOrganisationIdentification1,
    ) -> OrganisationIdentification29Builder {
        self.othr.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<OrganisationIdentification29, crate::common::BuilderError> {
        ::std::result::Result::Ok(OrganisationIdentification29 {
            any_bic: self.any_bic,
            lei: self.lei,
            othr: self.othr,
        })
    }
}
impl OrganisationIdentification29 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OrganisationIdentification29Builder {
        OrganisationIdentification29Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OrganisationIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalOrganisationIdentification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OriginalGroupHeader18 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: Max35Text,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<ISODateTime>,
    #[serde(rename = "RtrRsnInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rtr_rsn_inf: Vec<PaymentReturnReason6>,
}
/// Builder for [`OriginalGroupHeader18`]. Construct via [`OriginalGroupHeader18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalGroupHeader18Builder {
    orgnl_msg_id: ::std::option::Option<Max35Text>,
    orgnl_msg_nm_id: ::std::option::Option<Max35Text>,
    orgnl_cre_dt_tm: ::std::option::Option<ISODateTime>,
    rtr_rsn_inf: ::std::vec::Vec<PaymentReturnReason6>,
}
impl OriginalGroupHeader18Builder {
    /// Set the `orgnl_msg_id` field.
    #[must_use]
    pub fn orgnl_msg_id(mut self, value: Max35Text) -> OriginalGroupHeader18Builder {
        self.orgnl_msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_msg_nm_id` field.
    #[must_use]
    pub fn orgnl_msg_nm_id(mut self, value: Max35Text) -> OriginalGroupHeader18Builder {
        self.orgnl_msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cre_dt_tm` field.
    #[must_use]
    pub fn orgnl_cre_dt_tm(mut self, value: ISODateTime) -> OriginalGroupHeader18Builder {
        self.orgnl_cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtr_rsn_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rtr_rsn_inf(
        mut self,
        value: ::std::vec::Vec<PaymentReturnReason6>,
    ) -> OriginalGroupHeader18Builder {
        self.rtr_rsn_inf = value;
        self
    }
    /// Append one item to the `rtr_rsn_inf` field.
    #[must_use]
    pub fn add_rtr_rsn_inf(mut self, value: PaymentReturnReason6) -> OriginalGroupHeader18Builder {
        self.rtr_rsn_inf.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<OriginalGroupHeader18, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.orgnl_msg_id.is_none() {
            missing.push("orgnl_msg_id".to_owned());
        }
        if self.orgnl_msg_nm_id.is_none() {
            missing.push("orgnl_msg_nm_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OriginalGroupHeader18".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OriginalGroupHeader18 {
            orgnl_msg_id: self.orgnl_msg_id.unwrap(),
            orgnl_msg_nm_id: self.orgnl_msg_nm_id.unwrap(),
            orgnl_cre_dt_tm: self.orgnl_cre_dt_tm,
            rtr_rsn_inf: self.rtr_rsn_inf,
        })
    }
}
impl OriginalGroupHeader18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalGroupHeader18Builder {
        OriginalGroupHeader18Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OriginalGroupInformation29 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: Max35Text,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<ISODateTime>,
}
/// Builder for [`OriginalGroupInformation29`]. Construct via [`OriginalGroupInformation29::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalGroupInformation29Builder {
    orgnl_msg_id: ::std::option::Option<Max35Text>,
    orgnl_msg_nm_id: ::std::option::Option<Max35Text>,
    orgnl_cre_dt_tm: ::std::option::Option<ISODateTime>,
}
impl OriginalGroupInformation29Builder {
    /// Set the `orgnl_msg_id` field.
    #[must_use]
    pub fn orgnl_msg_id(mut self, value: Max35Text) -> OriginalGroupInformation29Builder {
        self.orgnl_msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_msg_nm_id` field.
    #[must_use]
    pub fn orgnl_msg_nm_id(mut self, value: Max35Text) -> OriginalGroupInformation29Builder {
        self.orgnl_msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cre_dt_tm` field.
    #[must_use]
    pub fn orgnl_cre_dt_tm(mut self, value: ISODateTime) -> OriginalGroupInformation29Builder {
        self.orgnl_cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<OriginalGroupInformation29, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.orgnl_msg_id.is_none() {
            missing.push("orgnl_msg_id".to_owned());
        }
        if self.orgnl_msg_nm_id.is_none() {
            missing.push("orgnl_msg_nm_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OriginalGroupInformation29".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OriginalGroupInformation29 {
            orgnl_msg_id: self.orgnl_msg_id.unwrap(),
            orgnl_msg_nm_id: self.orgnl_msg_nm_id.unwrap(),
            orgnl_cre_dt_tm: self.orgnl_cre_dt_tm,
        })
    }
}
impl OriginalGroupInformation29 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalGroupInformation29Builder {
        OriginalGroupInformation29Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OriginalTransactionReference36 {
    #[serde(rename = "IntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<crate::common::ChoiceWrapper<AmountType4Choice>>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "ReqdColltnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqd_colltn_dt: Option<ISODate>,
    #[serde(rename = "ReqdExctnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqd_exctn_dt: Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    #[serde(rename = "CdtrSchmeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_schme_id: Option<PartyIdentification135>,
    #[serde(rename = "SttlmInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_inf: Option<SettlementInstruction11>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation27>,
    #[serde(rename = "PmtMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_mtd: Option<PaymentMethod4Code>,
    #[serde(rename = "MndtRltdInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<crate::common::ChoiceWrapper<MandateRelatedData2Choice>>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation21>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Dbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "Cdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    #[serde(rename = "UndrlygCstmrCdtTrf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undrlyg_cstmr_cdt_trf: Option<CreditTransferTransaction52>,
}
/// Builder for [`OriginalTransactionReference36`]. Construct via [`OriginalTransactionReference36::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalTransactionReference36Builder {
    intr_bk_sttlm_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    amt: ::std::option::Option<crate::common::ChoiceWrapper<AmountType4Choice>>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    reqd_colltn_dt: ::std::option::Option<ISODate>,
    reqd_exctn_dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    sttlm_inf: ::std::option::Option<SettlementInstruction11>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation27>,
    pmt_mtd: ::std::option::Option<PaymentMethod4Code>,
    mndt_rltd_inf: ::std::option::Option<crate::common::ChoiceWrapper<MandateRelatedData2Choice>>,
    rmt_inf: ::std::option::Option<RemittanceInformation21>,
    ultmt_dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    cdtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    cdtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    purp: ::std::option::Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    undrlyg_cstmr_cdt_trf: ::std::option::Option<CreditTransferTransaction52>,
}
impl OriginalTransactionReference36Builder {
    /// Set the `intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn intr_bk_sttlm_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> OriginalTransactionReference36Builder {
        self.intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: crate::common::ChoiceWrapper<AmountType4Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> OriginalTransactionReference36Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_colltn_dt` field.
    #[must_use]
    pub fn reqd_colltn_dt(mut self, value: ISODate) -> OriginalTransactionReference36Builder {
        self.reqd_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_exctn_dt` field.
    #[must_use]
    pub fn reqd_exctn_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.reqd_exctn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_schme_id` field.
    #[must_use]
    pub fn cdtr_schme_id(
        mut self,
        value: PartyIdentification135,
    ) -> OriginalTransactionReference36Builder {
        self.cdtr_schme_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_inf` field.
    #[must_use]
    pub fn sttlm_inf(
        mut self,
        value: SettlementInstruction11,
    ) -> OriginalTransactionReference36Builder {
        self.sttlm_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(
        mut self,
        value: PaymentTypeInformation27,
    ) -> OriginalTransactionReference36Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_mtd` field.
    #[must_use]
    pub fn pmt_mtd(mut self, value: PaymentMethod4Code) -> OriginalTransactionReference36Builder {
        self.pmt_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mndt_rltd_inf` field.
    #[must_use]
    pub fn mndt_rltd_inf(
        mut self,
        value: crate::common::ChoiceWrapper<MandateRelatedData2Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.mndt_rltd_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(
        mut self,
        value: RemittanceInformation21,
    ) -> OriginalTransactionReference36Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> OriginalTransactionReference36Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> OriginalTransactionReference36Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(mut self, value: CashAccount40) -> OriginalTransactionReference36Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> OriginalTransactionReference36Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(mut self, value: CashAccount40) -> OriginalTransactionReference36Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> OriginalTransactionReference36Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `purp` field.
    #[must_use]
    pub fn purp(
        mut self,
        value: crate::common::ChoiceWrapper<Purpose2Choice>,
    ) -> OriginalTransactionReference36Builder {
        self.purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `undrlyg_cstmr_cdt_trf` field.
    #[must_use]
    pub fn undrlyg_cstmr_cdt_trf(
        mut self,
        value: CreditTransferTransaction52,
    ) -> OriginalTransactionReference36Builder {
        self.undrlyg_cstmr_cdt_trf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<OriginalTransactionReference36, crate::common::BuilderError> {
        ::std::result::Result::Ok(OriginalTransactionReference36 {
            intr_bk_sttlm_amt: self.intr_bk_sttlm_amt,
            amt: self.amt,
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            reqd_colltn_dt: self.reqd_colltn_dt,
            reqd_exctn_dt: self.reqd_exctn_dt,
            cdtr_schme_id: self.cdtr_schme_id,
            sttlm_inf: self.sttlm_inf,
            pmt_tp_inf: self.pmt_tp_inf,
            pmt_mtd: self.pmt_mtd,
            mndt_rltd_inf: self.mndt_rltd_inf,
            rmt_inf: self.rmt_inf,
            ultmt_dbtr: self.ultmt_dbtr,
            dbtr: self.dbtr,
            dbtr_acct: self.dbtr_acct,
            dbtr_agt: self.dbtr_agt,
            dbtr_agt_acct: self.dbtr_agt_acct,
            cdtr_agt: self.cdtr_agt,
            cdtr_agt_acct: self.cdtr_agt_acct,
            cdtr: self.cdtr,
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
            purp: self.purp,
            undrlyg_cstmr_cdt_trf: self.undrlyg_cstmr_cdt_trf,
        })
    }
}
impl OriginalTransactionReference36 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalTransactionReference36Builder {
        OriginalTransactionReference36Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtherContact1 {
    #[serde(rename = "ChanlTp")]
    pub chanl_tp: Max4Text,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Max128Text>,
}
/// Builder for [`OtherContact1`]. Construct via [`OtherContact1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OtherContact1Builder {
    chanl_tp: ::std::option::Option<Max4Text>,
    id: ::std::option::Option<Max128Text>,
}
impl OtherContact1Builder {
    /// Set the `chanl_tp` field.
    #[must_use]
    pub fn chanl_tp(mut self, value: Max4Text) -> OtherContact1Builder {
        self.chanl_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max128Text) -> OtherContact1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<OtherContact1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.chanl_tp.is_none() {
            missing.push("chanl_tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OtherContact1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OtherContact1 {
            chanl_tp: self.chanl_tp.unwrap(),
            id: self.id,
        })
    }
}
impl OtherContact1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OtherContact1Builder {
        OtherContact1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Party38Choice {
    #[serde(rename = "OrgId")]
    OrgId(OrganisationIdentification29),
    #[serde(rename = "PrvtId")]
    PrvtId(PersonIdentification13),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Party40Choice {
    #[serde(rename = "Pty")]
    Pty(PartyIdentification135),
    #[serde(rename = "Agt")]
    Agt(BranchAndFinancialInstitutionIdentification6),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartyIdentification135 {
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress24>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::common::ChoiceWrapper<Party38Choice>>,
    #[serde(rename = "CtryOfRes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact4>,
}
/// Builder for [`PartyIdentification135`]. Construct via [`PartyIdentification135::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PartyIdentification135Builder {
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress24>,
    id: ::std::option::Option<crate::common::ChoiceWrapper<Party38Choice>>,
    ctry_of_res: ::std::option::Option<CountryCode>,
    ctct_dtls: ::std::option::Option<Contact4>,
}
impl PartyIdentification135Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> PartyIdentification135Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: PostalAddress24) -> PartyIdentification135Builder {
        self.pstl_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `id` field.
    #[must_use]
    pub fn id(
        mut self,
        value: crate::common::ChoiceWrapper<Party38Choice>,
    ) -> PartyIdentification135Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry_of_res` field.
    #[must_use]
    pub fn ctry_of_res(mut self, value: CountryCode) -> PartyIdentification135Builder {
        self.ctry_of_res = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctct_dtls` field.
    #[must_use]
    pub fn ctct_dtls(mut self, value: Contact4) -> PartyIdentification135Builder {
        self.ctct_dtls = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<PartyIdentification135, crate::common::BuilderError> {
        ::std::result::Result::Ok(PartyIdentification135 {
            nm: self.nm,
            pstl_adr: self.pstl_adr,
            id: self.id,
            ctry_of_res: self.ctry_of_res,
            ctct_dtls: self.ctct_dtls,
        })
    }
}
impl PartyIdentification135 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PartyIdentification135Builder {
        PartyIdentification135Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentReturnReason6 {
    #[serde(rename = "Orgtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<crate::common::ChoiceWrapper<ReturnReason5Choice>>,
    #[serde(rename = "AddtlInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addtl_inf: Vec<Max105Text>,
}
/// Builder for [`PaymentReturnReason6`]. Construct via [`PaymentReturnReason6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentReturnReason6Builder {
    orgtr: ::std::option::Option<PartyIdentification135>,
    rsn: ::std::option::Option<crate::common::ChoiceWrapper<ReturnReason5Choice>>,
    addtl_inf: ::std::vec::Vec<Max105Text>,
}
impl PaymentReturnReason6Builder {
    /// Set the `orgtr` field.
    #[must_use]
    pub fn orgtr(mut self, value: PartyIdentification135) -> PaymentReturnReason6Builder {
        self.orgtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(
        mut self,
        value: crate::common::ChoiceWrapper<ReturnReason5Choice>,
    ) -> PaymentReturnReason6Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field (replaces any previously added items).
    #[must_use]
    pub fn addtl_inf(mut self, value: ::std::vec::Vec<Max105Text>) -> PaymentReturnReason6Builder {
        self.addtl_inf = value;
        self
    }
    /// Append one item to the `addtl_inf` field.
    #[must_use]
    pub fn add_addtl_inf(mut self, value: Max105Text) -> PaymentReturnReason6Builder {
        self.addtl_inf.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<PaymentReturnReason6, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentReturnReason6 {
            orgtr: self.orgtr,
            rsn: self.rsn,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl PaymentReturnReason6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentReturnReason6Builder {
        PaymentReturnReason6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentReturnV11 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader99,
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupHeader18>,
    #[serde(rename = "TxInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tx_inf: Vec<PaymentTransaction133>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`PaymentReturnV11`]. Construct via [`PaymentReturnV11::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentReturnV11Builder {
    grp_hdr: ::std::option::Option<GroupHeader99>,
    orgnl_grp_inf: ::std::option::Option<OriginalGroupHeader18>,
    tx_inf: ::std::vec::Vec<PaymentTransaction133>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl PaymentReturnV11Builder {
    /// Set the `grp_hdr` field.
    #[must_use]
    pub fn grp_hdr(mut self, value: GroupHeader99) -> PaymentReturnV11Builder {
        self.grp_hdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_grp_inf` field.
    #[must_use]
    pub fn orgnl_grp_inf(mut self, value: OriginalGroupHeader18) -> PaymentReturnV11Builder {
        self.orgnl_grp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_inf` field (replaces any previously added items).
    #[must_use]
    pub fn tx_inf(
        mut self,
        value: ::std::vec::Vec<PaymentTransaction133>,
    ) -> PaymentReturnV11Builder {
        self.tx_inf = value;
        self
    }
    /// Append one item to the `tx_inf` field.
    #[must_use]
    pub fn add_tx_inf(mut self, value: PaymentTransaction133) -> PaymentReturnV11Builder {
        self.tx_inf.push(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> PaymentReturnV11Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(mut self, value: SupplementaryData1) -> PaymentReturnV11Builder {
        self.splmtry_data.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<PaymentReturnV11, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.grp_hdr.is_none() {
            missing.push("grp_hdr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentReturnV11".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentReturnV11 {
            grp_hdr: self.grp_hdr.unwrap(),
            orgnl_grp_inf: self.orgnl_grp_inf,
            tx_inf: self.tx_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl PaymentReturnV11 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentReturnV11Builder {
        PaymentReturnV11Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentTransaction133 {
    #[serde(rename = "RtrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtr_id: Option<Max35Text>,
    #[serde(rename = "OrgnlGrpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_grp_inf: Option<OriginalGroupInformation29>,
    #[serde(rename = "OrgnlInstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_instr_id: Option<Max35Text>,
    #[serde(rename = "OrgnlEndToEndId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_end_to_end_id: Option<Max35Text>,
    #[serde(rename = "OrgnlTxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_id: Option<Max35Text>,
    #[serde(rename = "OrgnlUETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_uetr: Option<UUIDv4Identifier>,
    #[serde(rename = "OrgnlClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_clr_sys_ref: Option<Max35Text>,
    #[serde(rename = "OrgnlIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "OrgnlIntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,
    #[serde(rename = "RtrdIntrBkSttlmAmt")]
    pub rtrd_intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "SttlmPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_prty: Option<Priority3Code>,
    #[serde(rename = "SttlmTmIndctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_indctn: Option<SettlementDateTimeIndication1>,
    #[serde(rename = "SttlmTmReq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_tm_req: Option<SettlementTimeRequest2>,
    #[serde(rename = "RtrdInstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtrd_instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "CompstnAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compstn_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "ChrgBr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code>,
    #[serde(rename = "ChrgsInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chrgs_inf: Vec<Charges7>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<Max35Text>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RtrChain")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtr_chain: Option<TransactionParties10>,
    #[serde(rename = "RtrRsnInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rtr_rsn_inf: Vec<PaymentReturnReason6>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference36>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`PaymentTransaction133`]. Construct via [`PaymentTransaction133::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentTransaction133Builder {
    rtr_id: ::std::option::Option<Max35Text>,
    orgnl_grp_inf: ::std::option::Option<OriginalGroupInformation29>,
    orgnl_instr_id: ::std::option::Option<Max35Text>,
    orgnl_end_to_end_id: ::std::option::Option<Max35Text>,
    orgnl_tx_id: ::std::option::Option<Max35Text>,
    orgnl_uetr: ::std::option::Option<UUIDv4Identifier>,
    orgnl_clr_sys_ref: ::std::option::Option<Max35Text>,
    orgnl_intr_bk_sttlm_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    orgnl_intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation28>,
    rtrd_intr_bk_sttlm_amt: ::std::option::Option<ActiveCurrencyAndAmount>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    sttlm_prty: ::std::option::Option<Priority3Code>,
    sttlm_tm_indctn: ::std::option::Option<SettlementDateTimeIndication1>,
    sttlm_tm_req: ::std::option::Option<SettlementTimeRequest2>,
    rtrd_instd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    xchg_rate: ::std::option::Option<BaseOneRate>,
    compstn_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    chrg_br: ::std::option::Option<ChargeBearerType1Code>,
    chrgs_inf: ::std::vec::Vec<Charges7>,
    clr_sys_ref: ::std::option::Option<Max35Text>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    rtr_chain: ::std::option::Option<TransactionParties10>,
    rtr_rsn_inf: ::std::vec::Vec<PaymentReturnReason6>,
    orgnl_tx_ref: ::std::option::Option<OriginalTransactionReference36>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl PaymentTransaction133Builder {
    /// Set the `rtr_id` field.
    #[must_use]
    pub fn rtr_id(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.rtr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_grp_inf` field.
    #[must_use]
    pub fn orgnl_grp_inf(
        mut self,
        value: OriginalGroupInformation29,
    ) -> PaymentTransaction133Builder {
        self.orgnl_grp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_instr_id` field.
    #[must_use]
    pub fn orgnl_instr_id(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.orgnl_instr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_end_to_end_id` field.
    #[must_use]
    pub fn orgnl_end_to_end_id(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.orgnl_end_to_end_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_tx_id` field.
    #[must_use]
    pub fn orgnl_tx_id(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.orgnl_tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_uetr` field.
    #[must_use]
    pub fn orgnl_uetr(mut self, value: UUIDv4Identifier) -> PaymentTransaction133Builder {
        self.orgnl_uetr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_clr_sys_ref` field.
    #[must_use]
    pub fn orgnl_clr_sys_ref(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.orgnl_clr_sys_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn orgnl_intr_bk_sttlm_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> PaymentTransaction133Builder {
        self.orgnl_intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn orgnl_intr_bk_sttlm_dt(mut self, value: ISODate) -> PaymentTransaction133Builder {
        self.orgnl_intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(mut self, value: PaymentTypeInformation28) -> PaymentTransaction133Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtrd_intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn rtrd_intr_bk_sttlm_amt(
        mut self,
        value: ActiveCurrencyAndAmount,
    ) -> PaymentTransaction133Builder {
        self.rtrd_intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> PaymentTransaction133Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_prty` field.
    #[must_use]
    pub fn sttlm_prty(mut self, value: Priority3Code) -> PaymentTransaction133Builder {
        self.sttlm_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_tm_indctn` field.
    #[must_use]
    pub fn sttlm_tm_indctn(
        mut self,
        value: SettlementDateTimeIndication1,
    ) -> PaymentTransaction133Builder {
        self.sttlm_tm_indctn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_tm_req` field.
    #[must_use]
    pub fn sttlm_tm_req(mut self, value: SettlementTimeRequest2) -> PaymentTransaction133Builder {
        self.sttlm_tm_req = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtrd_instd_amt` field.
    #[must_use]
    pub fn rtrd_instd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> PaymentTransaction133Builder {
        self.rtrd_instd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate` field.
    #[must_use]
    pub fn xchg_rate(mut self, value: BaseOneRate) -> PaymentTransaction133Builder {
        self.xchg_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `compstn_amt` field.
    #[must_use]
    pub fn compstn_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> PaymentTransaction133Builder {
        self.compstn_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrg_br` field.
    #[must_use]
    pub fn chrg_br(mut self, value: ChargeBearerType1Code) -> PaymentTransaction133Builder {
        self.chrg_br = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs_inf` field (replaces any previously added items).
    #[must_use]
    pub fn chrgs_inf(mut self, value: ::std::vec::Vec<Charges7>) -> PaymentTransaction133Builder {
        self.chrgs_inf = value;
        self
    }
    /// Append one item to the `chrgs_inf` field.
    #[must_use]
    pub fn add_chrgs_inf(mut self, value: Charges7) -> PaymentTransaction133Builder {
        self.chrgs_inf.push(value);
        self
    }
    /// Set the `clr_sys_ref` field.
    #[must_use]
    pub fn clr_sys_ref(mut self, value: Max35Text) -> PaymentTransaction133Builder {
        self.clr_sys_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentTransaction133Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentTransaction133Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtr_chain` field.
    #[must_use]
    pub fn rtr_chain(mut self, value: TransactionParties10) -> PaymentTransaction133Builder {
        self.rtr_chain = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtr_rsn_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rtr_rsn_inf(
        mut self,
        value: ::std::vec::Vec<PaymentReturnReason6>,
    ) -> PaymentTransaction133Builder {
        self.rtr_rsn_inf = value;
        self
    }
    /// Append one item to the `rtr_rsn_inf` field.
    #[must_use]
    pub fn add_rtr_rsn_inf(mut self, value: PaymentReturnReason6) -> PaymentTransaction133Builder {
        self.rtr_rsn_inf.push(value);
        self
    }
    /// Set the `orgnl_tx_ref` field.
    #[must_use]
    pub fn orgnl_tx_ref(
        mut self,
        value: OriginalTransactionReference36,
    ) -> PaymentTransaction133Builder {
        self.orgnl_tx_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> PaymentTransaction133Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(mut self, value: SupplementaryData1) -> PaymentTransaction133Builder {
        self.splmtry_data.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<PaymentTransaction133, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.rtrd_intr_bk_sttlm_amt.is_none() {
            missing.push("rtrd_intr_bk_sttlm_amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentTransaction133".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentTransaction133 {
            rtr_id: self.rtr_id,
            orgnl_grp_inf: self.orgnl_grp_inf,
            orgnl_instr_id: self.orgnl_instr_id,
            orgnl_end_to_end_id: self.orgnl_end_to_end_id,
            orgnl_tx_id: self.orgnl_tx_id,
            orgnl_uetr: self.orgnl_uetr,
            orgnl_clr_sys_ref: self.orgnl_clr_sys_ref,
            orgnl_intr_bk_sttlm_amt: self.orgnl_intr_bk_sttlm_amt,
            orgnl_intr_bk_sttlm_dt: self.orgnl_intr_bk_sttlm_dt,
            pmt_tp_inf: self.pmt_tp_inf,
            rtrd_intr_bk_sttlm_amt: self.rtrd_intr_bk_sttlm_amt.unwrap(),
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            sttlm_prty: self.sttlm_prty,
            sttlm_tm_indctn: self.sttlm_tm_indctn,
            sttlm_tm_req: self.sttlm_tm_req,
            rtrd_instd_amt: self.rtrd_instd_amt,
            xchg_rate: self.xchg_rate,
            compstn_amt: self.compstn_amt,
            chrg_br: self.chrg_br,
            chrgs_inf: self.chrgs_inf,
            clr_sys_ref: self.clr_sys_ref,
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
            rtr_chain: self.rtr_chain,
            rtr_rsn_inf: self.rtr_rsn_inf,
            orgnl_tx_ref: self.orgnl_tx_ref,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl PaymentTransaction133 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentTransaction133Builder {
        PaymentTransaction133Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentTypeInformation27 {
    #[serde(rename = "InstrPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ClrChanl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "SvcLvl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub svc_lvl: Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    #[serde(rename = "SeqTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_tp: Option<SequenceType3Code>,
    #[serde(rename = "CtgyPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
}
/// Builder for [`PaymentTypeInformation27`]. Construct via [`PaymentTypeInformation27::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentTypeInformation27Builder {
    instr_prty: ::std::option::Option<Priority2Code>,
    clr_chanl: ::std::option::Option<ClearingChannel2Code>,
    svc_lvl: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    lcl_instrm: ::std::option::Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    seq_tp: ::std::option::Option<SequenceType3Code>,
    ctgy_purp: ::std::option::Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
}
impl PaymentTypeInformation27Builder {
    /// Set the `instr_prty` field.
    #[must_use]
    pub fn instr_prty(mut self, value: Priority2Code) -> PaymentTypeInformation27Builder {
        self.instr_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_chanl` field.
    #[must_use]
    pub fn clr_chanl(mut self, value: ClearingChannel2Code) -> PaymentTypeInformation27Builder {
        self.clr_chanl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `svc_lvl` field (replaces any previously added items).
    #[must_use]
    pub fn svc_lvl(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    ) -> PaymentTypeInformation27Builder {
        self.svc_lvl = value;
        self
    }
    /// Append one item to the `svc_lvl` field.
    #[must_use]
    pub fn add_svc_lvl(
        mut self,
        value: crate::common::ChoiceWrapper<ServiceLevel8Choice>,
    ) -> PaymentTypeInformation27Builder {
        self.svc_lvl.push(value);
        self
    }
    /// Set the `lcl_instrm` field.
    #[must_use]
    pub fn lcl_instrm(
        mut self,
        value: crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    ) -> PaymentTypeInformation27Builder {
        self.lcl_instrm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `seq_tp` field.
    #[must_use]
    pub fn seq_tp(mut self, value: SequenceType3Code) -> PaymentTypeInformation27Builder {
        self.seq_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy_purp` field.
    #[must_use]
    pub fn ctgy_purp(
        mut self,
        value: crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    ) -> PaymentTypeInformation27Builder {
        self.ctgy_purp = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<PaymentTypeInformation27, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentTypeInformation27 {
            instr_prty: self.instr_prty,
            clr_chanl: self.clr_chanl,
            svc_lvl: self.svc_lvl,
            lcl_instrm: self.lcl_instrm,
            seq_tp: self.seq_tp,
            ctgy_purp: self.ctgy_purp,
        })
    }
}
impl PaymentTypeInformation27 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentTypeInformation27Builder {
        PaymentTypeInformation27Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentTypeInformation28 {
    #[serde(rename = "InstrPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ClrChanl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_chanl: Option<ClearingChannel2Code>,
    #[serde(rename = "SvcLvl")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub svc_lvl: Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    #[serde(rename = "CtgyPurp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_purp: Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
}
/// Builder for [`PaymentTypeInformation28`]. Construct via [`PaymentTypeInformation28::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentTypeInformation28Builder {
    instr_prty: ::std::option::Option<Priority2Code>,
    clr_chanl: ::std::option::Option<ClearingChannel2Code>,
    svc_lvl: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    lcl_instrm: ::std::option::Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    ctgy_purp: ::std::option::Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
}
impl PaymentTypeInformation28Builder {
    /// Set the `instr_prty` field.
    #[must_use]
    pub fn instr_prty(mut self, value: Priority2Code) -> PaymentTypeInformation28Builder {
        self.instr_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_chanl` field.
    #[must_use]
    pub fn clr_chanl(mut self, value: ClearingChannel2Code) -> PaymentTypeInformation28Builder {
        self.clr_chanl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `svc_lvl` field (replaces any previously added items).
    #[must_use]
    pub fn svc_lvl(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    ) -> PaymentTypeInformation28Builder {
        self.svc_lvl = value;
        self
    }
    /// Append one item to the `svc_lvl` field.
    #[must_use]
    pub fn add_svc_lvl(
        mut self,
        value: crate::common::ChoiceWrapper<ServiceLevel8Choice>,
    ) -> PaymentTypeInformation28Builder {
        self.svc_lvl.push(value);
        self
    }
    /// Set the `lcl_instrm` field.
    #[must_use]
    pub fn lcl_instrm(
        mut self,
        value: crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    ) -> PaymentTypeInformation28Builder {
        self.lcl_instrm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy_purp` field.
    #[must_use]
    pub fn ctgy_purp(
        mut self,
        value: crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    ) -> PaymentTypeInformation28Builder {
        self.ctgy_purp = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<PaymentTypeInformation28, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentTypeInformation28 {
            instr_prty: self.instr_prty,
            clr_chanl: self.clr_chanl,
            svc_lvl: self.svc_lvl,
            lcl_instrm: self.lcl_instrm,
            ctgy_purp: self.ctgy_purp,
        })
    }
}
impl PaymentTypeInformation28 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentTypeInformation28Builder {
        PaymentTypeInformation28Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PersonIdentification13 {
    #[serde(rename = "DtAndPlcOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr: Vec<GenericPersonIdentification1>,
}
/// Builder for [`PersonIdentification13`]. Construct via [`PersonIdentification13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PersonIdentification13Builder {
    dt_and_plc_of_birth: ::std::option::Option<DateAndPlaceOfBirth1>,
    othr: ::std::vec::Vec<GenericPersonIdentification1>,
}
impl PersonIdentification13Builder {
    /// Set the `dt_and_plc_of_birth` field.
    #[must_use]
    pub fn dt_and_plc_of_birth(
        mut self,
        value: DateAndPlaceOfBirth1,
    ) -> PersonIdentification13Builder {
        self.dt_and_plc_of_birth = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(
        mut self,
        value: ::std::vec::Vec<GenericPersonIdentification1>,
    ) -> PersonIdentification13Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(
        mut self,
        value: GenericPersonIdentification1,
    ) -> PersonIdentification13Builder {
        self.othr.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<PersonIdentification13, crate::common::BuilderError> {
        ::std::result::Result::Ok(PersonIdentification13 {
            dt_and_plc_of_birth: self.dt_and_plc_of_birth,
            othr: self.othr,
        })
    }
}
impl PersonIdentification13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PersonIdentification13Builder {
        PersonIdentification13Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PersonIdentificationSchemeName1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalPersonIdentification1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PostalAddress24 {
    #[serde(rename = "AdrTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<crate::common::ChoiceWrapper<AddressType3Choice>>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
    #[serde(rename = "StrtNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max70Text>,
    #[serde(rename = "BldgNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "BldgNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<Max35Text>,
    #[serde(rename = "Flr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flr: Option<Max70Text>,
    #[serde(rename = "PstBx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pst_bx: Option<Max16Text>,
    #[serde(rename = "Room")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room: Option<Max70Text>,
    #[serde(rename = "PstCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pst_cd: Option<Max16Text>,
    #[serde(rename = "TwnNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_nm: Option<Max35Text>,
    #[serde(rename = "TwnLctnNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<Max35Text>,
    #[serde(rename = "DstrctNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<Max35Text>,
    #[serde(rename = "CtrySubDvsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_sub_dvsn: Option<Max35Text>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "AdrLine")]
    /// Maximum 7 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adr_line: Vec<Max70Text>,
}
/// Builder for [`PostalAddress24`]. Construct via [`PostalAddress24::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PostalAddress24Builder {
    adr_tp: ::std::option::Option<crate::common::ChoiceWrapper<AddressType3Choice>>,
    dept: ::std::option::Option<Max70Text>,
    sub_dept: ::std::option::Option<Max70Text>,
    strt_nm: ::std::option::Option<Max70Text>,
    bldg_nb: ::std::option::Option<Max16Text>,
    bldg_nm: ::std::option::Option<Max35Text>,
    flr: ::std::option::Option<Max70Text>,
    pst_bx: ::std::option::Option<Max16Text>,
    room: ::std::option::Option<Max70Text>,
    pst_cd: ::std::option::Option<Max16Text>,
    twn_nm: ::std::option::Option<Max35Text>,
    twn_lctn_nm: ::std::option::Option<Max35Text>,
    dstrct_nm: ::std::option::Option<Max35Text>,
    ctry_sub_dvsn: ::std::option::Option<Max35Text>,
    ctry: ::std::option::Option<CountryCode>,
    adr_line: ::std::vec::Vec<Max70Text>,
}
impl PostalAddress24Builder {
    /// Set the `adr_tp` field.
    #[must_use]
    pub fn adr_tp(
        mut self,
        value: crate::common::ChoiceWrapper<AddressType3Choice>,
    ) -> PostalAddress24Builder {
        self.adr_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dept` field.
    #[must_use]
    pub fn dept(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sub_dept` field.
    #[must_use]
    pub fn sub_dept(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.sub_dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `strt_nm` field.
    #[must_use]
    pub fn strt_nm(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.strt_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bldg_nb` field.
    #[must_use]
    pub fn bldg_nb(mut self, value: Max16Text) -> PostalAddress24Builder {
        self.bldg_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bldg_nm` field.
    #[must_use]
    pub fn bldg_nm(mut self, value: Max35Text) -> PostalAddress24Builder {
        self.bldg_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `flr` field.
    #[must_use]
    pub fn flr(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.flr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pst_bx` field.
    #[must_use]
    pub fn pst_bx(mut self, value: Max16Text) -> PostalAddress24Builder {
        self.pst_bx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `room` field.
    #[must_use]
    pub fn room(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.room = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pst_cd` field.
    #[must_use]
    pub fn pst_cd(mut self, value: Max16Text) -> PostalAddress24Builder {
        self.pst_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `twn_nm` field.
    #[must_use]
    pub fn twn_nm(mut self, value: Max35Text) -> PostalAddress24Builder {
        self.twn_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `twn_lctn_nm` field.
    #[must_use]
    pub fn twn_lctn_nm(mut self, value: Max35Text) -> PostalAddress24Builder {
        self.twn_lctn_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dstrct_nm` field.
    #[must_use]
    pub fn dstrct_nm(mut self, value: Max35Text) -> PostalAddress24Builder {
        self.dstrct_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry_sub_dvsn` field.
    #[must_use]
    pub fn ctry_sub_dvsn(mut self, value: Max35Text) -> PostalAddress24Builder {
        self.ctry_sub_dvsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry` field.
    #[must_use]
    pub fn ctry(mut self, value: CountryCode) -> PostalAddress24Builder {
        self.ctry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `adr_line` field (replaces any previously added items).
    #[must_use]
    pub fn adr_line(mut self, value: ::std::vec::Vec<Max70Text>) -> PostalAddress24Builder {
        self.adr_line = value;
        self
    }
    /// Append one item to the `adr_line` field.
    #[must_use]
    pub fn add_adr_line(mut self, value: Max70Text) -> PostalAddress24Builder {
        self.adr_line.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<PostalAddress24, crate::common::BuilderError> {
        ::std::result::Result::Ok(PostalAddress24 {
            adr_tp: self.adr_tp,
            dept: self.dept,
            sub_dept: self.sub_dept,
            strt_nm: self.strt_nm,
            bldg_nb: self.bldg_nb,
            bldg_nm: self.bldg_nm,
            flr: self.flr,
            pst_bx: self.pst_bx,
            room: self.room,
            pst_cd: self.pst_cd,
            twn_nm: self.twn_nm,
            twn_lctn_nm: self.twn_lctn_nm,
            dstrct_nm: self.dstrct_nm,
            ctry_sub_dvsn: self.ctry_sub_dvsn,
            ctry: self.ctry,
            adr_line: self.adr_line,
        })
    }
}
impl PostalAddress24 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PostalAddress24Builder {
        PostalAddress24Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProxyAccountIdentification1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<ProxyAccountType1Choice>>,
    #[serde(rename = "Id")]
    pub id: Max2048Text,
}
/// Builder for [`ProxyAccountIdentification1`]. Construct via [`ProxyAccountIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProxyAccountIdentification1Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<ProxyAccountType1Choice>>,
    id: ::std::option::Option<Max2048Text>,
}
impl ProxyAccountIdentification1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<ProxyAccountType1Choice>,
    ) -> ProxyAccountIdentification1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max2048Text) -> ProxyAccountIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<ProxyAccountIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProxyAccountIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProxyAccountIdentification1 {
            tp: self.tp,
            id: self.id.unwrap(),
        })
    }
}
impl ProxyAccountIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProxyAccountIdentification1Builder {
        ProxyAccountIdentification1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ProxyAccountType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalProxyAccountType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Purpose2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalPurpose1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferredDocumentInformation7 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<ReferredDocumentType4>,
    #[serde(rename = "Nb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
    #[serde(rename = "RltdDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<ISODate>,
    #[serde(rename = "LineDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub line_dtls: Vec<DocumentLineInformation1>,
}
/// Builder for [`ReferredDocumentInformation7`]. Construct via [`ReferredDocumentInformation7::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ReferredDocumentInformation7Builder {
    tp: ::std::option::Option<ReferredDocumentType4>,
    nb: ::std::option::Option<Max35Text>,
    rltd_dt: ::std::option::Option<ISODate>,
    line_dtls: ::std::vec::Vec<DocumentLineInformation1>,
}
impl ReferredDocumentInformation7Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: ReferredDocumentType4) -> ReferredDocumentInformation7Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb` field.
    #[must_use]
    pub fn nb(mut self, value: Max35Text) -> ReferredDocumentInformation7Builder {
        self.nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_dt` field.
    #[must_use]
    pub fn rltd_dt(mut self, value: ISODate) -> ReferredDocumentInformation7Builder {
        self.rltd_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `line_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn line_dtls(
        mut self,
        value: ::std::vec::Vec<DocumentLineInformation1>,
    ) -> ReferredDocumentInformation7Builder {
        self.line_dtls = value;
        self
    }
    /// Append one item to the `line_dtls` field.
    #[must_use]
    pub fn add_line_dtls(
        mut self,
        value: DocumentLineInformation1,
    ) -> ReferredDocumentInformation7Builder {
        self.line_dtls.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<ReferredDocumentInformation7, crate::common::BuilderError> {
        ::std::result::Result::Ok(ReferredDocumentInformation7 {
            tp: self.tp,
            nb: self.nb,
            rltd_dt: self.rltd_dt,
            line_dtls: self.line_dtls,
        })
    }
}
impl ReferredDocumentInformation7 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ReferredDocumentInformation7Builder {
        ReferredDocumentInformation7Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReferredDocumentType3Choice {
    #[serde(rename = "Cd")]
    Cd(DocumentType6Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReferredDocumentType4 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<ReferredDocumentType3Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`ReferredDocumentType4`]. Construct via [`ReferredDocumentType4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ReferredDocumentType4Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<ReferredDocumentType3Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl ReferredDocumentType4Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<ReferredDocumentType3Choice>,
    ) -> ReferredDocumentType4Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> ReferredDocumentType4Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<ReferredDocumentType4, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ReferredDocumentType4".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ReferredDocumentType4 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl ReferredDocumentType4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ReferredDocumentType4Builder {
        ReferredDocumentType4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceAmount2 {
    #[serde(rename = "DuePyblAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dscnt_apld_amt: Vec<DiscountAmountAndType1>,
    #[serde(rename = "CdtNoteAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax_amt: Vec<TaxAmountAndType1>,
    #[serde(rename = "AdjstmntAmtAndRsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
    #[serde(rename = "RmtdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
/// Builder for [`RemittanceAmount2`]. Construct via [`RemittanceAmount2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceAmount2Builder {
    due_pybl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    dscnt_apld_amt: ::std::vec::Vec<DiscountAmountAndType1>,
    cdt_note_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    tax_amt: ::std::vec::Vec<TaxAmountAndType1>,
    adjstmnt_amt_and_rsn: ::std::vec::Vec<DocumentAdjustment1>,
    rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl RemittanceAmount2Builder {
    /// Set the `due_pybl_amt` field.
    #[must_use]
    pub fn due_pybl_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount2Builder {
        self.due_pybl_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dscnt_apld_amt` field (replaces any previously added items).
    #[must_use]
    pub fn dscnt_apld_amt(
        mut self,
        value: ::std::vec::Vec<DiscountAmountAndType1>,
    ) -> RemittanceAmount2Builder {
        self.dscnt_apld_amt = value;
        self
    }
    /// Append one item to the `dscnt_apld_amt` field.
    #[must_use]
    pub fn add_dscnt_apld_amt(mut self, value: DiscountAmountAndType1) -> RemittanceAmount2Builder {
        self.dscnt_apld_amt.push(value);
        self
    }
    /// Set the `cdt_note_amt` field.
    #[must_use]
    pub fn cdt_note_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount2Builder {
        self.cdt_note_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_amt` field (replaces any previously added items).
    #[must_use]
    pub fn tax_amt(
        mut self,
        value: ::std::vec::Vec<TaxAmountAndType1>,
    ) -> RemittanceAmount2Builder {
        self.tax_amt = value;
        self
    }
    /// Append one item to the `tax_amt` field.
    #[must_use]
    pub fn add_tax_amt(mut self, value: TaxAmountAndType1) -> RemittanceAmount2Builder {
        self.tax_amt.push(value);
        self
    }
    /// Set the `adjstmnt_amt_and_rsn` field (replaces any previously added items).
    #[must_use]
    pub fn adjstmnt_amt_and_rsn(
        mut self,
        value: ::std::vec::Vec<DocumentAdjustment1>,
    ) -> RemittanceAmount2Builder {
        self.adjstmnt_amt_and_rsn = value;
        self
    }
    /// Append one item to the `adjstmnt_amt_and_rsn` field.
    #[must_use]
    pub fn add_adjstmnt_amt_and_rsn(
        mut self,
        value: DocumentAdjustment1,
    ) -> RemittanceAmount2Builder {
        self.adjstmnt_amt_and_rsn.push(value);
        self
    }
    /// Set the `rmtd_amt` field.
    #[must_use]
    pub fn rmtd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount2Builder {
        self.rmtd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<RemittanceAmount2, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceAmount2 {
            due_pybl_amt: self.due_pybl_amt,
            dscnt_apld_amt: self.dscnt_apld_amt,
            cdt_note_amt: self.cdt_note_amt,
            tax_amt: self.tax_amt,
            adjstmnt_amt_and_rsn: self.adjstmnt_amt_and_rsn,
            rmtd_amt: self.rmtd_amt,
        })
    }
}
impl RemittanceAmount2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceAmount2Builder {
        RemittanceAmount2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceAmount3 {
    #[serde(rename = "DuePyblAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_pybl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "DscntApldAmt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dscnt_apld_amt: Vec<DiscountAmountAndType1>,
    #[serde(rename = "CdtNoteAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_note_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TaxAmt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tax_amt: Vec<TaxAmountAndType1>,
    #[serde(rename = "AdjstmntAmtAndRsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
    #[serde(rename = "RmtdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmtd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
/// Builder for [`RemittanceAmount3`]. Construct via [`RemittanceAmount3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceAmount3Builder {
    due_pybl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    dscnt_apld_amt: ::std::vec::Vec<DiscountAmountAndType1>,
    cdt_note_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    tax_amt: ::std::vec::Vec<TaxAmountAndType1>,
    adjstmnt_amt_and_rsn: ::std::vec::Vec<DocumentAdjustment1>,
    rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl RemittanceAmount3Builder {
    /// Set the `due_pybl_amt` field.
    #[must_use]
    pub fn due_pybl_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount3Builder {
        self.due_pybl_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dscnt_apld_amt` field (replaces any previously added items).
    #[must_use]
    pub fn dscnt_apld_amt(
        mut self,
        value: ::std::vec::Vec<DiscountAmountAndType1>,
    ) -> RemittanceAmount3Builder {
        self.dscnt_apld_amt = value;
        self
    }
    /// Append one item to the `dscnt_apld_amt` field.
    #[must_use]
    pub fn add_dscnt_apld_amt(mut self, value: DiscountAmountAndType1) -> RemittanceAmount3Builder {
        self.dscnt_apld_amt.push(value);
        self
    }
    /// Set the `cdt_note_amt` field.
    #[must_use]
    pub fn cdt_note_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount3Builder {
        self.cdt_note_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_amt` field (replaces any previously added items).
    #[must_use]
    pub fn tax_amt(
        mut self,
        value: ::std::vec::Vec<TaxAmountAndType1>,
    ) -> RemittanceAmount3Builder {
        self.tax_amt = value;
        self
    }
    /// Append one item to the `tax_amt` field.
    #[must_use]
    pub fn add_tax_amt(mut self, value: TaxAmountAndType1) -> RemittanceAmount3Builder {
        self.tax_amt.push(value);
        self
    }
    /// Set the `adjstmnt_amt_and_rsn` field (replaces any previously added items).
    #[must_use]
    pub fn adjstmnt_amt_and_rsn(
        mut self,
        value: ::std::vec::Vec<DocumentAdjustment1>,
    ) -> RemittanceAmount3Builder {
        self.adjstmnt_amt_and_rsn = value;
        self
    }
    /// Append one item to the `adjstmnt_amt_and_rsn` field.
    #[must_use]
    pub fn add_adjstmnt_amt_and_rsn(
        mut self,
        value: DocumentAdjustment1,
    ) -> RemittanceAmount3Builder {
        self.adjstmnt_amt_and_rsn.push(value);
        self
    }
    /// Set the `rmtd_amt` field.
    #[must_use]
    pub fn rmtd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> RemittanceAmount3Builder {
        self.rmtd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<RemittanceAmount3, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceAmount3 {
            due_pybl_amt: self.due_pybl_amt,
            dscnt_apld_amt: self.dscnt_apld_amt,
            cdt_note_amt: self.cdt_note_amt,
            tax_amt: self.tax_amt,
            adjstmnt_amt_and_rsn: self.adjstmnt_amt_and_rsn,
            rmtd_amt: self.rmtd_amt,
        })
    }
}
impl RemittanceAmount3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceAmount3Builder {
        RemittanceAmount3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceInformation21 {
    #[serde(rename = "Ustrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ustrd: Vec<Max140Text>,
    #[serde(rename = "Strd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strd: Vec<StructuredRemittanceInformation17>,
}
/// Builder for [`RemittanceInformation21`]. Construct via [`RemittanceInformation21::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceInformation21Builder {
    ustrd: ::std::vec::Vec<Max140Text>,
    strd: ::std::vec::Vec<StructuredRemittanceInformation17>,
}
impl RemittanceInformation21Builder {
    /// Set the `ustrd` field (replaces any previously added items).
    #[must_use]
    pub fn ustrd(mut self, value: ::std::vec::Vec<Max140Text>) -> RemittanceInformation21Builder {
        self.ustrd = value;
        self
    }
    /// Append one item to the `ustrd` field.
    #[must_use]
    pub fn add_ustrd(mut self, value: Max140Text) -> RemittanceInformation21Builder {
        self.ustrd.push(value);
        self
    }
    /// Set the `strd` field (replaces any previously added items).
    #[must_use]
    pub fn strd(
        mut self,
        value: ::std::vec::Vec<StructuredRemittanceInformation17>,
    ) -> RemittanceInformation21Builder {
        self.strd = value;
        self
    }
    /// Append one item to the `strd` field.
    #[must_use]
    pub fn add_strd(
        mut self,
        value: StructuredRemittanceInformation17,
    ) -> RemittanceInformation21Builder {
        self.strd.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<RemittanceInformation21, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceInformation21 {
            ustrd: self.ustrd,
            strd: self.strd,
        })
    }
}
impl RemittanceInformation21 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceInformation21Builder {
        RemittanceInformation21Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReturnReason5Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalReturnReason1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceLevel8Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalServiceLevel1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementDateTimeIndication1 {
    #[serde(rename = "DbtDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_dt_tm: Option<ISODateTime>,
    #[serde(rename = "CdtDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dt_tm: Option<ISODateTime>,
}
/// Builder for [`SettlementDateTimeIndication1`]. Construct via [`SettlementDateTimeIndication1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SettlementDateTimeIndication1Builder {
    dbt_dt_tm: ::std::option::Option<ISODateTime>,
    cdt_dt_tm: ::std::option::Option<ISODateTime>,
}
impl SettlementDateTimeIndication1Builder {
    /// Set the `dbt_dt_tm` field.
    #[must_use]
    pub fn dbt_dt_tm(mut self, value: ISODateTime) -> SettlementDateTimeIndication1Builder {
        self.dbt_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dt_tm` field.
    #[must_use]
    pub fn cdt_dt_tm(mut self, value: ISODateTime) -> SettlementDateTimeIndication1Builder {
        self.cdt_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<SettlementDateTimeIndication1, crate::common::BuilderError> {
        ::std::result::Result::Ok(SettlementDateTimeIndication1 {
            dbt_dt_tm: self.dbt_dt_tm,
            cdt_dt_tm: self.cdt_dt_tm,
        })
    }
}
impl SettlementDateTimeIndication1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SettlementDateTimeIndication1Builder {
        SettlementDateTimeIndication1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementInstruction11 {
    #[serde(rename = "SttlmMtd")]
    pub sttlm_mtd: SettlementMethod1Code,
    #[serde(rename = "SttlmAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_acct: Option<CashAccount40>,
    #[serde(rename = "ClrSys")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys: Option<crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>>,
    #[serde(rename = "InstgRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
    #[serde(rename = "InstdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
    #[serde(rename = "ThrdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "ThrdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}
/// Builder for [`SettlementInstruction11`]. Construct via [`SettlementInstruction11::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SettlementInstruction11Builder {
    sttlm_mtd: ::std::option::Option<SettlementMethod1Code>,
    sttlm_acct: ::std::option::Option<CashAccount40>,
    clr_sys:
        ::std::option::Option<crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>>,
    instg_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instg_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    instd_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    thrd_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    thrd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
}
impl SettlementInstruction11Builder {
    /// Set the `sttlm_mtd` field.
    #[must_use]
    pub fn sttlm_mtd(mut self, value: SettlementMethod1Code) -> SettlementInstruction11Builder {
        self.sttlm_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_acct` field.
    #[must_use]
    pub fn sttlm_acct(mut self, value: CashAccount40) -> SettlementInstruction11Builder {
        self.sttlm_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys` field.
    #[must_use]
    pub fn clr_sys(
        mut self,
        value: crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>,
    ) -> SettlementInstruction11Builder {
        self.clr_sys = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_rmbrsmnt_agt` field.
    #[must_use]
    pub fn instg_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> SettlementInstruction11Builder {
        self.instg_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn instg_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction11Builder {
        self.instg_rmbrsmnt_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_rmbrsmnt_agt` field.
    #[must_use]
    pub fn instd_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> SettlementInstruction11Builder {
        self.instd_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn instd_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction11Builder {
        self.instd_rmbrsmnt_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `thrd_rmbrsmnt_agt` field.
    #[must_use]
    pub fn thrd_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> SettlementInstruction11Builder {
        self.thrd_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `thrd_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn thrd_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction11Builder {
        self.thrd_rmbrsmnt_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<SettlementInstruction11, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.sttlm_mtd.is_none() {
            missing.push("sttlm_mtd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "SettlementInstruction11".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(SettlementInstruction11 {
            sttlm_mtd: self.sttlm_mtd.unwrap(),
            sttlm_acct: self.sttlm_acct,
            clr_sys: self.clr_sys,
            instg_rmbrsmnt_agt: self.instg_rmbrsmnt_agt,
            instg_rmbrsmnt_agt_acct: self.instg_rmbrsmnt_agt_acct,
            instd_rmbrsmnt_agt: self.instd_rmbrsmnt_agt,
            instd_rmbrsmnt_agt_acct: self.instd_rmbrsmnt_agt_acct,
            thrd_rmbrsmnt_agt: self.thrd_rmbrsmnt_agt,
            thrd_rmbrsmnt_agt_acct: self.thrd_rmbrsmnt_agt_acct,
        })
    }
}
impl SettlementInstruction11 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SettlementInstruction11Builder {
        SettlementInstruction11Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementTimeRequest2 {
    #[serde(rename = "CLSTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cls_tm: Option<ISOTime>,
    #[serde(rename = "TillTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub till_tm: Option<ISOTime>,
    #[serde(rename = "FrTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_tm: Option<ISOTime>,
    #[serde(rename = "RjctTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rjct_tm: Option<ISOTime>,
}
/// Builder for [`SettlementTimeRequest2`]. Construct via [`SettlementTimeRequest2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SettlementTimeRequest2Builder {
    cls_tm: ::std::option::Option<ISOTime>,
    till_tm: ::std::option::Option<ISOTime>,
    fr_tm: ::std::option::Option<ISOTime>,
    rjct_tm: ::std::option::Option<ISOTime>,
}
impl SettlementTimeRequest2Builder {
    /// Set the `cls_tm` field.
    #[must_use]
    pub fn cls_tm(mut self, value: ISOTime) -> SettlementTimeRequest2Builder {
        self.cls_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `till_tm` field.
    #[must_use]
    pub fn till_tm(mut self, value: ISOTime) -> SettlementTimeRequest2Builder {
        self.till_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr_tm` field.
    #[must_use]
    pub fn fr_tm(mut self, value: ISOTime) -> SettlementTimeRequest2Builder {
        self.fr_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rjct_tm` field.
    #[must_use]
    pub fn rjct_tm(mut self, value: ISOTime) -> SettlementTimeRequest2Builder {
        self.rjct_tm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<SettlementTimeRequest2, crate::common::BuilderError> {
        ::std::result::Result::Ok(SettlementTimeRequest2 {
            cls_tm: self.cls_tm,
            till_tm: self.till_tm,
            fr_tm: self.fr_tm,
            rjct_tm: self.rjct_tm,
        })
    }
}
impl SettlementTimeRequest2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SettlementTimeRequest2Builder {
        SettlementTimeRequest2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StructuredRemittanceInformation17 {
    #[serde(rename = "RfrdDocInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rfrd_doc_inf: Vec<ReferredDocumentInformation7>,
    #[serde(rename = "RfrdDocAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount2>,
    #[serde(rename = "CdtrRefInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation2>,
    #[serde(rename = "Invcr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification135>,
    #[serde(rename = "Invcee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification135>,
    #[serde(rename = "TaxRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxData1>,
    #[serde(rename = "GrnshmtRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<Garnishment3>,
    #[serde(rename = "AddtlRmtInf")]
    /// Maximum 3 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addtl_rmt_inf: Vec<Max140Text>,
}
/// Builder for [`StructuredRemittanceInformation17`]. Construct via [`StructuredRemittanceInformation17::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct StructuredRemittanceInformation17Builder {
    rfrd_doc_inf: ::std::vec::Vec<ReferredDocumentInformation7>,
    rfrd_doc_amt: ::std::option::Option<RemittanceAmount2>,
    cdtr_ref_inf: ::std::option::Option<CreditorReferenceInformation2>,
    invcr: ::std::option::Option<PartyIdentification135>,
    invcee: ::std::option::Option<PartyIdentification135>,
    tax_rmt: ::std::option::Option<TaxData1>,
    grnshmt_rmt: ::std::option::Option<Garnishment3>,
    addtl_rmt_inf: ::std::vec::Vec<Max140Text>,
}
impl StructuredRemittanceInformation17Builder {
    /// Set the `rfrd_doc_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rfrd_doc_inf(
        mut self,
        value: ::std::vec::Vec<ReferredDocumentInformation7>,
    ) -> StructuredRemittanceInformation17Builder {
        self.rfrd_doc_inf = value;
        self
    }
    /// Append one item to the `rfrd_doc_inf` field.
    #[must_use]
    pub fn add_rfrd_doc_inf(
        mut self,
        value: ReferredDocumentInformation7,
    ) -> StructuredRemittanceInformation17Builder {
        self.rfrd_doc_inf.push(value);
        self
    }
    /// Set the `rfrd_doc_amt` field.
    #[must_use]
    pub fn rfrd_doc_amt(
        mut self,
        value: RemittanceAmount2,
    ) -> StructuredRemittanceInformation17Builder {
        self.rfrd_doc_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_ref_inf` field.
    #[must_use]
    pub fn cdtr_ref_inf(
        mut self,
        value: CreditorReferenceInformation2,
    ) -> StructuredRemittanceInformation17Builder {
        self.cdtr_ref_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `invcr` field.
    #[must_use]
    pub fn invcr(
        mut self,
        value: PartyIdentification135,
    ) -> StructuredRemittanceInformation17Builder {
        self.invcr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `invcee` field.
    #[must_use]
    pub fn invcee(
        mut self,
        value: PartyIdentification135,
    ) -> StructuredRemittanceInformation17Builder {
        self.invcee = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_rmt` field.
    #[must_use]
    pub fn tax_rmt(mut self, value: TaxData1) -> StructuredRemittanceInformation17Builder {
        self.tax_rmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshmt_rmt` field.
    #[must_use]
    pub fn grnshmt_rmt(mut self, value: Garnishment3) -> StructuredRemittanceInformation17Builder {
        self.grnshmt_rmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_rmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn addtl_rmt_inf(
        mut self,
        value: ::std::vec::Vec<Max140Text>,
    ) -> StructuredRemittanceInformation17Builder {
        self.addtl_rmt_inf = value;
        self
    }
    /// Append one item to the `addtl_rmt_inf` field.
    #[must_use]
    pub fn add_addtl_rmt_inf(
        mut self,
        value: Max140Text,
    ) -> StructuredRemittanceInformation17Builder {
        self.addtl_rmt_inf.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(
        self,
    ) -> ::std::result::Result<StructuredRemittanceInformation17, crate::common::BuilderError> {
        ::std::result::Result::Ok(StructuredRemittanceInformation17 {
            rfrd_doc_inf: self.rfrd_doc_inf,
            rfrd_doc_amt: self.rfrd_doc_amt,
            cdtr_ref_inf: self.cdtr_ref_inf,
            invcr: self.invcr,
            invcee: self.invcee,
            tax_rmt: self.tax_rmt,
            grnshmt_rmt: self.grnshmt_rmt,
            addtl_rmt_inf: self.addtl_rmt_inf,
        })
    }
}
impl StructuredRemittanceInformation17 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> StructuredRemittanceInformation17Builder {
        StructuredRemittanceInformation17Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplementaryData1 {
    #[serde(rename = "PlcAndNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plc_and_nm: Option<Max350Text>,
    #[serde(rename = "Envlp")]
    pub envlp: SupplementaryDataEnvelope1,
}
/// Builder for [`SupplementaryData1`]. Construct via [`SupplementaryData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SupplementaryData1Builder {
    plc_and_nm: ::std::option::Option<Max350Text>,
    envlp: ::std::option::Option<SupplementaryDataEnvelope1>,
}
impl SupplementaryData1Builder {
    /// Set the `plc_and_nm` field.
    #[must_use]
    pub fn plc_and_nm(mut self, value: Max350Text) -> SupplementaryData1Builder {
        self.plc_and_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `envlp` field.
    #[must_use]
    pub fn envlp(mut self, value: SupplementaryDataEnvelope1) -> SupplementaryData1Builder {
        self.envlp = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<SupplementaryData1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.envlp.is_none() {
            missing.push("envlp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "SupplementaryData1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(SupplementaryData1 {
            plc_and_nm: self.plc_and_nm,
            envlp: self.envlp.unwrap(),
        })
    }
}
impl SupplementaryData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SupplementaryData1Builder {
        SupplementaryData1Builder::default()
    }
}
/// Accepts content from namespace: `##any`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SupplementaryDataEnvelope1 {
    #[serde(rename = "$value")]
    pub value: String,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxAmount3 {
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "TaxblBaseAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtls: Vec<TaxRecordDetails3>,
}
/// Builder for [`TaxAmount3`]. Construct via [`TaxAmount3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxAmount3Builder {
    rate: ::std::option::Option<PercentageRate>,
    taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ttl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    dtls: ::std::vec::Vec<TaxRecordDetails3>,
}
impl TaxAmount3Builder {
    /// Set the `rate` field.
    #[must_use]
    pub fn rate(mut self, value: PercentageRate) -> TaxAmount3Builder {
        self.rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `taxbl_base_amt` field.
    #[must_use]
    pub fn taxbl_base_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxAmount3Builder {
        self.taxbl_base_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_amt` field.
    #[must_use]
    pub fn ttl_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxAmount3Builder {
        self.ttl_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dtls` field (replaces any previously added items).
    #[must_use]
    pub fn dtls(mut self, value: ::std::vec::Vec<TaxRecordDetails3>) -> TaxAmount3Builder {
        self.dtls = value;
        self
    }
    /// Append one item to the `dtls` field.
    #[must_use]
    pub fn add_dtls(mut self, value: TaxRecordDetails3) -> TaxAmount3Builder {
        self.dtls.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxAmount3, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxAmount3 {
            rate: self.rate,
            taxbl_base_amt: self.taxbl_base_amt,
            ttl_amt: self.ttl_amt,
            dtls: self.dtls,
        })
    }
}
impl TaxAmount3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxAmount3Builder {
        TaxAmount3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxAmountAndType1 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<TaxAmountType1Choice>>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}
/// Builder for [`TaxAmountAndType1`]. Construct via [`TaxAmountAndType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxAmountAndType1Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<TaxAmountType1Choice>>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl TaxAmountAndType1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<TaxAmountType1Choice>,
    ) -> TaxAmountAndType1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxAmountAndType1Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxAmountAndType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TaxAmountAndType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TaxAmountAndType1 {
            tp: self.tp,
            amt: self.amt.unwrap(),
        })
    }
}
impl TaxAmountAndType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxAmountAndType1Builder {
        TaxAmountAndType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TaxAmountType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalTaxAmountType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxAuthorisation1 {
    #[serde(rename = "Titl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub titl: Option<Max35Text>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
}
/// Builder for [`TaxAuthorisation1`]. Construct via [`TaxAuthorisation1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxAuthorisation1Builder {
    titl: ::std::option::Option<Max35Text>,
    nm: ::std::option::Option<Max140Text>,
}
impl TaxAuthorisation1Builder {
    /// Set the `titl` field.
    #[must_use]
    pub fn titl(mut self, value: Max35Text) -> TaxAuthorisation1Builder {
        self.titl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> TaxAuthorisation1Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxAuthorisation1, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxAuthorisation1 {
            titl: self.titl,
            nm: self.nm,
        })
    }
}
impl TaxAuthorisation1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxAuthorisation1Builder {
        TaxAuthorisation1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxData1 {
    #[serde(rename = "Cdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty1>,
    #[serde(rename = "Dbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty2>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<TaxParty2>,
    #[serde(rename = "AdmstnZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admstn_zone: Option<Max35Text>,
    #[serde(rename = "RefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<Max140Text>,
    #[serde(rename = "Mtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtd: Option<Max35Text>,
    #[serde(rename = "TtlTaxblBaseAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlTaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<ISODate>,
    #[serde(rename = "SeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "Rcrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rcrd: Vec<TaxRecord3>,
}
/// Builder for [`TaxData1`]. Construct via [`TaxData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxData1Builder {
    cdtr: ::std::option::Option<TaxParty1>,
    dbtr: ::std::option::Option<TaxParty2>,
    ultmt_dbtr: ::std::option::Option<TaxParty2>,
    admstn_zone: ::std::option::Option<Max35Text>,
    ref_nb: ::std::option::Option<Max140Text>,
    mtd: ::std::option::Option<Max35Text>,
    ttl_taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ttl_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    dt: ::std::option::Option<ISODate>,
    seq_nb: ::std::option::Option<Number>,
    rcrd: ::std::vec::Vec<TaxRecord3>,
}
impl TaxData1Builder {
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(mut self, value: TaxParty1) -> TaxData1Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(mut self, value: TaxParty2) -> TaxData1Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(mut self, value: TaxParty2) -> TaxData1Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `admstn_zone` field.
    #[must_use]
    pub fn admstn_zone(mut self, value: Max35Text) -> TaxData1Builder {
        self.admstn_zone = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref_nb` field.
    #[must_use]
    pub fn ref_nb(mut self, value: Max140Text) -> TaxData1Builder {
        self.ref_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mtd` field.
    #[must_use]
    pub fn mtd(mut self, value: Max35Text) -> TaxData1Builder {
        self.mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_taxbl_base_amt` field.
    #[must_use]
    pub fn ttl_taxbl_base_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxData1Builder {
        self.ttl_taxbl_base_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_tax_amt` field.
    #[must_use]
    pub fn ttl_tax_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxData1Builder {
        self.ttl_tax_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> TaxData1Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `seq_nb` field.
    #[must_use]
    pub fn seq_nb(mut self, value: Number) -> TaxData1Builder {
        self.seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rcrd` field (replaces any previously added items).
    #[must_use]
    pub fn rcrd(mut self, value: ::std::vec::Vec<TaxRecord3>) -> TaxData1Builder {
        self.rcrd = value;
        self
    }
    /// Append one item to the `rcrd` field.
    #[must_use]
    pub fn add_rcrd(mut self, value: TaxRecord3) -> TaxData1Builder {
        self.rcrd.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxData1, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxData1 {
            cdtr: self.cdtr,
            dbtr: self.dbtr,
            ultmt_dbtr: self.ultmt_dbtr,
            admstn_zone: self.admstn_zone,
            ref_nb: self.ref_nb,
            mtd: self.mtd,
            ttl_taxbl_base_amt: self.ttl_taxbl_base_amt,
            ttl_tax_amt: self.ttl_tax_amt,
            dt: self.dt,
            seq_nb: self.seq_nb,
            rcrd: self.rcrd,
        })
    }
}
impl TaxData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxData1Builder {
        TaxData1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxInformation10 {
    #[serde(rename = "Cdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<TaxParty1>,
    #[serde(rename = "Dbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<TaxParty2>,
    #[serde(rename = "AdmstnZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admstn_zone: Option<Max35Text>,
    #[serde(rename = "RefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_nb: Option<Max140Text>,
    #[serde(rename = "Mtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtd: Option<Max35Text>,
    #[serde(rename = "TtlTaxblBaseAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_taxbl_base_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "TtlTaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<ISODate>,
    #[serde(rename = "SeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Number>,
    #[serde(rename = "Rcrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rcrd: Vec<TaxRecord3>,
}
/// Builder for [`TaxInformation10`]. Construct via [`TaxInformation10::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxInformation10Builder {
    cdtr: ::std::option::Option<TaxParty1>,
    dbtr: ::std::option::Option<TaxParty2>,
    admstn_zone: ::std::option::Option<Max35Text>,
    ref_nb: ::std::option::Option<Max140Text>,
    mtd: ::std::option::Option<Max35Text>,
    ttl_taxbl_base_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ttl_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    dt: ::std::option::Option<ISODate>,
    seq_nb: ::std::option::Option<Number>,
    rcrd: ::std::vec::Vec<TaxRecord3>,
}
impl TaxInformation10Builder {
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(mut self, value: TaxParty1) -> TaxInformation10Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(mut self, value: TaxParty2) -> TaxInformation10Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `admstn_zone` field.
    #[must_use]
    pub fn admstn_zone(mut self, value: Max35Text) -> TaxInformation10Builder {
        self.admstn_zone = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref_nb` field.
    #[must_use]
    pub fn ref_nb(mut self, value: Max140Text) -> TaxInformation10Builder {
        self.ref_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mtd` field.
    #[must_use]
    pub fn mtd(mut self, value: Max35Text) -> TaxInformation10Builder {
        self.mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_taxbl_base_amt` field.
    #[must_use]
    pub fn ttl_taxbl_base_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxInformation10Builder {
        self.ttl_taxbl_base_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_tax_amt` field.
    #[must_use]
    pub fn ttl_tax_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxInformation10Builder {
        self.ttl_tax_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> TaxInformation10Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `seq_nb` field.
    #[must_use]
    pub fn seq_nb(mut self, value: Number) -> TaxInformation10Builder {
        self.seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rcrd` field (replaces any previously added items).
    #[must_use]
    pub fn rcrd(mut self, value: ::std::vec::Vec<TaxRecord3>) -> TaxInformation10Builder {
        self.rcrd = value;
        self
    }
    /// Append one item to the `rcrd` field.
    #[must_use]
    pub fn add_rcrd(mut self, value: TaxRecord3) -> TaxInformation10Builder {
        self.rcrd.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxInformation10, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxInformation10 {
            cdtr: self.cdtr,
            dbtr: self.dbtr,
            admstn_zone: self.admstn_zone,
            ref_nb: self.ref_nb,
            mtd: self.mtd,
            ttl_taxbl_base_amt: self.ttl_taxbl_base_amt,
            ttl_tax_amt: self.ttl_tax_amt,
            dt: self.dt,
            seq_nb: self.seq_nb,
            rcrd: self.rcrd,
        })
    }
}
impl TaxInformation10 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxInformation10Builder {
        TaxInformation10Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxParty1 {
    #[serde(rename = "TaxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<Max35Text>,
    #[serde(rename = "RegnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[serde(rename = "TaxTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
}
/// Builder for [`TaxParty1`]. Construct via [`TaxParty1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxParty1Builder {
    tax_id: ::std::option::Option<Max35Text>,
    regn_id: ::std::option::Option<Max35Text>,
    tax_tp: ::std::option::Option<Max35Text>,
}
impl TaxParty1Builder {
    /// Set the `tax_id` field.
    #[must_use]
    pub fn tax_id(mut self, value: Max35Text) -> TaxParty1Builder {
        self.tax_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `regn_id` field.
    #[must_use]
    pub fn regn_id(mut self, value: Max35Text) -> TaxParty1Builder {
        self.regn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_tp` field.
    #[must_use]
    pub fn tax_tp(mut self, value: Max35Text) -> TaxParty1Builder {
        self.tax_tp = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxParty1, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxParty1 {
            tax_id: self.tax_id,
            regn_id: self.regn_id,
            tax_tp: self.tax_tp,
        })
    }
}
impl TaxParty1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxParty1Builder {
        TaxParty1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxParty2 {
    #[serde(rename = "TaxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id: Option<Max35Text>,
    #[serde(rename = "RegnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub regn_id: Option<Max35Text>,
    #[serde(rename = "TaxTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
    #[serde(rename = "Authstn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authstn: Option<TaxAuthorisation1>,
}
/// Builder for [`TaxParty2`]. Construct via [`TaxParty2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxParty2Builder {
    tax_id: ::std::option::Option<Max35Text>,
    regn_id: ::std::option::Option<Max35Text>,
    tax_tp: ::std::option::Option<Max35Text>,
    authstn: ::std::option::Option<TaxAuthorisation1>,
}
impl TaxParty2Builder {
    /// Set the `tax_id` field.
    #[must_use]
    pub fn tax_id(mut self, value: Max35Text) -> TaxParty2Builder {
        self.tax_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `regn_id` field.
    #[must_use]
    pub fn regn_id(mut self, value: Max35Text) -> TaxParty2Builder {
        self.regn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_tp` field.
    #[must_use]
    pub fn tax_tp(mut self, value: Max35Text) -> TaxParty2Builder {
        self.tax_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authstn` field.
    #[must_use]
    pub fn authstn(mut self, value: TaxAuthorisation1) -> TaxParty2Builder {
        self.authstn = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxParty2, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxParty2 {
            tax_id: self.tax_id,
            regn_id: self.regn_id,
            tax_tp: self.tax_tp,
            authstn: self.authstn,
        })
    }
}
impl TaxParty2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxParty2Builder {
        TaxParty2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxPeriod3 {
    #[serde(rename = "Yr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yr: Option<ISOYear>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<TaxRecordPeriod1Code>,
    #[serde(rename = "FrToDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DatePeriod2>,
}
/// Builder for [`TaxPeriod3`]. Construct via [`TaxPeriod3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxPeriod3Builder {
    yr: ::std::option::Option<ISOYear>,
    tp: ::std::option::Option<TaxRecordPeriod1Code>,
    fr_to_dt: ::std::option::Option<DatePeriod2>,
}
impl TaxPeriod3Builder {
    /// Set the `yr` field.
    #[must_use]
    pub fn yr(mut self, value: ISOYear) -> TaxPeriod3Builder {
        self.yr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: TaxRecordPeriod1Code) -> TaxPeriod3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr_to_dt` field.
    #[must_use]
    pub fn fr_to_dt(mut self, value: DatePeriod2) -> TaxPeriod3Builder {
        self.fr_to_dt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxPeriod3, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxPeriod3 {
            yr: self.yr,
            tp: self.tp,
            fr_to_dt: self.fr_to_dt,
        })
    }
}
impl TaxPeriod3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxPeriod3Builder {
        TaxPeriod3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxRecord3 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Ctgy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy: Option<Max35Text>,
    #[serde(rename = "CtgyDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctgy_dtls: Option<Max35Text>,
    #[serde(rename = "DbtrSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_sts: Option<Max35Text>,
    #[serde(rename = "CertId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_id: Option<Max35Text>,
    #[serde(rename = "FrmsCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frms_cd: Option<Max35Text>,
    #[serde(rename = "Prd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<TaxPeriod3>,
    #[serde(rename = "TaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amt: Option<TaxAmount3>,
    #[serde(rename = "AddtlInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max140Text>,
}
/// Builder for [`TaxRecord3`]. Construct via [`TaxRecord3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxRecord3Builder {
    tp: ::std::option::Option<Max35Text>,
    ctgy: ::std::option::Option<Max35Text>,
    ctgy_dtls: ::std::option::Option<Max35Text>,
    dbtr_sts: ::std::option::Option<Max35Text>,
    cert_id: ::std::option::Option<Max35Text>,
    frms_cd: ::std::option::Option<Max35Text>,
    prd: ::std::option::Option<TaxPeriod3>,
    tax_amt: ::std::option::Option<TaxAmount3>,
    addtl_inf: ::std::option::Option<Max140Text>,
}
impl TaxRecord3Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy` field.
    #[must_use]
    pub fn ctgy(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.ctgy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy_dtls` field.
    #[must_use]
    pub fn ctgy_dtls(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.ctgy_dtls = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_sts` field.
    #[must_use]
    pub fn dbtr_sts(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.dbtr_sts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cert_id` field.
    #[must_use]
    pub fn cert_id(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.cert_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frms_cd` field.
    #[must_use]
    pub fn frms_cd(mut self, value: Max35Text) -> TaxRecord3Builder {
        self.frms_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prd` field.
    #[must_use]
    pub fn prd(mut self, value: TaxPeriod3) -> TaxRecord3Builder {
        self.prd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_amt` field.
    #[must_use]
    pub fn tax_amt(mut self, value: TaxAmount3) -> TaxRecord3Builder {
        self.tax_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field.
    #[must_use]
    pub fn addtl_inf(mut self, value: Max140Text) -> TaxRecord3Builder {
        self.addtl_inf = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxRecord3, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxRecord3 {
            tp: self.tp,
            ctgy: self.ctgy,
            ctgy_dtls: self.ctgy_dtls,
            dbtr_sts: self.dbtr_sts,
            cert_id: self.cert_id,
            frms_cd: self.frms_cd,
            prd: self.prd,
            tax_amt: self.tax_amt,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl TaxRecord3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxRecord3Builder {
        TaxRecord3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxRecordDetails3 {
    #[serde(rename = "Prd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prd: Option<TaxPeriod3>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}
/// Builder for [`TaxRecordDetails3`]. Construct via [`TaxRecordDetails3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxRecordDetails3Builder {
    prd: ::std::option::Option<TaxPeriod3>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl TaxRecordDetails3Builder {
    /// Set the `prd` field.
    #[must_use]
    pub fn prd(mut self, value: TaxPeriod3) -> TaxRecordDetails3Builder {
        self.prd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxRecordDetails3Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TaxRecordDetails3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TaxRecordDetails3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TaxRecordDetails3 {
            prd: self.prd,
            amt: self.amt.unwrap(),
        })
    }
}
impl TaxRecordDetails3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxRecordDetails3Builder {
        TaxRecordDetails3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionParties10 {
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Dbtr")]
    pub dbtr: crate::common::ChoiceWrapper<Party40Choice>,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "PrvsInstgAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3acct: Option<CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "Cdtr")]
    pub cdtr: crate::common::ChoiceWrapper<Party40Choice>,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
}
/// Builder for [`TransactionParties10`]. Construct via [`TransactionParties10::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionParties10Builder {
    ultmt_dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    initg_pty: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt1acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt2acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prvs_instg_agt3acct: ::std::option::Option<CashAccount40>,
    intrmy_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt1acct: ::std::option::Option<CashAccount40>,
    intrmy_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt2acct: ::std::option::Option<CashAccount40>,
    intrmy_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt3acct: ::std::option::Option<CashAccount40>,
    cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    cdtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    cdtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
}
impl TransactionParties10Builder {
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties10Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties10Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `initg_pty` field.
    #[must_use]
    pub fn initg_pty(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties10Builder {
        self.initg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1` field.
    #[must_use]
    pub fn prvs_instg_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.prvs_instg_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1acct` field.
    #[must_use]
    pub fn prvs_instg_agt1acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.prvs_instg_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2` field.
    #[must_use]
    pub fn prvs_instg_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.prvs_instg_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2acct` field.
    #[must_use]
    pub fn prvs_instg_agt2acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.prvs_instg_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3` field.
    #[must_use]
    pub fn prvs_instg_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.prvs_instg_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3acct` field.
    #[must_use]
    pub fn prvs_instg_agt3acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.prvs_instg_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1` field.
    #[must_use]
    pub fn intrmy_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.intrmy_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1acct` field.
    #[must_use]
    pub fn intrmy_agt1acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.intrmy_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2` field.
    #[must_use]
    pub fn intrmy_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.intrmy_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2acct` field.
    #[must_use]
    pub fn intrmy_agt2acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.intrmy_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3` field.
    #[must_use]
    pub fn intrmy_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.intrmy_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3acct` field.
    #[must_use]
    pub fn intrmy_agt3acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.intrmy_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionParties10Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties10Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> TransactionParties10Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties10Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<TransactionParties10, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.dbtr.is_none() {
            missing.push("dbtr".to_owned());
        }
        if self.cdtr.is_none() {
            missing.push("cdtr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TransactionParties10".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TransactionParties10 {
            ultmt_dbtr: self.ultmt_dbtr,
            dbtr: self.dbtr.unwrap(),
            dbtr_acct: self.dbtr_acct,
            initg_pty: self.initg_pty,
            dbtr_agt: self.dbtr_agt,
            dbtr_agt_acct: self.dbtr_agt_acct,
            prvs_instg_agt1: self.prvs_instg_agt1,
            prvs_instg_agt1acct: self.prvs_instg_agt1acct,
            prvs_instg_agt2: self.prvs_instg_agt2,
            prvs_instg_agt2acct: self.prvs_instg_agt2acct,
            prvs_instg_agt3: self.prvs_instg_agt3,
            prvs_instg_agt3acct: self.prvs_instg_agt3acct,
            intrmy_agt1: self.intrmy_agt1,
            intrmy_agt1acct: self.intrmy_agt1acct,
            intrmy_agt2: self.intrmy_agt2,
            intrmy_agt2acct: self.intrmy_agt2acct,
            intrmy_agt3: self.intrmy_agt3,
            intrmy_agt3acct: self.intrmy_agt3acct,
            cdtr_agt: self.cdtr_agt,
            cdtr_agt_acct: self.cdtr_agt_acct,
            cdtr: self.cdtr.unwrap(),
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
        })
    }
}
impl TransactionParties10 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionParties10Builder {
        TransactionParties10Builder::default()
    }
}
