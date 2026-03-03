/*! Generated from ISO 20022 XSD schema.
Namespace: `urn:iso:std:iso:20022:tech:xsd:pacs.008.001.13`*/
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
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalChargeType1Code(pub String);
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
pub struct ExternalCreditorReferenceType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDateType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentAmountType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentType1Code(pub String);
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
pub struct ExternalServiceLevel1Code(pub String);
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
/// Pattern: `[0-9a-fA-F]+`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct HexBinaryText(pub String);
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
/// Maximum length: 10240
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max10KBinary(pub String);
/// Minimum length: 1
/// Maximum length: 10
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max10Text(pub String);
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
/// Maximum length: 256
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max256Text(pub String);
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
pub enum PreferredContactMethod2Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "FAXX")]
    Faxx,
    #[serde(rename = "LETT")]
    Lett,
    #[serde(rename = "CELL")]
    Cell,
    #[serde(rename = "ONLI")]
    Onli,
    #[serde(rename = "PHON")]
    Phon,
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
pub enum RegulatoryReportingType1Code {
    #[serde(rename = "CRED")]
    Cred,
    #[serde(rename = "DEBT")]
    Debt,
    #[serde(rename = "BOTH")]
    Both,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RemittanceLocationMethod2Code {
    #[serde(rename = "FAXI")]
    Faxi,
    #[serde(rename = "EDIC")]
    Edic,
    #[serde(rename = "URID")]
    Urid,
    #[serde(rename = "EMAL")]
    Emal,
    #[serde(rename = "POST")]
    Post,
    #[serde(rename = "SMSM")]
    Smsm,
}
/// Pattern: `([0-9A-F][0-9A-F]){32}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct SHA256SignatureText(pub String);
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AdditionalDateTime1 {
    #[serde(rename = "AccptncDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<ISODateTime>,
    #[serde(rename = "PoolgAdjstmntDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poolg_adjstmnt_dt: Option<ISODate>,
    #[serde(rename = "XpryDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpry_dt_tm: Option<ISODateTime>,
}
/// Builder for [`AdditionalDateTime1`]. Construct via [`AdditionalDateTime1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AdditionalDateTime1Builder {
    accptnc_dt_tm: ::std::option::Option<ISODateTime>,
    poolg_adjstmnt_dt: ::std::option::Option<ISODate>,
    xpry_dt_tm: ::std::option::Option<ISODateTime>,
}
impl AdditionalDateTime1Builder {
    /// Set the `accptnc_dt_tm` field.
    #[must_use]
    pub fn accptnc_dt_tm(mut self, value: ISODateTime) -> AdditionalDateTime1Builder {
        self.accptnc_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `poolg_adjstmnt_dt` field.
    #[must_use]
    pub fn poolg_adjstmnt_dt(mut self, value: ISODate) -> AdditionalDateTime1Builder {
        self.poolg_adjstmnt_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xpry_dt_tm` field.
    #[must_use]
    pub fn xpry_dt_tm(mut self, value: ISODateTime) -> AdditionalDateTime1Builder {
        self.xpry_dt_tm = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<AdditionalDateTime1, crate::common::BuilderError> {
        ::std::result::Result::Ok(AdditionalDateTime1 {
            accptnc_dt_tm: self.accptnc_dt_tm,
            poolg_adjstmnt_dt: self.poolg_adjstmnt_dt,
            xpry_dt_tm: self.xpry_dt_tm,
        })
    }
}
impl AdditionalDateTime1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AdditionalDateTime1Builder {
        AdditionalDateTime1Builder::default()
    }
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
pub struct BranchAndFinancialInstitutionIdentification8 {
    #[serde(rename = "FinInstnId")]
    pub fin_instn_id: FinancialInstitutionIdentification23,
    #[serde(rename = "BrnchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brnch_id: Option<BranchData5>,
}
/// Builder for [`BranchAndFinancialInstitutionIdentification8`]. Construct via [`BranchAndFinancialInstitutionIdentification8::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BranchAndFinancialInstitutionIdentification8Builder {
    fin_instn_id: ::std::option::Option<FinancialInstitutionIdentification23>,
    brnch_id: ::std::option::Option<BranchData5>,
}
impl BranchAndFinancialInstitutionIdentification8Builder {
    /// Set the `fin_instn_id` field.
    #[must_use]
    pub fn fin_instn_id(
        mut self,
        value: FinancialInstitutionIdentification23,
    ) -> BranchAndFinancialInstitutionIdentification8Builder {
        self.fin_instn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `brnch_id` field.
    #[must_use]
    pub fn brnch_id(
        mut self,
        value: BranchData5,
    ) -> BranchAndFinancialInstitutionIdentification8Builder {
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
        BranchAndFinancialInstitutionIdentification8,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fin_instn_id.is_none() {
            missing.push("fin_instn_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BranchAndFinancialInstitutionIdentification8".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BranchAndFinancialInstitutionIdentification8 {
            fin_instn_id: self.fin_instn_id.unwrap(),
            brnch_id: self.brnch_id,
        })
    }
}
impl BranchAndFinancialInstitutionIdentification8 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BranchAndFinancialInstitutionIdentification8Builder {
        BranchAndFinancialInstitutionIdentification8Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BranchData5 {
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
    pub pstl_adr: Option<PostalAddress27>,
}
/// Builder for [`BranchData5`]. Construct via [`BranchData5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BranchData5Builder {
    id: ::std::option::Option<Max35Text>,
    lei: ::std::option::Option<LEIIdentifier>,
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress27>,
}
impl BranchData5Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> BranchData5Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> BranchData5Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> BranchData5Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: PostalAddress27) -> BranchData5Builder {
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
    pub fn build(self) -> ::std::result::Result<BranchData5, crate::common::BuilderError> {
        ::std::result::Result::Ok(BranchData5 {
            id: self.id,
            lei: self.lei,
            nm: self.nm,
            pstl_adr: self.pstl_adr,
        })
    }
}
impl BranchData5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BranchData5Builder {
        BranchData5Builder::default()
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChargeType3Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalChargeType1Code),
    #[serde(rename = "Prtry")]
    Prtry(GenericIdentification3),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Charges16 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification8,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<ChargeType3Choice>>,
}
/// Builder for [`Charges16`]. Construct via [`Charges16::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Charges16Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<ChargeType3Choice>>,
}
impl Charges16Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> Charges16Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `agt` field.
    #[must_use]
    pub fn agt(mut self, value: BranchAndFinancialInstitutionIdentification8) -> Charges16Builder {
        self.agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<ChargeType3Choice>,
    ) -> Charges16Builder {
        self.tp = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<Charges16, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.agt.is_none() {
            missing.push("agt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Charges16".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Charges16 {
            amt: self.amt.unwrap(),
            agt: self.agt.unwrap(),
            tp: self.tp,
        })
    }
}
impl Charges16 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Charges16Builder {
        Charges16Builder::default()
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
pub struct Contact13 {
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
    #[serde(rename = "URLAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url_adr: Option<Max2048Text>,
    #[serde(rename = "EmailAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_adr: Option<Max256Text>,
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
    pub prefrd_mtd: Option<PreferredContactMethod2Code>,
}
/// Builder for [`Contact13`]. Construct via [`Contact13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Contact13Builder {
    nm_prfx: ::std::option::Option<NamePrefix2Code>,
    nm: ::std::option::Option<Max140Text>,
    phne_nb: ::std::option::Option<PhoneNumber>,
    mob_nb: ::std::option::Option<PhoneNumber>,
    fax_nb: ::std::option::Option<PhoneNumber>,
    url_adr: ::std::option::Option<Max2048Text>,
    email_adr: ::std::option::Option<Max256Text>,
    email_purp: ::std::option::Option<Max35Text>,
    job_titl: ::std::option::Option<Max35Text>,
    rspnsblty: ::std::option::Option<Max35Text>,
    dept: ::std::option::Option<Max70Text>,
    othr: ::std::vec::Vec<OtherContact1>,
    prefrd_mtd: ::std::option::Option<PreferredContactMethod2Code>,
}
impl Contact13Builder {
    /// Set the `nm_prfx` field.
    #[must_use]
    pub fn nm_prfx(mut self, value: NamePrefix2Code) -> Contact13Builder {
        self.nm_prfx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> Contact13Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `phne_nb` field.
    #[must_use]
    pub fn phne_nb(mut self, value: PhoneNumber) -> Contact13Builder {
        self.phne_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mob_nb` field.
    #[must_use]
    pub fn mob_nb(mut self, value: PhoneNumber) -> Contact13Builder {
        self.mob_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fax_nb` field.
    #[must_use]
    pub fn fax_nb(mut self, value: PhoneNumber) -> Contact13Builder {
        self.fax_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `url_adr` field.
    #[must_use]
    pub fn url_adr(mut self, value: Max2048Text) -> Contact13Builder {
        self.url_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `email_adr` field.
    #[must_use]
    pub fn email_adr(mut self, value: Max256Text) -> Contact13Builder {
        self.email_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `email_purp` field.
    #[must_use]
    pub fn email_purp(mut self, value: Max35Text) -> Contact13Builder {
        self.email_purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `job_titl` field.
    #[must_use]
    pub fn job_titl(mut self, value: Max35Text) -> Contact13Builder {
        self.job_titl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rspnsblty` field.
    #[must_use]
    pub fn rspnsblty(mut self, value: Max35Text) -> Contact13Builder {
        self.rspnsblty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dept` field.
    #[must_use]
    pub fn dept(mut self, value: Max70Text) -> Contact13Builder {
        self.dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(mut self, value: ::std::vec::Vec<OtherContact1>) -> Contact13Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(mut self, value: OtherContact1) -> Contact13Builder {
        self.othr.push(value);
        self
    }
    /// Set the `prefrd_mtd` field.
    #[must_use]
    pub fn prefrd_mtd(mut self, value: PreferredContactMethod2Code) -> Contact13Builder {
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
    pub fn build(self) -> ::std::result::Result<Contact13, crate::common::BuilderError> {
        ::std::result::Result::Ok(Contact13 {
            nm_prfx: self.nm_prfx,
            nm: self.nm,
            phne_nb: self.phne_nb,
            mob_nb: self.mob_nb,
            fax_nb: self.fax_nb,
            url_adr: self.url_adr,
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
impl Contact13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Contact13Builder {
        Contact13Builder::default()
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
pub struct CreditTransferTransaction70 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification13,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,
    #[serde(rename = "IntrBkSttlmAmt")]
    pub intr_bk_sttlm_amt: ActiveCurrencyAndAmount,
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
    #[serde(rename = "AddtlDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_dt_tm: Option<AdditionalDateTime1>,
    #[serde(rename = "InstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "XchgRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "AgrdRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agrd_rate: Option<CurrencyExchange26>,
    #[serde(rename = "ChrgBr")]
    pub chrg_br: ChargeBearerType1Code,
    #[serde(rename = "ChrgsInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chrgs_inf: Vec<Charges16>,
    #[serde(rename = "MndtRltdInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
    #[serde(rename = "PmtSgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_sgntr: Option<crate::common::ChoiceWrapper<CryptographicKey1Choice>>,
    #[serde(rename = "PrvsInstgAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "PrvsInstgAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt1acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "PrvsInstgAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt2acct: Option<CashAccount40>,
    #[serde(rename = "PrvsInstgAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "PrvsInstgAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvs_instg_agt3acct: Option<CashAccount40>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "IntrmyAgt1Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "IntrmyAgt2Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2acct: Option<CashAccount40>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "IntrmyAgt3Acct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3acct: Option<CashAccount40>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification272>,
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<PartyIdentification272>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification272,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification8,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "CdtrAgt")]
    pub cdtr_agt: BranchAndFinancialInstitutionIdentification8,
    #[serde(rename = "CdtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "Cdtr")]
    pub cdtr: PartyIdentification272,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<PartyIdentification272>,
    #[serde(rename = "InstrForCdtrAgt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instr_for_cdtr_agt: Vec<InstructionForCreditorAgent3>,
    #[serde(rename = "InstrForNxtAgt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub instr_for_nxt_agt: Vec<InstructionForNextAgent1>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    #[serde(rename = "RgltryRptg")]
    /// Maximum 10 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rgltry_rptg: Vec<RegulatoryReporting3>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxData1>,
    #[serde(rename = "RltdRmtInf")]
    /// Maximum 10 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rltd_rmt_inf: Vec<RemittanceLocation8>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation22>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`CreditTransferTransaction70`]. Construct via [`CreditTransferTransaction70::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditTransferTransaction70Builder {
    pmt_id: ::std::option::Option<PaymentIdentification13>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation28>,
    intr_bk_sttlm_amt: ::std::option::Option<ActiveCurrencyAndAmount>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    sttlm_prty: ::std::option::Option<Priority3Code>,
    sttlm_tm_indctn: ::std::option::Option<SettlementDateTimeIndication1>,
    sttlm_tm_req: ::std::option::Option<SettlementTimeRequest2>,
    addtl_dt_tm: ::std::option::Option<AdditionalDateTime1>,
    instd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    xchg_rate: ::std::option::Option<BaseOneRate>,
    agrd_rate: ::std::option::Option<CurrencyExchange26>,
    chrg_br: ::std::option::Option<ChargeBearerType1Code>,
    chrgs_inf: ::std::vec::Vec<Charges16>,
    mndt_rltd_inf: ::std::option::Option<CreditTransferMandateData1>,
    pmt_sgntr: ::std::option::Option<crate::common::ChoiceWrapper<CryptographicKey1Choice>>,
    prvs_instg_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    prvs_instg_agt1acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    prvs_instg_agt2acct: ::std::option::Option<CashAccount40>,
    prvs_instg_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    prvs_instg_agt3acct: ::std::option::Option<CashAccount40>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    intrmy_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    intrmy_agt1acct: ::std::option::Option<CashAccount40>,
    intrmy_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    intrmy_agt2acct: ::std::option::Option<CashAccount40>,
    intrmy_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    intrmy_agt3acct: ::std::option::Option<CashAccount40>,
    ultmt_dbtr: ::std::option::Option<PartyIdentification272>,
    initg_pty: ::std::option::Option<PartyIdentification272>,
    dbtr: ::std::option::Option<PartyIdentification272>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    cdtr_agt_acct: ::std::option::Option<CashAccount40>,
    cdtr: ::std::option::Option<PartyIdentification272>,
    cdtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_cdtr: ::std::option::Option<PartyIdentification272>,
    instr_for_cdtr_agt: ::std::vec::Vec<InstructionForCreditorAgent3>,
    instr_for_nxt_agt: ::std::vec::Vec<InstructionForNextAgent1>,
    purp: ::std::option::Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    rgltry_rptg: ::std::vec::Vec<RegulatoryReporting3>,
    tax: ::std::option::Option<TaxData1>,
    rltd_rmt_inf: ::std::vec::Vec<RemittanceLocation8>,
    rmt_inf: ::std::option::Option<RemittanceInformation22>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl CreditTransferTransaction70Builder {
    /// Set the `pmt_id` field.
    #[must_use]
    pub fn pmt_id(mut self, value: PaymentIdentification13) -> CreditTransferTransaction70Builder {
        self.pmt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(
        mut self,
        value: PaymentTypeInformation28,
    ) -> CreditTransferTransaction70Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn intr_bk_sttlm_amt(
        mut self,
        value: ActiveCurrencyAndAmount,
    ) -> CreditTransferTransaction70Builder {
        self.intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> CreditTransferTransaction70Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_prty` field.
    #[must_use]
    pub fn sttlm_prty(mut self, value: Priority3Code) -> CreditTransferTransaction70Builder {
        self.sttlm_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_tm_indctn` field.
    #[must_use]
    pub fn sttlm_tm_indctn(
        mut self,
        value: SettlementDateTimeIndication1,
    ) -> CreditTransferTransaction70Builder {
        self.sttlm_tm_indctn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_tm_req` field.
    #[must_use]
    pub fn sttlm_tm_req(
        mut self,
        value: SettlementTimeRequest2,
    ) -> CreditTransferTransaction70Builder {
        self.sttlm_tm_req = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_dt_tm` field.
    #[must_use]
    pub fn addtl_dt_tm(mut self, value: AdditionalDateTime1) -> CreditTransferTransaction70Builder {
        self.addtl_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_amt` field.
    #[must_use]
    pub fn instd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> CreditTransferTransaction70Builder {
        self.instd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate` field.
    #[must_use]
    pub fn xchg_rate(mut self, value: BaseOneRate) -> CreditTransferTransaction70Builder {
        self.xchg_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `agrd_rate` field.
    #[must_use]
    pub fn agrd_rate(mut self, value: CurrencyExchange26) -> CreditTransferTransaction70Builder {
        self.agrd_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrg_br` field.
    #[must_use]
    pub fn chrg_br(mut self, value: ChargeBearerType1Code) -> CreditTransferTransaction70Builder {
        self.chrg_br = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs_inf` field (replaces any previously added items).
    #[must_use]
    pub fn chrgs_inf(
        mut self,
        value: ::std::vec::Vec<Charges16>,
    ) -> CreditTransferTransaction70Builder {
        self.chrgs_inf = value;
        self
    }
    /// Append one item to the `chrgs_inf` field.
    #[must_use]
    pub fn add_chrgs_inf(mut self, value: Charges16) -> CreditTransferTransaction70Builder {
        self.chrgs_inf.push(value);
        self
    }
    /// Set the `mndt_rltd_inf` field.
    #[must_use]
    pub fn mndt_rltd_inf(
        mut self,
        value: CreditTransferMandateData1,
    ) -> CreditTransferTransaction70Builder {
        self.mndt_rltd_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_sgntr` field.
    #[must_use]
    pub fn pmt_sgntr(
        mut self,
        value: crate::common::ChoiceWrapper<CryptographicKey1Choice>,
    ) -> CreditTransferTransaction70Builder {
        self.pmt_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1` field.
    #[must_use]
    pub fn prvs_instg_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt1acct` field.
    #[must_use]
    pub fn prvs_instg_agt1acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2` field.
    #[must_use]
    pub fn prvs_instg_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt2acct` field.
    #[must_use]
    pub fn prvs_instg_agt2acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3` field.
    #[must_use]
    pub fn prvs_instg_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvs_instg_agt3acct` field.
    #[must_use]
    pub fn prvs_instg_agt3acct(
        mut self,
        value: CashAccount40,
    ) -> CreditTransferTransaction70Builder {
        self.prvs_instg_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1` field.
    #[must_use]
    pub fn intrmy_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.intrmy_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1acct` field.
    #[must_use]
    pub fn intrmy_agt1acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.intrmy_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2` field.
    #[must_use]
    pub fn intrmy_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.intrmy_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2acct` field.
    #[must_use]
    pub fn intrmy_agt2acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.intrmy_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3` field.
    #[must_use]
    pub fn intrmy_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.intrmy_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3acct` field.
    #[must_use]
    pub fn intrmy_agt3acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.intrmy_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: PartyIdentification272,
    ) -> CreditTransferTransaction70Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `initg_pty` field.
    #[must_use]
    pub fn initg_pty(
        mut self,
        value: PartyIdentification272,
    ) -> CreditTransferTransaction70Builder {
        self.initg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(mut self, value: PartyIdentification272) -> CreditTransferTransaction70Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CreditTransferTransaction70Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(mut self, value: PartyIdentification272) -> CreditTransferTransaction70Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> CreditTransferTransaction70Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: PartyIdentification272,
    ) -> CreditTransferTransaction70Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_for_cdtr_agt` field (replaces any previously added items).
    #[must_use]
    pub fn instr_for_cdtr_agt(
        mut self,
        value: ::std::vec::Vec<InstructionForCreditorAgent3>,
    ) -> CreditTransferTransaction70Builder {
        self.instr_for_cdtr_agt = value;
        self
    }
    /// Append one item to the `instr_for_cdtr_agt` field.
    #[must_use]
    pub fn add_instr_for_cdtr_agt(
        mut self,
        value: InstructionForCreditorAgent3,
    ) -> CreditTransferTransaction70Builder {
        self.instr_for_cdtr_agt.push(value);
        self
    }
    /// Set the `instr_for_nxt_agt` field (replaces any previously added items).
    #[must_use]
    pub fn instr_for_nxt_agt(
        mut self,
        value: ::std::vec::Vec<InstructionForNextAgent1>,
    ) -> CreditTransferTransaction70Builder {
        self.instr_for_nxt_agt = value;
        self
    }
    /// Append one item to the `instr_for_nxt_agt` field.
    #[must_use]
    pub fn add_instr_for_nxt_agt(
        mut self,
        value: InstructionForNextAgent1,
    ) -> CreditTransferTransaction70Builder {
        self.instr_for_nxt_agt.push(value);
        self
    }
    /// Set the `purp` field.
    #[must_use]
    pub fn purp(
        mut self,
        value: crate::common::ChoiceWrapper<Purpose2Choice>,
    ) -> CreditTransferTransaction70Builder {
        self.purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rgltry_rptg` field (replaces any previously added items).
    #[must_use]
    pub fn rgltry_rptg(
        mut self,
        value: ::std::vec::Vec<RegulatoryReporting3>,
    ) -> CreditTransferTransaction70Builder {
        self.rgltry_rptg = value;
        self
    }
    /// Append one item to the `rgltry_rptg` field.
    #[must_use]
    pub fn add_rgltry_rptg(
        mut self,
        value: RegulatoryReporting3,
    ) -> CreditTransferTransaction70Builder {
        self.rgltry_rptg.push(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxData1) -> CreditTransferTransaction70Builder {
        self.tax = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_rmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rltd_rmt_inf(
        mut self,
        value: ::std::vec::Vec<RemittanceLocation8>,
    ) -> CreditTransferTransaction70Builder {
        self.rltd_rmt_inf = value;
        self
    }
    /// Append one item to the `rltd_rmt_inf` field.
    #[must_use]
    pub fn add_rltd_rmt_inf(
        mut self,
        value: RemittanceLocation8,
    ) -> CreditTransferTransaction70Builder {
        self.rltd_rmt_inf.push(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(mut self, value: RemittanceInformation22) -> CreditTransferTransaction70Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> CreditTransferTransaction70Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> CreditTransferTransaction70Builder {
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
    ) -> ::std::result::Result<CreditTransferTransaction70, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pmt_id.is_none() {
            missing.push("pmt_id".to_owned());
        }
        if self.intr_bk_sttlm_amt.is_none() {
            missing.push("intr_bk_sttlm_amt".to_owned());
        }
        if self.chrg_br.is_none() {
            missing.push("chrg_br".to_owned());
        }
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
                type_name: "CreditTransferTransaction70".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CreditTransferTransaction70 {
            pmt_id: self.pmt_id.unwrap(),
            pmt_tp_inf: self.pmt_tp_inf,
            intr_bk_sttlm_amt: self.intr_bk_sttlm_amt.unwrap(),
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            sttlm_prty: self.sttlm_prty,
            sttlm_tm_indctn: self.sttlm_tm_indctn,
            sttlm_tm_req: self.sttlm_tm_req,
            addtl_dt_tm: self.addtl_dt_tm,
            instd_amt: self.instd_amt,
            xchg_rate: self.xchg_rate,
            agrd_rate: self.agrd_rate,
            chrg_br: self.chrg_br.unwrap(),
            chrgs_inf: self.chrgs_inf,
            mndt_rltd_inf: self.mndt_rltd_inf,
            pmt_sgntr: self.pmt_sgntr,
            prvs_instg_agt1: self.prvs_instg_agt1,
            prvs_instg_agt1acct: self.prvs_instg_agt1acct,
            prvs_instg_agt2: self.prvs_instg_agt2,
            prvs_instg_agt2acct: self.prvs_instg_agt2acct,
            prvs_instg_agt3: self.prvs_instg_agt3,
            prvs_instg_agt3acct: self.prvs_instg_agt3acct,
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
            intrmy_agt1: self.intrmy_agt1,
            intrmy_agt1acct: self.intrmy_agt1acct,
            intrmy_agt2: self.intrmy_agt2,
            intrmy_agt2acct: self.intrmy_agt2acct,
            intrmy_agt3: self.intrmy_agt3,
            intrmy_agt3acct: self.intrmy_agt3acct,
            ultmt_dbtr: self.ultmt_dbtr,
            initg_pty: self.initg_pty,
            dbtr: self.dbtr.unwrap(),
            dbtr_acct: self.dbtr_acct,
            dbtr_agt: self.dbtr_agt.unwrap(),
            dbtr_agt_acct: self.dbtr_agt_acct,
            cdtr_agt: self.cdtr_agt.unwrap(),
            cdtr_agt_acct: self.cdtr_agt_acct,
            cdtr: self.cdtr.unwrap(),
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
            instr_for_cdtr_agt: self.instr_for_cdtr_agt,
            instr_for_nxt_agt: self.instr_for_nxt_agt,
            purp: self.purp,
            rgltry_rptg: self.rgltry_rptg,
            tax: self.tax,
            rltd_rmt_inf: self.rltd_rmt_inf,
            rmt_inf: self.rmt_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl CreditTransferTransaction70 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditTransferTransaction70Builder {
        CreditTransferTransaction70Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditorReferenceInformation3 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<CreditorReferenceType3>,
    #[serde(rename = "Ref")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<Max35Text>,
}
/// Builder for [`CreditorReferenceInformation3`]. Construct via [`CreditorReferenceInformation3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditorReferenceInformation3Builder {
    tp: ::std::option::Option<CreditorReferenceType3>,
    r#ref: ::std::option::Option<Max35Text>,
}
impl CreditorReferenceInformation3Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: CreditorReferenceType3) -> CreditorReferenceInformation3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref` field.
    #[must_use]
    pub fn r#ref(mut self, value: Max35Text) -> CreditorReferenceInformation3Builder {
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
    ) -> ::std::result::Result<CreditorReferenceInformation3, crate::common::BuilderError> {
        ::std::result::Result::Ok(CreditorReferenceInformation3 {
            tp: self.tp,
            r#ref: self.r#ref,
        })
    }
}
impl CreditorReferenceInformation3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditorReferenceInformation3Builder {
        CreditorReferenceInformation3Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CreditorReferenceType2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalCreditorReferenceType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreditorReferenceType3 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<CreditorReferenceType2Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`CreditorReferenceType3`]. Construct via [`CreditorReferenceType3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditorReferenceType3Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<CreditorReferenceType2Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl CreditorReferenceType3Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<CreditorReferenceType2Choice>,
    ) -> CreditorReferenceType3Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> CreditorReferenceType3Builder {
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
    ) -> ::std::result::Result<CreditorReferenceType3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CreditorReferenceType3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CreditorReferenceType3 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl CreditorReferenceType3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditorReferenceType3Builder {
        CreditorReferenceType3Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CryptographicKey1Choice {
    #[serde(rename = "ILPV4")]
    ILPV4(HexBinaryText),
    #[serde(rename = "Sgntr")]
    Sgntr(SHA256SignatureText),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurrencyExchange26 {
    #[serde(rename = "UnitCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "QtdCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtd_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "PreAgrdXchgRate")]
    pub pre_agrd_xchg_rate: BaseOneRate,
    #[serde(rename = "QtnDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtn_dt_tm: Option<ISODateTime>,
    #[serde(rename = "QtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qt_id: Option<UUIDv4Identifier>,
    #[serde(rename = "FXAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fx_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}
/// Builder for [`CurrencyExchange26`]. Construct via [`CurrencyExchange26::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CurrencyExchange26Builder {
    unit_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    qtd_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    pre_agrd_xchg_rate: ::std::option::Option<BaseOneRate>,
    qtn_dt_tm: ::std::option::Option<ISODateTime>,
    qt_id: ::std::option::Option<UUIDv4Identifier>,
    fx_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
}
impl CurrencyExchange26Builder {
    /// Set the `unit_ccy` field.
    #[must_use]
    pub fn unit_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CurrencyExchange26Builder {
        self.unit_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `qtd_ccy` field.
    #[must_use]
    pub fn qtd_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CurrencyExchange26Builder {
        self.qtd_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pre_agrd_xchg_rate` field.
    #[must_use]
    pub fn pre_agrd_xchg_rate(mut self, value: BaseOneRate) -> CurrencyExchange26Builder {
        self.pre_agrd_xchg_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `qtn_dt_tm` field.
    #[must_use]
    pub fn qtn_dt_tm(mut self, value: ISODateTime) -> CurrencyExchange26Builder {
        self.qtn_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `qt_id` field.
    #[must_use]
    pub fn qt_id(mut self, value: UUIDv4Identifier) -> CurrencyExchange26Builder {
        self.qt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fx_agt` field.
    #[must_use]
    pub fn fx_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> CurrencyExchange26Builder {
        self.fx_agt = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CurrencyExchange26, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pre_agrd_xchg_rate.is_none() {
            missing.push("pre_agrd_xchg_rate".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CurrencyExchange26".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CurrencyExchange26 {
            unit_ccy: self.unit_ccy,
            qtd_ccy: self.qtd_ccy,
            pre_agrd_xchg_rate: self.pre_agrd_xchg_rate.unwrap(),
            qtn_dt_tm: self.qtn_dt_tm,
            qt_id: self.qt_id,
            fx_agt: self.fx_agt,
        })
    }
}
impl CurrencyExchange26 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CurrencyExchange26Builder {
        CurrencyExchange26Builder::default()
    }
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
pub struct DateAndType1 {
    #[serde(rename = "Tp")]
    pub tp: crate::common::ChoiceWrapper<DateType2Choice>,
    #[serde(rename = "Dt")]
    pub dt: ISODate,
}
/// Builder for [`DateAndType1`]. Construct via [`DateAndType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DateAndType1Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<DateType2Choice>>,
    dt: ::std::option::Option<ISODate>,
}
impl DateAndType1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<DateType2Choice>,
    ) -> DateAndType1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> DateAndType1Builder {
        self.dt = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<DateAndType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.dt.is_none() {
            missing.push("dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DateAndType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DateAndType1 {
            tp: self.tp.unwrap(),
            dt: self.dt.unwrap(),
        })
    }
}
impl DateAndType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DateAndType1Builder {
        DateAndType1Builder::default()
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DateType2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalDateType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    #[serde(rename = "FIToFICstmrCdtTrf")]
    pub fi_to_fi_cstmr_cdt_trf: FIToFICustomerCreditTransferV13,
}
/// Builder for [`Document`]. Construct via [`Document::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentBuilder {
    fi_to_fi_cstmr_cdt_trf: ::std::option::Option<FIToFICustomerCreditTransferV13>,
}
impl DocumentBuilder {
    /// Set the `fi_to_fi_cstmr_cdt_trf` field.
    #[must_use]
    pub fn fi_to_fi_cstmr_cdt_trf(
        mut self,
        value: FIToFICustomerCreditTransferV13,
    ) -> DocumentBuilder {
        self.fi_to_fi_cstmr_cdt_trf = ::std::option::Option::Some(value);
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
        if self.fi_to_fi_cstmr_cdt_trf.is_none() {
            missing.push("fi_to_fi_cstmr_cdt_trf".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Document".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Document {
            fi_to_fi_cstmr_cdt_trf: self.fi_to_fi_cstmr_cdt_trf.unwrap(),
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
pub struct DocumentAmount1 {
    #[serde(rename = "Tp")]
    pub tp: crate::common::ChoiceWrapper<DocumentAmountType1Choice>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
}
/// Builder for [`DocumentAmount1`]. Construct via [`DocumentAmount1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentAmount1Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<DocumentAmountType1Choice>>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl DocumentAmount1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<DocumentAmountType1Choice>,
    ) -> DocumentAmount1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> DocumentAmount1Builder {
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
    pub fn build(self) -> ::std::result::Result<DocumentAmount1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DocumentAmount1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DocumentAmount1 {
            tp: self.tp.unwrap(),
            amt: self.amt.unwrap(),
        })
    }
}
impl DocumentAmount1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentAmount1Builder {
        DocumentAmount1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocumentAmountType1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalDocumentAmountType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
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
pub struct DocumentLineInformation2 {
    #[serde(rename = "Id")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub id: Vec<DocumentLineIdentification1>,
    #[serde(rename = "Desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max2048Text>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<RemittanceAmount4>,
}
/// Builder for [`DocumentLineInformation2`]. Construct via [`DocumentLineInformation2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentLineInformation2Builder {
    id: ::std::vec::Vec<DocumentLineIdentification1>,
    desc: ::std::option::Option<Max2048Text>,
    amt: ::std::option::Option<RemittanceAmount4>,
}
impl DocumentLineInformation2Builder {
    /// Set the `id` field (replaces any previously added items).
    #[must_use]
    pub fn id(
        mut self,
        value: ::std::vec::Vec<DocumentLineIdentification1>,
    ) -> DocumentLineInformation2Builder {
        self.id = value;
        self
    }
    /// Append one item to the `id` field.
    #[must_use]
    pub fn add_id(mut self, value: DocumentLineIdentification1) -> DocumentLineInformation2Builder {
        self.id.push(value);
        self
    }
    /// Set the `desc` field.
    #[must_use]
    pub fn desc(mut self, value: Max2048Text) -> DocumentLineInformation2Builder {
        self.desc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: RemittanceAmount4) -> DocumentLineInformation2Builder {
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
    ) -> ::std::result::Result<DocumentLineInformation2, crate::common::BuilderError> {
        ::std::result::Result::Ok(DocumentLineInformation2 {
            id: self.id,
            desc: self.desc,
            amt: self.amt,
        })
    }
}
impl DocumentLineInformation2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentLineInformation2Builder {
        DocumentLineInformation2Builder::default()
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
pub struct DocumentType1 {
    #[serde(rename = "CdOrPrtry")]
    pub cd_or_prtry: crate::common::ChoiceWrapper<DocumentType2Choice>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`DocumentType1`]. Construct via [`DocumentType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentType1Builder {
    cd_or_prtry: ::std::option::Option<crate::common::ChoiceWrapper<DocumentType2Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl DocumentType1Builder {
    /// Set the `cd_or_prtry` field.
    #[must_use]
    pub fn cd_or_prtry(
        mut self,
        value: crate::common::ChoiceWrapper<DocumentType2Choice>,
    ) -> DocumentType1Builder {
        self.cd_or_prtry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> DocumentType1Builder {
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
    pub fn build(self) -> ::std::result::Result<DocumentType1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd_or_prtry.is_none() {
            missing.push("cd_or_prtry".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DocumentType1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DocumentType1 {
            cd_or_prtry: self.cd_or_prtry.unwrap(),
            issr: self.issr,
        })
    }
}
impl DocumentType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DocumentType1Builder {
        DocumentType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DocumentType2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalDocumentType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FIToFICustomerCreditTransferV13 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader131,
    #[serde(rename = "CdtTrfTxInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cdt_trf_tx_inf: Vec<CreditTransferTransaction70>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`FIToFICustomerCreditTransferV13`]. Construct via [`FIToFICustomerCreditTransferV13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FIToFICustomerCreditTransferV13Builder {
    grp_hdr: ::std::option::Option<GroupHeader131>,
    cdt_trf_tx_inf: ::std::vec::Vec<CreditTransferTransaction70>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl FIToFICustomerCreditTransferV13Builder {
    /// Set the `grp_hdr` field.
    #[must_use]
    pub fn grp_hdr(mut self, value: GroupHeader131) -> FIToFICustomerCreditTransferV13Builder {
        self.grp_hdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_trf_tx_inf` field (replaces any previously added items).
    #[must_use]
    pub fn cdt_trf_tx_inf(
        mut self,
        value: ::std::vec::Vec<CreditTransferTransaction70>,
    ) -> FIToFICustomerCreditTransferV13Builder {
        self.cdt_trf_tx_inf = value;
        self
    }
    /// Append one item to the `cdt_trf_tx_inf` field.
    #[must_use]
    pub fn add_cdt_trf_tx_inf(
        mut self,
        value: CreditTransferTransaction70,
    ) -> FIToFICustomerCreditTransferV13Builder {
        self.cdt_trf_tx_inf.push(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> FIToFICustomerCreditTransferV13Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> FIToFICustomerCreditTransferV13Builder {
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
    ) -> ::std::result::Result<FIToFICustomerCreditTransferV13, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.grp_hdr.is_none() {
            missing.push("grp_hdr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "FIToFICustomerCreditTransferV13".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(FIToFICustomerCreditTransferV13 {
            grp_hdr: self.grp_hdr.unwrap(),
            cdt_trf_tx_inf: self.cdt_trf_tx_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl FIToFICustomerCreditTransferV13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FIToFICustomerCreditTransferV13Builder {
        FIToFICustomerCreditTransferV13Builder::default()
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
pub struct FinancialInstitutionIdentification23 {
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
    pub pstl_adr: Option<PostalAddress27>,
    #[serde(rename = "Othr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub othr: Option<GenericFinancialIdentification1>,
}
/// Builder for [`FinancialInstitutionIdentification23`]. Construct via [`FinancialInstitutionIdentification23::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FinancialInstitutionIdentification23Builder {
    bicfi: ::std::option::Option<BICFIDec2014Identifier>,
    clr_sys_mmb_id: ::std::option::Option<ClearingSystemMemberIdentification2>,
    lei: ::std::option::Option<LEIIdentifier>,
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress27>,
    othr: ::std::option::Option<GenericFinancialIdentification1>,
}
impl FinancialInstitutionIdentification23Builder {
    /// Set the `bicfi` field.
    #[must_use]
    pub fn bicfi(
        mut self,
        value: BICFIDec2014Identifier,
    ) -> FinancialInstitutionIdentification23Builder {
        self.bicfi = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys_mmb_id` field.
    #[must_use]
    pub fn clr_sys_mmb_id(
        mut self,
        value: ClearingSystemMemberIdentification2,
    ) -> FinancialInstitutionIdentification23Builder {
        self.clr_sys_mmb_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> FinancialInstitutionIdentification23Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> FinancialInstitutionIdentification23Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(
        mut self,
        value: PostalAddress27,
    ) -> FinancialInstitutionIdentification23Builder {
        self.pstl_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field.
    #[must_use]
    pub fn othr(
        mut self,
        value: GenericFinancialIdentification1,
    ) -> FinancialInstitutionIdentification23Builder {
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
    ) -> ::std::result::Result<FinancialInstitutionIdentification23, crate::common::BuilderError>
    {
        ::std::result::Result::Ok(FinancialInstitutionIdentification23 {
            bicfi: self.bicfi,
            clr_sys_mmb_id: self.clr_sys_mmb_id,
            lei: self.lei,
            nm: self.nm,
            pstl_adr: self.pstl_adr,
            othr: self.othr,
        })
    }
}
impl FinancialInstitutionIdentification23 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FinancialInstitutionIdentification23Builder {
        FinancialInstitutionIdentification23Builder::default()
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
pub struct Garnishment4 {
    #[serde(rename = "Tp")]
    pub tp: GarnishmentType1,
    #[serde(rename = "Grnshee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshee: Option<PartyIdentification272>,
    #[serde(rename = "GrnshmtAdmstr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_admstr: Option<PartyIdentification272>,
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
/// Builder for [`Garnishment4`]. Construct via [`Garnishment4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Garnishment4Builder {
    tp: ::std::option::Option<GarnishmentType1>,
    grnshee: ::std::option::Option<PartyIdentification272>,
    grnshmt_admstr: ::std::option::Option<PartyIdentification272>,
    ref_nb: ::std::option::Option<Max140Text>,
    dt: ::std::option::Option<ISODate>,
    rmtd_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    fmly_mdcl_insrnc_ind: ::std::option::Option<TrueFalseIndicator>,
    mplyee_termntn_ind: ::std::option::Option<TrueFalseIndicator>,
}
impl Garnishment4Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: GarnishmentType1) -> Garnishment4Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshee` field.
    #[must_use]
    pub fn grnshee(mut self, value: PartyIdentification272) -> Garnishment4Builder {
        self.grnshee = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshmt_admstr` field.
    #[must_use]
    pub fn grnshmt_admstr(mut self, value: PartyIdentification272) -> Garnishment4Builder {
        self.grnshmt_admstr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref_nb` field.
    #[must_use]
    pub fn ref_nb(mut self, value: Max140Text) -> Garnishment4Builder {
        self.ref_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> Garnishment4Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmtd_amt` field.
    #[must_use]
    pub fn rmtd_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> Garnishment4Builder {
        self.rmtd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fmly_mdcl_insrnc_ind` field.
    #[must_use]
    pub fn fmly_mdcl_insrnc_ind(mut self, value: TrueFalseIndicator) -> Garnishment4Builder {
        self.fmly_mdcl_insrnc_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mplyee_termntn_ind` field.
    #[must_use]
    pub fn mplyee_termntn_ind(mut self, value: TrueFalseIndicator) -> Garnishment4Builder {
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
    pub fn build(self) -> ::std::result::Result<Garnishment4, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Garnishment4".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Garnishment4 {
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
impl Garnishment4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Garnishment4Builder {
        Garnishment4Builder::default()
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
pub struct GenericIdentification3 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericIdentification3`]. Construct via [`GenericIdentification3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericIdentification3Builder {
    id: ::std::option::Option<Max35Text>,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericIdentification3Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericIdentification3Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericIdentification3Builder {
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
    ) -> ::std::result::Result<GenericIdentification3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericIdentification3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericIdentification3 {
            id: self.id.unwrap(),
            issr: self.issr,
        })
    }
}
impl GenericIdentification3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericIdentification3Builder {
        GenericIdentification3Builder::default()
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
pub struct GenericOrganisationIdentification3 {
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericOrganisationIdentification3`]. Construct via [`GenericOrganisationIdentification3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericOrganisationIdentification3Builder {
    id: ::std::option::Option<Max256Text>,
    schme_nm: ::std::option::Option<
        crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>,
    >,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericOrganisationIdentification3Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max256Text) -> GenericOrganisationIdentification3Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>,
    ) -> GenericOrganisationIdentification3Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericOrganisationIdentification3Builder {
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
    ) -> ::std::result::Result<GenericOrganisationIdentification3, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericOrganisationIdentification3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericOrganisationIdentification3 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericOrganisationIdentification3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericOrganisationIdentification3Builder {
        GenericOrganisationIdentification3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenericPersonIdentification2 {
    #[serde(rename = "Id")]
    pub id: Max256Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericPersonIdentification2`]. Construct via [`GenericPersonIdentification2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericPersonIdentification2Builder {
    id: ::std::option::Option<Max256Text>,
    schme_nm:
        ::std::option::Option<crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>>,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericPersonIdentification2Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max256Text) -> GenericPersonIdentification2Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(
        mut self,
        value: crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>,
    ) -> GenericPersonIdentification2Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericPersonIdentification2Builder {
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
    ) -> ::std::result::Result<GenericPersonIdentification2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericPersonIdentification2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericPersonIdentification2 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericPersonIdentification2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericPersonIdentification2Builder {
        GenericPersonIdentification2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GroupHeader131 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: ISODateTime,
    #[serde(rename = "XpryDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xpry_dt_tm: Option<ISODateTime>,
    #[serde(rename = "BtchBookg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<BatchBookingIndicator>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericText,
    #[serde(rename = "CtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[serde(rename = "TtlIntrBkSttlmAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_intr_bk_sttlm_amt: Option<ActiveCurrencyAndAmount>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "SttlmInf")]
    pub sttlm_inf: SettlementInstruction15,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation28>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification8>,
}
/// Builder for [`GroupHeader131`]. Construct via [`GroupHeader131::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GroupHeader131Builder {
    msg_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    xpry_dt_tm: ::std::option::Option<ISODateTime>,
    btch_bookg: ::std::option::Option<BatchBookingIndicator>,
    nb_of_txs: ::std::option::Option<Max15NumericText>,
    ctrl_sum: ::std::option::Option<DecimalNumber>,
    ttl_intr_bk_sttlm_amt: ::std::option::Option<ActiveCurrencyAndAmount>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    sttlm_inf: ::std::option::Option<SettlementInstruction15>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation28>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
}
impl GroupHeader131Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> GroupHeader131Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> GroupHeader131Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xpry_dt_tm` field.
    #[must_use]
    pub fn xpry_dt_tm(mut self, value: ISODateTime) -> GroupHeader131Builder {
        self.xpry_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `btch_bookg` field.
    #[must_use]
    pub fn btch_bookg(mut self, value: BatchBookingIndicator) -> GroupHeader131Builder {
        self.btch_bookg = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_txs` field.
    #[must_use]
    pub fn nb_of_txs(mut self, value: Max15NumericText) -> GroupHeader131Builder {
        self.nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrl_sum` field.
    #[must_use]
    pub fn ctrl_sum(mut self, value: DecimalNumber) -> GroupHeader131Builder {
        self.ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn ttl_intr_bk_sttlm_amt(
        mut self,
        value: ActiveCurrencyAndAmount,
    ) -> GroupHeader131Builder {
        self.ttl_intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> GroupHeader131Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_inf` field.
    #[must_use]
    pub fn sttlm_inf(mut self, value: SettlementInstruction15) -> GroupHeader131Builder {
        self.sttlm_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(mut self, value: PaymentTypeInformation28) -> GroupHeader131Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> GroupHeader131Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> GroupHeader131Builder {
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
    pub fn build(self) -> ::std::result::Result<GroupHeader131, crate::common::BuilderError> {
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
                type_name: "GroupHeader131".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GroupHeader131 {
            msg_id: self.msg_id.unwrap(),
            cre_dt_tm: self.cre_dt_tm.unwrap(),
            xpry_dt_tm: self.xpry_dt_tm,
            btch_bookg: self.btch_bookg,
            nb_of_txs: self.nb_of_txs.unwrap(),
            ctrl_sum: self.ctrl_sum,
            ttl_intr_bk_sttlm_amt: self.ttl_intr_bk_sttlm_amt,
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            sttlm_inf: self.sttlm_inf.unwrap(),
            pmt_tp_inf: self.pmt_tp_inf,
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
        })
    }
}
impl GroupHeader131 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GroupHeader131Builder {
        GroupHeader131Builder::default()
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
pub struct NameAndAddress18 {
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress27,
}
/// Builder for [`NameAndAddress18`]. Construct via [`NameAndAddress18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct NameAndAddress18Builder {
    nm: ::std::option::Option<Max140Text>,
    adr: ::std::option::Option<PostalAddress27>,
}
impl NameAndAddress18Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> NameAndAddress18Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `adr` field.
    #[must_use]
    pub fn adr(mut self, value: PostalAddress27) -> NameAndAddress18Builder {
        self.adr = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<NameAndAddress18, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.nm.is_none() {
            missing.push("nm".to_owned());
        }
        if self.adr.is_none() {
            missing.push("adr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "NameAndAddress18".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(NameAndAddress18 {
            nm: self.nm.unwrap(),
            adr: self.adr.unwrap(),
        })
    }
}
impl NameAndAddress18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> NameAndAddress18Builder {
        NameAndAddress18Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrganisationIdentification39 {
    #[serde(rename = "AnyBIC")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub any_bic: Option<AnyBICDec2014Identifier>,
    #[serde(rename = "LEI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lei: Option<LEIIdentifier>,
    #[serde(rename = "Othr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr: Vec<GenericOrganisationIdentification3>,
}
/// Builder for [`OrganisationIdentification39`]. Construct via [`OrganisationIdentification39::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OrganisationIdentification39Builder {
    any_bic: ::std::option::Option<AnyBICDec2014Identifier>,
    lei: ::std::option::Option<LEIIdentifier>,
    othr: ::std::vec::Vec<GenericOrganisationIdentification3>,
}
impl OrganisationIdentification39Builder {
    /// Set the `any_bic` field.
    #[must_use]
    pub fn any_bic(
        mut self,
        value: AnyBICDec2014Identifier,
    ) -> OrganisationIdentification39Builder {
        self.any_bic = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lei` field.
    #[must_use]
    pub fn lei(mut self, value: LEIIdentifier) -> OrganisationIdentification39Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(
        mut self,
        value: ::std::vec::Vec<GenericOrganisationIdentification3>,
    ) -> OrganisationIdentification39Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(
        mut self,
        value: GenericOrganisationIdentification3,
    ) -> OrganisationIdentification39Builder {
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
    ) -> ::std::result::Result<OrganisationIdentification39, crate::common::BuilderError> {
        ::std::result::Result::Ok(OrganisationIdentification39 {
            any_bic: self.any_bic,
            lei: self.lei,
            othr: self.othr,
        })
    }
}
impl OrganisationIdentification39 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OrganisationIdentification39Builder {
        OrganisationIdentification39Builder::default()
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
pub enum Party52Choice {
    #[serde(rename = "OrgId")]
    OrgId(OrganisationIdentification39),
    #[serde(rename = "PrvtId")]
    PrvtId(PersonIdentification18),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PartyIdentification272 {
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<PostalAddress27>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<crate::common::ChoiceWrapper<Party52Choice>>,
    #[serde(rename = "CtryOfRes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry_of_res: Option<CountryCode>,
    #[serde(rename = "CtctDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctct_dtls: Option<Contact13>,
}
/// Builder for [`PartyIdentification272`]. Construct via [`PartyIdentification272::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PartyIdentification272Builder {
    nm: ::std::option::Option<Max140Text>,
    pstl_adr: ::std::option::Option<PostalAddress27>,
    id: ::std::option::Option<crate::common::ChoiceWrapper<Party52Choice>>,
    ctry_of_res: ::std::option::Option<CountryCode>,
    ctct_dtls: ::std::option::Option<Contact13>,
}
impl PartyIdentification272Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> PartyIdentification272Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: PostalAddress27) -> PartyIdentification272Builder {
        self.pstl_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `id` field.
    #[must_use]
    pub fn id(
        mut self,
        value: crate::common::ChoiceWrapper<Party52Choice>,
    ) -> PartyIdentification272Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry_of_res` field.
    #[must_use]
    pub fn ctry_of_res(mut self, value: CountryCode) -> PartyIdentification272Builder {
        self.ctry_of_res = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctct_dtls` field.
    #[must_use]
    pub fn ctct_dtls(mut self, value: Contact13) -> PartyIdentification272Builder {
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
    ) -> ::std::result::Result<PartyIdentification272, crate::common::BuilderError> {
        ::std::result::Result::Ok(PartyIdentification272 {
            nm: self.nm,
            pstl_adr: self.pstl_adr,
            id: self.id,
            ctry_of_res: self.ctry_of_res,
            ctct_dtls: self.ctct_dtls,
        })
    }
}
impl PartyIdentification272 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PartyIdentification272Builder {
        PartyIdentification272Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentIdentification13 {
    #[serde(rename = "InstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<Max35Text>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Max35Text,
    #[serde(rename = "TxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<Max35Text>,
    #[serde(rename = "UETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uetr: Option<UUIDv4Identifier>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<Max35Text>,
}
/// Builder for [`PaymentIdentification13`]. Construct via [`PaymentIdentification13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentIdentification13Builder {
    instr_id: ::std::option::Option<Max35Text>,
    end_to_end_id: ::std::option::Option<Max35Text>,
    tx_id: ::std::option::Option<Max35Text>,
    uetr: ::std::option::Option<UUIDv4Identifier>,
    clr_sys_ref: ::std::option::Option<Max35Text>,
}
impl PaymentIdentification13Builder {
    /// Set the `instr_id` field.
    #[must_use]
    pub fn instr_id(mut self, value: Max35Text) -> PaymentIdentification13Builder {
        self.instr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `end_to_end_id` field.
    #[must_use]
    pub fn end_to_end_id(mut self, value: Max35Text) -> PaymentIdentification13Builder {
        self.end_to_end_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_id` field.
    #[must_use]
    pub fn tx_id(mut self, value: Max35Text) -> PaymentIdentification13Builder {
        self.tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `uetr` field.
    #[must_use]
    pub fn uetr(mut self, value: UUIDv4Identifier) -> PaymentIdentification13Builder {
        self.uetr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys_ref` field.
    #[must_use]
    pub fn clr_sys_ref(mut self, value: Max35Text) -> PaymentIdentification13Builder {
        self.clr_sys_ref = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<PaymentIdentification13, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.end_to_end_id.is_none() {
            missing.push("end_to_end_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentIdentification13".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentIdentification13 {
            instr_id: self.instr_id,
            end_to_end_id: self.end_to_end_id.unwrap(),
            tx_id: self.tx_id,
            uetr: self.uetr,
            clr_sys_ref: self.clr_sys_ref,
        })
    }
}
impl PaymentIdentification13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentIdentification13Builder {
        PaymentIdentification13Builder::default()
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
pub struct PersonIdentification18 {
    #[serde(rename = "DtAndPlcOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt_and_plc_of_birth: Option<DateAndPlaceOfBirth1>,
    #[serde(rename = "Othr")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr: Vec<GenericPersonIdentification2>,
}
/// Builder for [`PersonIdentification18`]. Construct via [`PersonIdentification18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PersonIdentification18Builder {
    dt_and_plc_of_birth: ::std::option::Option<DateAndPlaceOfBirth1>,
    othr: ::std::vec::Vec<GenericPersonIdentification2>,
}
impl PersonIdentification18Builder {
    /// Set the `dt_and_plc_of_birth` field.
    #[must_use]
    pub fn dt_and_plc_of_birth(
        mut self,
        value: DateAndPlaceOfBirth1,
    ) -> PersonIdentification18Builder {
        self.dt_and_plc_of_birth = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr` field (replaces any previously added items).
    #[must_use]
    pub fn othr(
        mut self,
        value: ::std::vec::Vec<GenericPersonIdentification2>,
    ) -> PersonIdentification18Builder {
        self.othr = value;
        self
    }
    /// Append one item to the `othr` field.
    #[must_use]
    pub fn add_othr(
        mut self,
        value: GenericPersonIdentification2,
    ) -> PersonIdentification18Builder {
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
    ) -> ::std::result::Result<PersonIdentification18, crate::common::BuilderError> {
        ::std::result::Result::Ok(PersonIdentification18 {
            dt_and_plc_of_birth: self.dt_and_plc_of_birth,
            othr: self.othr,
        })
    }
}
impl PersonIdentification18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PersonIdentification18Builder {
        PersonIdentification18Builder::default()
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
pub struct PostalAddress27 {
    #[serde(rename = "AdrTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adr_tp: Option<crate::common::ChoiceWrapper<AddressType3Choice>>,
    #[serde(rename = "CareOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub care_of: Option<Max140Text>,
    #[serde(rename = "Dept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dept: Option<Max70Text>,
    #[serde(rename = "SubDept")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_dept: Option<Max70Text>,
    #[serde(rename = "StrtNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strt_nm: Option<Max140Text>,
    #[serde(rename = "BldgNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nb: Option<Max16Text>,
    #[serde(rename = "BldgNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bldg_nm: Option<Max140Text>,
    #[serde(rename = "Flr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flr: Option<Max70Text>,
    #[serde(rename = "UnitNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_nb: Option<Max16Text>,
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
    pub twn_nm: Option<Max140Text>,
    #[serde(rename = "TwnLctnNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twn_lctn_nm: Option<Max140Text>,
    #[serde(rename = "DstrctNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dstrct_nm: Option<Max140Text>,
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
/// Builder for [`PostalAddress27`]. Construct via [`PostalAddress27::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PostalAddress27Builder {
    adr_tp: ::std::option::Option<crate::common::ChoiceWrapper<AddressType3Choice>>,
    care_of: ::std::option::Option<Max140Text>,
    dept: ::std::option::Option<Max70Text>,
    sub_dept: ::std::option::Option<Max70Text>,
    strt_nm: ::std::option::Option<Max140Text>,
    bldg_nb: ::std::option::Option<Max16Text>,
    bldg_nm: ::std::option::Option<Max140Text>,
    flr: ::std::option::Option<Max70Text>,
    unit_nb: ::std::option::Option<Max16Text>,
    pst_bx: ::std::option::Option<Max16Text>,
    room: ::std::option::Option<Max70Text>,
    pst_cd: ::std::option::Option<Max16Text>,
    twn_nm: ::std::option::Option<Max140Text>,
    twn_lctn_nm: ::std::option::Option<Max140Text>,
    dstrct_nm: ::std::option::Option<Max140Text>,
    ctry_sub_dvsn: ::std::option::Option<Max35Text>,
    ctry: ::std::option::Option<CountryCode>,
    adr_line: ::std::vec::Vec<Max70Text>,
}
impl PostalAddress27Builder {
    /// Set the `adr_tp` field.
    #[must_use]
    pub fn adr_tp(
        mut self,
        value: crate::common::ChoiceWrapper<AddressType3Choice>,
    ) -> PostalAddress27Builder {
        self.adr_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `care_of` field.
    #[must_use]
    pub fn care_of(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.care_of = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dept` field.
    #[must_use]
    pub fn dept(mut self, value: Max70Text) -> PostalAddress27Builder {
        self.dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sub_dept` field.
    #[must_use]
    pub fn sub_dept(mut self, value: Max70Text) -> PostalAddress27Builder {
        self.sub_dept = ::std::option::Option::Some(value);
        self
    }
    /// Set the `strt_nm` field.
    #[must_use]
    pub fn strt_nm(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.strt_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bldg_nb` field.
    #[must_use]
    pub fn bldg_nb(mut self, value: Max16Text) -> PostalAddress27Builder {
        self.bldg_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bldg_nm` field.
    #[must_use]
    pub fn bldg_nm(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.bldg_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `flr` field.
    #[must_use]
    pub fn flr(mut self, value: Max70Text) -> PostalAddress27Builder {
        self.flr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `unit_nb` field.
    #[must_use]
    pub fn unit_nb(mut self, value: Max16Text) -> PostalAddress27Builder {
        self.unit_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pst_bx` field.
    #[must_use]
    pub fn pst_bx(mut self, value: Max16Text) -> PostalAddress27Builder {
        self.pst_bx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `room` field.
    #[must_use]
    pub fn room(mut self, value: Max70Text) -> PostalAddress27Builder {
        self.room = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pst_cd` field.
    #[must_use]
    pub fn pst_cd(mut self, value: Max16Text) -> PostalAddress27Builder {
        self.pst_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `twn_nm` field.
    #[must_use]
    pub fn twn_nm(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.twn_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `twn_lctn_nm` field.
    #[must_use]
    pub fn twn_lctn_nm(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.twn_lctn_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dstrct_nm` field.
    #[must_use]
    pub fn dstrct_nm(mut self, value: Max140Text) -> PostalAddress27Builder {
        self.dstrct_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry_sub_dvsn` field.
    #[must_use]
    pub fn ctry_sub_dvsn(mut self, value: Max35Text) -> PostalAddress27Builder {
        self.ctry_sub_dvsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry` field.
    #[must_use]
    pub fn ctry(mut self, value: CountryCode) -> PostalAddress27Builder {
        self.ctry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `adr_line` field (replaces any previously added items).
    #[must_use]
    pub fn adr_line(mut self, value: ::std::vec::Vec<Max70Text>) -> PostalAddress27Builder {
        self.adr_line = value;
        self
    }
    /// Append one item to the `adr_line` field.
    #[must_use]
    pub fn add_adr_line(mut self, value: Max70Text) -> PostalAddress27Builder {
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
    pub fn build(self) -> ::std::result::Result<PostalAddress27, crate::common::BuilderError> {
        ::std::result::Result::Ok(PostalAddress27 {
            adr_tp: self.adr_tp,
            care_of: self.care_of,
            dept: self.dept,
            sub_dept: self.sub_dept,
            strt_nm: self.strt_nm,
            bldg_nb: self.bldg_nb,
            bldg_nm: self.bldg_nm,
            flr: self.flr,
            unit_nb: self.unit_nb,
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
impl PostalAddress27 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PostalAddress27Builder {
        PostalAddress27Builder::default()
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
pub struct ReferredDocumentInformation8 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<DocumentType1>,
    #[serde(rename = "Nb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb: Option<Max35Text>,
    #[serde(rename = "RltdDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dt: Option<DateAndType1>,
    #[serde(rename = "LineDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub line_dtls: Vec<DocumentLineInformation2>,
}
/// Builder for [`ReferredDocumentInformation8`]. Construct via [`ReferredDocumentInformation8::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ReferredDocumentInformation8Builder {
    tp: ::std::option::Option<DocumentType1>,
    nb: ::std::option::Option<Max35Text>,
    rltd_dt: ::std::option::Option<DateAndType1>,
    line_dtls: ::std::vec::Vec<DocumentLineInformation2>,
}
impl ReferredDocumentInformation8Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: DocumentType1) -> ReferredDocumentInformation8Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb` field.
    #[must_use]
    pub fn nb(mut self, value: Max35Text) -> ReferredDocumentInformation8Builder {
        self.nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_dt` field.
    #[must_use]
    pub fn rltd_dt(mut self, value: DateAndType1) -> ReferredDocumentInformation8Builder {
        self.rltd_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `line_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn line_dtls(
        mut self,
        value: ::std::vec::Vec<DocumentLineInformation2>,
    ) -> ReferredDocumentInformation8Builder {
        self.line_dtls = value;
        self
    }
    /// Append one item to the `line_dtls` field.
    #[must_use]
    pub fn add_line_dtls(
        mut self,
        value: DocumentLineInformation2,
    ) -> ReferredDocumentInformation8Builder {
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
    ) -> ::std::result::Result<ReferredDocumentInformation8, crate::common::BuilderError> {
        ::std::result::Result::Ok(ReferredDocumentInformation8 {
            tp: self.tp,
            nb: self.nb,
            rltd_dt: self.rltd_dt,
            line_dtls: self.line_dtls,
        })
    }
}
impl ReferredDocumentInformation8 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ReferredDocumentInformation8Builder {
        ReferredDocumentInformation8Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryAuthority2 {
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max140Text>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
}
/// Builder for [`RegulatoryAuthority2`]. Construct via [`RegulatoryAuthority2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RegulatoryAuthority2Builder {
    nm: ::std::option::Option<Max140Text>,
    ctry: ::std::option::Option<CountryCode>,
}
impl RegulatoryAuthority2Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> RegulatoryAuthority2Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry` field.
    #[must_use]
    pub fn ctry(mut self, value: CountryCode) -> RegulatoryAuthority2Builder {
        self.ctry = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<RegulatoryAuthority2, crate::common::BuilderError> {
        ::std::result::Result::Ok(RegulatoryAuthority2 {
            nm: self.nm,
            ctry: self.ctry,
        })
    }
}
impl RegulatoryAuthority2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RegulatoryAuthority2Builder {
        RegulatoryAuthority2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RegulatoryReporting3 {
    #[serde(rename = "DbtCdtRptgInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_cdt_rptg_ind: Option<RegulatoryReportingType1Code>,
    #[serde(rename = "Authrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authrty: Option<RegulatoryAuthority2>,
    #[serde(rename = "Dtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub dtls: Vec<StructuredRegulatoryReporting3>,
}
/// Builder for [`RegulatoryReporting3`]. Construct via [`RegulatoryReporting3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RegulatoryReporting3Builder {
    dbt_cdt_rptg_ind: ::std::option::Option<RegulatoryReportingType1Code>,
    authrty: ::std::option::Option<RegulatoryAuthority2>,
    dtls: ::std::vec::Vec<StructuredRegulatoryReporting3>,
}
impl RegulatoryReporting3Builder {
    /// Set the `dbt_cdt_rptg_ind` field.
    #[must_use]
    pub fn dbt_cdt_rptg_ind(
        mut self,
        value: RegulatoryReportingType1Code,
    ) -> RegulatoryReporting3Builder {
        self.dbt_cdt_rptg_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authrty` field.
    #[must_use]
    pub fn authrty(mut self, value: RegulatoryAuthority2) -> RegulatoryReporting3Builder {
        self.authrty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dtls` field (replaces any previously added items).
    #[must_use]
    pub fn dtls(
        mut self,
        value: ::std::vec::Vec<StructuredRegulatoryReporting3>,
    ) -> RegulatoryReporting3Builder {
        self.dtls = value;
        self
    }
    /// Append one item to the `dtls` field.
    #[must_use]
    pub fn add_dtls(
        mut self,
        value: StructuredRegulatoryReporting3,
    ) -> RegulatoryReporting3Builder {
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
    pub fn build(self) -> ::std::result::Result<RegulatoryReporting3, crate::common::BuilderError> {
        ::std::result::Result::Ok(RegulatoryReporting3 {
            dbt_cdt_rptg_ind: self.dbt_cdt_rptg_ind,
            authrty: self.authrty,
            dtls: self.dtls,
        })
    }
}
impl RegulatoryReporting3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RegulatoryReporting3Builder {
        RegulatoryReporting3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceAmount4 {
    #[serde(rename = "RmtAmtAndTp")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rmt_amt_and_tp: Vec<DocumentAmount1>,
    #[serde(rename = "AdjstmntAmtAndRsn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub adjstmnt_amt_and_rsn: Vec<DocumentAdjustment1>,
}
/// Builder for [`RemittanceAmount4`]. Construct via [`RemittanceAmount4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceAmount4Builder {
    rmt_amt_and_tp: ::std::vec::Vec<DocumentAmount1>,
    adjstmnt_amt_and_rsn: ::std::vec::Vec<DocumentAdjustment1>,
}
impl RemittanceAmount4Builder {
    /// Set the `rmt_amt_and_tp` field (replaces any previously added items).
    #[must_use]
    pub fn rmt_amt_and_tp(
        mut self,
        value: ::std::vec::Vec<DocumentAmount1>,
    ) -> RemittanceAmount4Builder {
        self.rmt_amt_and_tp = value;
        self
    }
    /// Append one item to the `rmt_amt_and_tp` field.
    #[must_use]
    pub fn add_rmt_amt_and_tp(mut self, value: DocumentAmount1) -> RemittanceAmount4Builder {
        self.rmt_amt_and_tp.push(value);
        self
    }
    /// Set the `adjstmnt_amt_and_rsn` field (replaces any previously added items).
    #[must_use]
    pub fn adjstmnt_amt_and_rsn(
        mut self,
        value: ::std::vec::Vec<DocumentAdjustment1>,
    ) -> RemittanceAmount4Builder {
        self.adjstmnt_amt_and_rsn = value;
        self
    }
    /// Append one item to the `adjstmnt_amt_and_rsn` field.
    #[must_use]
    pub fn add_adjstmnt_amt_and_rsn(
        mut self,
        value: DocumentAdjustment1,
    ) -> RemittanceAmount4Builder {
        self.adjstmnt_amt_and_rsn.push(value);
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
    pub fn build(self) -> ::std::result::Result<RemittanceAmount4, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceAmount4 {
            rmt_amt_and_tp: self.rmt_amt_and_tp,
            adjstmnt_amt_and_rsn: self.adjstmnt_amt_and_rsn,
        })
    }
}
impl RemittanceAmount4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceAmount4Builder {
        RemittanceAmount4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceInformation22 {
    #[serde(rename = "Ustrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ustrd: Vec<Max140Text>,
    #[serde(rename = "Strd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub strd: Vec<StructuredRemittanceInformation18>,
}
/// Builder for [`RemittanceInformation22`]. Construct via [`RemittanceInformation22::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceInformation22Builder {
    ustrd: ::std::vec::Vec<Max140Text>,
    strd: ::std::vec::Vec<StructuredRemittanceInformation18>,
}
impl RemittanceInformation22Builder {
    /// Set the `ustrd` field (replaces any previously added items).
    #[must_use]
    pub fn ustrd(mut self, value: ::std::vec::Vec<Max140Text>) -> RemittanceInformation22Builder {
        self.ustrd = value;
        self
    }
    /// Append one item to the `ustrd` field.
    #[must_use]
    pub fn add_ustrd(mut self, value: Max140Text) -> RemittanceInformation22Builder {
        self.ustrd.push(value);
        self
    }
    /// Set the `strd` field (replaces any previously added items).
    #[must_use]
    pub fn strd(
        mut self,
        value: ::std::vec::Vec<StructuredRemittanceInformation18>,
    ) -> RemittanceInformation22Builder {
        self.strd = value;
        self
    }
    /// Append one item to the `strd` field.
    #[must_use]
    pub fn add_strd(
        mut self,
        value: StructuredRemittanceInformation18,
    ) -> RemittanceInformation22Builder {
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
    ) -> ::std::result::Result<RemittanceInformation22, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceInformation22 {
            ustrd: self.ustrd,
            strd: self.strd,
        })
    }
}
impl RemittanceInformation22 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceInformation22Builder {
        RemittanceInformation22Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceLocation8 {
    #[serde(rename = "RmtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_id: Option<Max35Text>,
    #[serde(rename = "RmtLctnDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rmt_lctn_dtls: Vec<RemittanceLocationData2>,
}
/// Builder for [`RemittanceLocation8`]. Construct via [`RemittanceLocation8::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceLocation8Builder {
    rmt_id: ::std::option::Option<Max35Text>,
    rmt_lctn_dtls: ::std::vec::Vec<RemittanceLocationData2>,
}
impl RemittanceLocation8Builder {
    /// Set the `rmt_id` field.
    #[must_use]
    pub fn rmt_id(mut self, value: Max35Text) -> RemittanceLocation8Builder {
        self.rmt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmt_lctn_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn rmt_lctn_dtls(
        mut self,
        value: ::std::vec::Vec<RemittanceLocationData2>,
    ) -> RemittanceLocation8Builder {
        self.rmt_lctn_dtls = value;
        self
    }
    /// Append one item to the `rmt_lctn_dtls` field.
    #[must_use]
    pub fn add_rmt_lctn_dtls(
        mut self,
        value: RemittanceLocationData2,
    ) -> RemittanceLocation8Builder {
        self.rmt_lctn_dtls.push(value);
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
    pub fn build(self) -> ::std::result::Result<RemittanceLocation8, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceLocation8 {
            rmt_id: self.rmt_id,
            rmt_lctn_dtls: self.rmt_lctn_dtls,
        })
    }
}
impl RemittanceLocation8 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceLocation8Builder {
        RemittanceLocation8Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceLocationData2 {
    #[serde(rename = "Mtd")]
    pub mtd: RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr: Option<Max2048Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<NameAndAddress18>,
}
/// Builder for [`RemittanceLocationData2`]. Construct via [`RemittanceLocationData2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceLocationData2Builder {
    mtd: ::std::option::Option<RemittanceLocationMethod2Code>,
    elctrnc_adr: ::std::option::Option<Max2048Text>,
    pstl_adr: ::std::option::Option<NameAndAddress18>,
}
impl RemittanceLocationData2Builder {
    /// Set the `mtd` field.
    #[must_use]
    pub fn mtd(mut self, value: RemittanceLocationMethod2Code) -> RemittanceLocationData2Builder {
        self.mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_adr` field.
    #[must_use]
    pub fn elctrnc_adr(mut self, value: Max2048Text) -> RemittanceLocationData2Builder {
        self.elctrnc_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: NameAndAddress18) -> RemittanceLocationData2Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<RemittanceLocationData2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.mtd.is_none() {
            missing.push("mtd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "RemittanceLocationData2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(RemittanceLocationData2 {
            mtd: self.mtd.unwrap(),
            elctrnc_adr: self.elctrnc_adr,
            pstl_adr: self.pstl_adr,
        })
    }
}
impl RemittanceLocationData2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceLocationData2Builder {
        RemittanceLocationData2Builder::default()
    }
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
pub struct SettlementInstruction15 {
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
    pub instg_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "InstgRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_rmbrsmnt_agt_acct: Option<CashAccount40>,
    #[serde(rename = "InstdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "InstdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_rmbrsmnt_agt_acct: Option<CashAccount40>,
    #[serde(rename = "ThrdRmbrsmntAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt: Option<BranchAndFinancialInstitutionIdentification8>,
    #[serde(rename = "ThrdRmbrsmntAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thrd_rmbrsmnt_agt_acct: Option<CashAccount40>,
}
/// Builder for [`SettlementInstruction15`]. Construct via [`SettlementInstruction15::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SettlementInstruction15Builder {
    sttlm_mtd: ::std::option::Option<SettlementMethod1Code>,
    sttlm_acct: ::std::option::Option<CashAccount40>,
    clr_sys:
        ::std::option::Option<crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>>,
    instg_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    instg_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    instd_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    instd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    thrd_rmbrsmnt_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification8>,
    thrd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
}
impl SettlementInstruction15Builder {
    /// Set the `sttlm_mtd` field.
    #[must_use]
    pub fn sttlm_mtd(mut self, value: SettlementMethod1Code) -> SettlementInstruction15Builder {
        self.sttlm_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_acct` field.
    #[must_use]
    pub fn sttlm_acct(mut self, value: CashAccount40) -> SettlementInstruction15Builder {
        self.sttlm_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys` field.
    #[must_use]
    pub fn clr_sys(
        mut self,
        value: crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>,
    ) -> SettlementInstruction15Builder {
        self.clr_sys = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_rmbrsmnt_agt` field.
    #[must_use]
    pub fn instg_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> SettlementInstruction15Builder {
        self.instg_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn instg_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction15Builder {
        self.instg_rmbrsmnt_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_rmbrsmnt_agt` field.
    #[must_use]
    pub fn instd_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> SettlementInstruction15Builder {
        self.instd_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn instd_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction15Builder {
        self.instd_rmbrsmnt_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `thrd_rmbrsmnt_agt` field.
    #[must_use]
    pub fn thrd_rmbrsmnt_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification8,
    ) -> SettlementInstruction15Builder {
        self.thrd_rmbrsmnt_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `thrd_rmbrsmnt_agt_acct` field.
    #[must_use]
    pub fn thrd_rmbrsmnt_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> SettlementInstruction15Builder {
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
    ) -> ::std::result::Result<SettlementInstruction15, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.sttlm_mtd.is_none() {
            missing.push("sttlm_mtd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "SettlementInstruction15".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(SettlementInstruction15 {
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
impl SettlementInstruction15 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SettlementInstruction15Builder {
        SettlementInstruction15Builder::default()
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
pub struct StructuredRegulatoryReporting3 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<Max35Text>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<ISODate>,
    #[serde(rename = "Ctry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctry: Option<CountryCode>,
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<Max10Text>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Inf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub inf: Vec<Max35Text>,
}
/// Builder for [`StructuredRegulatoryReporting3`]. Construct via [`StructuredRegulatoryReporting3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct StructuredRegulatoryReporting3Builder {
    tp: ::std::option::Option<Max35Text>,
    dt: ::std::option::Option<ISODate>,
    ctry: ::std::option::Option<CountryCode>,
    cd: ::std::option::Option<Max10Text>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    inf: ::std::vec::Vec<Max35Text>,
}
impl StructuredRegulatoryReporting3Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> StructuredRegulatoryReporting3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(mut self, value: ISODate) -> StructuredRegulatoryReporting3Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctry` field.
    #[must_use]
    pub fn ctry(mut self, value: CountryCode) -> StructuredRegulatoryReporting3Builder {
        self.ctry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(mut self, value: Max10Text) -> StructuredRegulatoryReporting3Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> StructuredRegulatoryReporting3Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `inf` field (replaces any previously added items).
    #[must_use]
    pub fn inf(
        mut self,
        value: ::std::vec::Vec<Max35Text>,
    ) -> StructuredRegulatoryReporting3Builder {
        self.inf = value;
        self
    }
    /// Append one item to the `inf` field.
    #[must_use]
    pub fn add_inf(mut self, value: Max35Text) -> StructuredRegulatoryReporting3Builder {
        self.inf.push(value);
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
    ) -> ::std::result::Result<StructuredRegulatoryReporting3, crate::common::BuilderError> {
        ::std::result::Result::Ok(StructuredRegulatoryReporting3 {
            tp: self.tp,
            dt: self.dt,
            ctry: self.ctry,
            cd: self.cd,
            amt: self.amt,
            inf: self.inf,
        })
    }
}
impl StructuredRegulatoryReporting3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> StructuredRegulatoryReporting3Builder {
        StructuredRegulatoryReporting3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StructuredRemittanceInformation18 {
    #[serde(rename = "RfrdDocInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rfrd_doc_inf: Vec<ReferredDocumentInformation8>,
    #[serde(rename = "RfrdDocAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rfrd_doc_amt: Option<RemittanceAmount4>,
    #[serde(rename = "CdtrRefInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_ref_inf: Option<CreditorReferenceInformation3>,
    #[serde(rename = "Invcr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcr: Option<PartyIdentification272>,
    #[serde(rename = "Invcee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invcee: Option<PartyIdentification272>,
    #[serde(rename = "TaxRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rmt: Option<TaxData1>,
    #[serde(rename = "GrnshmtRmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grnshmt_rmt: Option<Garnishment4>,
    #[serde(rename = "AddtlRmtInf")]
    /// Maximum 3 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addtl_rmt_inf: Vec<Max140Text>,
}
/// Builder for [`StructuredRemittanceInformation18`]. Construct via [`StructuredRemittanceInformation18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct StructuredRemittanceInformation18Builder {
    rfrd_doc_inf: ::std::vec::Vec<ReferredDocumentInformation8>,
    rfrd_doc_amt: ::std::option::Option<RemittanceAmount4>,
    cdtr_ref_inf: ::std::option::Option<CreditorReferenceInformation3>,
    invcr: ::std::option::Option<PartyIdentification272>,
    invcee: ::std::option::Option<PartyIdentification272>,
    tax_rmt: ::std::option::Option<TaxData1>,
    grnshmt_rmt: ::std::option::Option<Garnishment4>,
    addtl_rmt_inf: ::std::vec::Vec<Max140Text>,
}
impl StructuredRemittanceInformation18Builder {
    /// Set the `rfrd_doc_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rfrd_doc_inf(
        mut self,
        value: ::std::vec::Vec<ReferredDocumentInformation8>,
    ) -> StructuredRemittanceInformation18Builder {
        self.rfrd_doc_inf = value;
        self
    }
    /// Append one item to the `rfrd_doc_inf` field.
    #[must_use]
    pub fn add_rfrd_doc_inf(
        mut self,
        value: ReferredDocumentInformation8,
    ) -> StructuredRemittanceInformation18Builder {
        self.rfrd_doc_inf.push(value);
        self
    }
    /// Set the `rfrd_doc_amt` field.
    #[must_use]
    pub fn rfrd_doc_amt(
        mut self,
        value: RemittanceAmount4,
    ) -> StructuredRemittanceInformation18Builder {
        self.rfrd_doc_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_ref_inf` field.
    #[must_use]
    pub fn cdtr_ref_inf(
        mut self,
        value: CreditorReferenceInformation3,
    ) -> StructuredRemittanceInformation18Builder {
        self.cdtr_ref_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `invcr` field.
    #[must_use]
    pub fn invcr(
        mut self,
        value: PartyIdentification272,
    ) -> StructuredRemittanceInformation18Builder {
        self.invcr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `invcee` field.
    #[must_use]
    pub fn invcee(
        mut self,
        value: PartyIdentification272,
    ) -> StructuredRemittanceInformation18Builder {
        self.invcee = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_rmt` field.
    #[must_use]
    pub fn tax_rmt(mut self, value: TaxData1) -> StructuredRemittanceInformation18Builder {
        self.tax_rmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshmt_rmt` field.
    #[must_use]
    pub fn grnshmt_rmt(mut self, value: Garnishment4) -> StructuredRemittanceInformation18Builder {
        self.grnshmt_rmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_rmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn addtl_rmt_inf(
        mut self,
        value: ::std::vec::Vec<Max140Text>,
    ) -> StructuredRemittanceInformation18Builder {
        self.addtl_rmt_inf = value;
        self
    }
    /// Append one item to the `addtl_rmt_inf` field.
    #[must_use]
    pub fn add_addtl_rmt_inf(
        mut self,
        value: Max140Text,
    ) -> StructuredRemittanceInformation18Builder {
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
    ) -> ::std::result::Result<StructuredRemittanceInformation18, crate::common::BuilderError> {
        ::std::result::Result::Ok(StructuredRemittanceInformation18 {
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
impl StructuredRemittanceInformation18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> StructuredRemittanceInformation18Builder {
        StructuredRemittanceInformation18Builder::default()
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
impl crate::common::validate::Validatable for ActiveCurrencyAndAmountSimpleType {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 5usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 5", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 18usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ActiveCurrencyCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 5usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 5", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 18usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AddressType2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AnyBICDec2014Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for BICFIDec2014Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for BaseOneRate {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 10usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 10", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 11usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 11", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for BatchBookingIndicator {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ChargeBearerType1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ClearingChannel2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CountryCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CreditDebitCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for DecimalNumber {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 17usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 17", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 18usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Exact2NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Exact4AlphaNumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ExternalAccountIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalCashAccountType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalCashClearingSystem1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 3usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 3", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalCategoryPurpose1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalChargeType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalClearingSystemIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 5usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 5", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalCreditorAgentInstruction1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalCreditorReferenceType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalDateType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalDocumentAmountType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalDocumentLineType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalDocumentType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalFinancialInstitutionIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalGarnishmentType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalLocalInstrument1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 35usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 35", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalMandateSetupReason1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalOrganisationIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalPersonIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalProxyAccountType1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalPurpose1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalServiceLevel1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Frequency6Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for HexBinaryText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for IBAN2007Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISODate {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISODateTime {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISOTime {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISOYear {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Instruction4Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for LEIIdentifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for MandateClassification1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Max10KBinary {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 10240usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 10240", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max10Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 10usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 10", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max128Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 128usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 128", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max140Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 140usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 140", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max15NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Max16Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 16usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 16", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max2048Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 2048usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 2048", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max256Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 256usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 256", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max34Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 34usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 34", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max350Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 350usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 350", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max35Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 35usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 35", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max4Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 4usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 4", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max70Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let len = self.0.chars().count();
            if len < 1usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value is shorter than minimum length 1", len),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let len = self.0.chars().count();
            if len > 70usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 70", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for NamePrefix2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Number {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 0usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 0", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 18usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for PercentageRate {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let frac_count = self.0.find('.').map_or(0, |dot| {
                self.0[dot + 1..]
                    .chars()
                    .filter(char::is_ascii_digit)
                    .count()
            });
            if frac_count > 10usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 10", frac_count
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let digit_count = self.0.chars().filter(char::is_ascii_digit).count();
            if digit_count > 11usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 11", digit_count
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for PhoneNumber {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for PreferredContactMethod2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Priority2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Priority3Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for RegulatoryReportingType1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for RemittanceLocationMethod2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for SHA256SignatureText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for SettlementMethod1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for TaxRecordPeriod1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for TrueFalseIndicator {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for UUIDv4Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AccountIdentification4Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::IBAN(inner) => {
                inner.validate_constraints(&format!("{path}/IBAN"), violations);
            }
            Self::Othr(inner) => {
                inner.validate_constraints(&format!("{path}/Othr"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for AccountSchemeName1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ActiveCurrencyAndAmount {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.value.validate_constraints(path, violations);
        self.ccy
            .validate_constraints(&format!("{path}/@Ccy"), violations);
    }
}
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAndAmount {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.value.validate_constraints(path, violations);
        self.ccy
            .validate_constraints(&format!("{path}/@Ccy"), violations);
    }
}
impl crate::common::validate::Validatable for AdditionalDateTime1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.accptnc_dt_tm {
            val.validate_constraints(&format!("{path}/AccptncDtTm"), violations);
        }
        if let Some(ref val) = self.poolg_adjstmnt_dt {
            val.validate_constraints(&format!("{path}/PoolgAdjstmntDt"), violations);
        }
        if let Some(ref val) = self.xpry_dt_tm {
            val.validate_constraints(&format!("{path}/XpryDtTm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AddressType3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for BranchAndFinancialInstitutionIdentification8 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fin_instn_id
            .validate_constraints(&format!("{path}/FinInstnId"), violations);
        if let Some(ref val) = self.brnch_id {
            val.validate_constraints(&format!("{path}/BrnchId"), violations);
        }
    }
}
impl crate::common::validate::Validatable for BranchData5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.id {
            val.validate_constraints(&format!("{path}/Id"), violations);
        }
        if let Some(ref val) = self.lei {
            val.validate_constraints(&format!("{path}/LEI"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate_constraints(&format!("{path}/PstlAdr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CashAccount40 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.id {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Id"), violations);
        }
        if let Some(ref wrapper) = self.tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.ccy {
            val.validate_constraints(&format!("{path}/Ccy"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.prxy {
            val.validate_constraints(&format!("{path}/Prxy"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CashAccountType2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for CategoryPurpose1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ChargeType3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for Charges16 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.agt
            .validate_constraints(&format!("{path}/Agt"), violations);
        if let Some(ref wrapper) = self.tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tp"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ClearingSystemIdentification2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ClearingSystemIdentification3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ClearingSystemMemberIdentification2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.clr_sys_id {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/ClrSysId"), violations);
        }
        self.mmb_id
            .validate_constraints(&format!("{path}/MmbId"), violations);
    }
}
impl crate::common::validate::Validatable for Contact13 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nm_prfx {
            val.validate_constraints(&format!("{path}/NmPrfx"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.phne_nb {
            val.validate_constraints(&format!("{path}/PhneNb"), violations);
        }
        if let Some(ref val) = self.mob_nb {
            val.validate_constraints(&format!("{path}/MobNb"), violations);
        }
        if let Some(ref val) = self.fax_nb {
            val.validate_constraints(&format!("{path}/FaxNb"), violations);
        }
        if let Some(ref val) = self.url_adr {
            val.validate_constraints(&format!("{path}/URLAdr"), violations);
        }
        if let Some(ref val) = self.email_adr {
            val.validate_constraints(&format!("{path}/EmailAdr"), violations);
        }
        if let Some(ref val) = self.email_purp {
            val.validate_constraints(&format!("{path}/EmailPurp"), violations);
        }
        if let Some(ref val) = self.job_titl {
            val.validate_constraints(&format!("{path}/JobTitl"), violations);
        }
        if let Some(ref val) = self.rspnsblty {
            val.validate_constraints(&format!("{path}/Rspnsblty"), violations);
        }
        if let Some(ref val) = self.dept {
            val.validate_constraints(&format!("{path}/Dept"), violations);
        }
        for (i, item) in self.othr.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Othr[{i}]"), violations);
        }
        if let Some(ref val) = self.prefrd_mtd {
            val.validate_constraints(&format!("{path}/PrefrdMtd"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CreditTransferMandateData1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.mndt_id {
            val.validate_constraints(&format!("{path}/MndtId"), violations);
        }
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.dt_of_sgntr {
            val.validate_constraints(&format!("{path}/DtOfSgntr"), violations);
        }
        if let Some(ref val) = self.dt_of_vrfctn {
            val.validate_constraints(&format!("{path}/DtOfVrfctn"), violations);
        }
        if let Some(ref val) = self.elctrnc_sgntr {
            val.validate_constraints(&format!("{path}/ElctrncSgntr"), violations);
        }
        if let Some(ref val) = self.frst_pmt_dt {
            val.validate_constraints(&format!("{path}/FrstPmtDt"), violations);
        }
        if let Some(ref val) = self.fnl_pmt_dt {
            val.validate_constraints(&format!("{path}/FnlPmtDt"), violations);
        }
        if let Some(ref wrapper) = self.frqcy {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Frqcy"), violations);
        }
        if let Some(ref wrapper) = self.rsn {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Rsn"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CreditTransferTransaction70 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.pmt_id
            .validate_constraints(&format!("{path}/PmtId"), violations);
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate_constraints(&format!("{path}/PmtTpInf"), violations);
        }
        self.intr_bk_sttlm_amt
            .validate_constraints(&format!("{path}/IntrBkSttlmAmt"), violations);
        if let Some(ref val) = self.intr_bk_sttlm_dt {
            val.validate_constraints(&format!("{path}/IntrBkSttlmDt"), violations);
        }
        if let Some(ref val) = self.sttlm_prty {
            val.validate_constraints(&format!("{path}/SttlmPrty"), violations);
        }
        if let Some(ref val) = self.sttlm_tm_indctn {
            val.validate_constraints(&format!("{path}/SttlmTmIndctn"), violations);
        }
        if let Some(ref val) = self.sttlm_tm_req {
            val.validate_constraints(&format!("{path}/SttlmTmReq"), violations);
        }
        if let Some(ref val) = self.addtl_dt_tm {
            val.validate_constraints(&format!("{path}/AddtlDtTm"), violations);
        }
        if let Some(ref val) = self.instd_amt {
            val.validate_constraints(&format!("{path}/InstdAmt"), violations);
        }
        if let Some(ref val) = self.xchg_rate {
            val.validate_constraints(&format!("{path}/XchgRate"), violations);
        }
        if let Some(ref val) = self.agrd_rate {
            val.validate_constraints(&format!("{path}/AgrdRate"), violations);
        }
        self.chrg_br
            .validate_constraints(&format!("{path}/ChrgBr"), violations);
        for (i, item) in self.chrgs_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/ChrgsInf[{i}]"), violations);
        }
        if let Some(ref val) = self.mndt_rltd_inf {
            val.validate_constraints(&format!("{path}/MndtRltdInf"), violations);
        }
        if let Some(ref wrapper) = self.pmt_sgntr {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/PmtSgntr"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt1 {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt1"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt1acct {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt1Acct"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt2 {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt2"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt2acct {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt2Acct"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt3 {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt3"), violations);
        }
        if let Some(ref val) = self.prvs_instg_agt3acct {
            val.validate_constraints(&format!("{path}/PrvsInstgAgt3Acct"), violations);
        }
        if let Some(ref val) = self.instg_agt {
            val.validate_constraints(&format!("{path}/InstgAgt"), violations);
        }
        if let Some(ref val) = self.instd_agt {
            val.validate_constraints(&format!("{path}/InstdAgt"), violations);
        }
        if let Some(ref val) = self.intrmy_agt1 {
            val.validate_constraints(&format!("{path}/IntrmyAgt1"), violations);
        }
        if let Some(ref val) = self.intrmy_agt1acct {
            val.validate_constraints(&format!("{path}/IntrmyAgt1Acct"), violations);
        }
        if let Some(ref val) = self.intrmy_agt2 {
            val.validate_constraints(&format!("{path}/IntrmyAgt2"), violations);
        }
        if let Some(ref val) = self.intrmy_agt2acct {
            val.validate_constraints(&format!("{path}/IntrmyAgt2Acct"), violations);
        }
        if let Some(ref val) = self.intrmy_agt3 {
            val.validate_constraints(&format!("{path}/IntrmyAgt3"), violations);
        }
        if let Some(ref val) = self.intrmy_agt3acct {
            val.validate_constraints(&format!("{path}/IntrmyAgt3Acct"), violations);
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate_constraints(&format!("{path}/UltmtDbtr"), violations);
        }
        if let Some(ref val) = self.initg_pty {
            val.validate_constraints(&format!("{path}/InitgPty"), violations);
        }
        self.dbtr
            .validate_constraints(&format!("{path}/Dbtr"), violations);
        if let Some(ref val) = self.dbtr_acct {
            val.validate_constraints(&format!("{path}/DbtrAcct"), violations);
        }
        self.dbtr_agt
            .validate_constraints(&format!("{path}/DbtrAgt"), violations);
        if let Some(ref val) = self.dbtr_agt_acct {
            val.validate_constraints(&format!("{path}/DbtrAgtAcct"), violations);
        }
        self.cdtr_agt
            .validate_constraints(&format!("{path}/CdtrAgt"), violations);
        if let Some(ref val) = self.cdtr_agt_acct {
            val.validate_constraints(&format!("{path}/CdtrAgtAcct"), violations);
        }
        self.cdtr
            .validate_constraints(&format!("{path}/Cdtr"), violations);
        if let Some(ref val) = self.cdtr_acct {
            val.validate_constraints(&format!("{path}/CdtrAcct"), violations);
        }
        if let Some(ref val) = self.ultmt_cdtr {
            val.validate_constraints(&format!("{path}/UltmtCdtr"), violations);
        }
        for (i, item) in self.instr_for_cdtr_agt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/InstrForCdtrAgt[{i}]"), violations);
        }
        for (i, item) in self.instr_for_nxt_agt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/InstrForNxtAgt[{i}]"), violations);
        }
        if let Some(ref wrapper) = self.purp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Purp"), violations);
        }
        for (i, item) in self.rgltry_rptg.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RgltryRptg[{i}]"), violations);
        }
        if let Some(ref val) = self.tax {
            val.validate_constraints(&format!("{path}/Tax"), violations);
        }
        for (i, item) in self.rltd_rmt_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RltdRmtInf[{i}]"), violations);
        }
        if let Some(ref val) = self.rmt_inf {
            val.validate_constraints(&format!("{path}/RmtInf"), violations);
        }
        for (i, item) in self.splmtry_data.iter().enumerate() {
            item.validate_constraints(&format!("{path}/SplmtryData[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CreditorReferenceInformation3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.r#ref {
            val.validate_constraints(&format!("{path}/Ref"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CreditorReferenceType2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for CreditorReferenceType3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd_or_prtry
            .inner
            .validate_constraints(&format!("{path}/CdOrPrtry"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CryptographicKey1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::ILPV4(inner) => {
                inner.validate_constraints(&format!("{path}/ILPV4"), violations);
            }
            Self::Sgntr(inner) => {
                inner.validate_constraints(&format!("{path}/Sgntr"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for CurrencyExchange26 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.unit_ccy {
            val.validate_constraints(&format!("{path}/UnitCcy"), violations);
        }
        if let Some(ref val) = self.qtd_ccy {
            val.validate_constraints(&format!("{path}/QtdCcy"), violations);
        }
        self.pre_agrd_xchg_rate
            .validate_constraints(&format!("{path}/PreAgrdXchgRate"), violations);
        if let Some(ref val) = self.qtn_dt_tm {
            val.validate_constraints(&format!("{path}/QtnDtTm"), violations);
        }
        if let Some(ref val) = self.qt_id {
            val.validate_constraints(&format!("{path}/QtId"), violations);
        }
        if let Some(ref val) = self.fx_agt {
            val.validate_constraints(&format!("{path}/FXAgt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DateAndPlaceOfBirth1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.birth_dt
            .validate_constraints(&format!("{path}/BirthDt"), violations);
        if let Some(ref val) = self.prvc_of_birth {
            val.validate_constraints(&format!("{path}/PrvcOfBirth"), violations);
        }
        self.city_of_birth
            .validate_constraints(&format!("{path}/CityOfBirth"), violations);
        self.ctry_of_birth
            .validate_constraints(&format!("{path}/CtryOfBirth"), violations);
    }
}
impl crate::common::validate::Validatable for DateAndType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .inner
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.dt
            .validate_constraints(&format!("{path}/Dt"), violations);
    }
}
impl crate::common::validate::Validatable for DatePeriod2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fr_dt
            .validate_constraints(&format!("{path}/FrDt"), violations);
        self.to_dt
            .validate_constraints(&format!("{path}/ToDt"), violations);
    }
}
impl crate::common::validate::Validatable for DateType2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for Document {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fi_to_fi_cstmr_cdt_trf
            .validate_constraints(&format!("{path}/FIToFICstmrCdtTrf"), violations);
    }
}
impl crate::common::validate::Validatable for DocumentAdjustment1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        }
        if let Some(ref val) = self.rsn {
            val.validate_constraints(&format!("{path}/Rsn"), violations);
        }
        if let Some(ref val) = self.addtl_inf {
            val.validate_constraints(&format!("{path}/AddtlInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DocumentAmount1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .inner
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
    }
}
impl crate::common::validate::Validatable for DocumentAmountType1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.nb {
            val.validate_constraints(&format!("{path}/Nb"), violations);
        }
        if let Some(ref val) = self.rltd_dt {
            val.validate_constraints(&format!("{path}/RltdDt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineInformation2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (i, item) in self.id.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Id[{i}]"), violations);
        }
        if let Some(ref val) = self.desc {
            val.validate_constraints(&format!("{path}/Desc"), violations);
        }
        if let Some(ref val) = self.amt {
            val.validate_constraints(&format!("{path}/Amt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd_or_prtry
            .inner
            .validate_constraints(&format!("{path}/CdOrPrtry"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineType1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for DocumentType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd_or_prtry
            .inner
            .validate_constraints(&format!("{path}/CdOrPrtry"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DocumentType2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for FIToFICustomerCreditTransferV13 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.grp_hdr
            .validate_constraints(&format!("{path}/GrpHdr"), violations);
        for (i, item) in self.cdt_trf_tx_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/CdtTrfTxInf[{i}]"), violations);
        }
        for (i, item) in self.splmtry_data.iter().enumerate() {
            item.validate_constraints(&format!("{path}/SplmtryData[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for FinancialIdentificationSchemeName1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for FinancialInstitutionIdentification23 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.bicfi {
            val.validate_constraints(&format!("{path}/BICFI"), violations);
        }
        if let Some(ref val) = self.clr_sys_mmb_id {
            val.validate_constraints(&format!("{path}/ClrSysMmbId"), violations);
        }
        if let Some(ref val) = self.lei {
            val.validate_constraints(&format!("{path}/LEI"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate_constraints(&format!("{path}/PstlAdr"), violations);
        }
        if let Some(ref val) = self.othr {
            val.validate_constraints(&format!("{path}/Othr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for Frequency36Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Tp(inner) => {
                inner.validate_constraints(&format!("{path}/Tp"), violations);
            }
            Self::Prd(inner) => {
                inner.validate_constraints(&format!("{path}/Prd"), violations);
            }
            Self::PtInTm(inner) => {
                inner.validate_constraints(&format!("{path}/PtInTm"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for FrequencyAndMoment1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.pt_in_tm
            .validate_constraints(&format!("{path}/PtInTm"), violations);
    }
}
impl crate::common::validate::Validatable for FrequencyPeriod1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.cnt_per_prd
            .validate_constraints(&format!("{path}/CntPerPrd"), violations);
    }
}
impl crate::common::validate::Validatable for Garnishment4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        if let Some(ref val) = self.grnshee {
            val.validate_constraints(&format!("{path}/Grnshee"), violations);
        }
        if let Some(ref val) = self.grnshmt_admstr {
            val.validate_constraints(&format!("{path}/GrnshmtAdmstr"), violations);
        }
        if let Some(ref val) = self.ref_nb {
            val.validate_constraints(&format!("{path}/RefNb"), violations);
        }
        if let Some(ref val) = self.dt {
            val.validate_constraints(&format!("{path}/Dt"), violations);
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate_constraints(&format!("{path}/RmtdAmt"), violations);
        }
        if let Some(ref val) = self.fmly_mdcl_insrnc_ind {
            val.validate_constraints(&format!("{path}/FmlyMdclInsrncInd"), violations);
        }
        if let Some(ref val) = self.mplyee_termntn_ind {
            val.validate_constraints(&format!("{path}/MplyeeTermntnInd"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GarnishmentType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd_or_prtry
            .inner
            .validate_constraints(&format!("{path}/CdOrPrtry"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GarnishmentType1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for GenericAccountIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref wrapper) = self.schme_nm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/SchmeNm"), violations);
        }
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericFinancialIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref wrapper) = self.schme_nm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/SchmeNm"), violations);
        }
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericIdentification3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericIdentification30 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        self.issr
            .validate_constraints(&format!("{path}/Issr"), violations);
        if let Some(ref val) = self.schme_nm {
            val.validate_constraints(&format!("{path}/SchmeNm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericOrganisationIdentification3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref wrapper) = self.schme_nm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/SchmeNm"), violations);
        }
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericPersonIdentification2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref wrapper) = self.schme_nm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/SchmeNm"), violations);
        }
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GroupHeader131 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.msg_id
            .validate_constraints(&format!("{path}/MsgId"), violations);
        self.cre_dt_tm
            .validate_constraints(&format!("{path}/CreDtTm"), violations);
        if let Some(ref val) = self.xpry_dt_tm {
            val.validate_constraints(&format!("{path}/XpryDtTm"), violations);
        }
        if let Some(ref val) = self.btch_bookg {
            val.validate_constraints(&format!("{path}/BtchBookg"), violations);
        }
        self.nb_of_txs
            .validate_constraints(&format!("{path}/NbOfTxs"), violations);
        if let Some(ref val) = self.ctrl_sum {
            val.validate_constraints(&format!("{path}/CtrlSum"), violations);
        }
        if let Some(ref val) = self.ttl_intr_bk_sttlm_amt {
            val.validate_constraints(&format!("{path}/TtlIntrBkSttlmAmt"), violations);
        }
        if let Some(ref val) = self.intr_bk_sttlm_dt {
            val.validate_constraints(&format!("{path}/IntrBkSttlmDt"), violations);
        }
        self.sttlm_inf
            .validate_constraints(&format!("{path}/SttlmInf"), violations);
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate_constraints(&format!("{path}/PmtTpInf"), violations);
        }
        if let Some(ref val) = self.instg_agt {
            val.validate_constraints(&format!("{path}/InstgAgt"), violations);
        }
        if let Some(ref val) = self.instd_agt {
            val.validate_constraints(&format!("{path}/InstdAgt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for InstructionForCreditorAgent3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.cd {
            val.validate_constraints(&format!("{path}/Cd"), violations);
        }
        if let Some(ref val) = self.instr_inf {
            val.validate_constraints(&format!("{path}/InstrInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for InstructionForNextAgent1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.cd {
            val.validate_constraints(&format!("{path}/Cd"), violations);
        }
        if let Some(ref val) = self.instr_inf {
            val.validate_constraints(&format!("{path}/InstrInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for LocalInstrument2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for MandateClassification1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for MandateSetupReason1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for MandateTypeInformation2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.svc_lvl {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/SvcLvl"), violations);
        }
        if let Some(ref wrapper) = self.lcl_instrm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/LclInstrm"), violations);
        }
        if let Some(ref wrapper) = self.ctgy_purp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/CtgyPurp"), violations);
        }
        if let Some(ref wrapper) = self.clssfctn {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Clssfctn"), violations);
        }
    }
}
impl crate::common::validate::Validatable for NameAndAddress18 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.nm
            .validate_constraints(&format!("{path}/Nm"), violations);
        self.adr
            .validate_constraints(&format!("{path}/Adr"), violations);
    }
}
impl crate::common::validate::Validatable for OrganisationIdentification39 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.any_bic {
            val.validate_constraints(&format!("{path}/AnyBIC"), violations);
        }
        if let Some(ref val) = self.lei {
            val.validate_constraints(&format!("{path}/LEI"), violations);
        }
        for (i, item) in self.othr.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Othr[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for OrganisationIdentificationSchemeName1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for OtherContact1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.chanl_tp
            .validate_constraints(&format!("{path}/ChanlTp"), violations);
        if let Some(ref val) = self.id {
            val.validate_constraints(&format!("{path}/Id"), violations);
        }
    }
}
impl crate::common::validate::Validatable for Party52Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::OrgId(inner) => {
                inner.validate_constraints(&format!("{path}/OrgId"), violations);
            }
            Self::PrvtId(inner) => {
                inner.validate_constraints(&format!("{path}/PrvtId"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for PartyIdentification272 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate_constraints(&format!("{path}/PstlAdr"), violations);
        }
        if let Some(ref wrapper) = self.id {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Id"), violations);
        }
        if let Some(ref val) = self.ctry_of_res {
            val.validate_constraints(&format!("{path}/CtryOfRes"), violations);
        }
        if let Some(ref val) = self.ctct_dtls {
            val.validate_constraints(&format!("{path}/CtctDtls"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentIdentification13 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.instr_id {
            val.validate_constraints(&format!("{path}/InstrId"), violations);
        }
        self.end_to_end_id
            .validate_constraints(&format!("{path}/EndToEndId"), violations);
        if let Some(ref val) = self.tx_id {
            val.validate_constraints(&format!("{path}/TxId"), violations);
        }
        if let Some(ref val) = self.uetr {
            val.validate_constraints(&format!("{path}/UETR"), violations);
        }
        if let Some(ref val) = self.clr_sys_ref {
            val.validate_constraints(&format!("{path}/ClrSysRef"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentTypeInformation28 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.instr_prty {
            val.validate_constraints(&format!("{path}/InstrPrty"), violations);
        }
        if let Some(ref val) = self.clr_chanl {
            val.validate_constraints(&format!("{path}/ClrChanl"), violations);
        }
        for (i, item) in self.svc_lvl.iter().enumerate() {
            item.inner
                .validate_constraints(&format!("{path}/SvcLvl[{i}]"), violations);
        }
        if let Some(ref wrapper) = self.lcl_instrm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/LclInstrm"), violations);
        }
        if let Some(ref wrapper) = self.ctgy_purp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/CtgyPurp"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PersonIdentification18 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.dt_and_plc_of_birth {
            val.validate_constraints(&format!("{path}/DtAndPlcOfBirth"), violations);
        }
        for (i, item) in self.othr.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Othr[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PersonIdentificationSchemeName1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for PostalAddress27 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.adr_tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/AdrTp"), violations);
        }
        if let Some(ref val) = self.care_of {
            val.validate_constraints(&format!("{path}/CareOf"), violations);
        }
        if let Some(ref val) = self.dept {
            val.validate_constraints(&format!("{path}/Dept"), violations);
        }
        if let Some(ref val) = self.sub_dept {
            val.validate_constraints(&format!("{path}/SubDept"), violations);
        }
        if let Some(ref val) = self.strt_nm {
            val.validate_constraints(&format!("{path}/StrtNm"), violations);
        }
        if let Some(ref val) = self.bldg_nb {
            val.validate_constraints(&format!("{path}/BldgNb"), violations);
        }
        if let Some(ref val) = self.bldg_nm {
            val.validate_constraints(&format!("{path}/BldgNm"), violations);
        }
        if let Some(ref val) = self.flr {
            val.validate_constraints(&format!("{path}/Flr"), violations);
        }
        if let Some(ref val) = self.unit_nb {
            val.validate_constraints(&format!("{path}/UnitNb"), violations);
        }
        if let Some(ref val) = self.pst_bx {
            val.validate_constraints(&format!("{path}/PstBx"), violations);
        }
        if let Some(ref val) = self.room {
            val.validate_constraints(&format!("{path}/Room"), violations);
        }
        if let Some(ref val) = self.pst_cd {
            val.validate_constraints(&format!("{path}/PstCd"), violations);
        }
        if let Some(ref val) = self.twn_nm {
            val.validate_constraints(&format!("{path}/TwnNm"), violations);
        }
        if let Some(ref val) = self.twn_lctn_nm {
            val.validate_constraints(&format!("{path}/TwnLctnNm"), violations);
        }
        if let Some(ref val) = self.dstrct_nm {
            val.validate_constraints(&format!("{path}/DstrctNm"), violations);
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            val.validate_constraints(&format!("{path}/CtrySubDvsn"), violations);
        }
        if let Some(ref val) = self.ctry {
            val.validate_constraints(&format!("{path}/Ctry"), violations);
        }
        for (i, item) in self.adr_line.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AdrLine[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ProxyAccountIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tp"), violations);
        }
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
    }
}
impl crate::common::validate::Validatable for ProxyAccountType1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for Purpose2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ReferredDocumentInformation8 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.nb {
            val.validate_constraints(&format!("{path}/Nb"), violations);
        }
        if let Some(ref val) = self.rltd_dt {
            val.validate_constraints(&format!("{path}/RltdDt"), violations);
        }
        for (i, item) in self.line_dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/LineDtls[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RegulatoryAuthority2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
        if let Some(ref val) = self.ctry {
            val.validate_constraints(&format!("{path}/Ctry"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RegulatoryReporting3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.dbt_cdt_rptg_ind {
            val.validate_constraints(&format!("{path}/DbtCdtRptgInd"), violations);
        }
        if let Some(ref val) = self.authrty {
            val.validate_constraints(&format!("{path}/Authrty"), violations);
        }
        for (i, item) in self.dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Dtls[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceAmount4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (i, item) in self.rmt_amt_and_tp.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RmtAmtAndTp[{i}]"), violations);
        }
        for (i, item) in self.adjstmnt_amt_and_rsn.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AdjstmntAmtAndRsn[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceInformation22 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (i, item) in self.ustrd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Ustrd[{i}]"), violations);
        }
        for (i, item) in self.strd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Strd[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceLocation8 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.rmt_id {
            val.validate_constraints(&format!("{path}/RmtId"), violations);
        }
        for (i, item) in self.rmt_lctn_dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RmtLctnDtls[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceLocationData2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.mtd
            .validate_constraints(&format!("{path}/Mtd"), violations);
        if let Some(ref val) = self.elctrnc_adr {
            val.validate_constraints(&format!("{path}/ElctrncAdr"), violations);
        }
        if let Some(ref val) = self.pstl_adr {
            val.validate_constraints(&format!("{path}/PstlAdr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ServiceLevel8Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                inner.validate_constraints(&format!("{path}/Cd"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for SettlementDateTimeIndication1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.dbt_dt_tm {
            val.validate_constraints(&format!("{path}/DbtDtTm"), violations);
        }
        if let Some(ref val) = self.cdt_dt_tm {
            val.validate_constraints(&format!("{path}/CdtDtTm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for SettlementInstruction15 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.sttlm_mtd
            .validate_constraints(&format!("{path}/SttlmMtd"), violations);
        if let Some(ref val) = self.sttlm_acct {
            val.validate_constraints(&format!("{path}/SttlmAcct"), violations);
        }
        if let Some(ref wrapper) = self.clr_sys {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/ClrSys"), violations);
        }
        if let Some(ref val) = self.instg_rmbrsmnt_agt {
            val.validate_constraints(&format!("{path}/InstgRmbrsmntAgt"), violations);
        }
        if let Some(ref val) = self.instg_rmbrsmnt_agt_acct {
            val.validate_constraints(&format!("{path}/InstgRmbrsmntAgtAcct"), violations);
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt {
            val.validate_constraints(&format!("{path}/InstdRmbrsmntAgt"), violations);
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt_acct {
            val.validate_constraints(&format!("{path}/InstdRmbrsmntAgtAcct"), violations);
        }
        if let Some(ref val) = self.thrd_rmbrsmnt_agt {
            val.validate_constraints(&format!("{path}/ThrdRmbrsmntAgt"), violations);
        }
        if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct {
            val.validate_constraints(&format!("{path}/ThrdRmbrsmntAgtAcct"), violations);
        }
    }
}
impl crate::common::validate::Validatable for SettlementTimeRequest2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.cls_tm {
            val.validate_constraints(&format!("{path}/CLSTm"), violations);
        }
        if let Some(ref val) = self.till_tm {
            val.validate_constraints(&format!("{path}/TillTm"), violations);
        }
        if let Some(ref val) = self.fr_tm {
            val.validate_constraints(&format!("{path}/FrTm"), violations);
        }
        if let Some(ref val) = self.rjct_tm {
            val.validate_constraints(&format!("{path}/RjctTm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for StructuredRegulatoryReporting3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.dt {
            val.validate_constraints(&format!("{path}/Dt"), violations);
        }
        if let Some(ref val) = self.ctry {
            val.validate_constraints(&format!("{path}/Ctry"), violations);
        }
        if let Some(ref val) = self.cd {
            val.validate_constraints(&format!("{path}/Cd"), violations);
        }
        if let Some(ref val) = self.amt {
            val.validate_constraints(&format!("{path}/Amt"), violations);
        }
        for (i, item) in self.inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Inf[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for StructuredRemittanceInformation18 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (i, item) in self.rfrd_doc_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RfrdDocInf[{i}]"), violations);
        }
        if let Some(ref val) = self.rfrd_doc_amt {
            val.validate_constraints(&format!("{path}/RfrdDocAmt"), violations);
        }
        if let Some(ref val) = self.cdtr_ref_inf {
            val.validate_constraints(&format!("{path}/CdtrRefInf"), violations);
        }
        if let Some(ref val) = self.invcr {
            val.validate_constraints(&format!("{path}/Invcr"), violations);
        }
        if let Some(ref val) = self.invcee {
            val.validate_constraints(&format!("{path}/Invcee"), violations);
        }
        if let Some(ref val) = self.tax_rmt {
            val.validate_constraints(&format!("{path}/TaxRmt"), violations);
        }
        if let Some(ref val) = self.grnshmt_rmt {
            val.validate_constraints(&format!("{path}/GrnshmtRmt"), violations);
        }
        for (i, item) in self.addtl_rmt_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AddtlRmtInf[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for SupplementaryData1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.plc_and_nm {
            val.validate_constraints(&format!("{path}/PlcAndNm"), violations);
        }
        self.envlp
            .validate_constraints(&format!("{path}/Envlp"), violations);
    }
}
impl crate::common::validate::Validatable for SupplementaryDataEnvelope1 {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for TaxAmount3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.rate {
            val.validate_constraints(&format!("{path}/Rate"), violations);
        }
        if let Some(ref val) = self.taxbl_base_amt {
            val.validate_constraints(&format!("{path}/TaxblBaseAmt"), violations);
        }
        if let Some(ref val) = self.ttl_amt {
            val.validate_constraints(&format!("{path}/TtlAmt"), violations);
        }
        for (i, item) in self.dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Dtls[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxAuthorisation1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.titl {
            val.validate_constraints(&format!("{path}/Titl"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxData1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.cdtr {
            val.validate_constraints(&format!("{path}/Cdtr"), violations);
        }
        if let Some(ref val) = self.dbtr {
            val.validate_constraints(&format!("{path}/Dbtr"), violations);
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate_constraints(&format!("{path}/UltmtDbtr"), violations);
        }
        if let Some(ref val) = self.admstn_zone {
            val.validate_constraints(&format!("{path}/AdmstnZone"), violations);
        }
        if let Some(ref val) = self.ref_nb {
            val.validate_constraints(&format!("{path}/RefNb"), violations);
        }
        if let Some(ref val) = self.mtd {
            val.validate_constraints(&format!("{path}/Mtd"), violations);
        }
        if let Some(ref val) = self.ttl_taxbl_base_amt {
            val.validate_constraints(&format!("{path}/TtlTaxblBaseAmt"), violations);
        }
        if let Some(ref val) = self.ttl_tax_amt {
            val.validate_constraints(&format!("{path}/TtlTaxAmt"), violations);
        }
        if let Some(ref val) = self.dt {
            val.validate_constraints(&format!("{path}/Dt"), violations);
        }
        if let Some(ref val) = self.seq_nb {
            val.validate_constraints(&format!("{path}/SeqNb"), violations);
        }
        for (i, item) in self.rcrd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Rcrd[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxParty1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tax_id {
            val.validate_constraints(&format!("{path}/TaxId"), violations);
        }
        if let Some(ref val) = self.regn_id {
            val.validate_constraints(&format!("{path}/RegnId"), violations);
        }
        if let Some(ref val) = self.tax_tp {
            val.validate_constraints(&format!("{path}/TaxTp"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxParty2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tax_id {
            val.validate_constraints(&format!("{path}/TaxId"), violations);
        }
        if let Some(ref val) = self.regn_id {
            val.validate_constraints(&format!("{path}/RegnId"), violations);
        }
        if let Some(ref val) = self.tax_tp {
            val.validate_constraints(&format!("{path}/TaxTp"), violations);
        }
        if let Some(ref val) = self.authstn {
            val.validate_constraints(&format!("{path}/Authstn"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxPeriod3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.yr {
            val.validate_constraints(&format!("{path}/Yr"), violations);
        }
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate_constraints(&format!("{path}/FrToDt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxRecord3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.ctgy {
            val.validate_constraints(&format!("{path}/Ctgy"), violations);
        }
        if let Some(ref val) = self.ctgy_dtls {
            val.validate_constraints(&format!("{path}/CtgyDtls"), violations);
        }
        if let Some(ref val) = self.dbtr_sts {
            val.validate_constraints(&format!("{path}/DbtrSts"), violations);
        }
        if let Some(ref val) = self.cert_id {
            val.validate_constraints(&format!("{path}/CertId"), violations);
        }
        if let Some(ref val) = self.frms_cd {
            val.validate_constraints(&format!("{path}/FrmsCd"), violations);
        }
        if let Some(ref val) = self.prd {
            val.validate_constraints(&format!("{path}/Prd"), violations);
        }
        if let Some(ref val) = self.tax_amt {
            val.validate_constraints(&format!("{path}/TaxAmt"), violations);
        }
        if let Some(ref val) = self.addtl_inf {
            val.validate_constraints(&format!("{path}/AddtlInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TaxRecordDetails3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.prd {
            val.validate_constraints(&format!("{path}/Prd"), violations);
        }
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
    }
}
impl crate::common::validate::IsoMessage for Document {
    fn message_type(&self) -> &'static str {
        "pacs.008.001.13"
    }
    fn root_path(&self) -> &'static str {
        "/Document"
    }
}
