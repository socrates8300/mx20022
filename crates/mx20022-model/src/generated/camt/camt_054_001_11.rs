/*! Generated from ISO 20022 XSD schema.
Namespace: `urn:iso:std:iso:20022:tech:xsd:camt.054.001.11`*/
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
/// Fraction digits: 13
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType(pub String);
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
pub enum AttendanceContext1Code {
    #[serde(rename = "ATTD")]
    Attd,
    #[serde(rename = "SATT")]
    Satt,
    #[serde(rename = "UATT")]
    Uatt,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AuthenticationEntity1Code {
    #[serde(rename = "ICCD")]
    Iccd,
    #[serde(rename = "AGNT")]
    Agnt,
    #[serde(rename = "MERC")]
    Merc,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AuthenticationMethod1Code {
    #[serde(rename = "UKNW")]
    Uknw,
    #[serde(rename = "BYPS")]
    Byps,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "FPIN")]
    Fpin,
    #[serde(rename = "CPSG")]
    Cpsg,
    #[serde(rename = "PPSG")]
    Ppsg,
    #[serde(rename = "MANU")]
    Manu,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "SCRT")]
    Scrt,
    #[serde(rename = "SNCT")]
    Snct,
    #[serde(rename = "SCNL")]
    Scnl,
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
pub enum CSCManagement1Code {
    #[serde(rename = "PRST")]
    Prst,
    #[serde(rename = "BYPS")]
    Byps,
    #[serde(rename = "UNRD")]
    Unrd,
    #[serde(rename = "NCSC")]
    Ncsc,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CardDataReading1Code {
    #[serde(rename = "TAGC")]
    Tagc,
    #[serde(rename = "PHYS")]
    Phys,
    #[serde(rename = "BRCD")]
    Brcd,
    #[serde(rename = "MGST")]
    Mgst,
    #[serde(rename = "CICC")]
    Cicc,
    #[serde(rename = "DFLE")]
    Dfle,
    #[serde(rename = "CTLS")]
    Ctls,
    #[serde(rename = "ECTL")]
    Ectl,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CardPaymentServiceType2Code {
    #[serde(rename = "AGGR")]
    Aggr,
    #[serde(rename = "DCCV")]
    Dccv,
    #[serde(rename = "GRTT")]
    Grtt,
    #[serde(rename = "INSP")]
    Insp,
    #[serde(rename = "LOYT")]
    Loyt,
    #[serde(rename = "NRES")]
    Nres,
    #[serde(rename = "PUCO")]
    Puco,
    #[serde(rename = "RECP")]
    Recp,
    #[serde(rename = "SOAF")]
    Soaf,
    #[serde(rename = "UNAF")]
    Unaf,
    #[serde(rename = "VCAU")]
    Vcau,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CardholderVerificationCapability1Code {
    #[serde(rename = "MNSG")]
    Mnsg,
    #[serde(rename = "NPIN")]
    Npin,
    #[serde(rename = "FCPN")]
    Fcpn,
    #[serde(rename = "FEPN")]
    Fepn,
    #[serde(rename = "FDSG")]
    Fdsg,
    #[serde(rename = "FBIO")]
    Fbio,
    #[serde(rename = "MNVR")]
    Mnvr,
    #[serde(rename = "FBIG")]
    Fbig,
    #[serde(rename = "APKI")]
    Apki,
    #[serde(rename = "PKIS")]
    Pkis,
    #[serde(rename = "CHDT")]
    Chdt,
    #[serde(rename = "SCEC")]
    Scec,
}
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
#[serde(transparent)]
pub struct ChargeIncludedIndicator(pub bool);
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CopyDuplicate1Code {
    #[serde(rename = "CODU")]
    Codu,
    #[serde(rename = "COPY")]
    Copy,
    #[serde(rename = "DUPL")]
    Dupl,
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
/// Pattern: `[0-9]`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Exact1NumericText(pub String);
/// Pattern: `[0-9]{3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Exact3NumericText(pub String);
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
pub struct ExternalBankTransactionDomain1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionFamily1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalBankTransactionSubFamily1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCardTransactionCategory1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code(pub String);
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
pub struct ExternalEntryStatus1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstrumentIdentificationType1Code(pub String);
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
pub struct ExternalRePresentmentReason1Code(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalReportingSource1Code(pub String);
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
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalTechnicalInputChannel1Code(pub String);
/// Pattern: `[A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct IBAN2007Identifier(pub String);
/// Pattern: `[A-Z]{2,2}[A-Z0-9]{9,9}[0-9]{1,1}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISINOct2015Identifier(pub String);
/// Pattern: `[a-z]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISO2ALanguageCode(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODate(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODateTime(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISOYear(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISOYearMonth(pub String);
/// Fraction digits: 5
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ImpliedCurrencyAndAmount(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InterestType1Code {
    #[serde(rename = "INDY")]
    Indy,
    #[serde(rename = "OVRN")]
    Ovrn,
}
/// Pattern: `[A-Z0-9]{18,18}[0-9]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier(pub String);
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
/// Pattern: `[\+]{0,1}[0-9]{1,15}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max15PlusSignedNumericText(pub String);
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
/// Pattern: `[0-9]{1,3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max3NumericText(pub String);
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max4Text(pub String);
/// Minimum length: 1
/// Maximum length: 500
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max500Text(pub String);
/// Pattern: `[0-9]{1,5}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max5NumericText(pub String);
/// Minimum length: 1
/// Maximum length: 70
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max70Text(pub String);
/// Pattern: `[0-9]{2,3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Min2Max3NumericText(pub String);
/// Pattern: `[0-9]{3,4}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Min3Max4NumericText(pub String);
/// Pattern: `[0-9]{8,28}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Min8Max28NumericText(pub String);
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
/// Fraction digits: 17
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct NonNegativeDecimalNumber(pub String);
/// Fraction digits: 0
/// Total digits: 18
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Number(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum OnLineCapability1Code {
    #[serde(rename = "OFLN")]
    Ofln,
    #[serde(rename = "ONLN")]
    Onln,
    #[serde(rename = "SMON")]
    Smon,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum POIComponentType1Code {
    #[serde(rename = "SOFT")]
    Soft,
    #[serde(rename = "EMVK")]
    Emvk,
    #[serde(rename = "EMVO")]
    Emvo,
    #[serde(rename = "MRIT")]
    Mrit,
    #[serde(rename = "CHIT")]
    Chit,
    #[serde(rename = "SECM")]
    Secm,
    #[serde(rename = "PEDV")]
    Pedv,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PartyType3Code {
    #[serde(rename = "OPOI")]
    Opoi,
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "DLIS")]
    Dlis,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PartyType4Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "ACCP")]
    Accp,
    #[serde(rename = "ITAG")]
    Itag,
    #[serde(rename = "ACQR")]
    Acqr,
    #[serde(rename = "CISS")]
    Ciss,
    #[serde(rename = "TAXH")]
    Taxh,
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
/// Fraction digits: 0
/// Total digits: 18
/// Minimum value (inclusive): 1
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PositiveNumber(pub String);
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
pub enum PriceValueType1Code {
    #[serde(rename = "DISC")]
    Disc,
    #[serde(rename = "PREM")]
    Prem,
    #[serde(rename = "PARV")]
    Parv,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Priority2Code {
    #[serde(rename = "HIGH")]
    High,
    #[serde(rename = "NORM")]
    Norm,
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
pub enum TransactionChannel1Code {
    #[serde(rename = "MAIL")]
    Mail,
    #[serde(rename = "TLPH")]
    Tlph,
    #[serde(rename = "ECOM")]
    Ecom,
    #[serde(rename = "TVPY")]
    Tvpy,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionEnvironment1Code {
    #[serde(rename = "MERC")]
    Merc,
    #[serde(rename = "PRIV")]
    Priv,
    #[serde(rename = "PUBL")]
    Publ,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct TrueFalseIndicator(pub bool);
/// Pattern: `[a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct UUIDv4Identifier(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UnitOfMeasure1Code {
    #[serde(rename = "PIEC")]
    Piec,
    #[serde(rename = "TONS")]
    Tons,
    #[serde(rename = "FOOT")]
    Foot,
    #[serde(rename = "GBGA")]
    Gbga,
    #[serde(rename = "USGA")]
    Usga,
    #[serde(rename = "GRAM")]
    Gram,
    #[serde(rename = "INCH")]
    Inch,
    #[serde(rename = "KILO")]
    Kilo,
    #[serde(rename = "PUND")]
    Pund,
    #[serde(rename = "METR")]
    Metr,
    #[serde(rename = "CMET")]
    Cmet,
    #[serde(rename = "MMET")]
    Mmet,
    #[serde(rename = "LITR")]
    Litr,
    #[serde(rename = "CELI")]
    Celi,
    #[serde(rename = "MILI")]
    Mili,
    #[serde(rename = "GBOU")]
    Gbou,
    #[serde(rename = "USOU")]
    Usou,
    #[serde(rename = "GBQA")]
    Gbqa,
    #[serde(rename = "USQA")]
    Usqa,
    #[serde(rename = "GBPI")]
    Gbpi,
    #[serde(rename = "USPI")]
    Uspi,
    #[serde(rename = "MILE")]
    Mile,
    #[serde(rename = "KMET")]
    Kmet,
    #[serde(rename = "YARD")]
    Yard,
    #[serde(rename = "SQKI")]
    Sqki,
    #[serde(rename = "HECT")]
    Hect,
    #[serde(rename = "ARES")]
    Ares,
    #[serde(rename = "SMET")]
    Smet,
    #[serde(rename = "SCMT")]
    Scmt,
    #[serde(rename = "SMIL")]
    Smil,
    #[serde(rename = "SQMI")]
    Sqmi,
    #[serde(rename = "SQYA")]
    Sqya,
    #[serde(rename = "SQFO")]
    Sqfo,
    #[serde(rename = "SQIN")]
    Sqin,
    #[serde(rename = "ACRE")]
    Acre,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum UserInterface2Code {
    #[serde(rename = "MDSP")]
    Mdsp,
    #[serde(rename = "CDSP")]
    Cdsp,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator(pub bool);
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AccountIdentification4Choice {
    #[serde(rename = "IBAN")]
    IBAN(IBAN2007Identifier),
    #[serde(rename = "Othr")]
    Othr(GenericAccountIdentification1),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountInterest4 {
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<InterestType1Choice>>,
    #[serde(rename = "Rate")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rate: Vec<Rate4>,
    #[serde(rename = "FrToDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max35Text>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}
/// Builder for [`AccountInterest4`]. Construct via [`AccountInterest4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AccountInterest4Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<InterestType1Choice>>,
    rate: ::std::vec::Vec<Rate4>,
    fr_to_dt: ::std::option::Option<DateTimePeriod1>,
    rsn: ::std::option::Option<Max35Text>,
    tax: ::std::option::Option<TaxCharges2>,
}
impl AccountInterest4Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<InterestType1Choice>,
    ) -> AccountInterest4Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rate` field (replaces any previously added items).
    #[must_use]
    pub fn rate(mut self, value: ::std::vec::Vec<Rate4>) -> AccountInterest4Builder {
        self.rate = value;
        self
    }
    /// Append one item to the `rate` field.
    #[must_use]
    pub fn add_rate(mut self, value: Rate4) -> AccountInterest4Builder {
        self.rate.push(value);
        self
    }
    /// Set the `fr_to_dt` field.
    #[must_use]
    pub fn fr_to_dt(mut self, value: DateTimePeriod1) -> AccountInterest4Builder {
        self.fr_to_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(mut self, value: Max35Text) -> AccountInterest4Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxCharges2) -> AccountInterest4Builder {
        self.tax = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<AccountInterest4, crate::common::BuilderError> {
        ::std::result::Result::Ok(AccountInterest4 {
            tp: self.tp,
            rate: self.rate,
            fr_to_dt: self.fr_to_dt,
            rsn: self.rsn,
            tax: self.tax,
        })
    }
}
impl AccountInterest4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AccountInterest4Builder {
        AccountInterest4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountNotification21 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "NtfctnPgntn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntfctn_pgntn: Option<Pagination1>,
    #[serde(rename = "ElctrncSeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_seq_nb: Option<Number>,
    #[serde(rename = "RptgSeq")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rptg_seq: Option<crate::common::ChoiceWrapper<SequenceRange1Choice>>,
    #[serde(rename = "LglSeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lgl_seq_nb: Option<Number>,
    #[serde(rename = "CreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<ISODateTime>,
    #[serde(rename = "FrToDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "CpyDplctInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpy_dplct_ind: Option<CopyDuplicate1Code>,
    #[serde(rename = "RptgSrc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rptg_src: Option<crate::common::ChoiceWrapper<ReportingSource1Choice>>,
    #[serde(rename = "Acct")]
    pub acct: CashAccount41,
    #[serde(rename = "RltdAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_acct: Option<CashAccount40>,
    #[serde(rename = "Intrst")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub intrst: Vec<AccountInterest4>,
    #[serde(rename = "TxsSummry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub txs_summry: Option<TotalTransactions6>,
    #[serde(rename = "Ntry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ntry: Vec<ReportEntry13>,
    #[serde(rename = "AddtlNtfctnInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_ntfctn_inf: Option<Max500Text>,
}
/// Builder for [`AccountNotification21`]. Construct via [`AccountNotification21::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AccountNotification21Builder {
    id: ::std::option::Option<Max35Text>,
    ntfctn_pgntn: ::std::option::Option<Pagination1>,
    elctrnc_seq_nb: ::std::option::Option<Number>,
    rptg_seq: ::std::option::Option<crate::common::ChoiceWrapper<SequenceRange1Choice>>,
    lgl_seq_nb: ::std::option::Option<Number>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    fr_to_dt: ::std::option::Option<DateTimePeriod1>,
    cpy_dplct_ind: ::std::option::Option<CopyDuplicate1Code>,
    rptg_src: ::std::option::Option<crate::common::ChoiceWrapper<ReportingSource1Choice>>,
    acct: ::std::option::Option<CashAccount41>,
    rltd_acct: ::std::option::Option<CashAccount40>,
    intrst: ::std::vec::Vec<AccountInterest4>,
    txs_summry: ::std::option::Option<TotalTransactions6>,
    ntry: ::std::vec::Vec<ReportEntry13>,
    addtl_ntfctn_inf: ::std::option::Option<Max500Text>,
}
impl AccountNotification21Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> AccountNotification21Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ntfctn_pgntn` field.
    #[must_use]
    pub fn ntfctn_pgntn(mut self, value: Pagination1) -> AccountNotification21Builder {
        self.ntfctn_pgntn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_seq_nb` field.
    #[must_use]
    pub fn elctrnc_seq_nb(mut self, value: Number) -> AccountNotification21Builder {
        self.elctrnc_seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rptg_seq` field.
    #[must_use]
    pub fn rptg_seq(
        mut self,
        value: crate::common::ChoiceWrapper<SequenceRange1Choice>,
    ) -> AccountNotification21Builder {
        self.rptg_seq = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lgl_seq_nb` field.
    #[must_use]
    pub fn lgl_seq_nb(mut self, value: Number) -> AccountNotification21Builder {
        self.lgl_seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> AccountNotification21Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr_to_dt` field.
    #[must_use]
    pub fn fr_to_dt(mut self, value: DateTimePeriod1) -> AccountNotification21Builder {
        self.fr_to_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cpy_dplct_ind` field.
    #[must_use]
    pub fn cpy_dplct_ind(mut self, value: CopyDuplicate1Code) -> AccountNotification21Builder {
        self.cpy_dplct_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rptg_src` field.
    #[must_use]
    pub fn rptg_src(
        mut self,
        value: crate::common::ChoiceWrapper<ReportingSource1Choice>,
    ) -> AccountNotification21Builder {
        self.rptg_src = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct` field.
    #[must_use]
    pub fn acct(mut self, value: CashAccount41) -> AccountNotification21Builder {
        self.acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_acct` field.
    #[must_use]
    pub fn rltd_acct(mut self, value: CashAccount40) -> AccountNotification21Builder {
        self.rltd_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrst` field (replaces any previously added items).
    #[must_use]
    pub fn intrst(
        mut self,
        value: ::std::vec::Vec<AccountInterest4>,
    ) -> AccountNotification21Builder {
        self.intrst = value;
        self
    }
    /// Append one item to the `intrst` field.
    #[must_use]
    pub fn add_intrst(mut self, value: AccountInterest4) -> AccountNotification21Builder {
        self.intrst.push(value);
        self
    }
    /// Set the `txs_summry` field.
    #[must_use]
    pub fn txs_summry(mut self, value: TotalTransactions6) -> AccountNotification21Builder {
        self.txs_summry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ntry` field (replaces any previously added items).
    #[must_use]
    pub fn ntry(mut self, value: ::std::vec::Vec<ReportEntry13>) -> AccountNotification21Builder {
        self.ntry = value;
        self
    }
    /// Append one item to the `ntry` field.
    #[must_use]
    pub fn add_ntry(mut self, value: ReportEntry13) -> AccountNotification21Builder {
        self.ntry.push(value);
        self
    }
    /// Set the `addtl_ntfctn_inf` field.
    #[must_use]
    pub fn addtl_ntfctn_inf(mut self, value: Max500Text) -> AccountNotification21Builder {
        self.addtl_ntfctn_inf = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<AccountNotification21, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if self.acct.is_none() {
            missing.push("acct".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "AccountNotification21".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(AccountNotification21 {
            id: self.id.unwrap(),
            ntfctn_pgntn: self.ntfctn_pgntn,
            elctrnc_seq_nb: self.elctrnc_seq_nb,
            rptg_seq: self.rptg_seq,
            lgl_seq_nb: self.lgl_seq_nb,
            cre_dt_tm: self.cre_dt_tm,
            fr_to_dt: self.fr_to_dt,
            cpy_dplct_ind: self.cpy_dplct_ind,
            rptg_src: self.rptg_src,
            acct: self.acct.unwrap(),
            rltd_acct: self.rltd_acct,
            intrst: self.intrst,
            txs_summry: self.txs_summry,
            ntry: self.ntry,
            addtl_ntfctn_inf: self.addtl_ntfctn_inf,
        })
    }
}
impl AccountNotification21 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AccountNotification21Builder {
        AccountNotification21Builder::default()
    }
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
pub struct ActiveOrHistoricCurrencyAnd13DecimalAmount {
    #[serde(rename = "$value")]
    pub value: ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmount {
    #[serde(rename = "$value")]
    pub value: ActiveOrHistoricCurrencyAndAmountSimpleType,
    #[serde(rename = "@Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2 {
    #[serde(rename = "Amt")]
    pub amt: crate::common::ChoiceWrapper<ImpliedCurrencyAmountRange1Choice>,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "Ccy")]
    pub ccy: ActiveOrHistoricCurrencyCode,
}
/// Builder for [`ActiveOrHistoricCurrencyAndAmountRange2`]. Construct via [`ActiveOrHistoricCurrencyAndAmountRange2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ActiveOrHistoricCurrencyAndAmountRange2Builder {
    amt: ::std::option::Option<crate::common::ChoiceWrapper<ImpliedCurrencyAmountRange1Choice>>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
}
impl ActiveOrHistoricCurrencyAndAmountRange2Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: crate::common::ChoiceWrapper<ImpliedCurrencyAmountRange1Choice>,
    ) -> ActiveOrHistoricCurrencyAndAmountRange2Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(
        mut self,
        value: CreditDebitCode,
    ) -> ActiveOrHistoricCurrencyAndAmountRange2Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy` field.
    #[must_use]
    pub fn ccy(
        mut self,
        value: ActiveOrHistoricCurrencyCode,
    ) -> ActiveOrHistoricCurrencyAndAmountRange2Builder {
        self.ccy = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<ActiveOrHistoricCurrencyAndAmountRange2, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.ccy.is_none() {
            missing.push("ccy".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ActiveOrHistoricCurrencyAndAmountRange2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ActiveOrHistoricCurrencyAndAmountRange2 {
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind,
            ccy: self.ccy.unwrap(),
        })
    }
}
impl ActiveOrHistoricCurrencyAndAmountRange2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ActiveOrHistoricCurrencyAndAmountRange2Builder {
        ActiveOrHistoricCurrencyAndAmountRange2Builder::default()
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
pub struct AmountAndCurrencyExchange4 {
    #[serde(rename = "InstdAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_amt: Option<AmountAndCurrencyExchangeDetails5>,
    #[serde(rename = "TxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_amt: Option<AmountAndCurrencyExchangeDetails5>,
    #[serde(rename = "CntrValAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cntr_val_amt: Option<AmountAndCurrencyExchangeDetails5>,
    #[serde(rename = "AnncdPstngAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anncd_pstng_amt: Option<AmountAndCurrencyExchangeDetails5>,
    #[serde(rename = "PrtryAmt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prtry_amt: Vec<AmountAndCurrencyExchangeDetails6>,
}
/// Builder for [`AmountAndCurrencyExchange4`]. Construct via [`AmountAndCurrencyExchange4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmountAndCurrencyExchange4Builder {
    instd_amt: ::std::option::Option<AmountAndCurrencyExchangeDetails5>,
    tx_amt: ::std::option::Option<AmountAndCurrencyExchangeDetails5>,
    cntr_val_amt: ::std::option::Option<AmountAndCurrencyExchangeDetails5>,
    anncd_pstng_amt: ::std::option::Option<AmountAndCurrencyExchangeDetails5>,
    prtry_amt: ::std::vec::Vec<AmountAndCurrencyExchangeDetails6>,
}
impl AmountAndCurrencyExchange4Builder {
    /// Set the `instd_amt` field.
    #[must_use]
    pub fn instd_amt(
        mut self,
        value: AmountAndCurrencyExchangeDetails5,
    ) -> AmountAndCurrencyExchange4Builder {
        self.instd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_amt` field.
    #[must_use]
    pub fn tx_amt(
        mut self,
        value: AmountAndCurrencyExchangeDetails5,
    ) -> AmountAndCurrencyExchange4Builder {
        self.tx_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cntr_val_amt` field.
    #[must_use]
    pub fn cntr_val_amt(
        mut self,
        value: AmountAndCurrencyExchangeDetails5,
    ) -> AmountAndCurrencyExchange4Builder {
        self.cntr_val_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `anncd_pstng_amt` field.
    #[must_use]
    pub fn anncd_pstng_amt(
        mut self,
        value: AmountAndCurrencyExchangeDetails5,
    ) -> AmountAndCurrencyExchange4Builder {
        self.anncd_pstng_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry_amt` field (replaces any previously added items).
    #[must_use]
    pub fn prtry_amt(
        mut self,
        value: ::std::vec::Vec<AmountAndCurrencyExchangeDetails6>,
    ) -> AmountAndCurrencyExchange4Builder {
        self.prtry_amt = value;
        self
    }
    /// Append one item to the `prtry_amt` field.
    #[must_use]
    pub fn add_prtry_amt(
        mut self,
        value: AmountAndCurrencyExchangeDetails6,
    ) -> AmountAndCurrencyExchange4Builder {
        self.prtry_amt.push(value);
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
    ) -> ::std::result::Result<AmountAndCurrencyExchange4, crate::common::BuilderError> {
        ::std::result::Result::Ok(AmountAndCurrencyExchange4 {
            instd_amt: self.instd_amt,
            tx_amt: self.tx_amt,
            cntr_val_amt: self.cntr_val_amt,
            anncd_pstng_amt: self.anncd_pstng_amt,
            prtry_amt: self.prtry_amt,
        })
    }
}
impl AmountAndCurrencyExchange4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmountAndCurrencyExchange4Builder {
        AmountAndCurrencyExchange4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmountAndCurrencyExchangeDetails5 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange24>,
}
/// Builder for [`AmountAndCurrencyExchangeDetails5`]. Construct via [`AmountAndCurrencyExchangeDetails5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmountAndCurrencyExchangeDetails5Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ccy_xchg: ::std::option::Option<CurrencyExchange24>,
}
impl AmountAndCurrencyExchangeDetails5Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> AmountAndCurrencyExchangeDetails5Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy_xchg` field.
    #[must_use]
    pub fn ccy_xchg(
        mut self,
        value: CurrencyExchange24,
    ) -> AmountAndCurrencyExchangeDetails5Builder {
        self.ccy_xchg = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<AmountAndCurrencyExchangeDetails5, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "AmountAndCurrencyExchangeDetails5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(AmountAndCurrencyExchangeDetails5 {
            amt: self.amt.unwrap(),
            ccy_xchg: self.ccy_xchg,
        })
    }
}
impl AmountAndCurrencyExchangeDetails5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmountAndCurrencyExchangeDetails5Builder {
        AmountAndCurrencyExchangeDetails5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmountAndCurrencyExchangeDetails6 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CcyXchg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ccy_xchg: Option<CurrencyExchange24>,
}
/// Builder for [`AmountAndCurrencyExchangeDetails6`]. Construct via [`AmountAndCurrencyExchangeDetails6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmountAndCurrencyExchangeDetails6Builder {
    tp: ::std::option::Option<Max35Text>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    ccy_xchg: ::std::option::Option<CurrencyExchange24>,
}
impl AmountAndCurrencyExchangeDetails6Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> AmountAndCurrencyExchangeDetails6Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> AmountAndCurrencyExchangeDetails6Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy_xchg` field.
    #[must_use]
    pub fn ccy_xchg(
        mut self,
        value: CurrencyExchange24,
    ) -> AmountAndCurrencyExchangeDetails6Builder {
        self.ccy_xchg = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<AmountAndCurrencyExchangeDetails6, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "AmountAndCurrencyExchangeDetails6".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(AmountAndCurrencyExchangeDetails6 {
            tp: self.tp.unwrap(),
            amt: self.amt.unwrap(),
            ccy_xchg: self.ccy_xchg,
        })
    }
}
impl AmountAndCurrencyExchangeDetails6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmountAndCurrencyExchangeDetails6Builder {
        AmountAndCurrencyExchangeDetails6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmountAndDirection35 {
    #[serde(rename = "Amt")]
    pub amt: NonNegativeDecimalNumber,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
}
/// Builder for [`AmountAndDirection35`]. Construct via [`AmountAndDirection35::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmountAndDirection35Builder {
    amt: ::std::option::Option<NonNegativeDecimalNumber>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
}
impl AmountAndDirection35Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: NonNegativeDecimalNumber) -> AmountAndDirection35Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> AmountAndDirection35Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<AmountAndDirection35, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.cdt_dbt_ind.is_none() {
            missing.push("cdt_dbt_ind".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "AmountAndDirection35".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(AmountAndDirection35 {
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind.unwrap(),
        })
    }
}
impl AmountAndDirection35 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmountAndDirection35Builder {
        AmountAndDirection35Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmountRangeBoundary1 {
    #[serde(rename = "BdryAmt")]
    pub bdry_amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "Incl")]
    pub incl: YesNoIndicator,
}
/// Builder for [`AmountRangeBoundary1`]. Construct via [`AmountRangeBoundary1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AmountRangeBoundary1Builder {
    bdry_amt: ::std::option::Option<ImpliedCurrencyAndAmount>,
    incl: ::std::option::Option<YesNoIndicator>,
}
impl AmountRangeBoundary1Builder {
    /// Set the `bdry_amt` field.
    #[must_use]
    pub fn bdry_amt(mut self, value: ImpliedCurrencyAndAmount) -> AmountRangeBoundary1Builder {
        self.bdry_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `incl` field.
    #[must_use]
    pub fn incl(mut self, value: YesNoIndicator) -> AmountRangeBoundary1Builder {
        self.incl = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<AmountRangeBoundary1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.bdry_amt.is_none() {
            missing.push("bdry_amt".to_owned());
        }
        if self.incl.is_none() {
            missing.push("incl".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "AmountRangeBoundary1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(AmountRangeBoundary1 {
            bdry_amt: self.bdry_amt.unwrap(),
            incl: self.incl.unwrap(),
        })
    }
}
impl AmountRangeBoundary1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AmountRangeBoundary1Builder {
        AmountRangeBoundary1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankToCustomerDebitCreditNotificationV11 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader81,
    #[serde(rename = "Ntfctn")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ntfctn: Vec<AccountNotification21>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`BankToCustomerDebitCreditNotificationV11`]. Construct via [`BankToCustomerDebitCreditNotificationV11::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BankToCustomerDebitCreditNotificationV11Builder {
    grp_hdr: ::std::option::Option<GroupHeader81>,
    ntfctn: ::std::vec::Vec<AccountNotification21>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl BankToCustomerDebitCreditNotificationV11Builder {
    /// Set the `grp_hdr` field.
    #[must_use]
    pub fn grp_hdr(
        mut self,
        value: GroupHeader81,
    ) -> BankToCustomerDebitCreditNotificationV11Builder {
        self.grp_hdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ntfctn` field (replaces any previously added items).
    #[must_use]
    pub fn ntfctn(
        mut self,
        value: ::std::vec::Vec<AccountNotification21>,
    ) -> BankToCustomerDebitCreditNotificationV11Builder {
        self.ntfctn = value;
        self
    }
    /// Append one item to the `ntfctn` field.
    #[must_use]
    pub fn add_ntfctn(
        mut self,
        value: AccountNotification21,
    ) -> BankToCustomerDebitCreditNotificationV11Builder {
        self.ntfctn.push(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> BankToCustomerDebitCreditNotificationV11Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> BankToCustomerDebitCreditNotificationV11Builder {
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
    ) -> ::std::result::Result<BankToCustomerDebitCreditNotificationV11, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.grp_hdr.is_none() {
            missing.push("grp_hdr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BankToCustomerDebitCreditNotificationV11".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BankToCustomerDebitCreditNotificationV11 {
            grp_hdr: self.grp_hdr.unwrap(),
            ntfctn: self.ntfctn,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl BankToCustomerDebitCreditNotificationV11 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BankToCustomerDebitCreditNotificationV11Builder {
        BankToCustomerDebitCreditNotificationV11Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankTransactionCodeStructure4 {
    #[serde(rename = "Domn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domn: Option<BankTransactionCodeStructure5>,
    #[serde(rename = "Prtry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prtry: Option<ProprietaryBankTransactionCodeStructure1>,
}
/// Builder for [`BankTransactionCodeStructure4`]. Construct via [`BankTransactionCodeStructure4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BankTransactionCodeStructure4Builder {
    domn: ::std::option::Option<BankTransactionCodeStructure5>,
    prtry: ::std::option::Option<ProprietaryBankTransactionCodeStructure1>,
}
impl BankTransactionCodeStructure4Builder {
    /// Set the `domn` field.
    #[must_use]
    pub fn domn(
        mut self,
        value: BankTransactionCodeStructure5,
    ) -> BankTransactionCodeStructure4Builder {
        self.domn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry` field.
    #[must_use]
    pub fn prtry(
        mut self,
        value: ProprietaryBankTransactionCodeStructure1,
    ) -> BankTransactionCodeStructure4Builder {
        self.prtry = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<BankTransactionCodeStructure4, crate::common::BuilderError> {
        ::std::result::Result::Ok(BankTransactionCodeStructure4 {
            domn: self.domn,
            prtry: self.prtry,
        })
    }
}
impl BankTransactionCodeStructure4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BankTransactionCodeStructure4Builder {
        BankTransactionCodeStructure4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankTransactionCodeStructure5 {
    #[serde(rename = "Cd")]
    pub cd: ExternalBankTransactionDomain1Code,
    #[serde(rename = "Fmly")]
    pub fmly: BankTransactionCodeStructure6,
}
/// Builder for [`BankTransactionCodeStructure5`]. Construct via [`BankTransactionCodeStructure5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BankTransactionCodeStructure5Builder {
    cd: ::std::option::Option<ExternalBankTransactionDomain1Code>,
    fmly: ::std::option::Option<BankTransactionCodeStructure6>,
}
impl BankTransactionCodeStructure5Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(
        mut self,
        value: ExternalBankTransactionDomain1Code,
    ) -> BankTransactionCodeStructure5Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fmly` field.
    #[must_use]
    pub fn fmly(
        mut self,
        value: BankTransactionCodeStructure6,
    ) -> BankTransactionCodeStructure5Builder {
        self.fmly = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<BankTransactionCodeStructure5, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd.is_none() {
            missing.push("cd".to_owned());
        }
        if self.fmly.is_none() {
            missing.push("fmly".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BankTransactionCodeStructure5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BankTransactionCodeStructure5 {
            cd: self.cd.unwrap(),
            fmly: self.fmly.unwrap(),
        })
    }
}
impl BankTransactionCodeStructure5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BankTransactionCodeStructure5Builder {
        BankTransactionCodeStructure5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BankTransactionCodeStructure6 {
    #[serde(rename = "Cd")]
    pub cd: ExternalBankTransactionFamily1Code,
    #[serde(rename = "SubFmlyCd")]
    pub sub_fmly_cd: ExternalBankTransactionSubFamily1Code,
}
/// Builder for [`BankTransactionCodeStructure6`]. Construct via [`BankTransactionCodeStructure6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BankTransactionCodeStructure6Builder {
    cd: ::std::option::Option<ExternalBankTransactionFamily1Code>,
    sub_fmly_cd: ::std::option::Option<ExternalBankTransactionSubFamily1Code>,
}
impl BankTransactionCodeStructure6Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(
        mut self,
        value: ExternalBankTransactionFamily1Code,
    ) -> BankTransactionCodeStructure6Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sub_fmly_cd` field.
    #[must_use]
    pub fn sub_fmly_cd(
        mut self,
        value: ExternalBankTransactionSubFamily1Code,
    ) -> BankTransactionCodeStructure6Builder {
        self.sub_fmly_cd = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<BankTransactionCodeStructure6, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd.is_none() {
            missing.push("cd".to_owned());
        }
        if self.sub_fmly_cd.is_none() {
            missing.push("sub_fmly_cd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BankTransactionCodeStructure6".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BankTransactionCodeStructure6 {
            cd: self.cd.unwrap(),
            sub_fmly_cd: self.sub_fmly_cd.unwrap(),
        })
    }
}
impl BankTransactionCodeStructure6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BankTransactionCodeStructure6Builder {
        BankTransactionCodeStructure6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchInformation2 {
    #[serde(rename = "MsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<Max35Text>,
    #[serde(rename = "PmtInfId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<Max35Text>,
    #[serde(rename = "NbOfTxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_of_txs: Option<Max15NumericText>,
    #[serde(rename = "TtlAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
}
/// Builder for [`BatchInformation2`]. Construct via [`BatchInformation2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BatchInformation2Builder {
    msg_id: ::std::option::Option<Max35Text>,
    pmt_inf_id: ::std::option::Option<Max35Text>,
    nb_of_txs: ::std::option::Option<Max15NumericText>,
    ttl_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
}
impl BatchInformation2Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> BatchInformation2Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_inf_id` field.
    #[must_use]
    pub fn pmt_inf_id(mut self, value: Max35Text) -> BatchInformation2Builder {
        self.pmt_inf_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_txs` field.
    #[must_use]
    pub fn nb_of_txs(mut self, value: Max15NumericText) -> BatchInformation2Builder {
        self.nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_amt` field.
    #[must_use]
    pub fn ttl_amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> BatchInformation2Builder {
        self.ttl_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> BatchInformation2Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<BatchInformation2, crate::common::BuilderError> {
        ::std::result::Result::Ok(BatchInformation2 {
            msg_id: self.msg_id,
            pmt_inf_id: self.pmt_inf_id,
            nb_of_txs: self.nb_of_txs,
            ttl_amt: self.ttl_amt,
            cdt_dbt_ind: self.cdt_dbt_ind,
        })
    }
}
impl BatchInformation2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BatchInformation2Builder {
        BatchInformation2Builder::default()
    }
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
pub struct CardAggregated2 {
    #[serde(rename = "AddtlSvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_svc: Option<CardPaymentServiceType2Code>,
    #[serde(rename = "TxCtgy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
    #[serde(rename = "SaleRcncltnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id: Option<Max35Text>,
    #[serde(rename = "SeqNbRg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nb_rg: Option<CardSequenceNumberRange1>,
    #[serde(rename = "TxDtRg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_dt_rg: Option<crate::common::ChoiceWrapper<DateOrDateTimePeriod1Choice>>,
}
/// Builder for [`CardAggregated2`]. Construct via [`CardAggregated2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardAggregated2Builder {
    addtl_svc: ::std::option::Option<CardPaymentServiceType2Code>,
    tx_ctgy: ::std::option::Option<ExternalCardTransactionCategory1Code>,
    sale_rcncltn_id: ::std::option::Option<Max35Text>,
    seq_nb_rg: ::std::option::Option<CardSequenceNumberRange1>,
    tx_dt_rg: ::std::option::Option<crate::common::ChoiceWrapper<DateOrDateTimePeriod1Choice>>,
}
impl CardAggregated2Builder {
    /// Set the `addtl_svc` field.
    #[must_use]
    pub fn addtl_svc(mut self, value: CardPaymentServiceType2Code) -> CardAggregated2Builder {
        self.addtl_svc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_ctgy` field.
    #[must_use]
    pub fn tx_ctgy(
        mut self,
        value: ExternalCardTransactionCategory1Code,
    ) -> CardAggregated2Builder {
        self.tx_ctgy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sale_rcncltn_id` field.
    #[must_use]
    pub fn sale_rcncltn_id(mut self, value: Max35Text) -> CardAggregated2Builder {
        self.sale_rcncltn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `seq_nb_rg` field.
    #[must_use]
    pub fn seq_nb_rg(mut self, value: CardSequenceNumberRange1) -> CardAggregated2Builder {
        self.seq_nb_rg = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_dt_rg` field.
    #[must_use]
    pub fn tx_dt_rg(
        mut self,
        value: crate::common::ChoiceWrapper<DateOrDateTimePeriod1Choice>,
    ) -> CardAggregated2Builder {
        self.tx_dt_rg = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CardAggregated2, crate::common::BuilderError> {
        ::std::result::Result::Ok(CardAggregated2 {
            addtl_svc: self.addtl_svc,
            tx_ctgy: self.tx_ctgy,
            sale_rcncltn_id: self.sale_rcncltn_id,
            seq_nb_rg: self.seq_nb_rg,
            tx_dt_rg: self.tx_dt_rg,
        })
    }
}
impl CardAggregated2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardAggregated2Builder {
        CardAggregated2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardEntry5 {
    #[serde(rename = "Card")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard4>,
    #[serde(rename = "POI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction1>,
    #[serde(rename = "AggtdNtry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggtd_ntry: Option<CardAggregated2>,
    #[serde(rename = "PrePdAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount40>,
}
/// Builder for [`CardEntry5`]. Construct via [`CardEntry5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardEntry5Builder {
    card: ::std::option::Option<PaymentCard4>,
    poi: ::std::option::Option<PointOfInteraction1>,
    aggtd_ntry: ::std::option::Option<CardAggregated2>,
    pre_pd_acct: ::std::option::Option<CashAccount40>,
}
impl CardEntry5Builder {
    /// Set the `card` field.
    #[must_use]
    pub fn card(mut self, value: PaymentCard4) -> CardEntry5Builder {
        self.card = ::std::option::Option::Some(value);
        self
    }
    /// Set the `poi` field.
    #[must_use]
    pub fn poi(mut self, value: PointOfInteraction1) -> CardEntry5Builder {
        self.poi = ::std::option::Option::Some(value);
        self
    }
    /// Set the `aggtd_ntry` field.
    #[must_use]
    pub fn aggtd_ntry(mut self, value: CardAggregated2) -> CardEntry5Builder {
        self.aggtd_ntry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pre_pd_acct` field.
    #[must_use]
    pub fn pre_pd_acct(mut self, value: CashAccount40) -> CardEntry5Builder {
        self.pre_pd_acct = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CardEntry5, crate::common::BuilderError> {
        ::std::result::Result::Ok(CardEntry5 {
            card: self.card,
            poi: self.poi,
            aggtd_ntry: self.aggtd_ntry,
            pre_pd_acct: self.pre_pd_acct,
        })
    }
}
impl CardEntry5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardEntry5Builder {
        CardEntry5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardIndividualTransaction2 {
    #[serde(rename = "ICCRltdData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icc_rltd_data: Option<Max1025Text>,
    #[serde(rename = "PmtCntxt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_cntxt: Option<PaymentContext3>,
    #[serde(rename = "AddtlSvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_svc: Option<CardPaymentServiceType2Code>,
    #[serde(rename = "TxCtgy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_ctgy: Option<ExternalCardTransactionCategory1Code>,
    #[serde(rename = "SaleRcncltnId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_rcncltn_id: Option<Max35Text>,
    #[serde(rename = "SaleRefNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sale_ref_nb: Option<Max35Text>,
    #[serde(rename = "RePresntmntRsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub re_presntmnt_rsn: Option<ExternalRePresentmentReason1Code>,
    #[serde(rename = "SeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_nb: Option<Max35Text>,
    #[serde(rename = "TxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<TransactionIdentifier1>,
    #[serde(rename = "Pdct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdct: Option<Product2>,
    #[serde(rename = "VldtnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vldtn_dt: Option<ISODate>,
    #[serde(rename = "VldtnSeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vldtn_seq_nb: Option<Max35Text>,
}
/// Builder for [`CardIndividualTransaction2`]. Construct via [`CardIndividualTransaction2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardIndividualTransaction2Builder {
    icc_rltd_data: ::std::option::Option<Max1025Text>,
    pmt_cntxt: ::std::option::Option<PaymentContext3>,
    addtl_svc: ::std::option::Option<CardPaymentServiceType2Code>,
    tx_ctgy: ::std::option::Option<ExternalCardTransactionCategory1Code>,
    sale_rcncltn_id: ::std::option::Option<Max35Text>,
    sale_ref_nb: ::std::option::Option<Max35Text>,
    re_presntmnt_rsn: ::std::option::Option<ExternalRePresentmentReason1Code>,
    seq_nb: ::std::option::Option<Max35Text>,
    tx_id: ::std::option::Option<TransactionIdentifier1>,
    pdct: ::std::option::Option<Product2>,
    vldtn_dt: ::std::option::Option<ISODate>,
    vldtn_seq_nb: ::std::option::Option<Max35Text>,
}
impl CardIndividualTransaction2Builder {
    /// Set the `icc_rltd_data` field.
    #[must_use]
    pub fn icc_rltd_data(mut self, value: Max1025Text) -> CardIndividualTransaction2Builder {
        self.icc_rltd_data = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_cntxt` field.
    #[must_use]
    pub fn pmt_cntxt(mut self, value: PaymentContext3) -> CardIndividualTransaction2Builder {
        self.pmt_cntxt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_svc` field.
    #[must_use]
    pub fn addtl_svc(
        mut self,
        value: CardPaymentServiceType2Code,
    ) -> CardIndividualTransaction2Builder {
        self.addtl_svc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_ctgy` field.
    #[must_use]
    pub fn tx_ctgy(
        mut self,
        value: ExternalCardTransactionCategory1Code,
    ) -> CardIndividualTransaction2Builder {
        self.tx_ctgy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sale_rcncltn_id` field.
    #[must_use]
    pub fn sale_rcncltn_id(mut self, value: Max35Text) -> CardIndividualTransaction2Builder {
        self.sale_rcncltn_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sale_ref_nb` field.
    #[must_use]
    pub fn sale_ref_nb(mut self, value: Max35Text) -> CardIndividualTransaction2Builder {
        self.sale_ref_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `re_presntmnt_rsn` field.
    #[must_use]
    pub fn re_presntmnt_rsn(
        mut self,
        value: ExternalRePresentmentReason1Code,
    ) -> CardIndividualTransaction2Builder {
        self.re_presntmnt_rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `seq_nb` field.
    #[must_use]
    pub fn seq_nb(mut self, value: Max35Text) -> CardIndividualTransaction2Builder {
        self.seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_id` field.
    #[must_use]
    pub fn tx_id(mut self, value: TransactionIdentifier1) -> CardIndividualTransaction2Builder {
        self.tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pdct` field.
    #[must_use]
    pub fn pdct(mut self, value: Product2) -> CardIndividualTransaction2Builder {
        self.pdct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `vldtn_dt` field.
    #[must_use]
    pub fn vldtn_dt(mut self, value: ISODate) -> CardIndividualTransaction2Builder {
        self.vldtn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `vldtn_seq_nb` field.
    #[must_use]
    pub fn vldtn_seq_nb(mut self, value: Max35Text) -> CardIndividualTransaction2Builder {
        self.vldtn_seq_nb = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<CardIndividualTransaction2, crate::common::BuilderError> {
        ::std::result::Result::Ok(CardIndividualTransaction2 {
            icc_rltd_data: self.icc_rltd_data,
            pmt_cntxt: self.pmt_cntxt,
            addtl_svc: self.addtl_svc,
            tx_ctgy: self.tx_ctgy,
            sale_rcncltn_id: self.sale_rcncltn_id,
            sale_ref_nb: self.sale_ref_nb,
            re_presntmnt_rsn: self.re_presntmnt_rsn,
            seq_nb: self.seq_nb,
            tx_id: self.tx_id,
            pdct: self.pdct,
            vldtn_dt: self.vldtn_dt,
            vldtn_seq_nb: self.vldtn_seq_nb,
        })
    }
}
impl CardIndividualTransaction2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardIndividualTransaction2Builder {
        CardIndividualTransaction2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardSecurityInformation1 {
    #[serde(rename = "CSCMgmt")]
    pub csc_mgmt: CSCManagement1Code,
    #[serde(rename = "CSCVal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csc_val: Option<Min3Max4NumericText>,
}
/// Builder for [`CardSecurityInformation1`]. Construct via [`CardSecurityInformation1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardSecurityInformation1Builder {
    csc_mgmt: ::std::option::Option<CSCManagement1Code>,
    csc_val: ::std::option::Option<Min3Max4NumericText>,
}
impl CardSecurityInformation1Builder {
    /// Set the `csc_mgmt` field.
    #[must_use]
    pub fn csc_mgmt(mut self, value: CSCManagement1Code) -> CardSecurityInformation1Builder {
        self.csc_mgmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `csc_val` field.
    #[must_use]
    pub fn csc_val(mut self, value: Min3Max4NumericText) -> CardSecurityInformation1Builder {
        self.csc_val = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<CardSecurityInformation1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.csc_mgmt.is_none() {
            missing.push("csc_mgmt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CardSecurityInformation1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CardSecurityInformation1 {
            csc_mgmt: self.csc_mgmt.unwrap(),
            csc_val: self.csc_val,
        })
    }
}
impl CardSecurityInformation1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardSecurityInformation1Builder {
        CardSecurityInformation1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardSequenceNumberRange1 {
    #[serde(rename = "FrstTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frst_tx: Option<Max35Text>,
    #[serde(rename = "LastTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tx: Option<Max35Text>,
}
/// Builder for [`CardSequenceNumberRange1`]. Construct via [`CardSequenceNumberRange1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardSequenceNumberRange1Builder {
    frst_tx: ::std::option::Option<Max35Text>,
    last_tx: ::std::option::Option<Max35Text>,
}
impl CardSequenceNumberRange1Builder {
    /// Set the `frst_tx` field.
    #[must_use]
    pub fn frst_tx(mut self, value: Max35Text) -> CardSequenceNumberRange1Builder {
        self.frst_tx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `last_tx` field.
    #[must_use]
    pub fn last_tx(mut self, value: Max35Text) -> CardSequenceNumberRange1Builder {
        self.last_tx = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<CardSequenceNumberRange1, crate::common::BuilderError> {
        ::std::result::Result::Ok(CardSequenceNumberRange1 {
            frst_tx: self.frst_tx,
            last_tx: self.last_tx,
        })
    }
}
impl CardSequenceNumberRange1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardSequenceNumberRange1Builder {
        CardSequenceNumberRange1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardTransaction18 {
    #[serde(rename = "Card")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PaymentCard4>,
    #[serde(rename = "POI")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poi: Option<PointOfInteraction1>,
    #[serde(rename = "Tx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx: Option<crate::common::ChoiceWrapper<CardTransaction3Choice>>,
    #[serde(rename = "PrePdAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_pd_acct: Option<CashAccount40>,
}
/// Builder for [`CardTransaction18`]. Construct via [`CardTransaction18::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardTransaction18Builder {
    card: ::std::option::Option<PaymentCard4>,
    poi: ::std::option::Option<PointOfInteraction1>,
    tx: ::std::option::Option<crate::common::ChoiceWrapper<CardTransaction3Choice>>,
    pre_pd_acct: ::std::option::Option<CashAccount40>,
}
impl CardTransaction18Builder {
    /// Set the `card` field.
    #[must_use]
    pub fn card(mut self, value: PaymentCard4) -> CardTransaction18Builder {
        self.card = ::std::option::Option::Some(value);
        self
    }
    /// Set the `poi` field.
    #[must_use]
    pub fn poi(mut self, value: PointOfInteraction1) -> CardTransaction18Builder {
        self.poi = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx` field.
    #[must_use]
    pub fn tx(
        mut self,
        value: crate::common::ChoiceWrapper<CardTransaction3Choice>,
    ) -> CardTransaction18Builder {
        self.tx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pre_pd_acct` field.
    #[must_use]
    pub fn pre_pd_acct(mut self, value: CashAccount40) -> CardTransaction18Builder {
        self.pre_pd_acct = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CardTransaction18, crate::common::BuilderError> {
        ::std::result::Result::Ok(CardTransaction18 {
            card: self.card,
            poi: self.poi,
            tx: self.tx,
            pre_pd_acct: self.pre_pd_acct,
        })
    }
}
impl CardTransaction18 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardTransaction18Builder {
        CardTransaction18Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CardTransaction3Choice {
    #[serde(rename = "Aggtd")]
    Aggtd(CardAggregated2),
    #[serde(rename = "Indv")]
    Indv(CardIndividualTransaction2),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardholderAuthentication2 {
    #[serde(rename = "AuthntcnMtd")]
    pub authntcn_mtd: AuthenticationMethod1Code,
    #[serde(rename = "AuthntcnNtty")]
    pub authntcn_ntty: AuthenticationEntity1Code,
}
/// Builder for [`CardholderAuthentication2`]. Construct via [`CardholderAuthentication2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CardholderAuthentication2Builder {
    authntcn_mtd: ::std::option::Option<AuthenticationMethod1Code>,
    authntcn_ntty: ::std::option::Option<AuthenticationEntity1Code>,
}
impl CardholderAuthentication2Builder {
    /// Set the `authntcn_mtd` field.
    #[must_use]
    pub fn authntcn_mtd(
        mut self,
        value: AuthenticationMethod1Code,
    ) -> CardholderAuthentication2Builder {
        self.authntcn_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authntcn_ntty` field.
    #[must_use]
    pub fn authntcn_ntty(
        mut self,
        value: AuthenticationEntity1Code,
    ) -> CardholderAuthentication2Builder {
        self.authntcn_ntty = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<CardholderAuthentication2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.authntcn_mtd.is_none() {
            missing.push("authntcn_mtd".to_owned());
        }
        if self.authntcn_ntty.is_none() {
            missing.push("authntcn_ntty".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CardholderAuthentication2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CardholderAuthentication2 {
            authntcn_mtd: self.authntcn_mtd.unwrap(),
            authntcn_ntty: self.authntcn_ntty.unwrap(),
        })
    }
}
impl CardholderAuthentication2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CardholderAuthentication2Builder {
        CardholderAuthentication2Builder::default()
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashAccount41 {
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
    #[serde(rename = "Ownr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownr: Option<PartyIdentification135>,
    #[serde(rename = "Svcr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svcr: Option<BranchAndFinancialInstitutionIdentification6>,
}
/// Builder for [`CashAccount41`]. Construct via [`CashAccount41::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CashAccount41Builder {
    id: ::std::option::Option<crate::common::ChoiceWrapper<AccountIdentification4Choice>>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<CashAccountType2Choice>>,
    ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    nm: ::std::option::Option<Max70Text>,
    prxy: ::std::option::Option<ProxyAccountIdentification1>,
    ownr: ::std::option::Option<PartyIdentification135>,
    svcr: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
}
impl CashAccount41Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(
        mut self,
        value: crate::common::ChoiceWrapper<AccountIdentification4Choice>,
    ) -> CashAccount41Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<CashAccountType2Choice>,
    ) -> CashAccount41Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy` field.
    #[must_use]
    pub fn ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CashAccount41Builder {
        self.ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max70Text) -> CashAccount41Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prxy` field.
    #[must_use]
    pub fn prxy(mut self, value: ProxyAccountIdentification1) -> CashAccount41Builder {
        self.prxy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ownr` field.
    #[must_use]
    pub fn ownr(mut self, value: PartyIdentification135) -> CashAccount41Builder {
        self.ownr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `svcr` field.
    #[must_use]
    pub fn svcr(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CashAccount41Builder {
        self.svcr = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CashAccount41, crate::common::BuilderError> {
        ::std::result::Result::Ok(CashAccount41 {
            id: self.id,
            tp: self.tp,
            ccy: self.ccy,
            nm: self.nm,
            prxy: self.prxy,
            ownr: self.ownr,
            svcr: self.svcr,
        })
    }
}
impl CashAccount41 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CashAccount41Builder {
        CashAccount41Builder::default()
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashAvailability1 {
    #[serde(rename = "Dt")]
    pub dt: crate::common::ChoiceWrapper<CashAvailabilityDate1Choice>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
}
/// Builder for [`CashAvailability1`]. Construct via [`CashAvailability1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CashAvailability1Builder {
    dt: ::std::option::Option<crate::common::ChoiceWrapper<CashAvailabilityDate1Choice>>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
}
impl CashAvailability1Builder {
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(
        mut self,
        value: crate::common::ChoiceWrapper<CashAvailabilityDate1Choice>,
    ) -> CashAvailability1Builder {
        self.dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> CashAvailability1Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> CashAvailability1Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CashAvailability1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.dt.is_none() {
            missing.push("dt".to_owned());
        }
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.cdt_dbt_ind.is_none() {
            missing.push("cdt_dbt_ind".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CashAvailability1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CashAvailability1 {
            dt: self.dt.unwrap(),
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind.unwrap(),
        })
    }
}
impl CashAvailability1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CashAvailability1Builder {
        CashAvailability1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum CashAvailabilityDate1Choice {
    #[serde(rename = "NbOfDays")]
    NbOfDays(Max15PlusSignedNumericText),
    #[serde(rename = "ActlDt")]
    ActlDt(ISODate),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CashDeposit1 {
    #[serde(rename = "NoteDnmtn")]
    pub note_dnmtn: ActiveCurrencyAndAmount,
    #[serde(rename = "NbOfNotes")]
    pub nb_of_notes: Max15NumericText,
    #[serde(rename = "Amt")]
    pub amt: ActiveCurrencyAndAmount,
}
/// Builder for [`CashDeposit1`]. Construct via [`CashDeposit1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CashDeposit1Builder {
    note_dnmtn: ::std::option::Option<ActiveCurrencyAndAmount>,
    nb_of_notes: ::std::option::Option<Max15NumericText>,
    amt: ::std::option::Option<ActiveCurrencyAndAmount>,
}
impl CashDeposit1Builder {
    /// Set the `note_dnmtn` field.
    #[must_use]
    pub fn note_dnmtn(mut self, value: ActiveCurrencyAndAmount) -> CashDeposit1Builder {
        self.note_dnmtn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_notes` field.
    #[must_use]
    pub fn nb_of_notes(mut self, value: Max15NumericText) -> CashDeposit1Builder {
        self.nb_of_notes = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveCurrencyAndAmount) -> CashDeposit1Builder {
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
    pub fn build(self) -> ::std::result::Result<CashDeposit1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.note_dnmtn.is_none() {
            missing.push("note_dnmtn".to_owned());
        }
        if self.nb_of_notes.is_none() {
            missing.push("nb_of_notes".to_owned());
        }
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CashDeposit1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CashDeposit1 {
            note_dnmtn: self.note_dnmtn.unwrap(),
            nb_of_notes: self.nb_of_notes.unwrap(),
            amt: self.amt.unwrap(),
        })
    }
}
impl CashDeposit1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CashDeposit1Builder {
        CashDeposit1Builder::default()
    }
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
pub struct Charges6 {
    #[serde(rename = "TtlChrgsAndTaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_chrgs_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rcrd: Vec<ChargesRecord3>,
}
/// Builder for [`Charges6`]. Construct via [`Charges6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Charges6Builder {
    ttl_chrgs_and_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    rcrd: ::std::vec::Vec<ChargesRecord3>,
}
impl Charges6Builder {
    /// Set the `ttl_chrgs_and_tax_amt` field.
    #[must_use]
    pub fn ttl_chrgs_and_tax_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> Charges6Builder {
        self.ttl_chrgs_and_tax_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rcrd` field (replaces any previously added items).
    #[must_use]
    pub fn rcrd(mut self, value: ::std::vec::Vec<ChargesRecord3>) -> Charges6Builder {
        self.rcrd = value;
        self
    }
    /// Append one item to the `rcrd` field.
    #[must_use]
    pub fn add_rcrd(mut self, value: ChargesRecord3) -> Charges6Builder {
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
    pub fn build(self) -> ::std::result::Result<Charges6, crate::common::BuilderError> {
        ::std::result::Result::Ok(Charges6 {
            ttl_chrgs_and_tax_amt: self.ttl_chrgs_and_tax_amt,
            rcrd: self.rcrd,
        })
    }
}
impl Charges6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Charges6Builder {
        Charges6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ChargesRecord3 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "ChrgInclInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrg_incl_ind: Option<ChargeIncludedIndicator>,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<ChargeType3Choice>>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Br")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub br: Option<ChargeBearerType1Code>,
    #[serde(rename = "Agt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}
/// Builder for [`ChargesRecord3`]. Construct via [`ChargesRecord3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ChargesRecord3Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    chrg_incl_ind: ::std::option::Option<ChargeIncludedIndicator>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<ChargeType3Choice>>,
    rate: ::std::option::Option<PercentageRate>,
    br: ::std::option::Option<ChargeBearerType1Code>,
    agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    tax: ::std::option::Option<TaxCharges2>,
}
impl ChargesRecord3Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> ChargesRecord3Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> ChargesRecord3Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrg_incl_ind` field.
    #[must_use]
    pub fn chrg_incl_ind(mut self, value: ChargeIncludedIndicator) -> ChargesRecord3Builder {
        self.chrg_incl_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<ChargeType3Choice>,
    ) -> ChargesRecord3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rate` field.
    #[must_use]
    pub fn rate(mut self, value: PercentageRate) -> ChargesRecord3Builder {
        self.rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `br` field.
    #[must_use]
    pub fn br(mut self, value: ChargeBearerType1Code) -> ChargesRecord3Builder {
        self.br = ::std::option::Option::Some(value);
        self
    }
    /// Set the `agt` field.
    #[must_use]
    pub fn agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> ChargesRecord3Builder {
        self.agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxCharges2) -> ChargesRecord3Builder {
        self.tax = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<ChargesRecord3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ChargesRecord3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ChargesRecord3 {
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind,
            chrg_incl_ind: self.chrg_incl_ind,
            tp: self.tp,
            rate: self.rate,
            br: self.br,
            agt: self.agt,
            tax: self.tax,
        })
    }
}
impl ChargesRecord3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ChargesRecord3Builder {
        ChargesRecord3Builder::default()
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
pub struct CorporateAction9 {
    #[serde(rename = "EvtTp")]
    pub evt_tp: Max35Text,
    #[serde(rename = "EvtId")]
    pub evt_id: Max35Text,
}
/// Builder for [`CorporateAction9`]. Construct via [`CorporateAction9::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CorporateAction9Builder {
    evt_tp: ::std::option::Option<Max35Text>,
    evt_id: ::std::option::Option<Max35Text>,
}
impl CorporateAction9Builder {
    /// Set the `evt_tp` field.
    #[must_use]
    pub fn evt_tp(mut self, value: Max35Text) -> CorporateAction9Builder {
        self.evt_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `evt_id` field.
    #[must_use]
    pub fn evt_id(mut self, value: Max35Text) -> CorporateAction9Builder {
        self.evt_id = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CorporateAction9, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.evt_tp.is_none() {
            missing.push("evt_tp".to_owned());
        }
        if self.evt_id.is_none() {
            missing.push("evt_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CorporateAction9".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CorporateAction9 {
            evt_tp: self.evt_tp.unwrap(),
            evt_id: self.evt_id.unwrap(),
        })
    }
}
impl CorporateAction9 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CorporateAction9Builder {
        CorporateAction9Builder::default()
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CurrencyExchange24 {
    #[serde(rename = "SrcCcy")]
    pub src_ccy: ActiveOrHistoricCurrencyCode,
    #[serde(rename = "TrgtCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trgt_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "UnitCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "XchgRate")]
    pub xchg_rate: BaseOneRate,
    #[serde(rename = "CtrctId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<Max35Text>,
    #[serde(rename = "QtnDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qtn_dt: Option<ISODateTime>,
    #[serde(rename = "XchgRateBase")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate_base: Option<PositiveNumber>,
}
/// Builder for [`CurrencyExchange24`]. Construct via [`CurrencyExchange24::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CurrencyExchange24Builder {
    src_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    trgt_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    unit_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    xchg_rate: ::std::option::Option<BaseOneRate>,
    ctrct_id: ::std::option::Option<Max35Text>,
    qtn_dt: ::std::option::Option<ISODateTime>,
    xchg_rate_base: ::std::option::Option<PositiveNumber>,
}
impl CurrencyExchange24Builder {
    /// Set the `src_ccy` field.
    #[must_use]
    pub fn src_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CurrencyExchange24Builder {
        self.src_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trgt_ccy` field.
    #[must_use]
    pub fn trgt_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CurrencyExchange24Builder {
        self.trgt_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `unit_ccy` field.
    #[must_use]
    pub fn unit_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> CurrencyExchange24Builder {
        self.unit_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate` field.
    #[must_use]
    pub fn xchg_rate(mut self, value: BaseOneRate) -> CurrencyExchange24Builder {
        self.xchg_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrct_id` field.
    #[must_use]
    pub fn ctrct_id(mut self, value: Max35Text) -> CurrencyExchange24Builder {
        self.ctrct_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `qtn_dt` field.
    #[must_use]
    pub fn qtn_dt(mut self, value: ISODateTime) -> CurrencyExchange24Builder {
        self.qtn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate_base` field.
    #[must_use]
    pub fn xchg_rate_base(mut self, value: PositiveNumber) -> CurrencyExchange24Builder {
        self.xchg_rate_base = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<CurrencyExchange24, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.src_ccy.is_none() {
            missing.push("src_ccy".to_owned());
        }
        if self.xchg_rate.is_none() {
            missing.push("xchg_rate".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CurrencyExchange24".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CurrencyExchange24 {
            src_ccy: self.src_ccy.unwrap(),
            trgt_ccy: self.trgt_ccy,
            unit_ccy: self.unit_ccy,
            xchg_rate: self.xchg_rate.unwrap(),
            ctrct_id: self.ctrct_id,
            qtn_dt: self.qtn_dt,
            xchg_rate_base: self.xchg_rate_base,
        })
    }
}
impl CurrencyExchange24 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CurrencyExchange24Builder {
        CurrencyExchange24Builder::default()
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum DateOrDateTimePeriod1Choice {
    #[serde(rename = "Dt")]
    Dt(DatePeriod2),
    #[serde(rename = "DtTm")]
    DtTm(DateTimePeriod1),
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
pub struct DateTimePeriod1 {
    #[serde(rename = "FrDtTm")]
    pub fr_dt_tm: ISODateTime,
    #[serde(rename = "ToDtTm")]
    pub to_dt_tm: ISODateTime,
}
/// Builder for [`DateTimePeriod1`]. Construct via [`DateTimePeriod1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DateTimePeriod1Builder {
    fr_dt_tm: ::std::option::Option<ISODateTime>,
    to_dt_tm: ::std::option::Option<ISODateTime>,
}
impl DateTimePeriod1Builder {
    /// Set the `fr_dt_tm` field.
    #[must_use]
    pub fn fr_dt_tm(mut self, value: ISODateTime) -> DateTimePeriod1Builder {
        self.fr_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to_dt_tm` field.
    #[must_use]
    pub fn to_dt_tm(mut self, value: ISODateTime) -> DateTimePeriod1Builder {
        self.to_dt_tm = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<DateTimePeriod1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr_dt_tm.is_none() {
            missing.push("fr_dt_tm".to_owned());
        }
        if self.to_dt_tm.is_none() {
            missing.push("to_dt_tm".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DateTimePeriod1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DateTimePeriod1 {
            fr_dt_tm: self.fr_dt_tm.unwrap(),
            to_dt_tm: self.to_dt_tm.unwrap(),
        })
    }
}
impl DateTimePeriod1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DateTimePeriod1Builder {
        DateTimePeriod1Builder::default()
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
pub struct DisplayCapabilities1 {
    #[serde(rename = "DispTp")]
    pub disp_tp: UserInterface2Code,
    #[serde(rename = "NbOfLines")]
    pub nb_of_lines: Max3NumericText,
    #[serde(rename = "LineWidth")]
    pub line_width: Max3NumericText,
}
/// Builder for [`DisplayCapabilities1`]. Construct via [`DisplayCapabilities1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DisplayCapabilities1Builder {
    disp_tp: ::std::option::Option<UserInterface2Code>,
    nb_of_lines: ::std::option::Option<Max3NumericText>,
    line_width: ::std::option::Option<Max3NumericText>,
}
impl DisplayCapabilities1Builder {
    /// Set the `disp_tp` field.
    #[must_use]
    pub fn disp_tp(mut self, value: UserInterface2Code) -> DisplayCapabilities1Builder {
        self.disp_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_lines` field.
    #[must_use]
    pub fn nb_of_lines(mut self, value: Max3NumericText) -> DisplayCapabilities1Builder {
        self.nb_of_lines = ::std::option::Option::Some(value);
        self
    }
    /// Set the `line_width` field.
    #[must_use]
    pub fn line_width(mut self, value: Max3NumericText) -> DisplayCapabilities1Builder {
        self.line_width = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<DisplayCapabilities1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.disp_tp.is_none() {
            missing.push("disp_tp".to_owned());
        }
        if self.nb_of_lines.is_none() {
            missing.push("nb_of_lines".to_owned());
        }
        if self.line_width.is_none() {
            missing.push("line_width".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "DisplayCapabilities1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(DisplayCapabilities1 {
            disp_tp: self.disp_tp.unwrap(),
            nb_of_lines: self.nb_of_lines.unwrap(),
            line_width: self.line_width.unwrap(),
        })
    }
}
impl DisplayCapabilities1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> DisplayCapabilities1Builder {
        DisplayCapabilities1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Document {
    #[serde(rename = "BkToCstmrDbtCdtNtfctn")]
    pub bk_to_cstmr_dbt_cdt_ntfctn: BankToCustomerDebitCreditNotificationV11,
}
/// Builder for [`Document`]. Construct via [`Document::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentBuilder {
    bk_to_cstmr_dbt_cdt_ntfctn: ::std::option::Option<BankToCustomerDebitCreditNotificationV11>,
}
impl DocumentBuilder {
    /// Set the `bk_to_cstmr_dbt_cdt_ntfctn` field.
    #[must_use]
    pub fn bk_to_cstmr_dbt_cdt_ntfctn(
        mut self,
        value: BankToCustomerDebitCreditNotificationV11,
    ) -> DocumentBuilder {
        self.bk_to_cstmr_dbt_cdt_ntfctn = ::std::option::Option::Some(value);
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
        if self.bk_to_cstmr_dbt_cdt_ntfctn.is_none() {
            missing.push("bk_to_cstmr_dbt_cdt_ntfctn".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Document".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Document {
            bk_to_cstmr_dbt_cdt_ntfctn: self.bk_to_cstmr_dbt_cdt_ntfctn.unwrap(),
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
pub struct EntryDetails12 {
    #[serde(rename = "Btch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch: Option<BatchInformation2>,
    #[serde(rename = "TxDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tx_dtls: Vec<EntryTransaction13>,
}
/// Builder for [`EntryDetails12`]. Construct via [`EntryDetails12::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct EntryDetails12Builder {
    btch: ::std::option::Option<BatchInformation2>,
    tx_dtls: ::std::vec::Vec<EntryTransaction13>,
}
impl EntryDetails12Builder {
    /// Set the `btch` field.
    #[must_use]
    pub fn btch(mut self, value: BatchInformation2) -> EntryDetails12Builder {
        self.btch = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn tx_dtls(mut self, value: ::std::vec::Vec<EntryTransaction13>) -> EntryDetails12Builder {
        self.tx_dtls = value;
        self
    }
    /// Append one item to the `tx_dtls` field.
    #[must_use]
    pub fn add_tx_dtls(mut self, value: EntryTransaction13) -> EntryDetails12Builder {
        self.tx_dtls.push(value);
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
    pub fn build(self) -> ::std::result::Result<EntryDetails12, crate::common::BuilderError> {
        ::std::result::Result::Ok(EntryDetails12 {
            btch: self.btch,
            tx_dtls: self.tx_dtls,
        })
    }
}
impl EntryDetails12 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> EntryDetails12Builder {
        EntryDetails12Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum EntryStatus1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalEntryStatus1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EntryTransaction13 {
    #[serde(rename = "Refs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refs: Option<TransactionReferences6>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "CdtDbtInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_dbt_ind: Option<CreditDebitCode>,
    #[serde(rename = "AmtDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange4>,
    #[serde(rename = "Avlbty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub avlbty: Vec<CashAvailability1>,
    #[serde(rename = "BkTxCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bk_tx_cd: Option<BankTransactionCodeStructure4>,
    #[serde(rename = "Chrgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges6>,
    #[serde(rename = "Intrst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest4>,
    #[serde(rename = "RltdPties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_pties: Option<TransactionParties9>,
    #[serde(rename = "RltdAgts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_agts: Option<TransactionAgents5>,
    #[serde(rename = "LclInstrm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lcl_instrm: Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation27>,
    #[serde(rename = "Purp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purp: Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    #[serde(rename = "RltdRmtInf")]
    /// Maximum 10 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rltd_rmt_inf: Vec<RemittanceLocation7>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation21>,
    #[serde(rename = "RltdDts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_dts: Option<TransactionDates3>,
    #[serde(rename = "RltdPric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rltd_pric: Option<crate::common::ChoiceWrapper<TransactionPrice4Choice>>,
    #[serde(rename = "RltdQties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rltd_qties: Vec<crate::common::ChoiceWrapper<TransactionQuantities3Choice>>,
    #[serde(rename = "FinInstrmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fin_instrm_id: Option<SecurityIdentification19>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxInformation10>,
    #[serde(rename = "RtrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rtr_inf: Option<PaymentReturnReason5>,
    #[serde(rename = "CorpActn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corp_actn: Option<CorporateAction9>,
    #[serde(rename = "SfkpgAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sfkpg_acct: Option<SecuritiesAccount19>,
    #[serde(rename = "CshDpst")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub csh_dpst: Vec<CashDeposit1>,
    #[serde(rename = "CardTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardTransaction18>,
    #[serde(rename = "AddtlTxInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_tx_inf: Option<Max500Text>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`EntryTransaction13`]. Construct via [`EntryTransaction13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct EntryTransaction13Builder {
    refs: ::std::option::Option<TransactionReferences6>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    amt_dtls: ::std::option::Option<AmountAndCurrencyExchange4>,
    avlbty: ::std::vec::Vec<CashAvailability1>,
    bk_tx_cd: ::std::option::Option<BankTransactionCodeStructure4>,
    chrgs: ::std::option::Option<Charges6>,
    intrst: ::std::option::Option<TransactionInterest4>,
    rltd_pties: ::std::option::Option<TransactionParties9>,
    rltd_agts: ::std::option::Option<TransactionAgents5>,
    lcl_instrm: ::std::option::Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation27>,
    purp: ::std::option::Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    rltd_rmt_inf: ::std::vec::Vec<RemittanceLocation7>,
    rmt_inf: ::std::option::Option<RemittanceInformation21>,
    rltd_dts: ::std::option::Option<TransactionDates3>,
    rltd_pric: ::std::option::Option<crate::common::ChoiceWrapper<TransactionPrice4Choice>>,
    rltd_qties: ::std::vec::Vec<crate::common::ChoiceWrapper<TransactionQuantities3Choice>>,
    fin_instrm_id: ::std::option::Option<SecurityIdentification19>,
    tax: ::std::option::Option<TaxInformation10>,
    rtr_inf: ::std::option::Option<PaymentReturnReason5>,
    corp_actn: ::std::option::Option<CorporateAction9>,
    sfkpg_acct: ::std::option::Option<SecuritiesAccount19>,
    csh_dpst: ::std::vec::Vec<CashDeposit1>,
    card_tx: ::std::option::Option<CardTransaction18>,
    addtl_tx_inf: ::std::option::Option<Max500Text>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl EntryTransaction13Builder {
    /// Set the `refs` field.
    #[must_use]
    pub fn refs(mut self, value: TransactionReferences6) -> EntryTransaction13Builder {
        self.refs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> EntryTransaction13Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> EntryTransaction13Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt_dtls` field.
    #[must_use]
    pub fn amt_dtls(mut self, value: AmountAndCurrencyExchange4) -> EntryTransaction13Builder {
        self.amt_dtls = ::std::option::Option::Some(value);
        self
    }
    /// Set the `avlbty` field (replaces any previously added items).
    #[must_use]
    pub fn avlbty(
        mut self,
        value: ::std::vec::Vec<CashAvailability1>,
    ) -> EntryTransaction13Builder {
        self.avlbty = value;
        self
    }
    /// Append one item to the `avlbty` field.
    #[must_use]
    pub fn add_avlbty(mut self, value: CashAvailability1) -> EntryTransaction13Builder {
        self.avlbty.push(value);
        self
    }
    /// Set the `bk_tx_cd` field.
    #[must_use]
    pub fn bk_tx_cd(mut self, value: BankTransactionCodeStructure4) -> EntryTransaction13Builder {
        self.bk_tx_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs` field.
    #[must_use]
    pub fn chrgs(mut self, value: Charges6) -> EntryTransaction13Builder {
        self.chrgs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrst` field.
    #[must_use]
    pub fn intrst(mut self, value: TransactionInterest4) -> EntryTransaction13Builder {
        self.intrst = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_pties` field.
    #[must_use]
    pub fn rltd_pties(mut self, value: TransactionParties9) -> EntryTransaction13Builder {
        self.rltd_pties = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_agts` field.
    #[must_use]
    pub fn rltd_agts(mut self, value: TransactionAgents5) -> EntryTransaction13Builder {
        self.rltd_agts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `lcl_instrm` field.
    #[must_use]
    pub fn lcl_instrm(
        mut self,
        value: crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    ) -> EntryTransaction13Builder {
        self.lcl_instrm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(mut self, value: PaymentTypeInformation27) -> EntryTransaction13Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `purp` field.
    #[must_use]
    pub fn purp(
        mut self,
        value: crate::common::ChoiceWrapper<Purpose2Choice>,
    ) -> EntryTransaction13Builder {
        self.purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_rmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rltd_rmt_inf(
        mut self,
        value: ::std::vec::Vec<RemittanceLocation7>,
    ) -> EntryTransaction13Builder {
        self.rltd_rmt_inf = value;
        self
    }
    /// Append one item to the `rltd_rmt_inf` field.
    #[must_use]
    pub fn add_rltd_rmt_inf(mut self, value: RemittanceLocation7) -> EntryTransaction13Builder {
        self.rltd_rmt_inf.push(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(mut self, value: RemittanceInformation21) -> EntryTransaction13Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_dts` field.
    #[must_use]
    pub fn rltd_dts(mut self, value: TransactionDates3) -> EntryTransaction13Builder {
        self.rltd_dts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_pric` field.
    #[must_use]
    pub fn rltd_pric(
        mut self,
        value: crate::common::ChoiceWrapper<TransactionPrice4Choice>,
    ) -> EntryTransaction13Builder {
        self.rltd_pric = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_qties` field (replaces any previously added items).
    #[must_use]
    pub fn rltd_qties(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<TransactionQuantities3Choice>>,
    ) -> EntryTransaction13Builder {
        self.rltd_qties = value;
        self
    }
    /// Append one item to the `rltd_qties` field.
    #[must_use]
    pub fn add_rltd_qties(
        mut self,
        value: crate::common::ChoiceWrapper<TransactionQuantities3Choice>,
    ) -> EntryTransaction13Builder {
        self.rltd_qties.push(value);
        self
    }
    /// Set the `fin_instrm_id` field.
    #[must_use]
    pub fn fin_instrm_id(mut self, value: SecurityIdentification19) -> EntryTransaction13Builder {
        self.fin_instrm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxInformation10) -> EntryTransaction13Builder {
        self.tax = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rtr_inf` field.
    #[must_use]
    pub fn rtr_inf(mut self, value: PaymentReturnReason5) -> EntryTransaction13Builder {
        self.rtr_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `corp_actn` field.
    #[must_use]
    pub fn corp_actn(mut self, value: CorporateAction9) -> EntryTransaction13Builder {
        self.corp_actn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sfkpg_acct` field.
    #[must_use]
    pub fn sfkpg_acct(mut self, value: SecuritiesAccount19) -> EntryTransaction13Builder {
        self.sfkpg_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `csh_dpst` field (replaces any previously added items).
    #[must_use]
    pub fn csh_dpst(mut self, value: ::std::vec::Vec<CashDeposit1>) -> EntryTransaction13Builder {
        self.csh_dpst = value;
        self
    }
    /// Append one item to the `csh_dpst` field.
    #[must_use]
    pub fn add_csh_dpst(mut self, value: CashDeposit1) -> EntryTransaction13Builder {
        self.csh_dpst.push(value);
        self
    }
    /// Set the `card_tx` field.
    #[must_use]
    pub fn card_tx(mut self, value: CardTransaction18) -> EntryTransaction13Builder {
        self.card_tx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_tx_inf` field.
    #[must_use]
    pub fn addtl_tx_inf(mut self, value: Max500Text) -> EntryTransaction13Builder {
        self.addtl_tx_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> EntryTransaction13Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(mut self, value: SupplementaryData1) -> EntryTransaction13Builder {
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
    pub fn build(self) -> ::std::result::Result<EntryTransaction13, crate::common::BuilderError> {
        ::std::result::Result::Ok(EntryTransaction13 {
            refs: self.refs,
            amt: self.amt,
            cdt_dbt_ind: self.cdt_dbt_ind,
            amt_dtls: self.amt_dtls,
            avlbty: self.avlbty,
            bk_tx_cd: self.bk_tx_cd,
            chrgs: self.chrgs,
            intrst: self.intrst,
            rltd_pties: self.rltd_pties,
            rltd_agts: self.rltd_agts,
            lcl_instrm: self.lcl_instrm,
            pmt_tp_inf: self.pmt_tp_inf,
            purp: self.purp,
            rltd_rmt_inf: self.rltd_rmt_inf,
            rmt_inf: self.rmt_inf,
            rltd_dts: self.rltd_dts,
            rltd_pric: self.rltd_pric,
            rltd_qties: self.rltd_qties,
            fin_instrm_id: self.fin_instrm_id,
            tax: self.tax,
            rtr_inf: self.rtr_inf,
            corp_actn: self.corp_actn,
            sfkpg_acct: self.sfkpg_acct,
            csh_dpst: self.csh_dpst,
            card_tx: self.card_tx,
            addtl_tx_inf: self.addtl_tx_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl EntryTransaction13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> EntryTransaction13Builder {
        EntryTransaction13Builder::default()
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
pub enum FinancialInstrumentQuantity1Choice {
    #[serde(rename = "Unit")]
    Unit(DecimalNumber),
    #[serde(rename = "FaceAmt")]
    FaceAmt(ImpliedCurrencyAndAmount),
    #[serde(rename = "AmtsdVal")]
    AmtsdVal(ImpliedCurrencyAndAmount),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FromToAmountRange1 {
    #[serde(rename = "FrAmt")]
    pub fr_amt: AmountRangeBoundary1,
    #[serde(rename = "ToAmt")]
    pub to_amt: AmountRangeBoundary1,
}
/// Builder for [`FromToAmountRange1`]. Construct via [`FromToAmountRange1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FromToAmountRange1Builder {
    fr_amt: ::std::option::Option<AmountRangeBoundary1>,
    to_amt: ::std::option::Option<AmountRangeBoundary1>,
}
impl FromToAmountRange1Builder {
    /// Set the `fr_amt` field.
    #[must_use]
    pub fn fr_amt(mut self, value: AmountRangeBoundary1) -> FromToAmountRange1Builder {
        self.fr_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to_amt` field.
    #[must_use]
    pub fn to_amt(mut self, value: AmountRangeBoundary1) -> FromToAmountRange1Builder {
        self.to_amt = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<FromToAmountRange1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr_amt.is_none() {
            missing.push("fr_amt".to_owned());
        }
        if self.to_amt.is_none() {
            missing.push("to_amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "FromToAmountRange1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(FromToAmountRange1 {
            fr_amt: self.fr_amt.unwrap(),
            to_amt: self.to_amt.unwrap(),
        })
    }
}
impl FromToAmountRange1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FromToAmountRange1Builder {
        FromToAmountRange1Builder::default()
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
pub struct GenericIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "SchmeNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schme_nm: Option<Max35Text>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericIdentification1`]. Construct via [`GenericIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    schme_nm: ::std::option::Option<Max35Text>,
    issr: ::std::option::Option<Max35Text>,
}
impl GenericIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `schme_nm` field.
    #[must_use]
    pub fn schme_nm(mut self, value: Max35Text) -> GenericIdentification1Builder {
        self.schme_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> GenericIdentification1Builder {
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
    ) -> ::std::result::Result<GenericIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericIdentification1 {
            id: self.id.unwrap(),
            schme_nm: self.schme_nm,
            issr: self.issr,
        })
    }
}
impl GenericIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericIdentification1Builder {
        GenericIdentification1Builder::default()
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
pub struct GenericIdentification32 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<PartyType3Code>,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<PartyType4Code>,
    #[serde(rename = "ShrtNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shrt_nm: Option<Max35Text>,
}
/// Builder for [`GenericIdentification32`]. Construct via [`GenericIdentification32::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericIdentification32Builder {
    id: ::std::option::Option<Max35Text>,
    tp: ::std::option::Option<PartyType3Code>,
    issr: ::std::option::Option<PartyType4Code>,
    shrt_nm: ::std::option::Option<Max35Text>,
}
impl GenericIdentification32Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> GenericIdentification32Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: PartyType3Code) -> GenericIdentification32Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: PartyType4Code) -> GenericIdentification32Builder {
        self.issr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `shrt_nm` field.
    #[must_use]
    pub fn shrt_nm(mut self, value: Max35Text) -> GenericIdentification32Builder {
        self.shrt_nm = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<GenericIdentification32, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GenericIdentification32".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GenericIdentification32 {
            id: self.id.unwrap(),
            tp: self.tp,
            issr: self.issr,
            shrt_nm: self.shrt_nm,
        })
    }
}
impl GenericIdentification32 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GenericIdentification32Builder {
        GenericIdentification32Builder::default()
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
pub struct GroupHeader81 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: ISODateTime,
    #[serde(rename = "MsgRcpt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_rcpt: Option<PartyIdentification135>,
    #[serde(rename = "MsgPgntn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_pgntn: Option<Pagination1>,
    #[serde(rename = "OrgnlBizQry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
    #[serde(rename = "AddtlInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf: Option<Max500Text>,
}
/// Builder for [`GroupHeader81`]. Construct via [`GroupHeader81::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GroupHeader81Builder {
    msg_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    msg_rcpt: ::std::option::Option<PartyIdentification135>,
    msg_pgntn: ::std::option::Option<Pagination1>,
    orgnl_biz_qry: ::std::option::Option<OriginalBusinessQuery1>,
    addtl_inf: ::std::option::Option<Max500Text>,
}
impl GroupHeader81Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> GroupHeader81Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> GroupHeader81Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_rcpt` field.
    #[must_use]
    pub fn msg_rcpt(mut self, value: PartyIdentification135) -> GroupHeader81Builder {
        self.msg_rcpt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_pgntn` field.
    #[must_use]
    pub fn msg_pgntn(mut self, value: Pagination1) -> GroupHeader81Builder {
        self.msg_pgntn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_biz_qry` field.
    #[must_use]
    pub fn orgnl_biz_qry(mut self, value: OriginalBusinessQuery1) -> GroupHeader81Builder {
        self.orgnl_biz_qry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field.
    #[must_use]
    pub fn addtl_inf(mut self, value: Max500Text) -> GroupHeader81Builder {
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
    pub fn build(self) -> ::std::result::Result<GroupHeader81, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.msg_id.is_none() {
            missing.push("msg_id".to_owned());
        }
        if self.cre_dt_tm.is_none() {
            missing.push("cre_dt_tm".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GroupHeader81".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GroupHeader81 {
            msg_id: self.msg_id.unwrap(),
            cre_dt_tm: self.cre_dt_tm.unwrap(),
            msg_rcpt: self.msg_rcpt,
            msg_pgntn: self.msg_pgntn,
            orgnl_biz_qry: self.orgnl_biz_qry,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl GroupHeader81 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GroupHeader81Builder {
        GroupHeader81Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IdentificationSource3Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalFinancialInstrumentIdentificationType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ImpliedCurrencyAmountRange1Choice {
    #[serde(rename = "FrAmt")]
    FrAmt(AmountRangeBoundary1),
    #[serde(rename = "ToAmt")]
    ToAmt(AmountRangeBoundary1),
    #[serde(rename = "FrToAmt")]
    FrToAmt(FromToAmountRange1),
    #[serde(rename = "EQAmt")]
    EQAmt(ImpliedCurrencyAndAmount),
    #[serde(rename = "NEQAmt")]
    NEQAmt(ImpliedCurrencyAndAmount),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InterestRecord2 {
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<crate::common::ChoiceWrapper<InterestType1Choice>>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<Rate4>,
    #[serde(rename = "FrToDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr_to_dt: Option<DateTimePeriod1>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<Max35Text>,
    #[serde(rename = "Tax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxCharges2>,
}
/// Builder for [`InterestRecord2`]. Construct via [`InterestRecord2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct InterestRecord2Builder {
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<InterestType1Choice>>,
    rate: ::std::option::Option<Rate4>,
    fr_to_dt: ::std::option::Option<DateTimePeriod1>,
    rsn: ::std::option::Option<Max35Text>,
    tax: ::std::option::Option<TaxCharges2>,
}
impl InterestRecord2Builder {
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> InterestRecord2Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> InterestRecord2Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<InterestType1Choice>,
    ) -> InterestRecord2Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rate` field.
    #[must_use]
    pub fn rate(mut self, value: Rate4) -> InterestRecord2Builder {
        self.rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr_to_dt` field.
    #[must_use]
    pub fn fr_to_dt(mut self, value: DateTimePeriod1) -> InterestRecord2Builder {
        self.fr_to_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(mut self, value: Max35Text) -> InterestRecord2Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxCharges2) -> InterestRecord2Builder {
        self.tax = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<InterestRecord2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.cdt_dbt_ind.is_none() {
            missing.push("cdt_dbt_ind".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "InterestRecord2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(InterestRecord2 {
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind.unwrap(),
            tp: self.tp,
            rate: self.rate,
            fr_to_dt: self.fr_to_dt,
            rsn: self.rsn,
            tax: self.tax,
        })
    }
}
impl InterestRecord2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> InterestRecord2Builder {
        InterestRecord2Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum InterestType1Choice {
    #[serde(rename = "Cd")]
    Cd(InterestType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum LocalInstrument2Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalLocalInstrument1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MessageIdentification2 {
    #[serde(rename = "MsgNmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<Max35Text>,
    #[serde(rename = "MsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<Max35Text>,
}
/// Builder for [`MessageIdentification2`]. Construct via [`MessageIdentification2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct MessageIdentification2Builder {
    msg_nm_id: ::std::option::Option<Max35Text>,
    msg_id: ::std::option::Option<Max35Text>,
}
impl MessageIdentification2Builder {
    /// Set the `msg_nm_id` field.
    #[must_use]
    pub fn msg_nm_id(mut self, value: Max35Text) -> MessageIdentification2Builder {
        self.msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> MessageIdentification2Builder {
        self.msg_id = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<MessageIdentification2, crate::common::BuilderError> {
        ::std::result::Result::Ok(MessageIdentification2 {
            msg_nm_id: self.msg_nm_id,
            msg_id: self.msg_id,
        })
    }
}
impl MessageIdentification2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> MessageIdentification2Builder {
        MessageIdentification2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NameAndAddress16 {
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "Adr")]
    pub adr: PostalAddress24,
}
/// Builder for [`NameAndAddress16`]. Construct via [`NameAndAddress16::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct NameAndAddress16Builder {
    nm: ::std::option::Option<Max140Text>,
    adr: ::std::option::Option<PostalAddress24>,
}
impl NameAndAddress16Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> NameAndAddress16Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `adr` field.
    #[must_use]
    pub fn adr(mut self, value: PostalAddress24) -> NameAndAddress16Builder {
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
    pub fn build(self) -> ::std::result::Result<NameAndAddress16, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.nm.is_none() {
            missing.push("nm".to_owned());
        }
        if self.adr.is_none() {
            missing.push("adr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "NameAndAddress16".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(NameAndAddress16 {
            nm: self.nm.unwrap(),
            adr: self.adr.unwrap(),
        })
    }
}
impl NameAndAddress16 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> NameAndAddress16Builder {
        NameAndAddress16Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NumberAndSumOfTransactions1 {
    #[serde(rename = "NbOfNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<Max15NumericText>,
    #[serde(rename = "Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<DecimalNumber>,
}
/// Builder for [`NumberAndSumOfTransactions1`]. Construct via [`NumberAndSumOfTransactions1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct NumberAndSumOfTransactions1Builder {
    nb_of_ntries: ::std::option::Option<Max15NumericText>,
    sum: ::std::option::Option<DecimalNumber>,
}
impl NumberAndSumOfTransactions1Builder {
    /// Set the `nb_of_ntries` field.
    #[must_use]
    pub fn nb_of_ntries(mut self, value: Max15NumericText) -> NumberAndSumOfTransactions1Builder {
        self.nb_of_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sum` field.
    #[must_use]
    pub fn sum(mut self, value: DecimalNumber) -> NumberAndSumOfTransactions1Builder {
        self.sum = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<NumberAndSumOfTransactions1, crate::common::BuilderError> {
        ::std::result::Result::Ok(NumberAndSumOfTransactions1 {
            nb_of_ntries: self.nb_of_ntries,
            sum: self.sum,
        })
    }
}
impl NumberAndSumOfTransactions1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> NumberAndSumOfTransactions1Builder {
        NumberAndSumOfTransactions1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NumberAndSumOfTransactions4 {
    #[serde(rename = "NbOfNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<Max15NumericText>,
    #[serde(rename = "Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<DecimalNumber>,
    #[serde(rename = "TtlNetNtry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry: Option<AmountAndDirection35>,
}
/// Builder for [`NumberAndSumOfTransactions4`]. Construct via [`NumberAndSumOfTransactions4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct NumberAndSumOfTransactions4Builder {
    nb_of_ntries: ::std::option::Option<Max15NumericText>,
    sum: ::std::option::Option<DecimalNumber>,
    ttl_net_ntry: ::std::option::Option<AmountAndDirection35>,
}
impl NumberAndSumOfTransactions4Builder {
    /// Set the `nb_of_ntries` field.
    #[must_use]
    pub fn nb_of_ntries(mut self, value: Max15NumericText) -> NumberAndSumOfTransactions4Builder {
        self.nb_of_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sum` field.
    #[must_use]
    pub fn sum(mut self, value: DecimalNumber) -> NumberAndSumOfTransactions4Builder {
        self.sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_net_ntry` field.
    #[must_use]
    pub fn ttl_net_ntry(
        mut self,
        value: AmountAndDirection35,
    ) -> NumberAndSumOfTransactions4Builder {
        self.ttl_net_ntry = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<NumberAndSumOfTransactions4, crate::common::BuilderError> {
        ::std::result::Result::Ok(NumberAndSumOfTransactions4 {
            nb_of_ntries: self.nb_of_ntries,
            sum: self.sum,
            ttl_net_ntry: self.ttl_net_ntry,
        })
    }
}
impl NumberAndSumOfTransactions4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> NumberAndSumOfTransactions4Builder {
        NumberAndSumOfTransactions4Builder::default()
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
pub struct OriginalAndCurrentQuantities1 {
    #[serde(rename = "FaceAmt")]
    pub face_amt: ImpliedCurrencyAndAmount,
    #[serde(rename = "AmtsdVal")]
    pub amtsd_val: ImpliedCurrencyAndAmount,
}
/// Builder for [`OriginalAndCurrentQuantities1`]. Construct via [`OriginalAndCurrentQuantities1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalAndCurrentQuantities1Builder {
    face_amt: ::std::option::Option<ImpliedCurrencyAndAmount>,
    amtsd_val: ::std::option::Option<ImpliedCurrencyAndAmount>,
}
impl OriginalAndCurrentQuantities1Builder {
    /// Set the `face_amt` field.
    #[must_use]
    pub fn face_amt(
        mut self,
        value: ImpliedCurrencyAndAmount,
    ) -> OriginalAndCurrentQuantities1Builder {
        self.face_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amtsd_val` field.
    #[must_use]
    pub fn amtsd_val(
        mut self,
        value: ImpliedCurrencyAndAmount,
    ) -> OriginalAndCurrentQuantities1Builder {
        self.amtsd_val = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<OriginalAndCurrentQuantities1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.face_amt.is_none() {
            missing.push("face_amt".to_owned());
        }
        if self.amtsd_val.is_none() {
            missing.push("amtsd_val".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OriginalAndCurrentQuantities1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OriginalAndCurrentQuantities1 {
            face_amt: self.face_amt.unwrap(),
            amtsd_val: self.amtsd_val.unwrap(),
        })
    }
}
impl OriginalAndCurrentQuantities1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalAndCurrentQuantities1Builder {
        OriginalAndCurrentQuantities1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OriginalBusinessQuery1 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "MsgNmId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_nm_id: Option<Max35Text>,
    #[serde(rename = "CreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cre_dt_tm: Option<ISODateTime>,
}
/// Builder for [`OriginalBusinessQuery1`]. Construct via [`OriginalBusinessQuery1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalBusinessQuery1Builder {
    msg_id: ::std::option::Option<Max35Text>,
    msg_nm_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
}
impl OriginalBusinessQuery1Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> OriginalBusinessQuery1Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_nm_id` field.
    #[must_use]
    pub fn msg_nm_id(mut self, value: Max35Text) -> OriginalBusinessQuery1Builder {
        self.msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> OriginalBusinessQuery1Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<OriginalBusinessQuery1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.msg_id.is_none() {
            missing.push("msg_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OriginalBusinessQuery1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OriginalBusinessQuery1 {
            msg_id: self.msg_id.unwrap(),
            msg_nm_id: self.msg_nm_id,
            cre_dt_tm: self.cre_dt_tm,
        })
    }
}
impl OriginalBusinessQuery1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalBusinessQuery1Builder {
        OriginalBusinessQuery1Builder::default()
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OtherIdentification1 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Sfx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sfx: Option<Max16Text>,
    #[serde(rename = "Tp")]
    pub tp: crate::common::ChoiceWrapper<IdentificationSource3Choice>,
}
/// Builder for [`OtherIdentification1`]. Construct via [`OtherIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OtherIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    sfx: ::std::option::Option<Max16Text>,
    tp: ::std::option::Option<crate::common::ChoiceWrapper<IdentificationSource3Choice>>,
}
impl OtherIdentification1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> OtherIdentification1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sfx` field.
    #[must_use]
    pub fn sfx(mut self, value: Max16Text) -> OtherIdentification1Builder {
        self.sfx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<IdentificationSource3Choice>,
    ) -> OtherIdentification1Builder {
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
    pub fn build(self) -> ::std::result::Result<OtherIdentification1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OtherIdentification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OtherIdentification1 {
            id: self.id.unwrap(),
            sfx: self.sfx,
            tp: self.tp.unwrap(),
        })
    }
}
impl OtherIdentification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OtherIdentification1Builder {
        OtherIdentification1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Pagination1 {
    #[serde(rename = "PgNb")]
    pub pg_nb: Max5NumericText,
    #[serde(rename = "LastPgInd")]
    pub last_pg_ind: YesNoIndicator,
}
/// Builder for [`Pagination1`]. Construct via [`Pagination1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Pagination1Builder {
    pg_nb: ::std::option::Option<Max5NumericText>,
    last_pg_ind: ::std::option::Option<YesNoIndicator>,
}
impl Pagination1Builder {
    /// Set the `pg_nb` field.
    #[must_use]
    pub fn pg_nb(mut self, value: Max5NumericText) -> Pagination1Builder {
        self.pg_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `last_pg_ind` field.
    #[must_use]
    pub fn last_pg_ind(mut self, value: YesNoIndicator) -> Pagination1Builder {
        self.last_pg_ind = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<Pagination1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pg_nb.is_none() {
            missing.push("pg_nb".to_owned());
        }
        if self.last_pg_ind.is_none() {
            missing.push("last_pg_ind".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Pagination1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Pagination1 {
            pg_nb: self.pg_nb.unwrap(),
            last_pg_ind: self.last_pg_ind.unwrap(),
        })
    }
}
impl Pagination1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Pagination1Builder {
        Pagination1Builder::default()
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
pub struct PaymentCard4 {
    #[serde(rename = "PlainCardData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plain_card_data: Option<PlainCardData1>,
    #[serde(rename = "CardCtryCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_ctry_cd: Option<Exact3NumericText>,
    #[serde(rename = "CardBrnd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_brnd: Option<GenericIdentification1>,
    #[serde(rename = "AddtlCardData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_card_data: Option<Max70Text>,
}
/// Builder for [`PaymentCard4`]. Construct via [`PaymentCard4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentCard4Builder {
    plain_card_data: ::std::option::Option<PlainCardData1>,
    card_ctry_cd: ::std::option::Option<Exact3NumericText>,
    card_brnd: ::std::option::Option<GenericIdentification1>,
    addtl_card_data: ::std::option::Option<Max70Text>,
}
impl PaymentCard4Builder {
    /// Set the `plain_card_data` field.
    #[must_use]
    pub fn plain_card_data(mut self, value: PlainCardData1) -> PaymentCard4Builder {
        self.plain_card_data = ::std::option::Option::Some(value);
        self
    }
    /// Set the `card_ctry_cd` field.
    #[must_use]
    pub fn card_ctry_cd(mut self, value: Exact3NumericText) -> PaymentCard4Builder {
        self.card_ctry_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `card_brnd` field.
    #[must_use]
    pub fn card_brnd(mut self, value: GenericIdentification1) -> PaymentCard4Builder {
        self.card_brnd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_card_data` field.
    #[must_use]
    pub fn addtl_card_data(mut self, value: Max70Text) -> PaymentCard4Builder {
        self.addtl_card_data = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<PaymentCard4, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentCard4 {
            plain_card_data: self.plain_card_data,
            card_ctry_cd: self.card_ctry_cd,
            card_brnd: self.card_brnd,
            addtl_card_data: self.addtl_card_data,
        })
    }
}
impl PaymentCard4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentCard4Builder {
        PaymentCard4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentContext3 {
    #[serde(rename = "CardPres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "CrdhldrPres")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crdhldr_pres: Option<TrueFalseIndicator>,
    #[serde(rename = "OnLineCntxt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_line_cntxt: Option<TrueFalseIndicator>,
    #[serde(rename = "AttndncCntxt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attndnc_cntxt: Option<AttendanceContext1Code>,
    #[serde(rename = "TxEnvt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_envt: Option<TransactionEnvironment1Code>,
    #[serde(rename = "TxChanl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_chanl: Option<TransactionChannel1Code>,
    #[serde(rename = "AttndntMsgCpbl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attndnt_msg_cpbl: Option<TrueFalseIndicator>,
    #[serde(rename = "AttndntLang")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attndnt_lang: Option<ISO2ALanguageCode>,
    #[serde(rename = "CardDataNtryMd")]
    pub card_data_ntry_md: CardDataReading1Code,
    #[serde(rename = "FllbckInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fllbck_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "AuthntcnMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authntcn_mtd: Option<CardholderAuthentication2>,
}
/// Builder for [`PaymentContext3`]. Construct via [`PaymentContext3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentContext3Builder {
    card_pres: ::std::option::Option<TrueFalseIndicator>,
    crdhldr_pres: ::std::option::Option<TrueFalseIndicator>,
    on_line_cntxt: ::std::option::Option<TrueFalseIndicator>,
    attndnc_cntxt: ::std::option::Option<AttendanceContext1Code>,
    tx_envt: ::std::option::Option<TransactionEnvironment1Code>,
    tx_chanl: ::std::option::Option<TransactionChannel1Code>,
    attndnt_msg_cpbl: ::std::option::Option<TrueFalseIndicator>,
    attndnt_lang: ::std::option::Option<ISO2ALanguageCode>,
    card_data_ntry_md: ::std::option::Option<CardDataReading1Code>,
    fllbck_ind: ::std::option::Option<TrueFalseIndicator>,
    authntcn_mtd: ::std::option::Option<CardholderAuthentication2>,
}
impl PaymentContext3Builder {
    /// Set the `card_pres` field.
    #[must_use]
    pub fn card_pres(mut self, value: TrueFalseIndicator) -> PaymentContext3Builder {
        self.card_pres = ::std::option::Option::Some(value);
        self
    }
    /// Set the `crdhldr_pres` field.
    #[must_use]
    pub fn crdhldr_pres(mut self, value: TrueFalseIndicator) -> PaymentContext3Builder {
        self.crdhldr_pres = ::std::option::Option::Some(value);
        self
    }
    /// Set the `on_line_cntxt` field.
    #[must_use]
    pub fn on_line_cntxt(mut self, value: TrueFalseIndicator) -> PaymentContext3Builder {
        self.on_line_cntxt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `attndnc_cntxt` field.
    #[must_use]
    pub fn attndnc_cntxt(mut self, value: AttendanceContext1Code) -> PaymentContext3Builder {
        self.attndnc_cntxt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_envt` field.
    #[must_use]
    pub fn tx_envt(mut self, value: TransactionEnvironment1Code) -> PaymentContext3Builder {
        self.tx_envt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_chanl` field.
    #[must_use]
    pub fn tx_chanl(mut self, value: TransactionChannel1Code) -> PaymentContext3Builder {
        self.tx_chanl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `attndnt_msg_cpbl` field.
    #[must_use]
    pub fn attndnt_msg_cpbl(mut self, value: TrueFalseIndicator) -> PaymentContext3Builder {
        self.attndnt_msg_cpbl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `attndnt_lang` field.
    #[must_use]
    pub fn attndnt_lang(mut self, value: ISO2ALanguageCode) -> PaymentContext3Builder {
        self.attndnt_lang = ::std::option::Option::Some(value);
        self
    }
    /// Set the `card_data_ntry_md` field.
    #[must_use]
    pub fn card_data_ntry_md(mut self, value: CardDataReading1Code) -> PaymentContext3Builder {
        self.card_data_ntry_md = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fllbck_ind` field.
    #[must_use]
    pub fn fllbck_ind(mut self, value: TrueFalseIndicator) -> PaymentContext3Builder {
        self.fllbck_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authntcn_mtd` field.
    #[must_use]
    pub fn authntcn_mtd(mut self, value: CardholderAuthentication2) -> PaymentContext3Builder {
        self.authntcn_mtd = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<PaymentContext3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.card_data_ntry_md.is_none() {
            missing.push("card_data_ntry_md".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentContext3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentContext3 {
            card_pres: self.card_pres,
            crdhldr_pres: self.crdhldr_pres,
            on_line_cntxt: self.on_line_cntxt,
            attndnc_cntxt: self.attndnc_cntxt,
            tx_envt: self.tx_envt,
            tx_chanl: self.tx_chanl,
            attndnt_msg_cpbl: self.attndnt_msg_cpbl,
            attndnt_lang: self.attndnt_lang,
            card_data_ntry_md: self.card_data_ntry_md.unwrap(),
            fllbck_ind: self.fllbck_ind,
            authntcn_mtd: self.authntcn_mtd,
        })
    }
}
impl PaymentContext3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentContext3Builder {
        PaymentContext3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentReturnReason5 {
    #[serde(rename = "OrgnlBkTxCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_bk_tx_cd: Option<BankTransactionCodeStructure4>,
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
/// Builder for [`PaymentReturnReason5`]. Construct via [`PaymentReturnReason5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentReturnReason5Builder {
    orgnl_bk_tx_cd: ::std::option::Option<BankTransactionCodeStructure4>,
    orgtr: ::std::option::Option<PartyIdentification135>,
    rsn: ::std::option::Option<crate::common::ChoiceWrapper<ReturnReason5Choice>>,
    addtl_inf: ::std::vec::Vec<Max105Text>,
}
impl PaymentReturnReason5Builder {
    /// Set the `orgnl_bk_tx_cd` field.
    #[must_use]
    pub fn orgnl_bk_tx_cd(
        mut self,
        value: BankTransactionCodeStructure4,
    ) -> PaymentReturnReason5Builder {
        self.orgnl_bk_tx_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgtr` field.
    #[must_use]
    pub fn orgtr(mut self, value: PartyIdentification135) -> PaymentReturnReason5Builder {
        self.orgtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(
        mut self,
        value: crate::common::ChoiceWrapper<ReturnReason5Choice>,
    ) -> PaymentReturnReason5Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field (replaces any previously added items).
    #[must_use]
    pub fn addtl_inf(mut self, value: ::std::vec::Vec<Max105Text>) -> PaymentReturnReason5Builder {
        self.addtl_inf = value;
        self
    }
    /// Append one item to the `addtl_inf` field.
    #[must_use]
    pub fn add_addtl_inf(mut self, value: Max105Text) -> PaymentReturnReason5Builder {
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
    pub fn build(self) -> ::std::result::Result<PaymentReturnReason5, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentReturnReason5 {
            orgnl_bk_tx_cd: self.orgnl_bk_tx_cd,
            orgtr: self.orgtr,
            rsn: self.rsn,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl PaymentReturnReason5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentReturnReason5Builder {
        PaymentReturnReason5Builder::default()
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
pub struct PlainCardData1 {
    #[serde(rename = "PAN")]
    pub pan: Min8Max28NumericText,
    #[serde(rename = "CardSeqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_seq_nb: Option<Min2Max3NumericText>,
    #[serde(rename = "FctvDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fctv_dt: Option<ISOYearMonth>,
    #[serde(rename = "XpryDt")]
    pub xpry_dt: ISOYearMonth,
    #[serde(rename = "SvcCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub svc_cd: Option<Exact3NumericText>,
    #[serde(rename = "TrckData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub trck_data: Vec<TrackData1>,
    #[serde(rename = "CardSctyCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_scty_cd: Option<CardSecurityInformation1>,
}
/// Builder for [`PlainCardData1`]. Construct via [`PlainCardData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PlainCardData1Builder {
    pan: ::std::option::Option<Min8Max28NumericText>,
    card_seq_nb: ::std::option::Option<Min2Max3NumericText>,
    fctv_dt: ::std::option::Option<ISOYearMonth>,
    xpry_dt: ::std::option::Option<ISOYearMonth>,
    svc_cd: ::std::option::Option<Exact3NumericText>,
    trck_data: ::std::vec::Vec<TrackData1>,
    card_scty_cd: ::std::option::Option<CardSecurityInformation1>,
}
impl PlainCardData1Builder {
    /// Set the `pan` field.
    #[must_use]
    pub fn pan(mut self, value: Min8Max28NumericText) -> PlainCardData1Builder {
        self.pan = ::std::option::Option::Some(value);
        self
    }
    /// Set the `card_seq_nb` field.
    #[must_use]
    pub fn card_seq_nb(mut self, value: Min2Max3NumericText) -> PlainCardData1Builder {
        self.card_seq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fctv_dt` field.
    #[must_use]
    pub fn fctv_dt(mut self, value: ISOYearMonth) -> PlainCardData1Builder {
        self.fctv_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xpry_dt` field.
    #[must_use]
    pub fn xpry_dt(mut self, value: ISOYearMonth) -> PlainCardData1Builder {
        self.xpry_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `svc_cd` field.
    #[must_use]
    pub fn svc_cd(mut self, value: Exact3NumericText) -> PlainCardData1Builder {
        self.svc_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trck_data` field (replaces any previously added items).
    #[must_use]
    pub fn trck_data(mut self, value: ::std::vec::Vec<TrackData1>) -> PlainCardData1Builder {
        self.trck_data = value;
        self
    }
    /// Append one item to the `trck_data` field.
    #[must_use]
    pub fn add_trck_data(mut self, value: TrackData1) -> PlainCardData1Builder {
        self.trck_data.push(value);
        self
    }
    /// Set the `card_scty_cd` field.
    #[must_use]
    pub fn card_scty_cd(mut self, value: CardSecurityInformation1) -> PlainCardData1Builder {
        self.card_scty_cd = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<PlainCardData1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pan.is_none() {
            missing.push("pan".to_owned());
        }
        if self.xpry_dt.is_none() {
            missing.push("xpry_dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PlainCardData1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PlainCardData1 {
            pan: self.pan.unwrap(),
            card_seq_nb: self.card_seq_nb,
            fctv_dt: self.fctv_dt,
            xpry_dt: self.xpry_dt.unwrap(),
            svc_cd: self.svc_cd,
            trck_data: self.trck_data,
            card_scty_cd: self.card_scty_cd,
        })
    }
}
impl PlainCardData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PlainCardData1Builder {
        PlainCardData1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointOfInteraction1 {
    #[serde(rename = "Id")]
    pub id: GenericIdentification32,
    #[serde(rename = "SysNm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_nm: Option<Max70Text>,
    #[serde(rename = "GrpId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_id: Option<Max35Text>,
    #[serde(rename = "Cpblties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpblties: Option<PointOfInteractionCapabilities1>,
    #[serde(rename = "Cmpnt")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cmpnt: Vec<PointOfInteractionComponent1>,
}
/// Builder for [`PointOfInteraction1`]. Construct via [`PointOfInteraction1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PointOfInteraction1Builder {
    id: ::std::option::Option<GenericIdentification32>,
    sys_nm: ::std::option::Option<Max70Text>,
    grp_id: ::std::option::Option<Max35Text>,
    cpblties: ::std::option::Option<PointOfInteractionCapabilities1>,
    cmpnt: ::std::vec::Vec<PointOfInteractionComponent1>,
}
impl PointOfInteraction1Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: GenericIdentification32) -> PointOfInteraction1Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sys_nm` field.
    #[must_use]
    pub fn sys_nm(mut self, value: Max70Text) -> PointOfInteraction1Builder {
        self.sys_nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grp_id` field.
    #[must_use]
    pub fn grp_id(mut self, value: Max35Text) -> PointOfInteraction1Builder {
        self.grp_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cpblties` field.
    #[must_use]
    pub fn cpblties(
        mut self,
        value: PointOfInteractionCapabilities1,
    ) -> PointOfInteraction1Builder {
        self.cpblties = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cmpnt` field (replaces any previously added items).
    #[must_use]
    pub fn cmpnt(
        mut self,
        value: ::std::vec::Vec<PointOfInteractionComponent1>,
    ) -> PointOfInteraction1Builder {
        self.cmpnt = value;
        self
    }
    /// Append one item to the `cmpnt` field.
    #[must_use]
    pub fn add_cmpnt(mut self, value: PointOfInteractionComponent1) -> PointOfInteraction1Builder {
        self.cmpnt.push(value);
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
    pub fn build(self) -> ::std::result::Result<PointOfInteraction1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PointOfInteraction1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PointOfInteraction1 {
            id: self.id.unwrap(),
            sys_nm: self.sys_nm,
            grp_id: self.grp_id,
            cpblties: self.cpblties,
            cmpnt: self.cmpnt,
        })
    }
}
impl PointOfInteraction1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PointOfInteraction1Builder {
        PointOfInteraction1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointOfInteractionCapabilities1 {
    #[serde(rename = "CardRdngCpblties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub card_rdng_cpblties: Vec<CardDataReading1Code>,
    #[serde(rename = "CrdhldrVrfctnCpblties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub crdhldr_vrfctn_cpblties: Vec<CardholderVerificationCapability1Code>,
    #[serde(rename = "OnLineCpblties")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_line_cpblties: Option<OnLineCapability1Code>,
    #[serde(rename = "DispCpblties")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub disp_cpblties: Vec<DisplayCapabilities1>,
    #[serde(rename = "PrtLineWidth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prt_line_width: Option<Max3NumericText>,
}
/// Builder for [`PointOfInteractionCapabilities1`]. Construct via [`PointOfInteractionCapabilities1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PointOfInteractionCapabilities1Builder {
    card_rdng_cpblties: ::std::vec::Vec<CardDataReading1Code>,
    crdhldr_vrfctn_cpblties: ::std::vec::Vec<CardholderVerificationCapability1Code>,
    on_line_cpblties: ::std::option::Option<OnLineCapability1Code>,
    disp_cpblties: ::std::vec::Vec<DisplayCapabilities1>,
    prt_line_width: ::std::option::Option<Max3NumericText>,
}
impl PointOfInteractionCapabilities1Builder {
    /// Set the `card_rdng_cpblties` field (replaces any previously added items).
    #[must_use]
    pub fn card_rdng_cpblties(
        mut self,
        value: ::std::vec::Vec<CardDataReading1Code>,
    ) -> PointOfInteractionCapabilities1Builder {
        self.card_rdng_cpblties = value;
        self
    }
    /// Append one item to the `card_rdng_cpblties` field.
    #[must_use]
    pub fn add_card_rdng_cpblties(
        mut self,
        value: CardDataReading1Code,
    ) -> PointOfInteractionCapabilities1Builder {
        self.card_rdng_cpblties.push(value);
        self
    }
    /// Set the `crdhldr_vrfctn_cpblties` field (replaces any previously added items).
    #[must_use]
    pub fn crdhldr_vrfctn_cpblties(
        mut self,
        value: ::std::vec::Vec<CardholderVerificationCapability1Code>,
    ) -> PointOfInteractionCapabilities1Builder {
        self.crdhldr_vrfctn_cpblties = value;
        self
    }
    /// Append one item to the `crdhldr_vrfctn_cpblties` field.
    #[must_use]
    pub fn add_crdhldr_vrfctn_cpblties(
        mut self,
        value: CardholderVerificationCapability1Code,
    ) -> PointOfInteractionCapabilities1Builder {
        self.crdhldr_vrfctn_cpblties.push(value);
        self
    }
    /// Set the `on_line_cpblties` field.
    #[must_use]
    pub fn on_line_cpblties(
        mut self,
        value: OnLineCapability1Code,
    ) -> PointOfInteractionCapabilities1Builder {
        self.on_line_cpblties = ::std::option::Option::Some(value);
        self
    }
    /// Set the `disp_cpblties` field (replaces any previously added items).
    #[must_use]
    pub fn disp_cpblties(
        mut self,
        value: ::std::vec::Vec<DisplayCapabilities1>,
    ) -> PointOfInteractionCapabilities1Builder {
        self.disp_cpblties = value;
        self
    }
    /// Append one item to the `disp_cpblties` field.
    #[must_use]
    pub fn add_disp_cpblties(
        mut self,
        value: DisplayCapabilities1,
    ) -> PointOfInteractionCapabilities1Builder {
        self.disp_cpblties.push(value);
        self
    }
    /// Set the `prt_line_width` field.
    #[must_use]
    pub fn prt_line_width(
        mut self,
        value: Max3NumericText,
    ) -> PointOfInteractionCapabilities1Builder {
        self.prt_line_width = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<PointOfInteractionCapabilities1, crate::common::BuilderError> {
        ::std::result::Result::Ok(PointOfInteractionCapabilities1 {
            card_rdng_cpblties: self.card_rdng_cpblties,
            crdhldr_vrfctn_cpblties: self.crdhldr_vrfctn_cpblties,
            on_line_cpblties: self.on_line_cpblties,
            disp_cpblties: self.disp_cpblties,
            prt_line_width: self.prt_line_width,
        })
    }
}
impl PointOfInteractionCapabilities1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PointOfInteractionCapabilities1Builder {
        PointOfInteractionCapabilities1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PointOfInteractionComponent1 {
    #[serde(rename = "POICmpntTp")]
    pub poi_cmpnt_tp: POIComponentType1Code,
    #[serde(rename = "ManfctrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manfctr_id: Option<Max35Text>,
    #[serde(rename = "Mdl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mdl: Option<Max35Text>,
    #[serde(rename = "VrsnNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrsn_nb: Option<Max16Text>,
    #[serde(rename = "SrlNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub srl_nb: Option<Max35Text>,
    #[serde(rename = "ApprvlNb")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub apprvl_nb: Vec<Max70Text>,
}
/// Builder for [`PointOfInteractionComponent1`]. Construct via [`PointOfInteractionComponent1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PointOfInteractionComponent1Builder {
    poi_cmpnt_tp: ::std::option::Option<POIComponentType1Code>,
    manfctr_id: ::std::option::Option<Max35Text>,
    mdl: ::std::option::Option<Max35Text>,
    vrsn_nb: ::std::option::Option<Max16Text>,
    srl_nb: ::std::option::Option<Max35Text>,
    apprvl_nb: ::std::vec::Vec<Max70Text>,
}
impl PointOfInteractionComponent1Builder {
    /// Set the `poi_cmpnt_tp` field.
    #[must_use]
    pub fn poi_cmpnt_tp(
        mut self,
        value: POIComponentType1Code,
    ) -> PointOfInteractionComponent1Builder {
        self.poi_cmpnt_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `manfctr_id` field.
    #[must_use]
    pub fn manfctr_id(mut self, value: Max35Text) -> PointOfInteractionComponent1Builder {
        self.manfctr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mdl` field.
    #[must_use]
    pub fn mdl(mut self, value: Max35Text) -> PointOfInteractionComponent1Builder {
        self.mdl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `vrsn_nb` field.
    #[must_use]
    pub fn vrsn_nb(mut self, value: Max16Text) -> PointOfInteractionComponent1Builder {
        self.vrsn_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `srl_nb` field.
    #[must_use]
    pub fn srl_nb(mut self, value: Max35Text) -> PointOfInteractionComponent1Builder {
        self.srl_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `apprvl_nb` field (replaces any previously added items).
    #[must_use]
    pub fn apprvl_nb(
        mut self,
        value: ::std::vec::Vec<Max70Text>,
    ) -> PointOfInteractionComponent1Builder {
        self.apprvl_nb = value;
        self
    }
    /// Append one item to the `apprvl_nb` field.
    #[must_use]
    pub fn add_apprvl_nb(mut self, value: Max70Text) -> PointOfInteractionComponent1Builder {
        self.apprvl_nb.push(value);
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
    ) -> ::std::result::Result<PointOfInteractionComponent1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.poi_cmpnt_tp.is_none() {
            missing.push("poi_cmpnt_tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PointOfInteractionComponent1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PointOfInteractionComponent1 {
            poi_cmpnt_tp: self.poi_cmpnt_tp.unwrap(),
            manfctr_id: self.manfctr_id,
            mdl: self.mdl,
            vrsn_nb: self.vrsn_nb,
            srl_nb: self.srl_nb,
            apprvl_nb: self.apprvl_nb,
        })
    }
}
impl PointOfInteractionComponent1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PointOfInteractionComponent1Builder {
        PointOfInteractionComponent1Builder::default()
    }
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
pub struct Price7 {
    #[serde(rename = "Tp")]
    pub tp: crate::common::ChoiceWrapper<YieldedOrValueType1Choice>,
    #[serde(rename = "Val")]
    pub val: crate::common::ChoiceWrapper<PriceRateOrAmount3Choice>,
}
/// Builder for [`Price7`]. Construct via [`Price7::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Price7Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<YieldedOrValueType1Choice>>,
    val: ::std::option::Option<crate::common::ChoiceWrapper<PriceRateOrAmount3Choice>>,
}
impl Price7Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(
        mut self,
        value: crate::common::ChoiceWrapper<YieldedOrValueType1Choice>,
    ) -> Price7Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `val` field.
    #[must_use]
    pub fn val(
        mut self,
        value: crate::common::ChoiceWrapper<PriceRateOrAmount3Choice>,
    ) -> Price7Builder {
        self.val = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<Price7, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.val.is_none() {
            missing.push("val".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Price7".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Price7 {
            tp: self.tp.unwrap(),
            val: self.val.unwrap(),
        })
    }
}
impl Price7 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Price7Builder {
        Price7Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PriceRateOrAmount3Choice {
    #[serde(rename = "Rate")]
    Rate(PercentageRate),
    #[serde(rename = "Amt")]
    Amt(ActiveOrHistoricCurrencyAnd13DecimalAmount),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Product2 {
    #[serde(rename = "PdctCd")]
    pub pdct_cd: Max70Text,
    #[serde(rename = "UnitOfMeasr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measr: Option<UnitOfMeasure1Code>,
    #[serde(rename = "PdctQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdct_qty: Option<DecimalNumber>,
    #[serde(rename = "UnitPric")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_pric: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "PdctAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pdct_amt: Option<ImpliedCurrencyAndAmount>,
    #[serde(rename = "TaxTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_tp: Option<Max35Text>,
    #[serde(rename = "AddtlPdctInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_pdct_inf: Option<Max35Text>,
}
/// Builder for [`Product2`]. Construct via [`Product2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Product2Builder {
    pdct_cd: ::std::option::Option<Max70Text>,
    unit_of_measr: ::std::option::Option<UnitOfMeasure1Code>,
    pdct_qty: ::std::option::Option<DecimalNumber>,
    unit_pric: ::std::option::Option<ImpliedCurrencyAndAmount>,
    pdct_amt: ::std::option::Option<ImpliedCurrencyAndAmount>,
    tax_tp: ::std::option::Option<Max35Text>,
    addtl_pdct_inf: ::std::option::Option<Max35Text>,
}
impl Product2Builder {
    /// Set the `pdct_cd` field.
    #[must_use]
    pub fn pdct_cd(mut self, value: Max70Text) -> Product2Builder {
        self.pdct_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `unit_of_measr` field.
    #[must_use]
    pub fn unit_of_measr(mut self, value: UnitOfMeasure1Code) -> Product2Builder {
        self.unit_of_measr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pdct_qty` field.
    #[must_use]
    pub fn pdct_qty(mut self, value: DecimalNumber) -> Product2Builder {
        self.pdct_qty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `unit_pric` field.
    #[must_use]
    pub fn unit_pric(mut self, value: ImpliedCurrencyAndAmount) -> Product2Builder {
        self.unit_pric = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pdct_amt` field.
    #[must_use]
    pub fn pdct_amt(mut self, value: ImpliedCurrencyAndAmount) -> Product2Builder {
        self.pdct_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tax_tp` field.
    #[must_use]
    pub fn tax_tp(mut self, value: Max35Text) -> Product2Builder {
        self.tax_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_pdct_inf` field.
    #[must_use]
    pub fn addtl_pdct_inf(mut self, value: Max35Text) -> Product2Builder {
        self.addtl_pdct_inf = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<Product2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pdct_cd.is_none() {
            missing.push("pdct_cd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Product2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Product2 {
            pdct_cd: self.pdct_cd.unwrap(),
            unit_of_measr: self.unit_of_measr,
            pdct_qty: self.pdct_qty,
            unit_pric: self.unit_pric,
            pdct_amt: self.pdct_amt,
            tax_tp: self.tax_tp,
            addtl_pdct_inf: self.addtl_pdct_inf,
        })
    }
}
impl Product2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Product2Builder {
        Product2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryAgent4 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Agt")]
    pub agt: BranchAndFinancialInstitutionIdentification6,
}
/// Builder for [`ProprietaryAgent4`]. Construct via [`ProprietaryAgent4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryAgent4Builder {
    tp: ::std::option::Option<Max35Text>,
    agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
}
impl ProprietaryAgent4Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryAgent4Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `agt` field.
    #[must_use]
    pub fn agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> ProprietaryAgent4Builder {
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
    pub fn build(self) -> ::std::result::Result<ProprietaryAgent4, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.agt.is_none() {
            missing.push("agt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryAgent4".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryAgent4 {
            tp: self.tp.unwrap(),
            agt: self.agt.unwrap(),
        })
    }
}
impl ProprietaryAgent4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryAgent4Builder {
        ProprietaryAgent4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryBankTransactionCodeStructure1 {
    #[serde(rename = "Cd")]
    pub cd: Max35Text,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`ProprietaryBankTransactionCodeStructure1`]. Construct via [`ProprietaryBankTransactionCodeStructure1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryBankTransactionCodeStructure1Builder {
    cd: ::std::option::Option<Max35Text>,
    issr: ::std::option::Option<Max35Text>,
}
impl ProprietaryBankTransactionCodeStructure1Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(mut self, value: Max35Text) -> ProprietaryBankTransactionCodeStructure1Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issr` field.
    #[must_use]
    pub fn issr(mut self, value: Max35Text) -> ProprietaryBankTransactionCodeStructure1Builder {
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
    ) -> ::std::result::Result<ProprietaryBankTransactionCodeStructure1, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.cd.is_none() {
            missing.push("cd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryBankTransactionCodeStructure1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryBankTransactionCodeStructure1 {
            cd: self.cd.unwrap(),
            issr: self.issr,
        })
    }
}
impl ProprietaryBankTransactionCodeStructure1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryBankTransactionCodeStructure1Builder {
        ProprietaryBankTransactionCodeStructure1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryDate3 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Dt")]
    pub dt: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
}
/// Builder for [`ProprietaryDate3`]. Construct via [`ProprietaryDate3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryDate3Builder {
    tp: ::std::option::Option<Max35Text>,
    dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
}
impl ProprietaryDate3Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryDate3Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> ProprietaryDate3Builder {
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
    pub fn build(self) -> ::std::result::Result<ProprietaryDate3, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.dt.is_none() {
            missing.push("dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryDate3".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryDate3 {
            tp: self.tp.unwrap(),
            dt: self.dt.unwrap(),
        })
    }
}
impl ProprietaryDate3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryDate3Builder {
        ProprietaryDate3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryParty5 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Pty")]
    pub pty: crate::common::ChoiceWrapper<Party40Choice>,
}
/// Builder for [`ProprietaryParty5`]. Construct via [`ProprietaryParty5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryParty5Builder {
    tp: ::std::option::Option<Max35Text>,
    pty: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
}
impl ProprietaryParty5Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryParty5Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pty` field.
    #[must_use]
    pub fn pty(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> ProprietaryParty5Builder {
        self.pty = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<ProprietaryParty5, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.pty.is_none() {
            missing.push("pty".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryParty5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryParty5 {
            tp: self.tp.unwrap(),
            pty: self.pty.unwrap(),
        })
    }
}
impl ProprietaryParty5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryParty5Builder {
        ProprietaryParty5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryPrice2 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Pric")]
    pub pric: ActiveOrHistoricCurrencyAndAmount,
}
/// Builder for [`ProprietaryPrice2`]. Construct via [`ProprietaryPrice2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryPrice2Builder {
    tp: ::std::option::Option<Max35Text>,
    pric: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl ProprietaryPrice2Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryPrice2Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pric` field.
    #[must_use]
    pub fn pric(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> ProprietaryPrice2Builder {
        self.pric = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<ProprietaryPrice2, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.pric.is_none() {
            missing.push("pric".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryPrice2".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryPrice2 {
            tp: self.tp.unwrap(),
            pric: self.pric.unwrap(),
        })
    }
}
impl ProprietaryPrice2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryPrice2Builder {
        ProprietaryPrice2Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryQuantity1 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Qty")]
    pub qty: Max35Text,
}
/// Builder for [`ProprietaryQuantity1`]. Construct via [`ProprietaryQuantity1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryQuantity1Builder {
    tp: ::std::option::Option<Max35Text>,
    qty: ::std::option::Option<Max35Text>,
}
impl ProprietaryQuantity1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryQuantity1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `qty` field.
    #[must_use]
    pub fn qty(mut self, value: Max35Text) -> ProprietaryQuantity1Builder {
        self.qty = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<ProprietaryQuantity1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.qty.is_none() {
            missing.push("qty".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryQuantity1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryQuantity1 {
            tp: self.tp.unwrap(),
            qty: self.qty.unwrap(),
        })
    }
}
impl ProprietaryQuantity1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryQuantity1Builder {
        ProprietaryQuantity1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProprietaryReference1 {
    #[serde(rename = "Tp")]
    pub tp: Max35Text,
    #[serde(rename = "Ref")]
    pub r#ref: Max35Text,
}
/// Builder for [`ProprietaryReference1`]. Construct via [`ProprietaryReference1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ProprietaryReference1Builder {
    tp: ::std::option::Option<Max35Text>,
    r#ref: ::std::option::Option<Max35Text>,
}
impl ProprietaryReference1Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: Max35Text) -> ProprietaryReference1Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ref` field.
    #[must_use]
    pub fn r#ref(mut self, value: Max35Text) -> ProprietaryReference1Builder {
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
    ) -> ::std::result::Result<ProprietaryReference1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if self.r#ref.is_none() {
            missing.push("ref".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ProprietaryReference1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ProprietaryReference1 {
            tp: self.tp.unwrap(),
            r#ref: self.r#ref.unwrap(),
        })
    }
}
impl ProprietaryReference1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ProprietaryReference1Builder {
        ProprietaryReference1Builder::default()
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
pub struct Rate4 {
    #[serde(rename = "Tp")]
    pub tp: crate::common::ChoiceWrapper<RateType4Choice>,
    #[serde(rename = "VldtyRg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vldty_rg: Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}
/// Builder for [`Rate4`]. Construct via [`Rate4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Rate4Builder {
    tp: ::std::option::Option<crate::common::ChoiceWrapper<RateType4Choice>>,
    vldty_rg: ::std::option::Option<ActiveOrHistoricCurrencyAndAmountRange2>,
}
impl Rate4Builder {
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: crate::common::ChoiceWrapper<RateType4Choice>) -> Rate4Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `vldty_rg` field.
    #[must_use]
    pub fn vldty_rg(mut self, value: ActiveOrHistoricCurrencyAndAmountRange2) -> Rate4Builder {
        self.vldty_rg = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<Rate4, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tp.is_none() {
            missing.push("tp".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Rate4".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Rate4 {
            tp: self.tp.unwrap(),
            vldty_rg: self.vldty_rg,
        })
    }
}
impl Rate4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Rate4Builder {
        Rate4Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum RateType4Choice {
    #[serde(rename = "Pctg")]
    Pctg(PercentageRate),
    #[serde(rename = "Othr")]
    Othr(Max35Text),
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceLocation7 {
    #[serde(rename = "RmtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_id: Option<Max35Text>,
    #[serde(rename = "RmtLctnDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rmt_lctn_dtls: Vec<RemittanceLocationData1>,
}
/// Builder for [`RemittanceLocation7`]. Construct via [`RemittanceLocation7::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceLocation7Builder {
    rmt_id: ::std::option::Option<Max35Text>,
    rmt_lctn_dtls: ::std::vec::Vec<RemittanceLocationData1>,
}
impl RemittanceLocation7Builder {
    /// Set the `rmt_id` field.
    #[must_use]
    pub fn rmt_id(mut self, value: Max35Text) -> RemittanceLocation7Builder {
        self.rmt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmt_lctn_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn rmt_lctn_dtls(
        mut self,
        value: ::std::vec::Vec<RemittanceLocationData1>,
    ) -> RemittanceLocation7Builder {
        self.rmt_lctn_dtls = value;
        self
    }
    /// Append one item to the `rmt_lctn_dtls` field.
    #[must_use]
    pub fn add_rmt_lctn_dtls(
        mut self,
        value: RemittanceLocationData1,
    ) -> RemittanceLocation7Builder {
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
    pub fn build(self) -> ::std::result::Result<RemittanceLocation7, crate::common::BuilderError> {
        ::std::result::Result::Ok(RemittanceLocation7 {
            rmt_id: self.rmt_id,
            rmt_lctn_dtls: self.rmt_lctn_dtls,
        })
    }
}
impl RemittanceLocation7 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceLocation7Builder {
        RemittanceLocation7Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RemittanceLocationData1 {
    #[serde(rename = "Mtd")]
    pub mtd: RemittanceLocationMethod2Code,
    #[serde(rename = "ElctrncAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub elctrnc_adr: Option<Max2048Text>,
    #[serde(rename = "PstlAdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pstl_adr: Option<NameAndAddress16>,
}
/// Builder for [`RemittanceLocationData1`]. Construct via [`RemittanceLocationData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct RemittanceLocationData1Builder {
    mtd: ::std::option::Option<RemittanceLocationMethod2Code>,
    elctrnc_adr: ::std::option::Option<Max2048Text>,
    pstl_adr: ::std::option::Option<NameAndAddress16>,
}
impl RemittanceLocationData1Builder {
    /// Set the `mtd` field.
    #[must_use]
    pub fn mtd(mut self, value: RemittanceLocationMethod2Code) -> RemittanceLocationData1Builder {
        self.mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_adr` field.
    #[must_use]
    pub fn elctrnc_adr(mut self, value: Max2048Text) -> RemittanceLocationData1Builder {
        self.elctrnc_adr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pstl_adr` field.
    #[must_use]
    pub fn pstl_adr(mut self, value: NameAndAddress16) -> RemittanceLocationData1Builder {
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
    ) -> ::std::result::Result<RemittanceLocationData1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.mtd.is_none() {
            missing.push("mtd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "RemittanceLocationData1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(RemittanceLocationData1 {
            mtd: self.mtd.unwrap(),
            elctrnc_adr: self.elctrnc_adr,
            pstl_adr: self.pstl_adr,
        })
    }
}
impl RemittanceLocationData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> RemittanceLocationData1Builder {
        RemittanceLocationData1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ReportEntry13 {
    #[serde(rename = "NtryRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ntry_ref: Option<Max35Text>,
    #[serde(rename = "Amt")]
    pub amt: ActiveOrHistoricCurrencyAndAmount,
    #[serde(rename = "CdtDbtInd")]
    pub cdt_dbt_ind: CreditDebitCode,
    #[serde(rename = "RvslInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rvsl_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "Sts")]
    pub sts: crate::common::ChoiceWrapper<EntryStatus1Choice>,
    #[serde(rename = "BookgDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookg_dt: Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    #[serde(rename = "ValDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub val_dt: Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<Max35Text>,
    #[serde(rename = "Avlbty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub avlbty: Vec<CashAvailability1>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: BankTransactionCodeStructure4,
    #[serde(rename = "ComssnWvrInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comssn_wvr_ind: Option<YesNoIndicator>,
    #[serde(rename = "AddtlInfInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_inf_ind: Option<MessageIdentification2>,
    #[serde(rename = "AmtDtls")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt_dtls: Option<AmountAndCurrencyExchange4>,
    #[serde(rename = "Chrgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrgs: Option<Charges6>,
    #[serde(rename = "TechInptChanl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tech_inpt_chanl: Option<crate::common::ChoiceWrapper<TechnicalInputChannel1Choice>>,
    #[serde(rename = "Intrst")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrst: Option<TransactionInterest4>,
    #[serde(rename = "CardTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_tx: Option<CardEntry5>,
    #[serde(rename = "NtryDtls")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ntry_dtls: Vec<EntryDetails12>,
    #[serde(rename = "AddtlNtryInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addtl_ntry_inf: Option<Max500Text>,
}
/// Builder for [`ReportEntry13`]. Construct via [`ReportEntry13::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ReportEntry13Builder {
    ntry_ref: ::std::option::Option<Max35Text>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    cdt_dbt_ind: ::std::option::Option<CreditDebitCode>,
    rvsl_ind: ::std::option::Option<TrueFalseIndicator>,
    sts: ::std::option::Option<crate::common::ChoiceWrapper<EntryStatus1Choice>>,
    bookg_dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    val_dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    acct_svcr_ref: ::std::option::Option<Max35Text>,
    avlbty: ::std::vec::Vec<CashAvailability1>,
    bk_tx_cd: ::std::option::Option<BankTransactionCodeStructure4>,
    comssn_wvr_ind: ::std::option::Option<YesNoIndicator>,
    addtl_inf_ind: ::std::option::Option<MessageIdentification2>,
    amt_dtls: ::std::option::Option<AmountAndCurrencyExchange4>,
    chrgs: ::std::option::Option<Charges6>,
    tech_inpt_chanl:
        ::std::option::Option<crate::common::ChoiceWrapper<TechnicalInputChannel1Choice>>,
    intrst: ::std::option::Option<TransactionInterest4>,
    card_tx: ::std::option::Option<CardEntry5>,
    ntry_dtls: ::std::vec::Vec<EntryDetails12>,
    addtl_ntry_inf: ::std::option::Option<Max500Text>,
}
impl ReportEntry13Builder {
    /// Set the `ntry_ref` field.
    #[must_use]
    pub fn ntry_ref(mut self, value: Max35Text) -> ReportEntry13Builder {
        self.ntry_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> ReportEntry13Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_dbt_ind` field.
    #[must_use]
    pub fn cdt_dbt_ind(mut self, value: CreditDebitCode) -> ReportEntry13Builder {
        self.cdt_dbt_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rvsl_ind` field.
    #[must_use]
    pub fn rvsl_ind(mut self, value: TrueFalseIndicator) -> ReportEntry13Builder {
        self.rvsl_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sts` field.
    #[must_use]
    pub fn sts(
        mut self,
        value: crate::common::ChoiceWrapper<EntryStatus1Choice>,
    ) -> ReportEntry13Builder {
        self.sts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bookg_dt` field.
    #[must_use]
    pub fn bookg_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> ReportEntry13Builder {
        self.bookg_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `val_dt` field.
    #[must_use]
    pub fn val_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> ReportEntry13Builder {
        self.val_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct_svcr_ref` field.
    #[must_use]
    pub fn acct_svcr_ref(mut self, value: Max35Text) -> ReportEntry13Builder {
        self.acct_svcr_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `avlbty` field (replaces any previously added items).
    #[must_use]
    pub fn avlbty(mut self, value: ::std::vec::Vec<CashAvailability1>) -> ReportEntry13Builder {
        self.avlbty = value;
        self
    }
    /// Append one item to the `avlbty` field.
    #[must_use]
    pub fn add_avlbty(mut self, value: CashAvailability1) -> ReportEntry13Builder {
        self.avlbty.push(value);
        self
    }
    /// Set the `bk_tx_cd` field.
    #[must_use]
    pub fn bk_tx_cd(mut self, value: BankTransactionCodeStructure4) -> ReportEntry13Builder {
        self.bk_tx_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `comssn_wvr_ind` field.
    #[must_use]
    pub fn comssn_wvr_ind(mut self, value: YesNoIndicator) -> ReportEntry13Builder {
        self.comssn_wvr_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf_ind` field.
    #[must_use]
    pub fn addtl_inf_ind(mut self, value: MessageIdentification2) -> ReportEntry13Builder {
        self.addtl_inf_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt_dtls` field.
    #[must_use]
    pub fn amt_dtls(mut self, value: AmountAndCurrencyExchange4) -> ReportEntry13Builder {
        self.amt_dtls = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs` field.
    #[must_use]
    pub fn chrgs(mut self, value: Charges6) -> ReportEntry13Builder {
        self.chrgs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tech_inpt_chanl` field.
    #[must_use]
    pub fn tech_inpt_chanl(
        mut self,
        value: crate::common::ChoiceWrapper<TechnicalInputChannel1Choice>,
    ) -> ReportEntry13Builder {
        self.tech_inpt_chanl = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrst` field.
    #[must_use]
    pub fn intrst(mut self, value: TransactionInterest4) -> ReportEntry13Builder {
        self.intrst = ::std::option::Option::Some(value);
        self
    }
    /// Set the `card_tx` field.
    #[must_use]
    pub fn card_tx(mut self, value: CardEntry5) -> ReportEntry13Builder {
        self.card_tx = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ntry_dtls` field (replaces any previously added items).
    #[must_use]
    pub fn ntry_dtls(mut self, value: ::std::vec::Vec<EntryDetails12>) -> ReportEntry13Builder {
        self.ntry_dtls = value;
        self
    }
    /// Append one item to the `ntry_dtls` field.
    #[must_use]
    pub fn add_ntry_dtls(mut self, value: EntryDetails12) -> ReportEntry13Builder {
        self.ntry_dtls.push(value);
        self
    }
    /// Set the `addtl_ntry_inf` field.
    #[must_use]
    pub fn addtl_ntry_inf(mut self, value: Max500Text) -> ReportEntry13Builder {
        self.addtl_ntry_inf = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<ReportEntry13, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if self.cdt_dbt_ind.is_none() {
            missing.push("cdt_dbt_ind".to_owned());
        }
        if self.sts.is_none() {
            missing.push("sts".to_owned());
        }
        if self.bk_tx_cd.is_none() {
            missing.push("bk_tx_cd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ReportEntry13".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ReportEntry13 {
            ntry_ref: self.ntry_ref,
            amt: self.amt.unwrap(),
            cdt_dbt_ind: self.cdt_dbt_ind.unwrap(),
            rvsl_ind: self.rvsl_ind,
            sts: self.sts.unwrap(),
            bookg_dt: self.bookg_dt,
            val_dt: self.val_dt,
            acct_svcr_ref: self.acct_svcr_ref,
            avlbty: self.avlbty,
            bk_tx_cd: self.bk_tx_cd.unwrap(),
            comssn_wvr_ind: self.comssn_wvr_ind,
            addtl_inf_ind: self.addtl_inf_ind,
            amt_dtls: self.amt_dtls,
            chrgs: self.chrgs,
            tech_inpt_chanl: self.tech_inpt_chanl,
            intrst: self.intrst,
            card_tx: self.card_tx,
            ntry_dtls: self.ntry_dtls,
            addtl_ntry_inf: self.addtl_ntry_inf,
        })
    }
}
impl ReportEntry13 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ReportEntry13Builder {
        ReportEntry13Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReportingSource1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalReportingSource1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ReturnReason5Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalReturnReason1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecuritiesAccount19 {
    #[serde(rename = "Id")]
    pub id: Max35Text,
    #[serde(rename = "Tp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp: Option<GenericIdentification30>,
    #[serde(rename = "Nm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nm: Option<Max70Text>,
}
/// Builder for [`SecuritiesAccount19`]. Construct via [`SecuritiesAccount19::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SecuritiesAccount19Builder {
    id: ::std::option::Option<Max35Text>,
    tp: ::std::option::Option<GenericIdentification30>,
    nm: ::std::option::Option<Max70Text>,
}
impl SecuritiesAccount19Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> SecuritiesAccount19Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tp` field.
    #[must_use]
    pub fn tp(mut self, value: GenericIdentification30) -> SecuritiesAccount19Builder {
        self.tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max70Text) -> SecuritiesAccount19Builder {
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
    pub fn build(self) -> ::std::result::Result<SecuritiesAccount19, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "SecuritiesAccount19".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(SecuritiesAccount19 {
            id: self.id.unwrap(),
            tp: self.tp,
            nm: self.nm,
        })
    }
}
impl SecuritiesAccount19 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SecuritiesAccount19Builder {
        SecuritiesAccount19Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SecurityIdentification19 {
    #[serde(rename = "ISIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isin: Option<ISINOct2015Identifier>,
    #[serde(rename = "OthrId")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub othr_id: Vec<OtherIdentification1>,
    #[serde(rename = "Desc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<Max140Text>,
}
/// Builder for [`SecurityIdentification19`]. Construct via [`SecurityIdentification19::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SecurityIdentification19Builder {
    isin: ::std::option::Option<ISINOct2015Identifier>,
    othr_id: ::std::vec::Vec<OtherIdentification1>,
    desc: ::std::option::Option<Max140Text>,
}
impl SecurityIdentification19Builder {
    /// Set the `isin` field.
    #[must_use]
    pub fn isin(mut self, value: ISINOct2015Identifier) -> SecurityIdentification19Builder {
        self.isin = ::std::option::Option::Some(value);
        self
    }
    /// Set the `othr_id` field (replaces any previously added items).
    #[must_use]
    pub fn othr_id(
        mut self,
        value: ::std::vec::Vec<OtherIdentification1>,
    ) -> SecurityIdentification19Builder {
        self.othr_id = value;
        self
    }
    /// Append one item to the `othr_id` field.
    #[must_use]
    pub fn add_othr_id(mut self, value: OtherIdentification1) -> SecurityIdentification19Builder {
        self.othr_id.push(value);
        self
    }
    /// Set the `desc` field.
    #[must_use]
    pub fn desc(mut self, value: Max140Text) -> SecurityIdentification19Builder {
        self.desc = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<SecurityIdentification19, crate::common::BuilderError> {
        ::std::result::Result::Ok(SecurityIdentification19 {
            isin: self.isin,
            othr_id: self.othr_id,
            desc: self.desc,
        })
    }
}
impl SecurityIdentification19 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SecurityIdentification19Builder {
        SecurityIdentification19Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SequenceRange1 {
    #[serde(rename = "FrSeq")]
    pub fr_seq: Max35Text,
    #[serde(rename = "ToSeq")]
    pub to_seq: Max35Text,
}
/// Builder for [`SequenceRange1`]. Construct via [`SequenceRange1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct SequenceRange1Builder {
    fr_seq: ::std::option::Option<Max35Text>,
    to_seq: ::std::option::Option<Max35Text>,
}
impl SequenceRange1Builder {
    /// Set the `fr_seq` field.
    #[must_use]
    pub fn fr_seq(mut self, value: Max35Text) -> SequenceRange1Builder {
        self.fr_seq = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to_seq` field.
    #[must_use]
    pub fn to_seq(mut self, value: Max35Text) -> SequenceRange1Builder {
        self.to_seq = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<SequenceRange1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr_seq.is_none() {
            missing.push("fr_seq".to_owned());
        }
        if self.to_seq.is_none() {
            missing.push("to_seq".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "SequenceRange1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(SequenceRange1 {
            fr_seq: self.fr_seq.unwrap(),
            to_seq: self.to_seq.unwrap(),
        })
    }
}
impl SequenceRange1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> SequenceRange1Builder {
        SequenceRange1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum SequenceRange1Choice {
    #[serde(rename = "FrSeq")]
    FrSeq(Max35Text),
    #[serde(rename = "ToSeq")]
    ToSeq(Max35Text),
    #[serde(rename = "FrToSeq")]
    FrToSeq(SequenceRange1),
    #[serde(rename = "EQSeq")]
    EQSeq(Max35Text),
    #[serde(rename = "NEQSeq")]
    NEQSeq(Max35Text),
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
pub struct TaxCharges2 {
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Max35Text>,
    #[serde(rename = "Rate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<PercentageRate>,
    #[serde(rename = "Amt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amt: Option<ActiveOrHistoricCurrencyAndAmount>,
}
/// Builder for [`TaxCharges2`]. Construct via [`TaxCharges2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TaxCharges2Builder {
    id: ::std::option::Option<Max35Text>,
    rate: ::std::option::Option<PercentageRate>,
    amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
}
impl TaxCharges2Builder {
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max35Text) -> TaxCharges2Builder {
        self.id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rate` field.
    #[must_use]
    pub fn rate(mut self, value: PercentageRate) -> TaxCharges2Builder {
        self.rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(mut self, value: ActiveOrHistoricCurrencyAndAmount) -> TaxCharges2Builder {
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
    pub fn build(self) -> ::std::result::Result<TaxCharges2, crate::common::BuilderError> {
        ::std::result::Result::Ok(TaxCharges2 {
            id: self.id,
            rate: self.rate,
            amt: self.amt,
        })
    }
}
impl TaxCharges2 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TaxCharges2Builder {
        TaxCharges2Builder::default()
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TechnicalInputChannel1Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalTechnicalInputChannel1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TotalTransactions6 {
    #[serde(rename = "TtlNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_ntries: Option<NumberAndSumOfTransactions4>,
    #[serde(rename = "TtlCdtNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_cdt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlDbtNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_dbt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "TtlNtriesPerBkTxCd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ttl_ntries_per_bk_tx_cd: Vec<TotalsPerBankTransactionCode5>,
}
/// Builder for [`TotalTransactions6`]. Construct via [`TotalTransactions6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TotalTransactions6Builder {
    ttl_ntries: ::std::option::Option<NumberAndSumOfTransactions4>,
    ttl_cdt_ntries: ::std::option::Option<NumberAndSumOfTransactions1>,
    ttl_dbt_ntries: ::std::option::Option<NumberAndSumOfTransactions1>,
    ttl_ntries_per_bk_tx_cd: ::std::vec::Vec<TotalsPerBankTransactionCode5>,
}
impl TotalTransactions6Builder {
    /// Set the `ttl_ntries` field.
    #[must_use]
    pub fn ttl_ntries(mut self, value: NumberAndSumOfTransactions4) -> TotalTransactions6Builder {
        self.ttl_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_cdt_ntries` field.
    #[must_use]
    pub fn ttl_cdt_ntries(
        mut self,
        value: NumberAndSumOfTransactions1,
    ) -> TotalTransactions6Builder {
        self.ttl_cdt_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_dbt_ntries` field.
    #[must_use]
    pub fn ttl_dbt_ntries(
        mut self,
        value: NumberAndSumOfTransactions1,
    ) -> TotalTransactions6Builder {
        self.ttl_dbt_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_ntries_per_bk_tx_cd` field (replaces any previously added items).
    #[must_use]
    pub fn ttl_ntries_per_bk_tx_cd(
        mut self,
        value: ::std::vec::Vec<TotalsPerBankTransactionCode5>,
    ) -> TotalTransactions6Builder {
        self.ttl_ntries_per_bk_tx_cd = value;
        self
    }
    /// Append one item to the `ttl_ntries_per_bk_tx_cd` field.
    #[must_use]
    pub fn add_ttl_ntries_per_bk_tx_cd(
        mut self,
        value: TotalsPerBankTransactionCode5,
    ) -> TotalTransactions6Builder {
        self.ttl_ntries_per_bk_tx_cd.push(value);
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
    pub fn build(self) -> ::std::result::Result<TotalTransactions6, crate::common::BuilderError> {
        ::std::result::Result::Ok(TotalTransactions6 {
            ttl_ntries: self.ttl_ntries,
            ttl_cdt_ntries: self.ttl_cdt_ntries,
            ttl_dbt_ntries: self.ttl_dbt_ntries,
            ttl_ntries_per_bk_tx_cd: self.ttl_ntries_per_bk_tx_cd,
        })
    }
}
impl TotalTransactions6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TotalTransactions6Builder {
        TotalTransactions6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TotalsPerBankTransactionCode5 {
    #[serde(rename = "NbOfNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_of_ntries: Option<Max15NumericText>,
    #[serde(rename = "Sum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sum: Option<DecimalNumber>,
    #[serde(rename = "TtlNetNtry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_net_ntry: Option<AmountAndDirection35>,
    #[serde(rename = "CdtNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "DbtNtries")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_ntries: Option<NumberAndSumOfTransactions1>,
    #[serde(rename = "FcstInd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fcst_ind: Option<TrueFalseIndicator>,
    #[serde(rename = "BkTxCd")]
    pub bk_tx_cd: BankTransactionCodeStructure4,
    #[serde(rename = "Avlbty")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub avlbty: Vec<CashAvailability1>,
    #[serde(rename = "Dt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dt: Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
}
/// Builder for [`TotalsPerBankTransactionCode5`]. Construct via [`TotalsPerBankTransactionCode5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TotalsPerBankTransactionCode5Builder {
    nb_of_ntries: ::std::option::Option<Max15NumericText>,
    sum: ::std::option::Option<DecimalNumber>,
    ttl_net_ntry: ::std::option::Option<AmountAndDirection35>,
    cdt_ntries: ::std::option::Option<NumberAndSumOfTransactions1>,
    dbt_ntries: ::std::option::Option<NumberAndSumOfTransactions1>,
    fcst_ind: ::std::option::Option<TrueFalseIndicator>,
    bk_tx_cd: ::std::option::Option<BankTransactionCodeStructure4>,
    avlbty: ::std::vec::Vec<CashAvailability1>,
    dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
}
impl TotalsPerBankTransactionCode5Builder {
    /// Set the `nb_of_ntries` field.
    #[must_use]
    pub fn nb_of_ntries(mut self, value: Max15NumericText) -> TotalsPerBankTransactionCode5Builder {
        self.nb_of_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sum` field.
    #[must_use]
    pub fn sum(mut self, value: DecimalNumber) -> TotalsPerBankTransactionCode5Builder {
        self.sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_net_ntry` field.
    #[must_use]
    pub fn ttl_net_ntry(
        mut self,
        value: AmountAndDirection35,
    ) -> TotalsPerBankTransactionCode5Builder {
        self.ttl_net_ntry = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_ntries` field.
    #[must_use]
    pub fn cdt_ntries(
        mut self,
        value: NumberAndSumOfTransactions1,
    ) -> TotalsPerBankTransactionCode5Builder {
        self.cdt_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbt_ntries` field.
    #[must_use]
    pub fn dbt_ntries(
        mut self,
        value: NumberAndSumOfTransactions1,
    ) -> TotalsPerBankTransactionCode5Builder {
        self.dbt_ntries = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fcst_ind` field.
    #[must_use]
    pub fn fcst_ind(mut self, value: TrueFalseIndicator) -> TotalsPerBankTransactionCode5Builder {
        self.fcst_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `bk_tx_cd` field.
    #[must_use]
    pub fn bk_tx_cd(
        mut self,
        value: BankTransactionCodeStructure4,
    ) -> TotalsPerBankTransactionCode5Builder {
        self.bk_tx_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `avlbty` field (replaces any previously added items).
    #[must_use]
    pub fn avlbty(
        mut self,
        value: ::std::vec::Vec<CashAvailability1>,
    ) -> TotalsPerBankTransactionCode5Builder {
        self.avlbty = value;
        self
    }
    /// Append one item to the `avlbty` field.
    #[must_use]
    pub fn add_avlbty(mut self, value: CashAvailability1) -> TotalsPerBankTransactionCode5Builder {
        self.avlbty.push(value);
        self
    }
    /// Set the `dt` field.
    #[must_use]
    pub fn dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> TotalsPerBankTransactionCode5Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<TotalsPerBankTransactionCode5, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.bk_tx_cd.is_none() {
            missing.push("bk_tx_cd".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TotalsPerBankTransactionCode5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TotalsPerBankTransactionCode5 {
            nb_of_ntries: self.nb_of_ntries,
            sum: self.sum,
            ttl_net_ntry: self.ttl_net_ntry,
            cdt_ntries: self.cdt_ntries,
            dbt_ntries: self.dbt_ntries,
            fcst_ind: self.fcst_ind,
            bk_tx_cd: self.bk_tx_cd.unwrap(),
            avlbty: self.avlbty,
            dt: self.dt,
        })
    }
}
impl TotalsPerBankTransactionCode5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TotalsPerBankTransactionCode5Builder {
        TotalsPerBankTransactionCode5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TrackData1 {
    #[serde(rename = "TrckNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trck_nb: Option<Exact1NumericText>,
    #[serde(rename = "TrckVal")]
    pub trck_val: Max140Text,
}
/// Builder for [`TrackData1`]. Construct via [`TrackData1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TrackData1Builder {
    trck_nb: ::std::option::Option<Exact1NumericText>,
    trck_val: ::std::option::Option<Max140Text>,
}
impl TrackData1Builder {
    /// Set the `trck_nb` field.
    #[must_use]
    pub fn trck_nb(mut self, value: Exact1NumericText) -> TrackData1Builder {
        self.trck_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trck_val` field.
    #[must_use]
    pub fn trck_val(mut self, value: Max140Text) -> TrackData1Builder {
        self.trck_val = ::std::option::Option::Some(value);
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
    pub fn build(self) -> ::std::result::Result<TrackData1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.trck_val.is_none() {
            missing.push("trck_val".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TrackData1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TrackData1 {
            trck_nb: self.trck_nb,
            trck_val: self.trck_val.unwrap(),
        })
    }
}
impl TrackData1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TrackData1Builder {
        TrackData1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionAgents5 {
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt1: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt2: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IntrmyAgt3")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intrmy_agt3: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "RcvgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rcvg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "DlvrgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlvrg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "IssgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "SttlmPlc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sttlm_plc: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "Prtry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prtry: Vec<ProprietaryAgent4>,
}
/// Builder for [`TransactionAgents5`]. Construct via [`TransactionAgents5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionAgents5Builder {
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    cdtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt1: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt2: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    intrmy_agt3: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    rcvg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dlvrg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    issg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    sttlm_plc: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    prtry: ::std::vec::Vec<ProprietaryAgent4>,
}
impl TransactionAgents5Builder {
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1` field.
    #[must_use]
    pub fn intrmy_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.intrmy_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2` field.
    #[must_use]
    pub fn intrmy_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.intrmy_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3` field.
    #[must_use]
    pub fn intrmy_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.intrmy_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rcvg_agt` field.
    #[must_use]
    pub fn rcvg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.rcvg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dlvrg_agt` field.
    #[must_use]
    pub fn dlvrg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.dlvrg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `issg_agt` field.
    #[must_use]
    pub fn issg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.issg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_plc` field.
    #[must_use]
    pub fn sttlm_plc(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> TransactionAgents5Builder {
        self.sttlm_plc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry` field (replaces any previously added items).
    #[must_use]
    pub fn prtry(mut self, value: ::std::vec::Vec<ProprietaryAgent4>) -> TransactionAgents5Builder {
        self.prtry = value;
        self
    }
    /// Append one item to the `prtry` field.
    #[must_use]
    pub fn add_prtry(mut self, value: ProprietaryAgent4) -> TransactionAgents5Builder {
        self.prtry.push(value);
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
    pub fn build(self) -> ::std::result::Result<TransactionAgents5, crate::common::BuilderError> {
        ::std::result::Result::Ok(TransactionAgents5 {
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
            dbtr_agt: self.dbtr_agt,
            cdtr_agt: self.cdtr_agt,
            intrmy_agt1: self.intrmy_agt1,
            intrmy_agt2: self.intrmy_agt2,
            intrmy_agt3: self.intrmy_agt3,
            rcvg_agt: self.rcvg_agt,
            dlvrg_agt: self.dlvrg_agt,
            issg_agt: self.issg_agt,
            sttlm_plc: self.sttlm_plc,
            prtry: self.prtry,
        })
    }
}
impl TransactionAgents5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionAgents5Builder {
        TransactionAgents5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionDates3 {
    #[serde(rename = "AccptncDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<ISODateTime>,
    #[serde(rename = "TradActvtyCtrctlSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trad_actvty_ctrctl_sttlm_dt: Option<ISODate>,
    #[serde(rename = "TradDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trad_dt: Option<ISODate>,
    #[serde(rename = "IntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intr_bk_sttlm_dt: Option<ISODate>,
    #[serde(rename = "StartDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_dt: Option<ISODate>,
    #[serde(rename = "EndDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_dt: Option<ISODate>,
    #[serde(rename = "TxDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_dt_tm: Option<ISODateTime>,
    #[serde(rename = "Prtry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prtry: Vec<ProprietaryDate3>,
}
/// Builder for [`TransactionDates3`]. Construct via [`TransactionDates3::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionDates3Builder {
    accptnc_dt_tm: ::std::option::Option<ISODateTime>,
    trad_actvty_ctrctl_sttlm_dt: ::std::option::Option<ISODate>,
    trad_dt: ::std::option::Option<ISODate>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    start_dt: ::std::option::Option<ISODate>,
    end_dt: ::std::option::Option<ISODate>,
    tx_dt_tm: ::std::option::Option<ISODateTime>,
    prtry: ::std::vec::Vec<ProprietaryDate3>,
}
impl TransactionDates3Builder {
    /// Set the `accptnc_dt_tm` field.
    #[must_use]
    pub fn accptnc_dt_tm(mut self, value: ISODateTime) -> TransactionDates3Builder {
        self.accptnc_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trad_actvty_ctrctl_sttlm_dt` field.
    #[must_use]
    pub fn trad_actvty_ctrctl_sttlm_dt(mut self, value: ISODate) -> TransactionDates3Builder {
        self.trad_actvty_ctrctl_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `trad_dt` field.
    #[must_use]
    pub fn trad_dt(mut self, value: ISODate) -> TransactionDates3Builder {
        self.trad_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(mut self, value: ISODate) -> TransactionDates3Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `start_dt` field.
    #[must_use]
    pub fn start_dt(mut self, value: ISODate) -> TransactionDates3Builder {
        self.start_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `end_dt` field.
    #[must_use]
    pub fn end_dt(mut self, value: ISODate) -> TransactionDates3Builder {
        self.end_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_dt_tm` field.
    #[must_use]
    pub fn tx_dt_tm(mut self, value: ISODateTime) -> TransactionDates3Builder {
        self.tx_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry` field (replaces any previously added items).
    #[must_use]
    pub fn prtry(mut self, value: ::std::vec::Vec<ProprietaryDate3>) -> TransactionDates3Builder {
        self.prtry = value;
        self
    }
    /// Append one item to the `prtry` field.
    #[must_use]
    pub fn add_prtry(mut self, value: ProprietaryDate3) -> TransactionDates3Builder {
        self.prtry.push(value);
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
    pub fn build(self) -> ::std::result::Result<TransactionDates3, crate::common::BuilderError> {
        ::std::result::Result::Ok(TransactionDates3 {
            accptnc_dt_tm: self.accptnc_dt_tm,
            trad_actvty_ctrctl_sttlm_dt: self.trad_actvty_ctrctl_sttlm_dt,
            trad_dt: self.trad_dt,
            intr_bk_sttlm_dt: self.intr_bk_sttlm_dt,
            start_dt: self.start_dt,
            end_dt: self.end_dt,
            tx_dt_tm: self.tx_dt_tm,
            prtry: self.prtry,
        })
    }
}
impl TransactionDates3 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionDates3Builder {
        TransactionDates3Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionIdentifier1 {
    #[serde(rename = "TxDtTm")]
    pub tx_dt_tm: ISODateTime,
    #[serde(rename = "TxRef")]
    pub tx_ref: Max35Text,
}
/// Builder for [`TransactionIdentifier1`]. Construct via [`TransactionIdentifier1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionIdentifier1Builder {
    tx_dt_tm: ::std::option::Option<ISODateTime>,
    tx_ref: ::std::option::Option<Max35Text>,
}
impl TransactionIdentifier1Builder {
    /// Set the `tx_dt_tm` field.
    #[must_use]
    pub fn tx_dt_tm(mut self, value: ISODateTime) -> TransactionIdentifier1Builder {
        self.tx_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_ref` field.
    #[must_use]
    pub fn tx_ref(mut self, value: Max35Text) -> TransactionIdentifier1Builder {
        self.tx_ref = ::std::option::Option::Some(value);
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
    ) -> ::std::result::Result<TransactionIdentifier1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.tx_dt_tm.is_none() {
            missing.push("tx_dt_tm".to_owned());
        }
        if self.tx_ref.is_none() {
            missing.push("tx_ref".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "TransactionIdentifier1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(TransactionIdentifier1 {
            tx_dt_tm: self.tx_dt_tm.unwrap(),
            tx_ref: self.tx_ref.unwrap(),
        })
    }
}
impl TransactionIdentifier1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionIdentifier1Builder {
        TransactionIdentifier1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionInterest4 {
    #[serde(rename = "TtlIntrstAndTaxAmt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_intrst_and_tax_amt: Option<ActiveOrHistoricCurrencyAndAmount>,
    #[serde(rename = "Rcrd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rcrd: Vec<InterestRecord2>,
}
/// Builder for [`TransactionInterest4`]. Construct via [`TransactionInterest4::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionInterest4Builder {
    ttl_intrst_and_tax_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    rcrd: ::std::vec::Vec<InterestRecord2>,
}
impl TransactionInterest4Builder {
    /// Set the `ttl_intrst_and_tax_amt` field.
    #[must_use]
    pub fn ttl_intrst_and_tax_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TransactionInterest4Builder {
        self.ttl_intrst_and_tax_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rcrd` field (replaces any previously added items).
    #[must_use]
    pub fn rcrd(mut self, value: ::std::vec::Vec<InterestRecord2>) -> TransactionInterest4Builder {
        self.rcrd = value;
        self
    }
    /// Append one item to the `rcrd` field.
    #[must_use]
    pub fn add_rcrd(mut self, value: InterestRecord2) -> TransactionInterest4Builder {
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
    pub fn build(self) -> ::std::result::Result<TransactionInterest4, crate::common::BuilderError> {
        ::std::result::Result::Ok(TransactionInterest4 {
            ttl_intrst_and_tax_amt: self.ttl_intrst_and_tax_amt,
            rcrd: self.rcrd,
        })
    }
}
impl TransactionInterest4 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionInterest4Builder {
        TransactionInterest4Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionParties9 {
    #[serde(rename = "InitgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initg_pty: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Dbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "DbtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Cdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "CdtrAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr_acct: Option<CashAccount40>,
    #[serde(rename = "UltmtCdtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_cdtr: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "TradgPty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tradg_pty: Option<crate::common::ChoiceWrapper<Party40Choice>>,
    #[serde(rename = "Prtry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prtry: Vec<ProprietaryParty5>,
}
/// Builder for [`TransactionParties9`]. Construct via [`TransactionParties9::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionParties9Builder {
    initg_pty: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_dbtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    cdtr_acct: ::std::option::Option<CashAccount40>,
    ultmt_cdtr: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    tradg_pty: ::std::option::Option<crate::common::ChoiceWrapper<Party40Choice>>,
    prtry: ::std::vec::Vec<ProprietaryParty5>,
}
impl TransactionParties9Builder {
    /// Set the `initg_pty` field.
    #[must_use]
    pub fn initg_pty(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.initg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> TransactionParties9Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> TransactionParties9Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tradg_pty` field.
    #[must_use]
    pub fn tradg_pty(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> TransactionParties9Builder {
        self.tradg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry` field (replaces any previously added items).
    #[must_use]
    pub fn prtry(
        mut self,
        value: ::std::vec::Vec<ProprietaryParty5>,
    ) -> TransactionParties9Builder {
        self.prtry = value;
        self
    }
    /// Append one item to the `prtry` field.
    #[must_use]
    pub fn add_prtry(mut self, value: ProprietaryParty5) -> TransactionParties9Builder {
        self.prtry.push(value);
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
    pub fn build(self) -> ::std::result::Result<TransactionParties9, crate::common::BuilderError> {
        ::std::result::Result::Ok(TransactionParties9 {
            initg_pty: self.initg_pty,
            dbtr: self.dbtr,
            dbtr_acct: self.dbtr_acct,
            ultmt_dbtr: self.ultmt_dbtr,
            cdtr: self.cdtr,
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
            tradg_pty: self.tradg_pty,
            prtry: self.prtry,
        })
    }
}
impl TransactionParties9 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionParties9Builder {
        TransactionParties9Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionPrice4Choice {
    #[serde(rename = "DealPric")]
    DealPric(Price7),
    #[serde(rename = "Prtry")]
    Prtry(ProprietaryPrice2),
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum TransactionQuantities3Choice {
    #[serde(rename = "Qty")]
    Qty(FinancialInstrumentQuantity1Choice),
    #[serde(rename = "OrgnlAndCurFaceAmt")]
    OrgnlAndCurFaceAmt(OriginalAndCurrentQuantities1),
    #[serde(rename = "Prtry")]
    Prtry(ProprietaryQuantity1),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TransactionReferences6 {
    #[serde(rename = "MsgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_id: Option<Max35Text>,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<Max35Text>,
    #[serde(rename = "PmtInfId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_inf_id: Option<Max35Text>,
    #[serde(rename = "InstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<Max35Text>,
    #[serde(rename = "EndToEndId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_to_end_id: Option<Max35Text>,
    #[serde(rename = "UETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uetr: Option<UUIDv4Identifier>,
    #[serde(rename = "TxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<Max35Text>,
    #[serde(rename = "MndtId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_id: Option<Max35Text>,
    #[serde(rename = "ChqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_nb: Option<Max35Text>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<Max35Text>,
    #[serde(rename = "AcctOwnrTxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_ownr_tx_id: Option<Max35Text>,
    #[serde(rename = "AcctSvcrTxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_svcr_tx_id: Option<Max35Text>,
    #[serde(rename = "MktInfrstrctrTxId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mkt_infrstrctr_tx_id: Option<Max35Text>,
    #[serde(rename = "PrcgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prcg_id: Option<Max35Text>,
    #[serde(rename = "Prtry")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub prtry: Vec<ProprietaryReference1>,
}
/// Builder for [`TransactionReferences6`]. Construct via [`TransactionReferences6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct TransactionReferences6Builder {
    msg_id: ::std::option::Option<Max35Text>,
    acct_svcr_ref: ::std::option::Option<Max35Text>,
    pmt_inf_id: ::std::option::Option<Max35Text>,
    instr_id: ::std::option::Option<Max35Text>,
    end_to_end_id: ::std::option::Option<Max35Text>,
    uetr: ::std::option::Option<UUIDv4Identifier>,
    tx_id: ::std::option::Option<Max35Text>,
    mndt_id: ::std::option::Option<Max35Text>,
    chq_nb: ::std::option::Option<Max35Text>,
    clr_sys_ref: ::std::option::Option<Max35Text>,
    acct_ownr_tx_id: ::std::option::Option<Max35Text>,
    acct_svcr_tx_id: ::std::option::Option<Max35Text>,
    mkt_infrstrctr_tx_id: ::std::option::Option<Max35Text>,
    prcg_id: ::std::option::Option<Max35Text>,
    prtry: ::std::vec::Vec<ProprietaryReference1>,
}
impl TransactionReferences6Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct_svcr_ref` field.
    #[must_use]
    pub fn acct_svcr_ref(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.acct_svcr_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_inf_id` field.
    #[must_use]
    pub fn pmt_inf_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.pmt_inf_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_id` field.
    #[must_use]
    pub fn instr_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.instr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `end_to_end_id` field.
    #[must_use]
    pub fn end_to_end_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.end_to_end_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `uetr` field.
    #[must_use]
    pub fn uetr(mut self, value: UUIDv4Identifier) -> TransactionReferences6Builder {
        self.uetr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_id` field.
    #[must_use]
    pub fn tx_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mndt_id` field.
    #[must_use]
    pub fn mndt_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.mndt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chq_nb` field.
    #[must_use]
    pub fn chq_nb(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.chq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys_ref` field.
    #[must_use]
    pub fn clr_sys_ref(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.clr_sys_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct_ownr_tx_id` field.
    #[must_use]
    pub fn acct_ownr_tx_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.acct_ownr_tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct_svcr_tx_id` field.
    #[must_use]
    pub fn acct_svcr_tx_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.acct_svcr_tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mkt_infrstrctr_tx_id` field.
    #[must_use]
    pub fn mkt_infrstrctr_tx_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.mkt_infrstrctr_tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prcg_id` field.
    #[must_use]
    pub fn prcg_id(mut self, value: Max35Text) -> TransactionReferences6Builder {
        self.prcg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prtry` field (replaces any previously added items).
    #[must_use]
    pub fn prtry(
        mut self,
        value: ::std::vec::Vec<ProprietaryReference1>,
    ) -> TransactionReferences6Builder {
        self.prtry = value;
        self
    }
    /// Append one item to the `prtry` field.
    #[must_use]
    pub fn add_prtry(mut self, value: ProprietaryReference1) -> TransactionReferences6Builder {
        self.prtry.push(value);
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
    ) -> ::std::result::Result<TransactionReferences6, crate::common::BuilderError> {
        ::std::result::Result::Ok(TransactionReferences6 {
            msg_id: self.msg_id,
            acct_svcr_ref: self.acct_svcr_ref,
            pmt_inf_id: self.pmt_inf_id,
            instr_id: self.instr_id,
            end_to_end_id: self.end_to_end_id,
            uetr: self.uetr,
            tx_id: self.tx_id,
            mndt_id: self.mndt_id,
            chq_nb: self.chq_nb,
            clr_sys_ref: self.clr_sys_ref,
            acct_ownr_tx_id: self.acct_ownr_tx_id,
            acct_svcr_tx_id: self.acct_svcr_tx_id,
            mkt_infrstrctr_tx_id: self.mkt_infrstrctr_tx_id,
            prcg_id: self.prcg_id,
            prtry: self.prtry,
        })
    }
}
impl TransactionReferences6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> TransactionReferences6Builder {
        TransactionReferences6Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum YieldedOrValueType1Choice {
    #[serde(rename = "Yldd")]
    Yldd(YesNoIndicator),
    #[serde(rename = "ValTp")]
    ValTp(PriceValueType1Code),
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
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAnd13DecimalAmountSimpleType {
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
            if frac_count > 13usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 13", frac_count
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
impl crate::common::validate::Validatable for AttendanceContext1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AuthenticationEntity1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AuthenticationMethod1Code {
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
impl crate::common::validate::Validatable for CSCManagement1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CardDataReading1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CardPaymentServiceType2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CardholderVerificationCapability1Code {
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
impl crate::common::validate::Validatable for ChargeIncludedIndicator {
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
impl crate::common::validate::Validatable for CopyDuplicate1Code {
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
impl crate::common::validate::Validatable for DocumentType3Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for DocumentType6Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Exact1NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Exact3NumericText {
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
impl crate::common::validate::Validatable for ExternalBankTransactionDomain1Code {
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
impl crate::common::validate::Validatable for ExternalBankTransactionFamily1Code {
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
impl crate::common::validate::Validatable for ExternalBankTransactionSubFamily1Code {
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
impl crate::common::validate::Validatable for ExternalCardTransactionCategory1Code {
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
impl crate::common::validate::Validatable for ExternalDiscountAmountType1Code {
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
impl crate::common::validate::Validatable for ExternalEntryStatus1Code {
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
impl crate::common::validate::Validatable for ExternalFinancialInstrumentIdentificationType1Code {
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
impl crate::common::validate::Validatable for ExternalRePresentmentReason1Code {
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
impl crate::common::validate::Validatable for ExternalReportingSource1Code {
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
impl crate::common::validate::Validatable for ExternalReturnReason1Code {
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
impl crate::common::validate::Validatable for ExternalTaxAmountType1Code {
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
impl crate::common::validate::Validatable for ExternalTechnicalInputChannel1Code {
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
impl crate::common::validate::Validatable for IBAN2007Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISINOct2015Identifier {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISO2ALanguageCode {
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
impl crate::common::validate::Validatable for ISOYear {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ISOYearMonth {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ImpliedCurrencyAndAmount {
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
impl crate::common::validate::Validatable for InterestType1Code {
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
impl crate::common::validate::Validatable for Max1025Text {
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
            if len > 1025usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 1025", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max105Text {
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
            if len > 105usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 105", len),
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
impl crate::common::validate::Validatable for Max15PlusSignedNumericText {
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
impl crate::common::validate::Validatable for Max3NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
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
impl crate::common::validate::Validatable for Max500Text {
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
            if len > 500usize {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!("{} (got {})", "value exceeds maximum length 500", len),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max5NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
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
impl crate::common::validate::Validatable for Min2Max3NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Min3Max4NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for Min8Max28NumericText {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
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
impl crate::common::validate::Validatable for NonNegativeDecimalNumber {
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
impl crate::common::validate::Validatable for OnLineCapability1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for POIComponentType1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for PartyType3Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for PartyType4Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
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
impl crate::common::validate::Validatable for PositiveNumber {
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
impl crate::common::validate::Validatable for PreferredContactMethod1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for PriceValueType1Code {
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
impl crate::common::validate::Validatable for RemittanceLocationMethod2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for SequenceType3Code {
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
impl crate::common::validate::Validatable for TransactionChannel1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for TransactionEnvironment1Code {
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
impl crate::common::validate::Validatable for UnitOfMeasure1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for UserInterface2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for YesNoIndicator {
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
impl crate::common::validate::Validatable for AccountInterest4 {
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
        for (i, item) in self.rate.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Rate[{i}]"), violations);
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate_constraints(&format!("{path}/FrToDt"), violations);
        }
        if let Some(ref val) = self.rsn {
            val.validate_constraints(&format!("{path}/Rsn"), violations);
        }
        if let Some(ref val) = self.tax {
            val.validate_constraints(&format!("{path}/Tax"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AccountNotification21 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.ntfctn_pgntn {
            val.validate_constraints(&format!("{path}/NtfctnPgntn"), violations);
        }
        if let Some(ref val) = self.elctrnc_seq_nb {
            val.validate_constraints(&format!("{path}/ElctrncSeqNb"), violations);
        }
        if let Some(ref wrapper) = self.rptg_seq {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/RptgSeq"), violations);
        }
        if let Some(ref val) = self.lgl_seq_nb {
            val.validate_constraints(&format!("{path}/LglSeqNb"), violations);
        }
        if let Some(ref val) = self.cre_dt_tm {
            val.validate_constraints(&format!("{path}/CreDtTm"), violations);
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate_constraints(&format!("{path}/FrToDt"), violations);
        }
        if let Some(ref val) = self.cpy_dplct_ind {
            val.validate_constraints(&format!("{path}/CpyDplctInd"), violations);
        }
        if let Some(ref wrapper) = self.rptg_src {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/RptgSrc"), violations);
        }
        self.acct
            .validate_constraints(&format!("{path}/Acct"), violations);
        if let Some(ref val) = self.rltd_acct {
            val.validate_constraints(&format!("{path}/RltdAcct"), violations);
        }
        for (i, item) in self.intrst.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Intrst[{i}]"), violations);
        }
        if let Some(ref val) = self.txs_summry {
            val.validate_constraints(&format!("{path}/TxsSummry"), violations);
        }
        for (i, item) in self.ntry.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Ntry[{i}]"), violations);
        }
        if let Some(ref val) = self.addtl_ntfctn_inf {
            val.validate_constraints(&format!("{path}/AddtlNtfctnInf"), violations);
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
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAnd13DecimalAmount {
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
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAndAmountRange2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .inner
            .validate_constraints(&format!("{path}/Amt"), violations);
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        }
        self.ccy
            .validate_constraints(&format!("{path}/Ccy"), violations);
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
impl crate::common::validate::Validatable for AmountAndCurrencyExchange4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.instd_amt {
            val.validate_constraints(&format!("{path}/InstdAmt"), violations);
        }
        if let Some(ref val) = self.tx_amt {
            val.validate_constraints(&format!("{path}/TxAmt"), violations);
        }
        if let Some(ref val) = self.cntr_val_amt {
            val.validate_constraints(&format!("{path}/CntrValAmt"), violations);
        }
        if let Some(ref val) = self.anncd_pstng_amt {
            val.validate_constraints(&format!("{path}/AnncdPstngAmt"), violations);
        }
        for (i, item) in self.prtry_amt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/PrtryAmt[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AmountAndCurrencyExchangeDetails5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        if let Some(ref val) = self.ccy_xchg {
            val.validate_constraints(&format!("{path}/CcyXchg"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AmountAndCurrencyExchangeDetails6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        if let Some(ref val) = self.ccy_xchg {
            val.validate_constraints(&format!("{path}/CcyXchg"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AmountAndDirection35 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.cdt_dbt_ind
            .validate_constraints(&format!("{path}/CdtDbtInd"), violations);
    }
}
impl crate::common::validate::Validatable for AmountRangeBoundary1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.bdry_amt
            .validate_constraints(&format!("{path}/BdryAmt"), violations);
        self.incl
            .validate_constraints(&format!("{path}/Incl"), violations);
    }
}
impl crate::common::validate::Validatable for BankToCustomerDebitCreditNotificationV11 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.grp_hdr
            .validate_constraints(&format!("{path}/GrpHdr"), violations);
        for (i, item) in self.ntfctn.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Ntfctn[{i}]"), violations);
        }
        for (i, item) in self.splmtry_data.iter().enumerate() {
            item.validate_constraints(&format!("{path}/SplmtryData[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for BankTransactionCodeStructure4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.domn {
            val.validate_constraints(&format!("{path}/Domn"), violations);
        }
        if let Some(ref val) = self.prtry {
            val.validate_constraints(&format!("{path}/Prtry"), violations);
        }
    }
}
impl crate::common::validate::Validatable for BankTransactionCodeStructure5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd
            .validate_constraints(&format!("{path}/Cd"), violations);
        self.fmly
            .validate_constraints(&format!("{path}/Fmly"), violations);
    }
}
impl crate::common::validate::Validatable for BankTransactionCodeStructure6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd
            .validate_constraints(&format!("{path}/Cd"), violations);
        self.sub_fmly_cd
            .validate_constraints(&format!("{path}/SubFmlyCd"), violations);
    }
}
impl crate::common::validate::Validatable for BatchInformation2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.msg_id {
            val.validate_constraints(&format!("{path}/MsgId"), violations);
        }
        if let Some(ref val) = self.pmt_inf_id {
            val.validate_constraints(&format!("{path}/PmtInfId"), violations);
        }
        if let Some(ref val) = self.nb_of_txs {
            val.validate_constraints(&format!("{path}/NbOfTxs"), violations);
        }
        if let Some(ref val) = self.ttl_amt {
            val.validate_constraints(&format!("{path}/TtlAmt"), violations);
        }
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        }
    }
}
impl crate::common::validate::Validatable for BranchAndFinancialInstitutionIdentification6 {
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
impl crate::common::validate::Validatable for BranchData3 {
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
impl crate::common::validate::Validatable for CardAggregated2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.addtl_svc {
            val.validate_constraints(&format!("{path}/AddtlSvc"), violations);
        }
        if let Some(ref val) = self.tx_ctgy {
            val.validate_constraints(&format!("{path}/TxCtgy"), violations);
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            val.validate_constraints(&format!("{path}/SaleRcncltnId"), violations);
        }
        if let Some(ref val) = self.seq_nb_rg {
            val.validate_constraints(&format!("{path}/SeqNbRg"), violations);
        }
        if let Some(ref wrapper) = self.tx_dt_rg {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/TxDtRg"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardEntry5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.card {
            val.validate_constraints(&format!("{path}/Card"), violations);
        }
        if let Some(ref val) = self.poi {
            val.validate_constraints(&format!("{path}/POI"), violations);
        }
        if let Some(ref val) = self.aggtd_ntry {
            val.validate_constraints(&format!("{path}/AggtdNtry"), violations);
        }
        if let Some(ref val) = self.pre_pd_acct {
            val.validate_constraints(&format!("{path}/PrePdAcct"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardIndividualTransaction2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.icc_rltd_data {
            val.validate_constraints(&format!("{path}/ICCRltdData"), violations);
        }
        if let Some(ref val) = self.pmt_cntxt {
            val.validate_constraints(&format!("{path}/PmtCntxt"), violations);
        }
        if let Some(ref val) = self.addtl_svc {
            val.validate_constraints(&format!("{path}/AddtlSvc"), violations);
        }
        if let Some(ref val) = self.tx_ctgy {
            val.validate_constraints(&format!("{path}/TxCtgy"), violations);
        }
        if let Some(ref val) = self.sale_rcncltn_id {
            val.validate_constraints(&format!("{path}/SaleRcncltnId"), violations);
        }
        if let Some(ref val) = self.sale_ref_nb {
            val.validate_constraints(&format!("{path}/SaleRefNb"), violations);
        }
        if let Some(ref val) = self.re_presntmnt_rsn {
            val.validate_constraints(&format!("{path}/RePresntmntRsn"), violations);
        }
        if let Some(ref val) = self.seq_nb {
            val.validate_constraints(&format!("{path}/SeqNb"), violations);
        }
        if let Some(ref val) = self.tx_id {
            val.validate_constraints(&format!("{path}/TxId"), violations);
        }
        if let Some(ref val) = self.pdct {
            val.validate_constraints(&format!("{path}/Pdct"), violations);
        }
        if let Some(ref val) = self.vldtn_dt {
            val.validate_constraints(&format!("{path}/VldtnDt"), violations);
        }
        if let Some(ref val) = self.vldtn_seq_nb {
            val.validate_constraints(&format!("{path}/VldtnSeqNb"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardSecurityInformation1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.csc_mgmt
            .validate_constraints(&format!("{path}/CSCMgmt"), violations);
        if let Some(ref val) = self.csc_val {
            val.validate_constraints(&format!("{path}/CSCVal"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardSequenceNumberRange1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.frst_tx {
            val.validate_constraints(&format!("{path}/FrstTx"), violations);
        }
        if let Some(ref val) = self.last_tx {
            val.validate_constraints(&format!("{path}/LastTx"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardTransaction18 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.card {
            val.validate_constraints(&format!("{path}/Card"), violations);
        }
        if let Some(ref val) = self.poi {
            val.validate_constraints(&format!("{path}/POI"), violations);
        }
        if let Some(ref wrapper) = self.tx {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tx"), violations);
        }
        if let Some(ref val) = self.pre_pd_acct {
            val.validate_constraints(&format!("{path}/PrePdAcct"), violations);
        }
    }
}
impl crate::common::validate::Validatable for CardTransaction3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Aggtd(inner) => {
                inner.validate_constraints(&format!("{path}/Aggtd"), violations);
            }
            Self::Indv(inner) => {
                inner.validate_constraints(&format!("{path}/Indv"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for CardholderAuthentication2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.authntcn_mtd
            .validate_constraints(&format!("{path}/AuthntcnMtd"), violations);
        self.authntcn_ntty
            .validate_constraints(&format!("{path}/AuthntcnNtty"), violations);
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
impl crate::common::validate::Validatable for CashAccount41 {
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
        if let Some(ref val) = self.ownr {
            val.validate_constraints(&format!("{path}/Ownr"), violations);
        }
        if let Some(ref val) = self.svcr {
            val.validate_constraints(&format!("{path}/Svcr"), violations);
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
impl crate::common::validate::Validatable for CashAvailability1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.dt
            .inner
            .validate_constraints(&format!("{path}/Dt"), violations);
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.cdt_dbt_ind
            .validate_constraints(&format!("{path}/CdtDbtInd"), violations);
    }
}
impl crate::common::validate::Validatable for CashAvailabilityDate1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::NbOfDays(inner) => {
                inner.validate_constraints(&format!("{path}/NbOfDays"), violations);
            }
            Self::ActlDt(inner) => {
                inner.validate_constraints(&format!("{path}/ActlDt"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for CashDeposit1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.note_dnmtn
            .validate_constraints(&format!("{path}/NoteDnmtn"), violations);
        self.nb_of_notes
            .validate_constraints(&format!("{path}/NbOfNotes"), violations);
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
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
impl crate::common::validate::Validatable for Charges6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.ttl_chrgs_and_tax_amt {
            val.validate_constraints(&format!("{path}/TtlChrgsAndTaxAmt"), violations);
        }
        for (i, item) in self.rcrd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Rcrd[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ChargesRecord3 {
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
        if let Some(ref val) = self.chrg_incl_ind {
            val.validate_constraints(&format!("{path}/ChrgInclInd"), violations);
        }
        if let Some(ref wrapper) = self.tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.rate {
            val.validate_constraints(&format!("{path}/Rate"), violations);
        }
        if let Some(ref val) = self.br {
            val.validate_constraints(&format!("{path}/Br"), violations);
        }
        if let Some(ref val) = self.agt {
            val.validate_constraints(&format!("{path}/Agt"), violations);
        }
        if let Some(ref val) = self.tax {
            val.validate_constraints(&format!("{path}/Tax"), violations);
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
impl crate::common::validate::Validatable for Contact4 {
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
impl crate::common::validate::Validatable for CorporateAction9 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.evt_tp
            .validate_constraints(&format!("{path}/EvtTp"), violations);
        self.evt_id
            .validate_constraints(&format!("{path}/EvtId"), violations);
    }
}
impl crate::common::validate::Validatable for CreditorReferenceInformation2 {
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
impl crate::common::validate::Validatable for CreditorReferenceType1Choice {
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
impl crate::common::validate::Validatable for CreditorReferenceType2 {
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
impl crate::common::validate::Validatable for CurrencyExchange24 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.src_ccy
            .validate_constraints(&format!("{path}/SrcCcy"), violations);
        if let Some(ref val) = self.trgt_ccy {
            val.validate_constraints(&format!("{path}/TrgtCcy"), violations);
        }
        if let Some(ref val) = self.unit_ccy {
            val.validate_constraints(&format!("{path}/UnitCcy"), violations);
        }
        self.xchg_rate
            .validate_constraints(&format!("{path}/XchgRate"), violations);
        if let Some(ref val) = self.ctrct_id {
            val.validate_constraints(&format!("{path}/CtrctId"), violations);
        }
        if let Some(ref val) = self.qtn_dt {
            val.validate_constraints(&format!("{path}/QtnDt"), violations);
        }
        if let Some(ref val) = self.xchg_rate_base {
            val.validate_constraints(&format!("{path}/XchgRateBase"), violations);
        }
    }
}
impl crate::common::validate::Validatable for DateAndDateTime2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Dt(inner) => {
                inner.validate_constraints(&format!("{path}/Dt"), violations);
            }
            Self::DtTm(inner) => {
                inner.validate_constraints(&format!("{path}/DtTm"), violations);
            }
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
impl crate::common::validate::Validatable for DateOrDateTimePeriod1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Dt(inner) => {
                inner.validate_constraints(&format!("{path}/Dt"), violations);
            }
            Self::DtTm(inner) => {
                inner.validate_constraints(&format!("{path}/DtTm"), violations);
            }
        }
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
impl crate::common::validate::Validatable for DateTimePeriod1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fr_dt_tm
            .validate_constraints(&format!("{path}/FrDtTm"), violations);
        self.to_dt_tm
            .validate_constraints(&format!("{path}/ToDtTm"), violations);
    }
}
impl crate::common::validate::Validatable for DiscountAmountAndType1 {
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
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
    }
}
impl crate::common::validate::Validatable for DiscountAmountType1Choice {
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
impl crate::common::validate::Validatable for DisplayCapabilities1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.disp_tp
            .validate_constraints(&format!("{path}/DispTp"), violations);
        self.nb_of_lines
            .validate_constraints(&format!("{path}/NbOfLines"), violations);
        self.line_width
            .validate_constraints(&format!("{path}/LineWidth"), violations);
    }
}
impl crate::common::validate::Validatable for Document {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.bk_to_cstmr_dbt_cdt_ntfctn
            .validate_constraints(&format!("{path}/BkToCstmrDbtCdtNtfctn"), violations);
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
impl crate::common::validate::Validatable for DocumentLineInformation1 {
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
impl crate::common::validate::Validatable for EntryDetails12 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.btch {
            val.validate_constraints(&format!("{path}/Btch"), violations);
        }
        for (i, item) in self.tx_dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/TxDtls[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for EntryStatus1Choice {
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
impl crate::common::validate::Validatable for EntryTransaction13 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.refs {
            val.validate_constraints(&format!("{path}/Refs"), violations);
        }
        if let Some(ref val) = self.amt {
            val.validate_constraints(&format!("{path}/Amt"), violations);
        }
        if let Some(ref val) = self.cdt_dbt_ind {
            val.validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        }
        if let Some(ref val) = self.amt_dtls {
            val.validate_constraints(&format!("{path}/AmtDtls"), violations);
        }
        for (i, item) in self.avlbty.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Avlbty[{i}]"), violations);
        }
        if let Some(ref val) = self.bk_tx_cd {
            val.validate_constraints(&format!("{path}/BkTxCd"), violations);
        }
        if let Some(ref val) = self.chrgs {
            val.validate_constraints(&format!("{path}/Chrgs"), violations);
        }
        if let Some(ref val) = self.intrst {
            val.validate_constraints(&format!("{path}/Intrst"), violations);
        }
        if let Some(ref val) = self.rltd_pties {
            val.validate_constraints(&format!("{path}/RltdPties"), violations);
        }
        if let Some(ref val) = self.rltd_agts {
            val.validate_constraints(&format!("{path}/RltdAgts"), violations);
        }
        if let Some(ref wrapper) = self.lcl_instrm {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/LclInstrm"), violations);
        }
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate_constraints(&format!("{path}/PmtTpInf"), violations);
        }
        if let Some(ref wrapper) = self.purp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Purp"), violations);
        }
        for (i, item) in self.rltd_rmt_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/RltdRmtInf[{i}]"), violations);
        }
        if let Some(ref val) = self.rmt_inf {
            val.validate_constraints(&format!("{path}/RmtInf"), violations);
        }
        if let Some(ref val) = self.rltd_dts {
            val.validate_constraints(&format!("{path}/RltdDts"), violations);
        }
        if let Some(ref wrapper) = self.rltd_pric {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/RltdPric"), violations);
        }
        for (i, item) in self.rltd_qties.iter().enumerate() {
            item.inner
                .validate_constraints(&format!("{path}/RltdQties[{i}]"), violations);
        }
        if let Some(ref val) = self.fin_instrm_id {
            val.validate_constraints(&format!("{path}/FinInstrmId"), violations);
        }
        if let Some(ref val) = self.tax {
            val.validate_constraints(&format!("{path}/Tax"), violations);
        }
        if let Some(ref val) = self.rtr_inf {
            val.validate_constraints(&format!("{path}/RtrInf"), violations);
        }
        if let Some(ref val) = self.corp_actn {
            val.validate_constraints(&format!("{path}/CorpActn"), violations);
        }
        if let Some(ref val) = self.sfkpg_acct {
            val.validate_constraints(&format!("{path}/SfkpgAcct"), violations);
        }
        for (i, item) in self.csh_dpst.iter().enumerate() {
            item.validate_constraints(&format!("{path}/CshDpst[{i}]"), violations);
        }
        if let Some(ref val) = self.card_tx {
            val.validate_constraints(&format!("{path}/CardTx"), violations);
        }
        if let Some(ref val) = self.addtl_tx_inf {
            val.validate_constraints(&format!("{path}/AddtlTxInf"), violations);
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
impl crate::common::validate::Validatable for FinancialInstitutionIdentification18 {
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
impl crate::common::validate::Validatable for FinancialInstrumentQuantity1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Unit(inner) => {
                inner.validate_constraints(&format!("{path}/Unit"), violations);
            }
            Self::FaceAmt(inner) => {
                inner.validate_constraints(&format!("{path}/FaceAmt"), violations);
            }
            Self::AmtsdVal(inner) => {
                inner.validate_constraints(&format!("{path}/AmtsdVal"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for FromToAmountRange1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fr_amt
            .validate_constraints(&format!("{path}/FrAmt"), violations);
        self.to_amt
            .validate_constraints(&format!("{path}/ToAmt"), violations);
    }
}
impl crate::common::validate::Validatable for Garnishment3 {
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
impl crate::common::validate::Validatable for GenericIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.schme_nm {
            val.validate_constraints(&format!("{path}/SchmeNm"), violations);
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
impl crate::common::validate::Validatable for GenericIdentification32 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
        if let Some(ref val) = self.shrt_nm {
            val.validate_constraints(&format!("{path}/ShrtNm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for GenericOrganisationIdentification1 {
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
impl crate::common::validate::Validatable for GenericPersonIdentification1 {
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
impl crate::common::validate::Validatable for GroupHeader81 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.msg_id
            .validate_constraints(&format!("{path}/MsgId"), violations);
        self.cre_dt_tm
            .validate_constraints(&format!("{path}/CreDtTm"), violations);
        if let Some(ref val) = self.msg_rcpt {
            val.validate_constraints(&format!("{path}/MsgRcpt"), violations);
        }
        if let Some(ref val) = self.msg_pgntn {
            val.validate_constraints(&format!("{path}/MsgPgntn"), violations);
        }
        if let Some(ref val) = self.orgnl_biz_qry {
            val.validate_constraints(&format!("{path}/OrgnlBizQry"), violations);
        }
        if let Some(ref val) = self.addtl_inf {
            val.validate_constraints(&format!("{path}/AddtlInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for IdentificationSource3Choice {
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
impl crate::common::validate::Validatable for ImpliedCurrencyAmountRange1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::FrAmt(inner) => {
                inner.validate_constraints(&format!("{path}/FrAmt"), violations);
            }
            Self::ToAmt(inner) => {
                inner.validate_constraints(&format!("{path}/ToAmt"), violations);
            }
            Self::FrToAmt(inner) => {
                inner.validate_constraints(&format!("{path}/FrToAmt"), violations);
            }
            Self::EQAmt(inner) => {
                inner.validate_constraints(&format!("{path}/EQAmt"), violations);
            }
            Self::NEQAmt(inner) => {
                inner.validate_constraints(&format!("{path}/NEQAmt"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for InterestRecord2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.cdt_dbt_ind
            .validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        if let Some(ref wrapper) = self.tp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.rate {
            val.validate_constraints(&format!("{path}/Rate"), violations);
        }
        if let Some(ref val) = self.fr_to_dt {
            val.validate_constraints(&format!("{path}/FrToDt"), violations);
        }
        if let Some(ref val) = self.rsn {
            val.validate_constraints(&format!("{path}/Rsn"), violations);
        }
        if let Some(ref val) = self.tax {
            val.validate_constraints(&format!("{path}/Tax"), violations);
        }
    }
}
impl crate::common::validate::Validatable for InterestType1Choice {
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
impl crate::common::validate::Validatable for MessageIdentification2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.msg_nm_id {
            val.validate_constraints(&format!("{path}/MsgNmId"), violations);
        }
        if let Some(ref val) = self.msg_id {
            val.validate_constraints(&format!("{path}/MsgId"), violations);
        }
    }
}
impl crate::common::validate::Validatable for NameAndAddress16 {
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
impl crate::common::validate::Validatable for NumberAndSumOfTransactions1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nb_of_ntries {
            val.validate_constraints(&format!("{path}/NbOfNtries"), violations);
        }
        if let Some(ref val) = self.sum {
            val.validate_constraints(&format!("{path}/Sum"), violations);
        }
    }
}
impl crate::common::validate::Validatable for NumberAndSumOfTransactions4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nb_of_ntries {
            val.validate_constraints(&format!("{path}/NbOfNtries"), violations);
        }
        if let Some(ref val) = self.sum {
            val.validate_constraints(&format!("{path}/Sum"), violations);
        }
        if let Some(ref val) = self.ttl_net_ntry {
            val.validate_constraints(&format!("{path}/TtlNetNtry"), violations);
        }
    }
}
impl crate::common::validate::Validatable for OrganisationIdentification29 {
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
impl crate::common::validate::Validatable for OriginalAndCurrentQuantities1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.face_amt
            .validate_constraints(&format!("{path}/FaceAmt"), violations);
        self.amtsd_val
            .validate_constraints(&format!("{path}/AmtsdVal"), violations);
    }
}
impl crate::common::validate::Validatable for OriginalBusinessQuery1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.msg_id
            .validate_constraints(&format!("{path}/MsgId"), violations);
        if let Some(ref val) = self.msg_nm_id {
            val.validate_constraints(&format!("{path}/MsgNmId"), violations);
        }
        if let Some(ref val) = self.cre_dt_tm {
            val.validate_constraints(&format!("{path}/CreDtTm"), violations);
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
impl crate::common::validate::Validatable for OtherIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.sfx {
            val.validate_constraints(&format!("{path}/Sfx"), violations);
        }
        self.tp
            .inner
            .validate_constraints(&format!("{path}/Tp"), violations);
    }
}
impl crate::common::validate::Validatable for Pagination1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.pg_nb
            .validate_constraints(&format!("{path}/PgNb"), violations);
        self.last_pg_ind
            .validate_constraints(&format!("{path}/LastPgInd"), violations);
    }
}
impl crate::common::validate::Validatable for Party38Choice {
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
impl crate::common::validate::Validatable for Party40Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Pty(inner) => {
                inner.validate_constraints(&format!("{path}/Pty"), violations);
            }
            Self::Agt(inner) => {
                inner.validate_constraints(&format!("{path}/Agt"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for PartyIdentification135 {
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
impl crate::common::validate::Validatable for PaymentCard4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.plain_card_data {
            val.validate_constraints(&format!("{path}/PlainCardData"), violations);
        }
        if let Some(ref val) = self.card_ctry_cd {
            val.validate_constraints(&format!("{path}/CardCtryCd"), violations);
        }
        if let Some(ref val) = self.card_brnd {
            val.validate_constraints(&format!("{path}/CardBrnd"), violations);
        }
        if let Some(ref val) = self.addtl_card_data {
            val.validate_constraints(&format!("{path}/AddtlCardData"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentContext3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.card_pres {
            val.validate_constraints(&format!("{path}/CardPres"), violations);
        }
        if let Some(ref val) = self.crdhldr_pres {
            val.validate_constraints(&format!("{path}/CrdhldrPres"), violations);
        }
        if let Some(ref val) = self.on_line_cntxt {
            val.validate_constraints(&format!("{path}/OnLineCntxt"), violations);
        }
        if let Some(ref val) = self.attndnc_cntxt {
            val.validate_constraints(&format!("{path}/AttndncCntxt"), violations);
        }
        if let Some(ref val) = self.tx_envt {
            val.validate_constraints(&format!("{path}/TxEnvt"), violations);
        }
        if let Some(ref val) = self.tx_chanl {
            val.validate_constraints(&format!("{path}/TxChanl"), violations);
        }
        if let Some(ref val) = self.attndnt_msg_cpbl {
            val.validate_constraints(&format!("{path}/AttndntMsgCpbl"), violations);
        }
        if let Some(ref val) = self.attndnt_lang {
            val.validate_constraints(&format!("{path}/AttndntLang"), violations);
        }
        self.card_data_ntry_md
            .validate_constraints(&format!("{path}/CardDataNtryMd"), violations);
        if let Some(ref val) = self.fllbck_ind {
            val.validate_constraints(&format!("{path}/FllbckInd"), violations);
        }
        if let Some(ref val) = self.authntcn_mtd {
            val.validate_constraints(&format!("{path}/AuthntcnMtd"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentReturnReason5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.orgnl_bk_tx_cd {
            val.validate_constraints(&format!("{path}/OrgnlBkTxCd"), violations);
        }
        if let Some(ref val) = self.orgtr {
            val.validate_constraints(&format!("{path}/Orgtr"), violations);
        }
        if let Some(ref wrapper) = self.rsn {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Rsn"), violations);
        }
        for (i, item) in self.addtl_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AddtlInf[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentTypeInformation27 {
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
        if let Some(ref val) = self.seq_tp {
            val.validate_constraints(&format!("{path}/SeqTp"), violations);
        }
        if let Some(ref wrapper) = self.ctgy_purp {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/CtgyPurp"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PersonIdentification13 {
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
impl crate::common::validate::Validatable for PlainCardData1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.pan
            .validate_constraints(&format!("{path}/PAN"), violations);
        if let Some(ref val) = self.card_seq_nb {
            val.validate_constraints(&format!("{path}/CardSeqNb"), violations);
        }
        if let Some(ref val) = self.fctv_dt {
            val.validate_constraints(&format!("{path}/FctvDt"), violations);
        }
        self.xpry_dt
            .validate_constraints(&format!("{path}/XpryDt"), violations);
        if let Some(ref val) = self.svc_cd {
            val.validate_constraints(&format!("{path}/SvcCd"), violations);
        }
        for (i, item) in self.trck_data.iter().enumerate() {
            item.validate_constraints(&format!("{path}/TrckData[{i}]"), violations);
        }
        if let Some(ref val) = self.card_scty_cd {
            val.validate_constraints(&format!("{path}/CardSctyCd"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PointOfInteraction1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.sys_nm {
            val.validate_constraints(&format!("{path}/SysNm"), violations);
        }
        if let Some(ref val) = self.grp_id {
            val.validate_constraints(&format!("{path}/GrpId"), violations);
        }
        if let Some(ref val) = self.cpblties {
            val.validate_constraints(&format!("{path}/Cpblties"), violations);
        }
        for (i, item) in self.cmpnt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Cmpnt[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PointOfInteractionCapabilities1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (i, item) in self.card_rdng_cpblties.iter().enumerate() {
            item.validate_constraints(&format!("{path}/CardRdngCpblties[{i}]"), violations);
        }
        for (i, item) in self.crdhldr_vrfctn_cpblties.iter().enumerate() {
            item.validate_constraints(&format!("{path}/CrdhldrVrfctnCpblties[{i}]"), violations);
        }
        if let Some(ref val) = self.on_line_cpblties {
            val.validate_constraints(&format!("{path}/OnLineCpblties"), violations);
        }
        for (i, item) in self.disp_cpblties.iter().enumerate() {
            item.validate_constraints(&format!("{path}/DispCpblties[{i}]"), violations);
        }
        if let Some(ref val) = self.prt_line_width {
            val.validate_constraints(&format!("{path}/PrtLineWidth"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PointOfInteractionComponent1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.poi_cmpnt_tp
            .validate_constraints(&format!("{path}/POICmpntTp"), violations);
        if let Some(ref val) = self.manfctr_id {
            val.validate_constraints(&format!("{path}/ManfctrId"), violations);
        }
        if let Some(ref val) = self.mdl {
            val.validate_constraints(&format!("{path}/Mdl"), violations);
        }
        if let Some(ref val) = self.vrsn_nb {
            val.validate_constraints(&format!("{path}/VrsnNb"), violations);
        }
        if let Some(ref val) = self.srl_nb {
            val.validate_constraints(&format!("{path}/SrlNb"), violations);
        }
        for (i, item) in self.apprvl_nb.iter().enumerate() {
            item.validate_constraints(&format!("{path}/ApprvlNb[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PostalAddress24 {
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
impl crate::common::validate::Validatable for Price7 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .inner
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.val
            .inner
            .validate_constraints(&format!("{path}/Val"), violations);
    }
}
impl crate::common::validate::Validatable for PriceRateOrAmount3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Rate(inner) => {
                inner.validate_constraints(&format!("{path}/Rate"), violations);
            }
            Self::Amt(inner) => {
                inner.validate_constraints(&format!("{path}/Amt"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for Product2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.pdct_cd
            .validate_constraints(&format!("{path}/PdctCd"), violations);
        if let Some(ref val) = self.unit_of_measr {
            val.validate_constraints(&format!("{path}/UnitOfMeasr"), violations);
        }
        if let Some(ref val) = self.pdct_qty {
            val.validate_constraints(&format!("{path}/PdctQty"), violations);
        }
        if let Some(ref val) = self.unit_pric {
            val.validate_constraints(&format!("{path}/UnitPric"), violations);
        }
        if let Some(ref val) = self.pdct_amt {
            val.validate_constraints(&format!("{path}/PdctAmt"), violations);
        }
        if let Some(ref val) = self.tax_tp {
            val.validate_constraints(&format!("{path}/TaxTp"), violations);
        }
        if let Some(ref val) = self.addtl_pdct_inf {
            val.validate_constraints(&format!("{path}/AddtlPdctInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ProprietaryAgent4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.agt
            .validate_constraints(&format!("{path}/Agt"), violations);
    }
}
impl crate::common::validate::Validatable for ProprietaryBankTransactionCodeStructure1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cd
            .validate_constraints(&format!("{path}/Cd"), violations);
        if let Some(ref val) = self.issr {
            val.validate_constraints(&format!("{path}/Issr"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ProprietaryDate3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.dt
            .inner
            .validate_constraints(&format!("{path}/Dt"), violations);
    }
}
impl crate::common::validate::Validatable for ProprietaryParty5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.pty
            .inner
            .validate_constraints(&format!("{path}/Pty"), violations);
    }
}
impl crate::common::validate::Validatable for ProprietaryPrice2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.pric
            .validate_constraints(&format!("{path}/Pric"), violations);
    }
}
impl crate::common::validate::Validatable for ProprietaryQuantity1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.qty
            .validate_constraints(&format!("{path}/Qty"), violations);
    }
}
impl crate::common::validate::Validatable for ProprietaryReference1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .validate_constraints(&format!("{path}/Tp"), violations);
        self.r#ref
            .validate_constraints(&format!("{path}/Ref"), violations);
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
impl crate::common::validate::Validatable for Rate4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tp
            .inner
            .validate_constraints(&format!("{path}/Tp"), violations);
        if let Some(ref val) = self.vldty_rg {
            val.validate_constraints(&format!("{path}/VldtyRg"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RateType4Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Pctg(inner) => {
                inner.validate_constraints(&format!("{path}/Pctg"), violations);
            }
            Self::Othr(inner) => {
                inner.validate_constraints(&format!("{path}/Othr"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for ReferredDocumentInformation7 {
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
impl crate::common::validate::Validatable for ReferredDocumentType3Choice {
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
impl crate::common::validate::Validatable for ReferredDocumentType4 {
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
impl crate::common::validate::Validatable for RemittanceAmount2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.due_pybl_amt {
            val.validate_constraints(&format!("{path}/DuePyblAmt"), violations);
        }
        for (i, item) in self.dscnt_apld_amt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/DscntApldAmt[{i}]"), violations);
        }
        if let Some(ref val) = self.cdt_note_amt {
            val.validate_constraints(&format!("{path}/CdtNoteAmt"), violations);
        }
        for (i, item) in self.tax_amt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/TaxAmt[{i}]"), violations);
        }
        for (i, item) in self.adjstmnt_amt_and_rsn.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AdjstmntAmtAndRsn[{i}]"), violations);
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate_constraints(&format!("{path}/RmtdAmt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceAmount3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.due_pybl_amt {
            val.validate_constraints(&format!("{path}/DuePyblAmt"), violations);
        }
        for (i, item) in self.dscnt_apld_amt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/DscntApldAmt[{i}]"), violations);
        }
        if let Some(ref val) = self.cdt_note_amt {
            val.validate_constraints(&format!("{path}/CdtNoteAmt"), violations);
        }
        for (i, item) in self.tax_amt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/TaxAmt[{i}]"), violations);
        }
        for (i, item) in self.adjstmnt_amt_and_rsn.iter().enumerate() {
            item.validate_constraints(&format!("{path}/AdjstmntAmtAndRsn[{i}]"), violations);
        }
        if let Some(ref val) = self.rmtd_amt {
            val.validate_constraints(&format!("{path}/RmtdAmt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for RemittanceInformation21 {
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
impl crate::common::validate::Validatable for RemittanceLocation7 {
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
impl crate::common::validate::Validatable for RemittanceLocationData1 {
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
impl crate::common::validate::Validatable for ReportEntry13 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.ntry_ref {
            val.validate_constraints(&format!("{path}/NtryRef"), violations);
        }
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.cdt_dbt_ind
            .validate_constraints(&format!("{path}/CdtDbtInd"), violations);
        if let Some(ref val) = self.rvsl_ind {
            val.validate_constraints(&format!("{path}/RvslInd"), violations);
        }
        self.sts
            .inner
            .validate_constraints(&format!("{path}/Sts"), violations);
        if let Some(ref wrapper) = self.bookg_dt {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/BookgDt"), violations);
        }
        if let Some(ref wrapper) = self.val_dt {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/ValDt"), violations);
        }
        if let Some(ref val) = self.acct_svcr_ref {
            val.validate_constraints(&format!("{path}/AcctSvcrRef"), violations);
        }
        for (i, item) in self.avlbty.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Avlbty[{i}]"), violations);
        }
        self.bk_tx_cd
            .validate_constraints(&format!("{path}/BkTxCd"), violations);
        if let Some(ref val) = self.comssn_wvr_ind {
            val.validate_constraints(&format!("{path}/ComssnWvrInd"), violations);
        }
        if let Some(ref val) = self.addtl_inf_ind {
            val.validate_constraints(&format!("{path}/AddtlInfInd"), violations);
        }
        if let Some(ref val) = self.amt_dtls {
            val.validate_constraints(&format!("{path}/AmtDtls"), violations);
        }
        if let Some(ref val) = self.chrgs {
            val.validate_constraints(&format!("{path}/Chrgs"), violations);
        }
        if let Some(ref wrapper) = self.tech_inpt_chanl {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/TechInptChanl"), violations);
        }
        if let Some(ref val) = self.intrst {
            val.validate_constraints(&format!("{path}/Intrst"), violations);
        }
        if let Some(ref val) = self.card_tx {
            val.validate_constraints(&format!("{path}/CardTx"), violations);
        }
        for (i, item) in self.ntry_dtls.iter().enumerate() {
            item.validate_constraints(&format!("{path}/NtryDtls[{i}]"), violations);
        }
        if let Some(ref val) = self.addtl_ntry_inf {
            val.validate_constraints(&format!("{path}/AddtlNtryInf"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ReportingSource1Choice {
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
impl crate::common::validate::Validatable for ReturnReason5Choice {
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
impl crate::common::validate::Validatable for SecuritiesAccount19 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.id
            .validate_constraints(&format!("{path}/Id"), violations);
        if let Some(ref val) = self.tp {
            val.validate_constraints(&format!("{path}/Tp"), violations);
        }
        if let Some(ref val) = self.nm {
            val.validate_constraints(&format!("{path}/Nm"), violations);
        }
    }
}
impl crate::common::validate::Validatable for SecurityIdentification19 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.isin {
            val.validate_constraints(&format!("{path}/ISIN"), violations);
        }
        for (i, item) in self.othr_id.iter().enumerate() {
            item.validate_constraints(&format!("{path}/OthrId[{i}]"), violations);
        }
        if let Some(ref val) = self.desc {
            val.validate_constraints(&format!("{path}/Desc"), violations);
        }
    }
}
impl crate::common::validate::Validatable for SequenceRange1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.fr_seq
            .validate_constraints(&format!("{path}/FrSeq"), violations);
        self.to_seq
            .validate_constraints(&format!("{path}/ToSeq"), violations);
    }
}
impl crate::common::validate::Validatable for SequenceRange1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::FrSeq(inner) => {
                inner.validate_constraints(&format!("{path}/FrSeq"), violations);
            }
            Self::ToSeq(inner) => {
                inner.validate_constraints(&format!("{path}/ToSeq"), violations);
            }
            Self::FrToSeq(inner) => {
                inner.validate_constraints(&format!("{path}/FrToSeq"), violations);
            }
            Self::EQSeq(inner) => {
                inner.validate_constraints(&format!("{path}/EQSeq"), violations);
            }
            Self::NEQSeq(inner) => {
                inner.validate_constraints(&format!("{path}/NEQSeq"), violations);
            }
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
impl crate::common::validate::Validatable for StructuredRemittanceInformation17 {
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
impl crate::common::validate::Validatable for TaxAmountAndType1 {
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
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
    }
}
impl crate::common::validate::Validatable for TaxAmountType1Choice {
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
impl crate::common::validate::Validatable for TaxCharges2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.id {
            val.validate_constraints(&format!("{path}/Id"), violations);
        }
        if let Some(ref val) = self.rate {
            val.validate_constraints(&format!("{path}/Rate"), violations);
        }
        if let Some(ref val) = self.amt {
            val.validate_constraints(&format!("{path}/Amt"), violations);
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
impl crate::common::validate::Validatable for TaxInformation10 {
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
impl crate::common::validate::Validatable for TechnicalInputChannel1Choice {
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
impl crate::common::validate::Validatable for TotalTransactions6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.ttl_ntries {
            val.validate_constraints(&format!("{path}/TtlNtries"), violations);
        }
        if let Some(ref val) = self.ttl_cdt_ntries {
            val.validate_constraints(&format!("{path}/TtlCdtNtries"), violations);
        }
        if let Some(ref val) = self.ttl_dbt_ntries {
            val.validate_constraints(&format!("{path}/TtlDbtNtries"), violations);
        }
        for (i, item) in self.ttl_ntries_per_bk_tx_cd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/TtlNtriesPerBkTxCd[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TotalsPerBankTransactionCode5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nb_of_ntries {
            val.validate_constraints(&format!("{path}/NbOfNtries"), violations);
        }
        if let Some(ref val) = self.sum {
            val.validate_constraints(&format!("{path}/Sum"), violations);
        }
        if let Some(ref val) = self.ttl_net_ntry {
            val.validate_constraints(&format!("{path}/TtlNetNtry"), violations);
        }
        if let Some(ref val) = self.cdt_ntries {
            val.validate_constraints(&format!("{path}/CdtNtries"), violations);
        }
        if let Some(ref val) = self.dbt_ntries {
            val.validate_constraints(&format!("{path}/DbtNtries"), violations);
        }
        if let Some(ref val) = self.fcst_ind {
            val.validate_constraints(&format!("{path}/FcstInd"), violations);
        }
        self.bk_tx_cd
            .validate_constraints(&format!("{path}/BkTxCd"), violations);
        for (i, item) in self.avlbty.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Avlbty[{i}]"), violations);
        }
        if let Some(ref wrapper) = self.dt {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Dt"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TrackData1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.trck_nb {
            val.validate_constraints(&format!("{path}/TrckNb"), violations);
        }
        self.trck_val
            .validate_constraints(&format!("{path}/TrckVal"), violations);
    }
}
impl crate::common::validate::Validatable for TransactionAgents5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.instg_agt {
            val.validate_constraints(&format!("{path}/InstgAgt"), violations);
        }
        if let Some(ref val) = self.instd_agt {
            val.validate_constraints(&format!("{path}/InstdAgt"), violations);
        }
        if let Some(ref val) = self.dbtr_agt {
            val.validate_constraints(&format!("{path}/DbtrAgt"), violations);
        }
        if let Some(ref val) = self.cdtr_agt {
            val.validate_constraints(&format!("{path}/CdtrAgt"), violations);
        }
        if let Some(ref val) = self.intrmy_agt1 {
            val.validate_constraints(&format!("{path}/IntrmyAgt1"), violations);
        }
        if let Some(ref val) = self.intrmy_agt2 {
            val.validate_constraints(&format!("{path}/IntrmyAgt2"), violations);
        }
        if let Some(ref val) = self.intrmy_agt3 {
            val.validate_constraints(&format!("{path}/IntrmyAgt3"), violations);
        }
        if let Some(ref val) = self.rcvg_agt {
            val.validate_constraints(&format!("{path}/RcvgAgt"), violations);
        }
        if let Some(ref val) = self.dlvrg_agt {
            val.validate_constraints(&format!("{path}/DlvrgAgt"), violations);
        }
        if let Some(ref val) = self.issg_agt {
            val.validate_constraints(&format!("{path}/IssgAgt"), violations);
        }
        if let Some(ref val) = self.sttlm_plc {
            val.validate_constraints(&format!("{path}/SttlmPlc"), violations);
        }
        for (i, item) in self.prtry.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Prtry[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TransactionDates3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.accptnc_dt_tm {
            val.validate_constraints(&format!("{path}/AccptncDtTm"), violations);
        }
        if let Some(ref val) = self.trad_actvty_ctrctl_sttlm_dt {
            val.validate_constraints(&format!("{path}/TradActvtyCtrctlSttlmDt"), violations);
        }
        if let Some(ref val) = self.trad_dt {
            val.validate_constraints(&format!("{path}/TradDt"), violations);
        }
        if let Some(ref val) = self.intr_bk_sttlm_dt {
            val.validate_constraints(&format!("{path}/IntrBkSttlmDt"), violations);
        }
        if let Some(ref val) = self.start_dt {
            val.validate_constraints(&format!("{path}/StartDt"), violations);
        }
        if let Some(ref val) = self.end_dt {
            val.validate_constraints(&format!("{path}/EndDt"), violations);
        }
        if let Some(ref val) = self.tx_dt_tm {
            val.validate_constraints(&format!("{path}/TxDtTm"), violations);
        }
        for (i, item) in self.prtry.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Prtry[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TransactionIdentifier1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.tx_dt_tm
            .validate_constraints(&format!("{path}/TxDtTm"), violations);
        self.tx_ref
            .validate_constraints(&format!("{path}/TxRef"), violations);
    }
}
impl crate::common::validate::Validatable for TransactionInterest4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.ttl_intrst_and_tax_amt {
            val.validate_constraints(&format!("{path}/TtlIntrstAndTaxAmt"), violations);
        }
        for (i, item) in self.rcrd.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Rcrd[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TransactionParties9 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.initg_pty {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/InitgPty"), violations);
        }
        if let Some(ref wrapper) = self.dbtr {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Dbtr"), violations);
        }
        if let Some(ref val) = self.dbtr_acct {
            val.validate_constraints(&format!("{path}/DbtrAcct"), violations);
        }
        if let Some(ref wrapper) = self.ultmt_dbtr {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/UltmtDbtr"), violations);
        }
        if let Some(ref wrapper) = self.cdtr {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/Cdtr"), violations);
        }
        if let Some(ref val) = self.cdtr_acct {
            val.validate_constraints(&format!("{path}/CdtrAcct"), violations);
        }
        if let Some(ref wrapper) = self.ultmt_cdtr {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/UltmtCdtr"), violations);
        }
        if let Some(ref wrapper) = self.tradg_pty {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/TradgPty"), violations);
        }
        for (i, item) in self.prtry.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Prtry[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for TransactionPrice4Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::DealPric(inner) => {
                inner.validate_constraints(&format!("{path}/DealPric"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for TransactionQuantities3Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Qty(inner) => {
                inner.validate_constraints(&format!("{path}/Qty"), violations);
            }
            Self::OrgnlAndCurFaceAmt(inner) => {
                inner.validate_constraints(&format!("{path}/OrgnlAndCurFaceAmt"), violations);
            }
            Self::Prtry(inner) => {
                inner.validate_constraints(&format!("{path}/Prtry"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for TransactionReferences6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.msg_id {
            val.validate_constraints(&format!("{path}/MsgId"), violations);
        }
        if let Some(ref val) = self.acct_svcr_ref {
            val.validate_constraints(&format!("{path}/AcctSvcrRef"), violations);
        }
        if let Some(ref val) = self.pmt_inf_id {
            val.validate_constraints(&format!("{path}/PmtInfId"), violations);
        }
        if let Some(ref val) = self.instr_id {
            val.validate_constraints(&format!("{path}/InstrId"), violations);
        }
        if let Some(ref val) = self.end_to_end_id {
            val.validate_constraints(&format!("{path}/EndToEndId"), violations);
        }
        if let Some(ref val) = self.uetr {
            val.validate_constraints(&format!("{path}/UETR"), violations);
        }
        if let Some(ref val) = self.tx_id {
            val.validate_constraints(&format!("{path}/TxId"), violations);
        }
        if let Some(ref val) = self.mndt_id {
            val.validate_constraints(&format!("{path}/MndtId"), violations);
        }
        if let Some(ref val) = self.chq_nb {
            val.validate_constraints(&format!("{path}/ChqNb"), violations);
        }
        if let Some(ref val) = self.clr_sys_ref {
            val.validate_constraints(&format!("{path}/ClrSysRef"), violations);
        }
        if let Some(ref val) = self.acct_ownr_tx_id {
            val.validate_constraints(&format!("{path}/AcctOwnrTxId"), violations);
        }
        if let Some(ref val) = self.acct_svcr_tx_id {
            val.validate_constraints(&format!("{path}/AcctSvcrTxId"), violations);
        }
        if let Some(ref val) = self.mkt_infrstrctr_tx_id {
            val.validate_constraints(&format!("{path}/MktInfrstrctrTxId"), violations);
        }
        if let Some(ref val) = self.prcg_id {
            val.validate_constraints(&format!("{path}/PrcgId"), violations);
        }
        for (i, item) in self.prtry.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Prtry[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for YieldedOrValueType1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Yldd(inner) => {
                inner.validate_constraints(&format!("{path}/Yldd"), violations);
            }
            Self::ValTp(inner) => {
                inner.validate_constraints(&format!("{path}/ValTp"), violations);
            }
        }
    }
}
impl crate::common::validate::IsoMessage for Document {
    fn message_type(&self) -> &'static str {
        "camt.054.001.11"
    }
    fn root_path(&self) -> &'static str {
        "/Document"
    }
}
