/*! Generated from ISO 20022 XSD schema.
Namespace: `urn:iso:std:iso:20022:tech:xsd:pain.001.001.11`*/
/// Fraction digits: 5
/// Total digits: 18
/// Minimum value (inclusive): 0
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyAndAmountSimpleType(pub String);
impl TryFrom<String> for ActiveOrHistoricCurrencyAndAmountSimpleType {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let frac_count = value.find('.').map_or(0, |dot| {
                        value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count()
                    });
                    frac_count > 5usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum fraction digits 5",
                            value.find('.').map_or(0, |dot| value[dot + 1..]
                                .chars()
                                .filter(char::is_ascii_digit)
                                .count())
                        ),
                    });
                }
            }
            {
                let violated = {
                    let digit_count = value.chars().filter(char::is_ascii_digit).count();
                    digit_count > 18usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum total digits 18",
                            value.chars().filter(char::is_ascii_digit).count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ActiveOrHistoricCurrencyAndAmountSimpleType {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ActiveOrHistoricCurrencyAndAmountSimpleType> for String {
    fn from(v: ActiveOrHistoricCurrencyAndAmountSimpleType) -> Self {
        v.0
    }
}
/// Pattern: `[A-Z]{3,3}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ActiveOrHistoricCurrencyCode(pub String);
impl TryFrom<String> for ActiveOrHistoricCurrencyCode {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 3usize
                        || ({
                            let b = bytes[0usize];
                            !(65u8..=90u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(65u8..=90u8).contains(&b)
                        })
                        || ({
                            let b = bytes[2usize];
                            !(65u8..=90u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [A-Z]{3,3}".to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ActiveOrHistoricCurrencyCode {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ActiveOrHistoricCurrencyCode> for String {
    fn from(v: ActiveOrHistoricCurrencyCode) -> Self {
        v.0
    }
}
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AdviceType1Code {
    #[serde(rename = "ADWD")]
    Adwd,
    #[serde(rename = "ADND")]
    Adnd,
}
/// Pattern: `[A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct AnyBICDec2014Identifier(pub String);
impl TryFrom<String> for AnyBICDec2014Identifier {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    let len = bytes.len();
                    let result: bool = (|| -> bool {
                        let mut pos: usize = 0;
                        if !(8usize..=11usize).contains(&len) {
                            return true;
                        }
                        {
                            let end = pos + 4usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let saved = pos;
                            let matched: bool = (|| -> bool {
                                {
                                    let end = pos + 3usize;
                                    if end > len {
                                        return true;
                                    }
                                    for &b in &bytes[pos..end] {
                                        if !(65u8..=90u8).contains(&b)
                                            && !(48u8..=57u8).contains(&b)
                                        {
                                            return true;
                                        }
                                    }
                                    pos = end;
                                }
                                false
                            })();
                            if matched {
                                pos = saved;
                            }
                        }
                        if pos != len {
                            return true;
                        }
                        false
                    })();
                    result
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"
                            .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl AnyBICDec2014Identifier {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<AnyBICDec2014Identifier> for String {
    fn from(v: AnyBICDec2014Identifier) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for BICFIDec2014Identifier {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    let len = bytes.len();
                    let result: bool = (|| -> bool {
                        let mut pos: usize = 0;
                        if !(8usize..=11usize).contains(&len) {
                            return true;
                        }
                        {
                            let end = pos + 4usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let saved = pos;
                            let matched: bool = (|| -> bool {
                                {
                                    let end = pos + 3usize;
                                    if end > len {
                                        return true;
                                    }
                                    for &b in &bytes[pos..end] {
                                        if !(65u8..=90u8).contains(&b)
                                            && !(48u8..=57u8).contains(&b)
                                        {
                                            return true;
                                        }
                                    }
                                    pos = end;
                                }
                                false
                            })();
                            if matched {
                                pos = saved;
                            }
                        }
                        if pos != len {
                            return true;
                        }
                        false
                    })();
                    result
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"
                            .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl BICFIDec2014Identifier {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<BICFIDec2014Identifier> for String {
    fn from(v: BICFIDec2014Identifier) -> Self {
        v.0
    }
}
/// Fraction digits: 10
/// Total digits: 11
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct BaseOneRate(pub String);
impl TryFrom<String> for BaseOneRate {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let frac_count = value.find('.').map_or(0, |dot| {
                        value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count()
                    });
                    frac_count > 10usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum fraction digits 10",
                            value.find('.').map_or(0, |dot| value[dot + 1..]
                                .chars()
                                .filter(char::is_ascii_digit)
                                .count())
                        ),
                    });
                }
            }
            {
                let violated = {
                    let digit_count = value.chars().filter(char::is_ascii_digit).count();
                    digit_count > 11usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum total digits 11",
                            value.chars().filter(char::is_ascii_digit).count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl BaseOneRate {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<BaseOneRate> for String {
    fn from(v: BaseOneRate) -> Self {
        v.0
    }
}
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
pub enum ChequeDelivery1Code {
    #[serde(rename = "MLDB")]
    Mldb,
    #[serde(rename = "MLCD")]
    Mlcd,
    #[serde(rename = "MLFA")]
    Mlfa,
    #[serde(rename = "CRDB")]
    Crdb,
    #[serde(rename = "CRCD")]
    Crcd,
    #[serde(rename = "CRFA")]
    Crfa,
    #[serde(rename = "PUDB")]
    Pudb,
    #[serde(rename = "PUCD")]
    Pucd,
    #[serde(rename = "PUFA")]
    Pufa,
    #[serde(rename = "RGDB")]
    Rgdb,
    #[serde(rename = "RGCD")]
    Rgcd,
    #[serde(rename = "RGFA")]
    Rgfa,
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChequeType2Code {
    #[serde(rename = "CCHQ")]
    Cchq,
    #[serde(rename = "CCCH")]
    Ccch,
    #[serde(rename = "BCHQ")]
    Bchq,
    #[serde(rename = "DRFT")]
    Drft,
    #[serde(rename = "ELDR")]
    Eldr,
}
/// Pattern: `[A-Z]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct CountryCode(pub String);
impl TryFrom<String> for CountryCode {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 2usize
                        || ({
                            let b = bytes[0usize];
                            !(65u8..=90u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(65u8..=90u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [A-Z]{2,2}".to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl CountryCode {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<CountryCode> for String {
    fn from(v: CountryCode) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for DecimalNumber {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let frac_count = value.find('.').map_or(0, |dot| {
                        value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count()
                    });
                    frac_count > 17usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum fraction digits 17",
                            value.find('.').map_or(0, |dot| value[dot + 1..]
                                .chars()
                                .filter(char::is_ascii_digit)
                                .count())
                        ),
                    });
                }
            }
            {
                let violated = {
                    let digit_count = value.chars().filter(char::is_ascii_digit).count();
                    digit_count > 18usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum total digits 18",
                            value.chars().filter(char::is_ascii_digit).count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl DecimalNumber {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<DecimalNumber> for String {
    fn from(v: DecimalNumber) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for Exact2NumericText {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 2usize
                        || ({
                            let b = bytes[0usize];
                            !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(48u8..=57u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [0-9]{2}".to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Exact2NumericText {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Exact2NumericText> for String {
    fn from(v: Exact2NumericText) -> Self {
        v.0
    }
}
/// Pattern: `[a-zA-Z0-9]{4}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Exact4AlphaNumericText(pub String);
impl TryFrom<String> for Exact4AlphaNumericText {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 4usize
                        || ({
                            let b = bytes[0usize];
                            !(97u8..=122u8).contains(&b)
                                && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(97u8..=122u8).contains(&b)
                                && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[2usize];
                            !(97u8..=122u8).contains(&b)
                                && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[3usize];
                            !(97u8..=122u8).contains(&b)
                                && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [a-zA-Z0-9]{4}".to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Exact4AlphaNumericText {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Exact4AlphaNumericText> for String {
    fn from(v: Exact4AlphaNumericText) -> Self {
        v.0
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ExchangeRateType1Code {
    #[serde(rename = "SPOT")]
    Spot,
    #[serde(rename = "SALE")]
    Sale,
    #[serde(rename = "AGRD")]
    Agrd,
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalAccountIdentification1Code(pub String);
impl TryFrom<String> for ExternalAccountIdentification1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalAccountIdentification1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalAccountIdentification1Code> for String {
    fn from(v: ExternalAccountIdentification1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCashAccountType1Code(pub String);
impl TryFrom<String> for ExternalCashAccountType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalCashAccountType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalCashAccountType1Code> for String {
    fn from(v: ExternalCashAccountType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCategoryPurpose1Code(pub String);
impl TryFrom<String> for ExternalCategoryPurpose1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalCategoryPurpose1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalCategoryPurpose1Code> for String {
    fn from(v: ExternalCategoryPurpose1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 5
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalClearingSystemIdentification1Code(pub String);
impl TryFrom<String> for ExternalClearingSystemIdentification1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 5usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 5",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalClearingSystemIdentification1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalClearingSystemIdentification1Code> for String {
    fn from(v: ExternalClearingSystemIdentification1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCreditorAgentInstruction1Code(pub String);
impl TryFrom<String> for ExternalCreditorAgentInstruction1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalCreditorAgentInstruction1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalCreditorAgentInstruction1Code> for String {
    fn from(v: ExternalCreditorAgentInstruction1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDebtorAgentInstruction1Code(pub String);
impl TryFrom<String> for ExternalDebtorAgentInstruction1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalDebtorAgentInstruction1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalDebtorAgentInstruction1Code> for String {
    fn from(v: ExternalDebtorAgentInstruction1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDiscountAmountType1Code(pub String);
impl TryFrom<String> for ExternalDiscountAmountType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalDiscountAmountType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalDiscountAmountType1Code> for String {
    fn from(v: ExternalDiscountAmountType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalDocumentLineType1Code(pub String);
impl TryFrom<String> for ExternalDocumentLineType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalDocumentLineType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalDocumentLineType1Code> for String {
    fn from(v: ExternalDocumentLineType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalFinancialInstitutionIdentification1Code(pub String);
impl TryFrom<String> for ExternalFinancialInstitutionIdentification1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalFinancialInstitutionIdentification1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalFinancialInstitutionIdentification1Code> for String {
    fn from(v: ExternalFinancialInstitutionIdentification1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalGarnishmentType1Code(pub String);
impl TryFrom<String> for ExternalGarnishmentType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalGarnishmentType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalGarnishmentType1Code> for String {
    fn from(v: ExternalGarnishmentType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 35
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalLocalInstrument1Code(pub String);
impl TryFrom<String> for ExternalLocalInstrument1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 35usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 35",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalLocalInstrument1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalLocalInstrument1Code> for String {
    fn from(v: ExternalLocalInstrument1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalMandateSetupReason1Code(pub String);
impl TryFrom<String> for ExternalMandateSetupReason1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalMandateSetupReason1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalMandateSetupReason1Code> for String {
    fn from(v: ExternalMandateSetupReason1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalOrganisationIdentification1Code(pub String);
impl TryFrom<String> for ExternalOrganisationIdentification1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalOrganisationIdentification1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalOrganisationIdentification1Code> for String {
    fn from(v: ExternalOrganisationIdentification1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalPersonIdentification1Code(pub String);
impl TryFrom<String> for ExternalPersonIdentification1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalPersonIdentification1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalPersonIdentification1Code> for String {
    fn from(v: ExternalPersonIdentification1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalProxyAccountType1Code(pub String);
impl TryFrom<String> for ExternalProxyAccountType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalProxyAccountType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalProxyAccountType1Code> for String {
    fn from(v: ExternalProxyAccountType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalPurpose1Code(pub String);
impl TryFrom<String> for ExternalPurpose1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalPurpose1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalPurpose1Code> for String {
    fn from(v: ExternalPurpose1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalServiceLevel1Code(pub String);
impl TryFrom<String> for ExternalServiceLevel1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalServiceLevel1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalServiceLevel1Code> for String {
    fn from(v: ExternalServiceLevel1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalTaxAmountType1Code(pub String);
impl TryFrom<String> for ExternalTaxAmountType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalTaxAmountType1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalTaxAmountType1Code> for String {
    fn from(v: ExternalTaxAmountType1Code) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for IBAN2007Identifier {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    let len = bytes.len();
                    let result: bool = (|| -> bool {
                        let mut pos: usize = 0;
                        if !(5usize..=34usize).contains(&len) {
                            return true;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(65u8..=90u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let end = pos + 2usize;
                            if end > len {
                                return true;
                            }
                            for &b in &bytes[pos..end] {
                                if !(48u8..=57u8).contains(&b) {
                                    return true;
                                }
                            }
                            pos = end;
                        }
                        {
                            let start = pos;
                            let limit = if pos + 30usize < len {
                                pos + 30usize
                            } else {
                                len
                            };
                            while pos < limit {
                                let b = bytes[pos];
                                if !(97u8..=122u8).contains(&b)
                                    && !(65u8..=90u8).contains(&b)
                                    && !(48u8..=57u8).contains(&b)
                                {
                                    break;
                                }
                                pos += 1;
                            }
                            let matched = pos - start;
                            if matched < 1usize {
                                return true;
                            }
                        }
                        if pos != len {
                            return true;
                        }
                        false
                    })();
                    result
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message:
                            "value does not match pattern [A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"
                                .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl IBAN2007Identifier {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<IBAN2007Identifier> for String {
    fn from(v: IBAN2007Identifier) -> Self {
        v.0
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODate(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODateTime(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISOYear(pub String);
/// Pattern: `[A-Z0-9]{18,18}[0-9]{2,2}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct LEIIdentifier(pub String);
impl TryFrom<String> for LEIIdentifier {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 20usize
                        || ({
                            let b = bytes[0usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[2usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[3usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[4usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[5usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[6usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[7usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[8usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[9usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[10usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[11usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[12usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[13usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[14usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[15usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[16usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[17usize];
                            !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[18usize];
                            !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[19usize];
                            !(48u8..=57u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [A-Z0-9]{18,18}[0-9]{2,2}"
                            .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl LEIIdentifier {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<LEIIdentifier> for String {
    fn from(v: LEIIdentifier) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for Max10KBinary {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 10240usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 10240",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max10KBinary {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max10KBinary> for String {
    fn from(v: Max10KBinary) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 10
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max10Text(pub String);
impl TryFrom<String> for Max10Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 10usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 10",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max10Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max10Text> for String {
    fn from(v: Max10Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 128
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max128Text(pub String);
impl TryFrom<String> for Max128Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 128usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 128",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max128Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max128Text> for String {
    fn from(v: Max128Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 140
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max140Text(pub String);
impl TryFrom<String> for Max140Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 140usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 140",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max140Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max140Text> for String {
    fn from(v: Max140Text) -> Self {
        v.0
    }
}
/// Pattern: `[0-9]{1,15}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max15NumericText(pub String);
impl TryFrom<String> for Max15NumericText {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    let len = bytes.len();
                    let result: bool = (|| -> bool {
                        let mut pos: usize = 0;
                        if !(1usize..=15usize).contains(&len) {
                            return true;
                        }
                        {
                            let start = pos;
                            let limit = if pos + 15usize < len {
                                pos + 15usize
                            } else {
                                len
                            };
                            while pos < limit {
                                let b = bytes[pos];
                                if !(48u8..=57u8).contains(&b) {
                                    break;
                                }
                                pos += 1;
                            }
                            let matched = pos - start;
                            if matched < 1usize {
                                return true;
                            }
                        }
                        if pos != len {
                            return true;
                        }
                        false
                    })();
                    result
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [0-9]{1,15}".to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max15NumericText {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max15NumericText> for String {
    fn from(v: Max15NumericText) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 16
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max16Text(pub String);
impl TryFrom<String> for Max16Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 16usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 16",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max16Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max16Text> for String {
    fn from(v: Max16Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 2048
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max2048Text(pub String);
impl TryFrom<String> for Max2048Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 2048usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 2048",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max2048Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max2048Text> for String {
    fn from(v: Max2048Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 34
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max34Text(pub String);
impl TryFrom<String> for Max34Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 34usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 34",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max34Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max34Text> for String {
    fn from(v: Max34Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 350
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max350Text(pub String);
impl TryFrom<String> for Max350Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 350usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 350",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max350Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max350Text> for String {
    fn from(v: Max350Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 35
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max35Text(pub String);
impl TryFrom<String> for Max35Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 35usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 35",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max35Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max35Text> for String {
    fn from(v: Max35Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max4Text(pub String);
impl TryFrom<String> for Max4Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 4usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 4",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max4Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max4Text> for String {
    fn from(v: Max4Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 70
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max70Text(pub String);
impl TryFrom<String> for Max70Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let len = value.chars().count();
                    len < 1usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})",
                            "value is shorter than minimum length 1",
                            value.chars().count()
                        ),
                    });
                }
            }
            {
                let violated = {
                    let len = value.chars().count();
                    len > 70usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum length 70",
                            value.chars().count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max70Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max70Text> for String {
    fn from(v: Max70Text) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for Number {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let frac_count = value.find('.').map_or(0, |dot| {
                        value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count()
                    });
                    frac_count > 0usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum fraction digits 0",
                            value.find('.').map_or(0, |dot| value[dot + 1..]
                                .chars()
                                .filter(char::is_ascii_digit)
                                .count())
                        ),
                    });
                }
            }
            {
                let violated = {
                    let digit_count = value.chars().filter(char::is_ascii_digit).count();
                    digit_count > 18usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum total digits 18",
                            value.chars().filter(char::is_ascii_digit).count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Number {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Number> for String {
    fn from(v: Number) -> Self {
        v.0
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PaymentMethod3Code {
    #[serde(rename = "CHK")]
    Chk,
    #[serde(rename = "TRF")]
    Trf,
    #[serde(rename = "TRA")]
    Tra,
}
/// Fraction digits: 10
/// Total digits: 11
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PercentageRate(pub String);
impl TryFrom<String> for PercentageRate {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let frac_count = value.find('.').map_or(0, |dot| {
                        value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count()
                    });
                    frac_count > 10usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum fraction digits 10",
                            value.find('.').map_or(0, |dot| value[dot + 1..]
                                .chars()
                                .filter(char::is_ascii_digit)
                                .count())
                        ),
                    });
                }
            }
            {
                let violated = {
                    let digit_count = value.chars().filter(char::is_ascii_digit).count();
                    digit_count > 11usize
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})",
                            "value exceeds maximum total digits 11",
                            value.chars().filter(char::is_ascii_digit).count()
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl PercentageRate {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<PercentageRate> for String {
    fn from(v: PercentageRate) -> Self {
        v.0
    }
}
/// Pattern: `\+[0-9]{1,3}-[0-9()+\-]{1,30}`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PhoneNumber(pub String);
impl TryFrom<String> for PhoneNumber {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    let len = bytes.len();
                    let result: bool = (|| -> bool {
                        let mut pos: usize = 0;
                        if !(4usize..=35usize).contains(&len) {
                            return true;
                        }
                        if pos >= len || bytes[pos] != 43u8 {
                            return true;
                        }
                        pos += 1;
                        {
                            let start = pos;
                            let limit = if pos + 3usize < len {
                                pos + 3usize
                            } else {
                                len
                            };
                            while pos < limit {
                                let b = bytes[pos];
                                if !(48u8..=57u8).contains(&b) {
                                    break;
                                }
                                pos += 1;
                            }
                            let matched = pos - start;
                            if matched < 1usize {
                                return true;
                            }
                        }
                        if pos >= len || bytes[pos] != 45u8 {
                            return true;
                        }
                        pos += 1;
                        {
                            let start = pos;
                            let limit = if pos + 30usize < len {
                                pos + 30usize
                            } else {
                                len
                            };
                            while pos < limit {
                                let b = bytes[pos];
                                if !(48u8..=57u8).contains(&b)
                                    && b != 40u8
                                    && b != 41u8
                                    && b != 43u8
                                    && b != 45u8
                                {
                                    break;
                                }
                                pos += 1;
                            }
                            let matched = pos - start;
                            if matched < 1usize {
                                return true;
                            }
                        }
                        if pos != len {
                            return true;
                        }
                        false
                    })();
                    result
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern \\+[0-9]{1,3}-[0-9()+\\-]{1,30}"
                            .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl PhoneNumber {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<PhoneNumber> for String {
    fn from(v: PhoneNumber) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for UUIDv4Identifier {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let violated = {
                    let bytes = value.as_bytes();
                    bytes.len() != 36usize
                        || ({
                            let b = bytes[0usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[2usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[3usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[4usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[5usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[6usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[7usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || bytes[8usize] != 45u8
                        || ({
                            let b = bytes[9usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[10usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[11usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[12usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || bytes[13usize] != 45u8
                        || bytes[14usize] != 52u8
                        || ({
                            let b = bytes[15usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[16usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[17usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || bytes[18usize] != 45u8
                        || ({
                            let b = bytes[19usize];
                            b != 56u8 && b != 57u8 && b != 97u8 && b != 98u8
                        })
                        || ({
                            let b = bytes[20usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[21usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[22usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || bytes[23usize] != 45u8
                        || ({
                            let b = bytes[24usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[25usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[26usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[27usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[28usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[29usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[30usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[31usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[32usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[33usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[34usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[35usize];
                            !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"
                            .to_string(),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl UUIDv4Identifier {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(value: impl Into<String>) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<UUIDv4Identifier> for String {
    fn from(v: UUIDv4Identifier) -> Self {
        v.0
    }
}
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
pub struct AdviceType1 {
    #[serde(rename = "CdtAdvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdt_advc: Option<crate::common::ChoiceWrapper<AdviceType1Choice>>,
    #[serde(rename = "DbtAdvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbt_advc: Option<crate::common::ChoiceWrapper<AdviceType1Choice>>,
}
/// Builder for [`AdviceType1`]. Construct via [`AdviceType1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct AdviceType1Builder {
    cdt_advc: ::std::option::Option<crate::common::ChoiceWrapper<AdviceType1Choice>>,
    dbt_advc: ::std::option::Option<crate::common::ChoiceWrapper<AdviceType1Choice>>,
}
impl AdviceType1Builder {
    /// Set the `cdt_advc` field.
    #[must_use]
    pub fn cdt_advc(
        mut self,
        value: crate::common::ChoiceWrapper<AdviceType1Choice>,
    ) -> AdviceType1Builder {
        self.cdt_advc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbt_advc` field.
    #[must_use]
    pub fn dbt_advc(
        mut self,
        value: crate::common::ChoiceWrapper<AdviceType1Choice>,
    ) -> AdviceType1Builder {
        self.dbt_advc = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<AdviceType1, crate::common::BuilderError> {
        ::std::result::Result::Ok(AdviceType1 {
            cdt_advc: self.cdt_advc,
            dbt_advc: self.dbt_advc,
        })
    }
}
impl AdviceType1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> AdviceType1Builder {
        AdviceType1Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AdviceType1Choice {
    #[serde(rename = "Cd")]
    Cd(AdviceType1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
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
pub struct Cheque11 {
    #[serde(rename = "ChqTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_tp: Option<ChequeType2Code>,
    #[serde(rename = "ChqNb")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_nb: Option<Max35Text>,
    #[serde(rename = "ChqFr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_fr: Option<NameAndAddress16>,
    #[serde(rename = "DlvryMtd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlvry_mtd: Option<crate::common::ChoiceWrapper<ChequeDeliveryMethod1Choice>>,
    #[serde(rename = "DlvrTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dlvr_to: Option<NameAndAddress16>,
    #[serde(rename = "InstrPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
    #[serde(rename = "ChqMtrtyDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_mtrty_dt: Option<ISODate>,
    #[serde(rename = "FrmsCd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frms_cd: Option<Max35Text>,
    #[serde(rename = "MemoFld")]
    /// Maximum 2 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub memo_fld: Vec<Max35Text>,
    #[serde(rename = "RgnlClrZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rgnl_clr_zone: Option<Max35Text>,
    #[serde(rename = "PrtLctn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prt_lctn: Option<Max35Text>,
    #[serde(rename = "Sgntr")]
    /// Maximum 5 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sgntr: Vec<Max70Text>,
}
/// Builder for [`Cheque11`]. Construct via [`Cheque11::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct Cheque11Builder {
    chq_tp: ::std::option::Option<ChequeType2Code>,
    chq_nb: ::std::option::Option<Max35Text>,
    chq_fr: ::std::option::Option<NameAndAddress16>,
    dlvry_mtd: ::std::option::Option<crate::common::ChoiceWrapper<ChequeDeliveryMethod1Choice>>,
    dlvr_to: ::std::option::Option<NameAndAddress16>,
    instr_prty: ::std::option::Option<Priority2Code>,
    chq_mtrty_dt: ::std::option::Option<ISODate>,
    frms_cd: ::std::option::Option<Max35Text>,
    memo_fld: ::std::vec::Vec<Max35Text>,
    rgnl_clr_zone: ::std::option::Option<Max35Text>,
    prt_lctn: ::std::option::Option<Max35Text>,
    sgntr: ::std::vec::Vec<Max70Text>,
}
impl Cheque11Builder {
    /// Set the `chq_tp` field.
    #[must_use]
    pub fn chq_tp(mut self, value: ChequeType2Code) -> Cheque11Builder {
        self.chq_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chq_nb` field.
    #[must_use]
    pub fn chq_nb(mut self, value: Max35Text) -> Cheque11Builder {
        self.chq_nb = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chq_fr` field.
    #[must_use]
    pub fn chq_fr(mut self, value: NameAndAddress16) -> Cheque11Builder {
        self.chq_fr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dlvry_mtd` field.
    #[must_use]
    pub fn dlvry_mtd(
        mut self,
        value: crate::common::ChoiceWrapper<ChequeDeliveryMethod1Choice>,
    ) -> Cheque11Builder {
        self.dlvry_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dlvr_to` field.
    #[must_use]
    pub fn dlvr_to(mut self, value: NameAndAddress16) -> Cheque11Builder {
        self.dlvr_to = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_prty` field.
    #[must_use]
    pub fn instr_prty(mut self, value: Priority2Code) -> Cheque11Builder {
        self.instr_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chq_mtrty_dt` field.
    #[must_use]
    pub fn chq_mtrty_dt(mut self, value: ISODate) -> Cheque11Builder {
        self.chq_mtrty_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frms_cd` field.
    #[must_use]
    pub fn frms_cd(mut self, value: Max35Text) -> Cheque11Builder {
        self.frms_cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `memo_fld` field (replaces any previously added items).
    #[must_use]
    pub fn memo_fld(mut self, value: ::std::vec::Vec<Max35Text>) -> Cheque11Builder {
        self.memo_fld = value;
        self
    }
    /// Append one item to the `memo_fld` field.
    #[must_use]
    pub fn add_memo_fld(mut self, value: Max35Text) -> Cheque11Builder {
        self.memo_fld.push(value);
        self
    }
    /// Set the `rgnl_clr_zone` field.
    #[must_use]
    pub fn rgnl_clr_zone(mut self, value: Max35Text) -> Cheque11Builder {
        self.rgnl_clr_zone = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prt_lctn` field.
    #[must_use]
    pub fn prt_lctn(mut self, value: Max35Text) -> Cheque11Builder {
        self.prt_lctn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sgntr` field (replaces any previously added items).
    #[must_use]
    pub fn sgntr(mut self, value: ::std::vec::Vec<Max70Text>) -> Cheque11Builder {
        self.sgntr = value;
        self
    }
    /// Append one item to the `sgntr` field.
    #[must_use]
    pub fn add_sgntr(mut self, value: Max70Text) -> Cheque11Builder {
        self.sgntr.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<Cheque11, crate::common::BuilderError> {
        ::std::result::Result::Ok(Cheque11 {
            chq_tp: self.chq_tp,
            chq_nb: self.chq_nb,
            chq_fr: self.chq_fr,
            dlvry_mtd: self.dlvry_mtd,
            dlvr_to: self.dlvr_to,
            instr_prty: self.instr_prty,
            chq_mtrty_dt: self.chq_mtrty_dt,
            frms_cd: self.frms_cd,
            memo_fld: self.memo_fld,
            rgnl_clr_zone: self.rgnl_clr_zone,
            prt_lctn: self.prt_lctn,
            sgntr: self.sgntr,
        })
    }
}
impl Cheque11 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> Cheque11Builder {
        Cheque11Builder::default()
    }
}
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ChequeDeliveryMethod1Choice {
    #[serde(rename = "Cd")]
    Cd(ChequeDelivery1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
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
pub struct CreditTransferTransaction54 {
    #[serde(rename = "PmtId")]
    pub pmt_id: PaymentIdentification6,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation26>,
    #[serde(rename = "Amt")]
    pub amt: crate::common::ChoiceWrapper<AmountType4Choice>,
    #[serde(rename = "XchgRateInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate_inf: Option<ExchangeRate1>,
    #[serde(rename = "ChrgBr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code>,
    #[serde(rename = "MndtRltdInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mndt_rltd_inf: Option<CreditTransferMandateData1>,
    #[serde(rename = "ChqInstr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chq_instr: Option<Cheque11>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification135>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdtr: Option<PartyIdentification135>,
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
    #[serde(rename = "InstrForDbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt: Option<InstructionForDebtorAgent1>,
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
    pub tax: Option<TaxInformation10>,
    #[serde(rename = "RltdRmtInf")]
    /// Maximum 10 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rltd_rmt_inf: Vec<RemittanceLocation7>,
    #[serde(rename = "RmtInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rmt_inf: Option<RemittanceInformation21>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`CreditTransferTransaction54`]. Construct via [`CreditTransferTransaction54::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CreditTransferTransaction54Builder {
    pmt_id: ::std::option::Option<PaymentIdentification6>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation26>,
    amt: ::std::option::Option<crate::common::ChoiceWrapper<AmountType4Choice>>,
    xchg_rate_inf: ::std::option::Option<ExchangeRate1>,
    chrg_br: ::std::option::Option<ChargeBearerType1Code>,
    mndt_rltd_inf: ::std::option::Option<CreditTransferMandateData1>,
    chq_instr: ::std::option::Option<Cheque11>,
    ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
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
    instr_for_dbtr_agt: ::std::option::Option<InstructionForDebtorAgent1>,
    purp: ::std::option::Option<crate::common::ChoiceWrapper<Purpose2Choice>>,
    rgltry_rptg: ::std::vec::Vec<RegulatoryReporting3>,
    tax: ::std::option::Option<TaxInformation10>,
    rltd_rmt_inf: ::std::vec::Vec<RemittanceLocation7>,
    rmt_inf: ::std::option::Option<RemittanceInformation21>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl CreditTransferTransaction54Builder {
    /// Set the `pmt_id` field.
    #[must_use]
    pub fn pmt_id(mut self, value: PaymentIdentification6) -> CreditTransferTransaction54Builder {
        self.pmt_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(
        mut self,
        value: PaymentTypeInformation26,
    ) -> CreditTransferTransaction54Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: crate::common::ChoiceWrapper<AmountType4Choice>,
    ) -> CreditTransferTransaction54Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate_inf` field.
    #[must_use]
    pub fn xchg_rate_inf(mut self, value: ExchangeRate1) -> CreditTransferTransaction54Builder {
        self.xchg_rate_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrg_br` field.
    #[must_use]
    pub fn chrg_br(mut self, value: ChargeBearerType1Code) -> CreditTransferTransaction54Builder {
        self.chrg_br = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mndt_rltd_inf` field.
    #[must_use]
    pub fn mndt_rltd_inf(
        mut self,
        value: CreditTransferMandateData1,
    ) -> CreditTransferTransaction54Builder {
        self.mndt_rltd_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chq_instr` field.
    #[must_use]
    pub fn chq_instr(mut self, value: Cheque11) -> CreditTransferTransaction54Builder {
        self.chq_instr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: PartyIdentification135,
    ) -> CreditTransferTransaction54Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1` field.
    #[must_use]
    pub fn intrmy_agt1(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction54Builder {
        self.intrmy_agt1 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt1acct` field.
    #[must_use]
    pub fn intrmy_agt1acct(mut self, value: CashAccount40) -> CreditTransferTransaction54Builder {
        self.intrmy_agt1acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2` field.
    #[must_use]
    pub fn intrmy_agt2(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction54Builder {
        self.intrmy_agt2 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt2acct` field.
    #[must_use]
    pub fn intrmy_agt2acct(mut self, value: CashAccount40) -> CreditTransferTransaction54Builder {
        self.intrmy_agt2acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3` field.
    #[must_use]
    pub fn intrmy_agt3(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction54Builder {
        self.intrmy_agt3 = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intrmy_agt3acct` field.
    #[must_use]
    pub fn intrmy_agt3acct(mut self, value: CashAccount40) -> CreditTransferTransaction54Builder {
        self.intrmy_agt3acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> CreditTransferTransaction54Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(mut self, value: CashAccount40) -> CreditTransferTransaction54Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(mut self, value: PartyIdentification135) -> CreditTransferTransaction54Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(mut self, value: CashAccount40) -> CreditTransferTransaction54Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: PartyIdentification135,
    ) -> CreditTransferTransaction54Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_for_cdtr_agt` field (replaces any previously added items).
    #[must_use]
    pub fn instr_for_cdtr_agt(
        mut self,
        value: ::std::vec::Vec<InstructionForCreditorAgent3>,
    ) -> CreditTransferTransaction54Builder {
        self.instr_for_cdtr_agt = value;
        self
    }
    /// Append one item to the `instr_for_cdtr_agt` field.
    #[must_use]
    pub fn add_instr_for_cdtr_agt(
        mut self,
        value: InstructionForCreditorAgent3,
    ) -> CreditTransferTransaction54Builder {
        self.instr_for_cdtr_agt.push(value);
        self
    }
    /// Set the `instr_for_dbtr_agt` field.
    #[must_use]
    pub fn instr_for_dbtr_agt(
        mut self,
        value: InstructionForDebtorAgent1,
    ) -> CreditTransferTransaction54Builder {
        self.instr_for_dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `purp` field.
    #[must_use]
    pub fn purp(
        mut self,
        value: crate::common::ChoiceWrapper<Purpose2Choice>,
    ) -> CreditTransferTransaction54Builder {
        self.purp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rgltry_rptg` field (replaces any previously added items).
    #[must_use]
    pub fn rgltry_rptg(
        mut self,
        value: ::std::vec::Vec<RegulatoryReporting3>,
    ) -> CreditTransferTransaction54Builder {
        self.rgltry_rptg = value;
        self
    }
    /// Append one item to the `rgltry_rptg` field.
    #[must_use]
    pub fn add_rgltry_rptg(
        mut self,
        value: RegulatoryReporting3,
    ) -> CreditTransferTransaction54Builder {
        self.rgltry_rptg.push(value);
        self
    }
    /// Set the `tax` field.
    #[must_use]
    pub fn tax(mut self, value: TaxInformation10) -> CreditTransferTransaction54Builder {
        self.tax = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd_rmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn rltd_rmt_inf(
        mut self,
        value: ::std::vec::Vec<RemittanceLocation7>,
    ) -> CreditTransferTransaction54Builder {
        self.rltd_rmt_inf = value;
        self
    }
    /// Append one item to the `rltd_rmt_inf` field.
    #[must_use]
    pub fn add_rltd_rmt_inf(
        mut self,
        value: RemittanceLocation7,
    ) -> CreditTransferTransaction54Builder {
        self.rltd_rmt_inf.push(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(mut self, value: RemittanceInformation21) -> CreditTransferTransaction54Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> CreditTransferTransaction54Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> CreditTransferTransaction54Builder {
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
    ) -> ::std::result::Result<CreditTransferTransaction54, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pmt_id.is_none() {
            missing.push("pmt_id".to_owned());
        }
        if self.amt.is_none() {
            missing.push("amt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CreditTransferTransaction54".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CreditTransferTransaction54 {
            pmt_id: self.pmt_id.unwrap(),
            pmt_tp_inf: self.pmt_tp_inf,
            amt: self.amt.unwrap(),
            xchg_rate_inf: self.xchg_rate_inf,
            chrg_br: self.chrg_br,
            mndt_rltd_inf: self.mndt_rltd_inf,
            chq_instr: self.chq_instr,
            ultmt_dbtr: self.ultmt_dbtr,
            intrmy_agt1: self.intrmy_agt1,
            intrmy_agt1acct: self.intrmy_agt1acct,
            intrmy_agt2: self.intrmy_agt2,
            intrmy_agt2acct: self.intrmy_agt2acct,
            intrmy_agt3: self.intrmy_agt3,
            intrmy_agt3acct: self.intrmy_agt3acct,
            cdtr_agt: self.cdtr_agt,
            cdtr_agt_acct: self.cdtr_agt_acct,
            cdtr: self.cdtr,
            cdtr_acct: self.cdtr_acct,
            ultmt_cdtr: self.ultmt_cdtr,
            instr_for_cdtr_agt: self.instr_for_cdtr_agt,
            instr_for_dbtr_agt: self.instr_for_dbtr_agt,
            purp: self.purp,
            rgltry_rptg: self.rgltry_rptg,
            tax: self.tax,
            rltd_rmt_inf: self.rltd_rmt_inf,
            rmt_inf: self.rmt_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl CreditTransferTransaction54 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CreditTransferTransaction54Builder {
        CreditTransferTransaction54Builder::default()
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
pub struct CustomerCreditTransferInitiationV11 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader95,
    #[serde(rename = "PmtInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub pmt_inf: Vec<PaymentInstruction40>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`CustomerCreditTransferInitiationV11`]. Construct via [`CustomerCreditTransferInitiationV11::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct CustomerCreditTransferInitiationV11Builder {
    grp_hdr: ::std::option::Option<GroupHeader95>,
    pmt_inf: ::std::vec::Vec<PaymentInstruction40>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl CustomerCreditTransferInitiationV11Builder {
    /// Set the `grp_hdr` field.
    #[must_use]
    pub fn grp_hdr(mut self, value: GroupHeader95) -> CustomerCreditTransferInitiationV11Builder {
        self.grp_hdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_inf` field (replaces any previously added items).
    #[must_use]
    pub fn pmt_inf(
        mut self,
        value: ::std::vec::Vec<PaymentInstruction40>,
    ) -> CustomerCreditTransferInitiationV11Builder {
        self.pmt_inf = value;
        self
    }
    /// Append one item to the `pmt_inf` field.
    #[must_use]
    pub fn add_pmt_inf(
        mut self,
        value: PaymentInstruction40,
    ) -> CustomerCreditTransferInitiationV11Builder {
        self.pmt_inf.push(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> CustomerCreditTransferInitiationV11Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> CustomerCreditTransferInitiationV11Builder {
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
    ) -> ::std::result::Result<CustomerCreditTransferInitiationV11, crate::common::BuilderError>
    {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.grp_hdr.is_none() {
            missing.push("grp_hdr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "CustomerCreditTransferInitiationV11".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(CustomerCreditTransferInitiationV11 {
            grp_hdr: self.grp_hdr.unwrap(),
            pmt_inf: self.pmt_inf,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl CustomerCreditTransferInitiationV11 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> CustomerCreditTransferInitiationV11Builder {
        CustomerCreditTransferInitiationV11Builder::default()
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
    #[serde(rename = "CstmrCdtTrfInitn")]
    pub cstmr_cdt_trf_initn: CustomerCreditTransferInitiationV11,
}
/// Builder for [`Document`]. Construct via [`Document::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentBuilder {
    cstmr_cdt_trf_initn: ::std::option::Option<CustomerCreditTransferInitiationV11>,
}
impl DocumentBuilder {
    /// Set the `cstmr_cdt_trf_initn` field.
    #[must_use]
    pub fn cstmr_cdt_trf_initn(
        mut self,
        value: CustomerCreditTransferInitiationV11,
    ) -> DocumentBuilder {
        self.cstmr_cdt_trf_initn = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
        if self.cstmr_cdt_trf_initn.is_none() {
            missing.push("cstmr_cdt_trf_initn".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Document".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Document {
            cstmr_cdt_trf_initn: self.cstmr_cdt_trf_initn.unwrap(),
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeRate1 {
    #[serde(rename = "UnitCcy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_ccy: Option<ActiveOrHistoricCurrencyCode>,
    #[serde(rename = "XchgRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xchg_rate: Option<BaseOneRate>,
    #[serde(rename = "RateTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate_tp: Option<ExchangeRateType1Code>,
    #[serde(rename = "CtrctId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrct_id: Option<Max35Text>,
}
/// Builder for [`ExchangeRate1`]. Construct via [`ExchangeRate1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ExchangeRate1Builder {
    unit_ccy: ::std::option::Option<ActiveOrHistoricCurrencyCode>,
    xchg_rate: ::std::option::Option<BaseOneRate>,
    rate_tp: ::std::option::Option<ExchangeRateType1Code>,
    ctrct_id: ::std::option::Option<Max35Text>,
}
impl ExchangeRate1Builder {
    /// Set the `unit_ccy` field.
    #[must_use]
    pub fn unit_ccy(mut self, value: ActiveOrHistoricCurrencyCode) -> ExchangeRate1Builder {
        self.unit_ccy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `xchg_rate` field.
    #[must_use]
    pub fn xchg_rate(mut self, value: BaseOneRate) -> ExchangeRate1Builder {
        self.xchg_rate = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rate_tp` field.
    #[must_use]
    pub fn rate_tp(mut self, value: ExchangeRateType1Code) -> ExchangeRate1Builder {
        self.rate_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrct_id` field.
    #[must_use]
    pub fn ctrct_id(mut self, value: Max35Text) -> ExchangeRate1Builder {
        self.ctrct_id = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<ExchangeRate1, crate::common::BuilderError> {
        ::std::result::Result::Ok(ExchangeRate1 {
            unit_ccy: self.unit_ccy,
            xchg_rate: self.xchg_rate,
            rate_tp: self.rate_tp,
            ctrct_id: self.ctrct_id,
        })
    }
}
impl ExchangeRate1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ExchangeRate1Builder {
        ExchangeRate1Builder::default()
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
pub struct GroupHeader95 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: ISODateTime,
    #[serde(rename = "Authstn")]
    /// Maximum 2 occurrences.
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub authstn: Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    #[serde(rename = "NbOfTxs")]
    pub nb_of_txs: Max15NumericText,
    #[serde(rename = "CtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[serde(rename = "InitgPty")]
    pub initg_pty: PartyIdentification135,
    #[serde(rename = "FwdgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fwdg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InitnSrc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initn_src: Option<PaymentInitiationSource1>,
}
/// Builder for [`GroupHeader95`]. Construct via [`GroupHeader95::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GroupHeader95Builder {
    msg_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    authstn: ::std::vec::Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    nb_of_txs: ::std::option::Option<Max15NumericText>,
    ctrl_sum: ::std::option::Option<DecimalNumber>,
    initg_pty: ::std::option::Option<PartyIdentification135>,
    fwdg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    initn_src: ::std::option::Option<PaymentInitiationSource1>,
}
impl GroupHeader95Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> GroupHeader95Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> GroupHeader95Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `authstn` field (replaces any previously added items).
    #[must_use]
    pub fn authstn(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<Authorisation1Choice>>,
    ) -> GroupHeader95Builder {
        self.authstn = value;
        self
    }
    /// Append one item to the `authstn` field.
    #[must_use]
    pub fn add_authstn(
        mut self,
        value: crate::common::ChoiceWrapper<Authorisation1Choice>,
    ) -> GroupHeader95Builder {
        self.authstn.push(value);
        self
    }
    /// Set the `nb_of_txs` field.
    #[must_use]
    pub fn nb_of_txs(mut self, value: Max15NumericText) -> GroupHeader95Builder {
        self.nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrl_sum` field.
    #[must_use]
    pub fn ctrl_sum(mut self, value: DecimalNumber) -> GroupHeader95Builder {
        self.ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `initg_pty` field.
    #[must_use]
    pub fn initg_pty(mut self, value: PartyIdentification135) -> GroupHeader95Builder {
        self.initg_pty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fwdg_agt` field.
    #[must_use]
    pub fn fwdg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> GroupHeader95Builder {
        self.fwdg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `initn_src` field.
    #[must_use]
    pub fn initn_src(mut self, value: PaymentInitiationSource1) -> GroupHeader95Builder {
        self.initn_src = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<GroupHeader95, crate::common::BuilderError> {
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
        if self.initg_pty.is_none() {
            missing.push("initg_pty".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GroupHeader95".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GroupHeader95 {
            msg_id: self.msg_id.unwrap(),
            cre_dt_tm: self.cre_dt_tm.unwrap(),
            authstn: self.authstn,
            nb_of_txs: self.nb_of_txs.unwrap(),
            ctrl_sum: self.ctrl_sum,
            initg_pty: self.initg_pty.unwrap(),
            fwdg_agt: self.fwdg_agt,
            initn_src: self.initn_src,
        })
    }
}
impl GroupHeader95 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GroupHeader95Builder {
        GroupHeader95Builder::default()
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
pub struct InstructionForDebtorAgent1 {
    #[serde(rename = "Cd")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cd: Option<ExternalDebtorAgentInstruction1Code>,
    #[serde(rename = "InstrInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_inf: Option<Max140Text>,
}
/// Builder for [`InstructionForDebtorAgent1`]. Construct via [`InstructionForDebtorAgent1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct InstructionForDebtorAgent1Builder {
    cd: ::std::option::Option<ExternalDebtorAgentInstruction1Code>,
    instr_inf: ::std::option::Option<Max140Text>,
}
impl InstructionForDebtorAgent1Builder {
    /// Set the `cd` field.
    #[must_use]
    pub fn cd(
        mut self,
        value: ExternalDebtorAgentInstruction1Code,
    ) -> InstructionForDebtorAgent1Builder {
        self.cd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_inf` field.
    #[must_use]
    pub fn instr_inf(mut self, value: Max140Text) -> InstructionForDebtorAgent1Builder {
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
    ) -> ::std::result::Result<InstructionForDebtorAgent1, crate::common::BuilderError> {
        ::std::result::Result::Ok(InstructionForDebtorAgent1 {
            cd: self.cd,
            instr_inf: self.instr_inf,
        })
    }
}
impl InstructionForDebtorAgent1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> InstructionForDebtorAgent1Builder {
        InstructionForDebtorAgent1Builder::default()
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
pub struct PaymentIdentification6 {
    #[serde(rename = "InstrId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_id: Option<Max35Text>,
    #[serde(rename = "EndToEndId")]
    pub end_to_end_id: Max35Text,
    #[serde(rename = "UETR")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uetr: Option<UUIDv4Identifier>,
}
/// Builder for [`PaymentIdentification6`]. Construct via [`PaymentIdentification6::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentIdentification6Builder {
    instr_id: ::std::option::Option<Max35Text>,
    end_to_end_id: ::std::option::Option<Max35Text>,
    uetr: ::std::option::Option<UUIDv4Identifier>,
}
impl PaymentIdentification6Builder {
    /// Set the `instr_id` field.
    #[must_use]
    pub fn instr_id(mut self, value: Max35Text) -> PaymentIdentification6Builder {
        self.instr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `end_to_end_id` field.
    #[must_use]
    pub fn end_to_end_id(mut self, value: Max35Text) -> PaymentIdentification6Builder {
        self.end_to_end_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `uetr` field.
    #[must_use]
    pub fn uetr(mut self, value: UUIDv4Identifier) -> PaymentIdentification6Builder {
        self.uetr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
    ) -> ::std::result::Result<PaymentIdentification6, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.end_to_end_id.is_none() {
            missing.push("end_to_end_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentIdentification6".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentIdentification6 {
            instr_id: self.instr_id,
            end_to_end_id: self.end_to_end_id.unwrap(),
            uetr: self.uetr,
        })
    }
}
impl PaymentIdentification6 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentIdentification6Builder {
        PaymentIdentification6Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentInitiationSource1 {
    #[serde(rename = "Nm")]
    pub nm: Max140Text,
    #[serde(rename = "Prvdr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prvdr: Option<Max35Text>,
    #[serde(rename = "Vrsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vrsn: Option<Max35Text>,
}
/// Builder for [`PaymentInitiationSource1`]. Construct via [`PaymentInitiationSource1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentInitiationSource1Builder {
    nm: ::std::option::Option<Max140Text>,
    prvdr: ::std::option::Option<Max35Text>,
    vrsn: ::std::option::Option<Max35Text>,
}
impl PaymentInitiationSource1Builder {
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(mut self, value: Max140Text) -> PaymentInitiationSource1Builder {
        self.nm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prvdr` field.
    #[must_use]
    pub fn prvdr(mut self, value: Max35Text) -> PaymentInitiationSource1Builder {
        self.prvdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `vrsn` field.
    #[must_use]
    pub fn vrsn(mut self, value: Max35Text) -> PaymentInitiationSource1Builder {
        self.vrsn = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
    ) -> ::std::result::Result<PaymentInitiationSource1, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.nm.is_none() {
            missing.push("nm".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentInitiationSource1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentInitiationSource1 {
            nm: self.nm.unwrap(),
            prvdr: self.prvdr,
            vrsn: self.vrsn,
        })
    }
}
impl PaymentInitiationSource1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentInitiationSource1Builder {
        PaymentInitiationSource1Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentInstruction40 {
    #[serde(rename = "PmtInfId")]
    pub pmt_inf_id: Max35Text,
    #[serde(rename = "PmtMtd")]
    pub pmt_mtd: PaymentMethod3Code,
    #[serde(rename = "ReqdAdvcTp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reqd_advc_tp: Option<AdviceType1>,
    #[serde(rename = "BtchBookg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub btch_bookg: Option<BatchBookingIndicator>,
    #[serde(rename = "NbOfTxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nb_of_txs: Option<Max15NumericText>,
    #[serde(rename = "CtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ctrl_sum: Option<DecimalNumber>,
    #[serde(rename = "PmtTpInf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pmt_tp_inf: Option<PaymentTypeInformation26>,
    #[serde(rename = "ReqdExctnDt")]
    pub reqd_exctn_dt: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    #[serde(rename = "PoolgAdjstmntDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub poolg_adjstmnt_dt: Option<ISODate>,
    #[serde(rename = "Dbtr")]
    pub dbtr: PartyIdentification135,
    #[serde(rename = "DbtrAcct")]
    pub dbtr_acct: CashAccount40,
    #[serde(rename = "DbtrAgt")]
    pub dbtr_agt: BranchAndFinancialInstitutionIdentification6,
    #[serde(rename = "DbtrAgtAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dbtr_agt_acct: Option<CashAccount40>,
    #[serde(rename = "InstrForDbtrAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_for_dbtr_agt: Option<Max140Text>,
    #[serde(rename = "UltmtDbtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ultmt_dbtr: Option<PartyIdentification135>,
    #[serde(rename = "ChrgBr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrg_br: Option<ChargeBearerType1Code>,
    #[serde(rename = "ChrgsAcct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrgs_acct: Option<CashAccount40>,
    #[serde(rename = "ChrgsAcctAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chrgs_acct_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "CdtTrfTxInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub cdt_trf_tx_inf: Vec<CreditTransferTransaction54>,
}
/// Builder for [`PaymentInstruction40`]. Construct via [`PaymentInstruction40::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentInstruction40Builder {
    pmt_inf_id: ::std::option::Option<Max35Text>,
    pmt_mtd: ::std::option::Option<PaymentMethod3Code>,
    reqd_advc_tp: ::std::option::Option<AdviceType1>,
    btch_bookg: ::std::option::Option<BatchBookingIndicator>,
    nb_of_txs: ::std::option::Option<Max15NumericText>,
    ctrl_sum: ::std::option::Option<DecimalNumber>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation26>,
    reqd_exctn_dt: ::std::option::Option<crate::common::ChoiceWrapper<DateAndDateTime2Choice>>,
    poolg_adjstmnt_dt: ::std::option::Option<ISODate>,
    dbtr: ::std::option::Option<PartyIdentification135>,
    dbtr_acct: ::std::option::Option<CashAccount40>,
    dbtr_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    dbtr_agt_acct: ::std::option::Option<CashAccount40>,
    instr_for_dbtr_agt: ::std::option::Option<Max140Text>,
    ultmt_dbtr: ::std::option::Option<PartyIdentification135>,
    chrg_br: ::std::option::Option<ChargeBearerType1Code>,
    chrgs_acct: ::std::option::Option<CashAccount40>,
    chrgs_acct_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    cdt_trf_tx_inf: ::std::vec::Vec<CreditTransferTransaction54>,
}
impl PaymentInstruction40Builder {
    /// Set the `pmt_inf_id` field.
    #[must_use]
    pub fn pmt_inf_id(mut self, value: Max35Text) -> PaymentInstruction40Builder {
        self.pmt_inf_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_mtd` field.
    #[must_use]
    pub fn pmt_mtd(mut self, value: PaymentMethod3Code) -> PaymentInstruction40Builder {
        self.pmt_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_advc_tp` field.
    #[must_use]
    pub fn reqd_advc_tp(mut self, value: AdviceType1) -> PaymentInstruction40Builder {
        self.reqd_advc_tp = ::std::option::Option::Some(value);
        self
    }
    /// Set the `btch_bookg` field.
    #[must_use]
    pub fn btch_bookg(mut self, value: BatchBookingIndicator) -> PaymentInstruction40Builder {
        self.btch_bookg = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nb_of_txs` field.
    #[must_use]
    pub fn nb_of_txs(mut self, value: Max15NumericText) -> PaymentInstruction40Builder {
        self.nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctrl_sum` field.
    #[must_use]
    pub fn ctrl_sum(mut self, value: DecimalNumber) -> PaymentInstruction40Builder {
        self.ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(mut self, value: PaymentTypeInformation26) -> PaymentInstruction40Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_exctn_dt` field.
    #[must_use]
    pub fn reqd_exctn_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> PaymentInstruction40Builder {
        self.reqd_exctn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `poolg_adjstmnt_dt` field.
    #[must_use]
    pub fn poolg_adjstmnt_dt(mut self, value: ISODate) -> PaymentInstruction40Builder {
        self.poolg_adjstmnt_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(mut self, value: PartyIdentification135) -> PaymentInstruction40Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(mut self, value: CashAccount40) -> PaymentInstruction40Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentInstruction40Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(mut self, value: CashAccount40) -> PaymentInstruction40Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instr_for_dbtr_agt` field.
    #[must_use]
    pub fn instr_for_dbtr_agt(mut self, value: Max140Text) -> PaymentInstruction40Builder {
        self.instr_for_dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(mut self, value: PartyIdentification135) -> PaymentInstruction40Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrg_br` field.
    #[must_use]
    pub fn chrg_br(mut self, value: ChargeBearerType1Code) -> PaymentInstruction40Builder {
        self.chrg_br = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs_acct` field.
    #[must_use]
    pub fn chrgs_acct(mut self, value: CashAccount40) -> PaymentInstruction40Builder {
        self.chrgs_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `chrgs_acct_agt` field.
    #[must_use]
    pub fn chrgs_acct_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentInstruction40Builder {
        self.chrgs_acct_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdt_trf_tx_inf` field (replaces any previously added items).
    #[must_use]
    pub fn cdt_trf_tx_inf(
        mut self,
        value: ::std::vec::Vec<CreditTransferTransaction54>,
    ) -> PaymentInstruction40Builder {
        self.cdt_trf_tx_inf = value;
        self
    }
    /// Append one item to the `cdt_trf_tx_inf` field.
    #[must_use]
    pub fn add_cdt_trf_tx_inf(
        mut self,
        value: CreditTransferTransaction54,
    ) -> PaymentInstruction40Builder {
        self.cdt_trf_tx_inf.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
    /// # Errors
    ///
    /// Returns [`crate::common::BuilderError`] listing the names of any
    /// required fields that were not set.
    ///
    /// # Panics
    ///
    /// Does not panic — all `.unwrap()` calls are guarded by the
    /// missing-field check above.
    pub fn build(self) -> ::std::result::Result<PaymentInstruction40, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.pmt_inf_id.is_none() {
            missing.push("pmt_inf_id".to_owned());
        }
        if self.pmt_mtd.is_none() {
            missing.push("pmt_mtd".to_owned());
        }
        if self.reqd_exctn_dt.is_none() {
            missing.push("reqd_exctn_dt".to_owned());
        }
        if self.dbtr.is_none() {
            missing.push("dbtr".to_owned());
        }
        if self.dbtr_acct.is_none() {
            missing.push("dbtr_acct".to_owned());
        }
        if self.dbtr_agt.is_none() {
            missing.push("dbtr_agt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "PaymentInstruction40".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(PaymentInstruction40 {
            pmt_inf_id: self.pmt_inf_id.unwrap(),
            pmt_mtd: self.pmt_mtd.unwrap(),
            reqd_advc_tp: self.reqd_advc_tp,
            btch_bookg: self.btch_bookg,
            nb_of_txs: self.nb_of_txs,
            ctrl_sum: self.ctrl_sum,
            pmt_tp_inf: self.pmt_tp_inf,
            reqd_exctn_dt: self.reqd_exctn_dt.unwrap(),
            poolg_adjstmnt_dt: self.poolg_adjstmnt_dt,
            dbtr: self.dbtr.unwrap(),
            dbtr_acct: self.dbtr_acct.unwrap(),
            dbtr_agt: self.dbtr_agt.unwrap(),
            dbtr_agt_acct: self.dbtr_agt_acct,
            instr_for_dbtr_agt: self.instr_for_dbtr_agt,
            ultmt_dbtr: self.ultmt_dbtr,
            chrg_br: self.chrg_br,
            chrgs_acct: self.chrgs_acct,
            chrgs_acct_agt: self.chrgs_acct_agt,
            cdt_trf_tx_inf: self.cdt_trf_tx_inf,
        })
    }
}
impl PaymentInstruction40 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentInstruction40Builder {
        PaymentInstruction40Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PaymentTypeInformation26 {
    #[serde(rename = "InstrPrty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instr_prty: Option<Priority2Code>,
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
/// Builder for [`PaymentTypeInformation26`]. Construct via [`PaymentTypeInformation26::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentTypeInformation26Builder {
    instr_prty: ::std::option::Option<Priority2Code>,
    svc_lvl: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    lcl_instrm: ::std::option::Option<crate::common::ChoiceWrapper<LocalInstrument2Choice>>,
    ctgy_purp: ::std::option::Option<crate::common::ChoiceWrapper<CategoryPurpose1Choice>>,
}
impl PaymentTypeInformation26Builder {
    /// Set the `instr_prty` field.
    #[must_use]
    pub fn instr_prty(mut self, value: Priority2Code) -> PaymentTypeInformation26Builder {
        self.instr_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `svc_lvl` field (replaces any previously added items).
    #[must_use]
    pub fn svc_lvl(
        mut self,
        value: ::std::vec::Vec<crate::common::ChoiceWrapper<ServiceLevel8Choice>>,
    ) -> PaymentTypeInformation26Builder {
        self.svc_lvl = value;
        self
    }
    /// Append one item to the `svc_lvl` field.
    #[must_use]
    pub fn add_svc_lvl(
        mut self,
        value: crate::common::ChoiceWrapper<ServiceLevel8Choice>,
    ) -> PaymentTypeInformation26Builder {
        self.svc_lvl.push(value);
        self
    }
    /// Set the `lcl_instrm` field.
    #[must_use]
    pub fn lcl_instrm(
        mut self,
        value: crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    ) -> PaymentTypeInformation26Builder {
        self.lcl_instrm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ctgy_purp` field.
    #[must_use]
    pub fn ctgy_purp(
        mut self,
        value: crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    ) -> PaymentTypeInformation26Builder {
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
    ) -> ::std::result::Result<PaymentTypeInformation26, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentTypeInformation26 {
            instr_prty: self.instr_prty,
            svc_lvl: self.svc_lvl,
            lcl_instrm: self.lcl_instrm,
            ctgy_purp: self.ctgy_purp,
        })
    }
}
impl PaymentTypeInformation26 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentTypeInformation26Builder {
        PaymentTypeInformation26Builder::default()
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum ServiceLevel8Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalServiceLevel1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
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
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let frac_count = value.find('.').map_or(0, |dot| {
                    value[dot + 1..]
                        .chars()
                        .filter(char::is_ascii_digit)
                        .count()
                });
                frac_count > 5usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 5",
                        value.find('.').map_or(0, |dot| value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count())
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                digit_count > 18usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18",
                        value.chars().filter(char::is_ascii_digit).count()
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ActiveOrHistoricCurrencyCode {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 3usize
                    || ({
                        let b = bytes[0usize];
                        !(65u8..=90u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(65u8..=90u8).contains(&b)
                    })
                    || ({
                        let b = bytes[2usize];
                        !(65u8..=90u8).contains(&b)
                    })
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [A-Z]{3,3}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
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
impl crate::common::validate::Validatable for AdviceType1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for AnyBICDec2014Identifier {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                let len = bytes.len();
                let result: bool = (|| -> bool {
                    let mut pos: usize = 0;
                    if !(8usize..=11usize).contains(&len) {
                        return true;
                    }
                    {
                        let end = pos + 4usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let saved = pos;
                        let matched: bool = (|| -> bool {
                            {
                                let end = pos + 3usize;
                                if end > len {
                                    return true;
                                }
                                for &b in &bytes[pos..end] {
                                    if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                        return true;
                                    }
                                }
                                pos = end;
                            }
                            false
                        })();
                        if matched {
                            pos = saved;
                        }
                    }
                    if pos != len {
                        return true;
                    }
                    false
                })();
                result
            };
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: "value does not match pattern [A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"
                            .to_string(),
                        kind: crate::common::validate::ConstraintKind::Pattern,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for Authorisation1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for BICFIDec2014Identifier {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                let len = bytes.len();
                let result: bool = (|| -> bool {
                    let mut pos: usize = 0;
                    if !(8usize..=11usize).contains(&len) {
                        return true;
                    }
                    {
                        let end = pos + 4usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let saved = pos;
                        let matched: bool = (|| -> bool {
                            {
                                let end = pos + 3usize;
                                if end > len {
                                    return true;
                                }
                                for &b in &bytes[pos..end] {
                                    if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b) {
                                        return true;
                                    }
                                }
                                pos = end;
                            }
                            false
                        })();
                        if matched {
                            pos = saved;
                        }
                    }
                    if pos != len {
                        return true;
                    }
                    false
                })();
                result
            };
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: "value does not match pattern [A-Z0-9]{4,4}[A-Z]{2,2}[A-Z0-9]{2,2}([A-Z0-9]{3,3}){0,1}"
                            .to_string(),
                        kind: crate::common::validate::ConstraintKind::Pattern,
                    });
            }
        }
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
            let value: &str = &self.0;
            let violated = {
                let frac_count = value.find('.').map_or(0, |dot| {
                    value[dot + 1..]
                        .chars()
                        .filter(char::is_ascii_digit)
                        .count()
                });
                frac_count > 10usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 10",
                        value.find('.').map_or(0, |dot| value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count())
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                digit_count > 11usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 11",
                        value.chars().filter(char::is_ascii_digit).count()
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
impl crate::common::validate::Validatable for ChequeDelivery1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for ChequeType2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
    }
}
impl crate::common::validate::Validatable for CountryCode {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 2usize
                    || ({
                        let b = bytes[0usize];
                        !(65u8..=90u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(65u8..=90u8).contains(&b)
                    })
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [A-Z]{2,2}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
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
            let value: &str = &self.0;
            let violated = {
                let frac_count = value.find('.').map_or(0, |dot| {
                    value[dot + 1..]
                        .chars()
                        .filter(char::is_ascii_digit)
                        .count()
                });
                frac_count > 17usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 17",
                        value.find('.').map_or(0, |dot| value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count())
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                digit_count > 18usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18",
                        value.chars().filter(char::is_ascii_digit).count()
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
impl crate::common::validate::Validatable for Exact2NumericText {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 2usize
                    || ({
                        let b = bytes[0usize];
                        !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(48u8..=57u8).contains(&b)
                    })
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [0-9]{2}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Exact4AlphaNumericText {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 4usize
                    || ({
                        let b = bytes[0usize];
                        !(97u8..=122u8).contains(&b)
                            && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(97u8..=122u8).contains(&b)
                            && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[2usize];
                        !(97u8..=122u8).contains(&b)
                            && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[3usize];
                        !(97u8..=122u8).contains(&b)
                            && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [a-zA-Z0-9]{4}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExchangeRateType1Code {
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 5usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 5",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalDebtorAgentInstruction1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 35usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 35",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
impl crate::common::validate::Validatable for IBAN2007Identifier {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                let len = bytes.len();
                let result: bool = (|| -> bool {
                    let mut pos: usize = 0;
                    if !(5usize..=34usize).contains(&len) {
                        return true;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(65u8..=90u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let end = pos + 2usize;
                        if end > len {
                            return true;
                        }
                        for &b in &bytes[pos..end] {
                            if !(48u8..=57u8).contains(&b) {
                                return true;
                            }
                        }
                        pos = end;
                    }
                    {
                        let start = pos;
                        let limit = if pos + 30usize < len {
                            pos + 30usize
                        } else {
                            len
                        };
                        while pos < limit {
                            let b = bytes[pos];
                            if !(97u8..=122u8).contains(&b)
                                && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                            {
                                break;
                            }
                            pos += 1;
                        }
                        let matched = pos - start;
                        if matched < 1usize {
                            return true;
                        }
                    }
                    if pos != len {
                        return true;
                    }
                    false
                })();
                result
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"
                        .to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
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
impl crate::common::validate::Validatable for LEIIdentifier {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 20usize
                    || ({
                        let b = bytes[0usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[2usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[3usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[4usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[5usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[6usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[7usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[8usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[9usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[10usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[11usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[12usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[13usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[14usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[15usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[16usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[17usize];
                        !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[18usize];
                        !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[19usize];
                        !(48u8..=57u8).contains(&b)
                    })
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [A-Z0-9]{18,18}[0-9]{2,2}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 10240usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 10240",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 10usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 10",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 128usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 128",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 140usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 140",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MaxLength,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max15NumericText {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                let len = bytes.len();
                let result: bool = (|| -> bool {
                    let mut pos: usize = 0;
                    if !(1usize..=15usize).contains(&len) {
                        return true;
                    }
                    {
                        let start = pos;
                        let limit = if pos + 15usize < len {
                            pos + 15usize
                        } else {
                            len
                        };
                        while pos < limit {
                            let b = bytes[pos];
                            if !(48u8..=57u8).contains(&b) {
                                break;
                            }
                            pos += 1;
                        }
                        let matched = pos - start;
                        if matched < 1usize {
                            return true;
                        }
                    }
                    if pos != len {
                        return true;
                    }
                    false
                })();
                result
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern [0-9]{1,15}".to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
                });
            }
        }
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 16usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 16",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 2048usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 2048",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 34usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 34",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 350usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 350",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 35usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 35",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 4usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 4",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len < 1usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value is shorter than minimum length 1",
                        value.chars().count()
                    ),
                    kind: crate::common::validate::ConstraintKind::MinLength,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let len = value.chars().count();
                len > 70usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum length 70",
                        value.chars().count()
                    ),
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
            let value: &str = &self.0;
            let violated = {
                let frac_count = value.find('.').map_or(0, |dot| {
                    value[dot + 1..]
                        .chars()
                        .filter(char::is_ascii_digit)
                        .count()
                });
                frac_count > 0usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 0",
                        value.find('.').map_or(0, |dot| value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count())
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                digit_count > 18usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 18",
                        value.chars().filter(char::is_ascii_digit).count()
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for PaymentMethod3Code {
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
            let value: &str = &self.0;
            let violated = {
                let frac_count = value.find('.').map_or(0, |dot| {
                    value[dot + 1..]
                        .chars()
                        .filter(char::is_ascii_digit)
                        .count()
                });
                frac_count > 10usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum fraction digits 10",
                        value.find('.').map_or(0, |dot| value[dot + 1..]
                            .chars()
                            .filter(char::is_ascii_digit)
                            .count())
                    ),
                    kind: crate::common::validate::ConstraintKind::FractionDigits,
                });
            }
        }
        {
            let value: &str = &self.0;
            let violated = {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                digit_count > 11usize
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: format!(
                        "{} (got {})",
                        "value exceeds maximum total digits 11",
                        value.chars().filter(char::is_ascii_digit).count()
                    ),
                    kind: crate::common::validate::ConstraintKind::TotalDigits,
                });
            }
        }
    }
}
impl crate::common::validate::Validatable for PhoneNumber {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                let len = bytes.len();
                let result: bool = (|| -> bool {
                    let mut pos: usize = 0;
                    if !(4usize..=35usize).contains(&len) {
                        return true;
                    }
                    if pos >= len || bytes[pos] != 43u8 {
                        return true;
                    }
                    pos += 1;
                    {
                        let start = pos;
                        let limit = if pos + 3usize < len {
                            pos + 3usize
                        } else {
                            len
                        };
                        while pos < limit {
                            let b = bytes[pos];
                            if !(48u8..=57u8).contains(&b) {
                                break;
                            }
                            pos += 1;
                        }
                        let matched = pos - start;
                        if matched < 1usize {
                            return true;
                        }
                    }
                    if pos >= len || bytes[pos] != 45u8 {
                        return true;
                    }
                    pos += 1;
                    {
                        let start = pos;
                        let limit = if pos + 30usize < len {
                            pos + 30usize
                        } else {
                            len
                        };
                        while pos < limit {
                            let b = bytes[pos];
                            if !(48u8..=57u8).contains(&b)
                                && b != 40u8
                                && b != 41u8
                                && b != 43u8
                                && b != 45u8
                            {
                                break;
                            }
                            pos += 1;
                        }
                        let matched = pos - start;
                        if matched < 1usize {
                            return true;
                        }
                    }
                    if pos != len {
                        return true;
                    }
                    false
                })();
                result
            };
            if violated {
                violations.push(crate::common::validate::ConstraintViolation {
                    path: path.to_string(),
                    message: "value does not match pattern \\+[0-9]{1,3}-[0-9()+\\-]{1,30}"
                        .to_string(),
                    kind: crate::common::validate::ConstraintKind::Pattern,
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
impl crate::common::validate::Validatable for Priority2Code {
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
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let violated = {
                let bytes = value.as_bytes();
                bytes.len() != 36usize
                    || ({
                        let b = bytes[0usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[2usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[3usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[4usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[5usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[6usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[7usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || bytes[8usize] != 45u8
                    || ({
                        let b = bytes[9usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[10usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[11usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[12usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || bytes[13usize] != 45u8
                    || bytes[14usize] != 52u8
                    || ({
                        let b = bytes[15usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[16usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[17usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || bytes[18usize] != 45u8
                    || ({
                        let b = bytes[19usize];
                        b != 56u8 && b != 57u8 && b != 97u8 && b != 98u8
                    })
                    || ({
                        let b = bytes[20usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[21usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[22usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || bytes[23usize] != 45u8
                    || ({
                        let b = bytes[24usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[25usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[26usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[27usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[28usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[29usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[30usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[31usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[32usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[33usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[34usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[35usize];
                        !(97u8..=102u8).contains(&b) && !(48u8..=57u8).contains(&b)
                    })
            };
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: "value does not match pattern [a-f0-9]{8}-[a-f0-9]{4}-4[a-f0-9]{3}-[89ab][a-f0-9]{3}-[a-f0-9]{12}"
                            .to_string(),
                        kind: crate::common::validate::ConstraintKind::Pattern,
                    });
            }
        }
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
impl crate::common::validate::Validatable for AdviceType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.cdt_advc {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/CdtAdvc"), violations);
        }
        if let Some(ref wrapper) = self.dbt_advc {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/DbtAdvc"), violations);
        }
    }
}
impl crate::common::validate::Validatable for AdviceType1Choice {
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
impl crate::common::validate::Validatable for AmountType4Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::InstdAmt(inner) => {
                inner.validate_constraints(&format!("{path}/InstdAmt"), violations);
            }
            Self::EqvtAmt(inner) => {
                inner.validate_constraints(&format!("{path}/EqvtAmt"), violations);
            }
        }
    }
}
impl crate::common::validate::Validatable for Authorisation1Choice {
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
impl crate::common::validate::Validatable for Cheque11 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.chq_tp {
            val.validate_constraints(&format!("{path}/ChqTp"), violations);
        }
        if let Some(ref val) = self.chq_nb {
            val.validate_constraints(&format!("{path}/ChqNb"), violations);
        }
        if let Some(ref val) = self.chq_fr {
            val.validate_constraints(&format!("{path}/ChqFr"), violations);
        }
        if let Some(ref wrapper) = self.dlvry_mtd {
            wrapper
                .inner
                .validate_constraints(&format!("{path}/DlvryMtd"), violations);
        }
        if let Some(ref val) = self.dlvr_to {
            val.validate_constraints(&format!("{path}/DlvrTo"), violations);
        }
        if let Some(ref val) = self.instr_prty {
            val.validate_constraints(&format!("{path}/InstrPrty"), violations);
        }
        if let Some(ref val) = self.chq_mtrty_dt {
            val.validate_constraints(&format!("{path}/ChqMtrtyDt"), violations);
        }
        if let Some(ref val) = self.frms_cd {
            val.validate_constraints(&format!("{path}/FrmsCd"), violations);
        }
        for (i, item) in self.memo_fld.iter().enumerate() {
            item.validate_constraints(&format!("{path}/MemoFld[{i}]"), violations);
        }
        if let Some(ref val) = self.rgnl_clr_zone {
            val.validate_constraints(&format!("{path}/RgnlClrZone"), violations);
        }
        if let Some(ref val) = self.prt_lctn {
            val.validate_constraints(&format!("{path}/PrtLctn"), violations);
        }
        for (i, item) in self.sgntr.iter().enumerate() {
            item.validate_constraints(&format!("{path}/Sgntr[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for ChequeDeliveryMethod1Choice {
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
impl crate::common::validate::Validatable for CreditTransferTransaction54 {
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
        self.amt
            .inner
            .validate_constraints(&format!("{path}/Amt"), violations);
        if let Some(ref val) = self.xchg_rate_inf {
            val.validate_constraints(&format!("{path}/XchgRateInf"), violations);
        }
        if let Some(ref val) = self.chrg_br {
            val.validate_constraints(&format!("{path}/ChrgBr"), violations);
        }
        if let Some(ref val) = self.mndt_rltd_inf {
            val.validate_constraints(&format!("{path}/MndtRltdInf"), violations);
        }
        if let Some(ref val) = self.chq_instr {
            val.validate_constraints(&format!("{path}/ChqInstr"), violations);
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate_constraints(&format!("{path}/UltmtDbtr"), violations);
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
        if let Some(ref val) = self.cdtr_agt {
            val.validate_constraints(&format!("{path}/CdtrAgt"), violations);
        }
        if let Some(ref val) = self.cdtr_agt_acct {
            val.validate_constraints(&format!("{path}/CdtrAgtAcct"), violations);
        }
        if let Some(ref val) = self.cdtr {
            val.validate_constraints(&format!("{path}/Cdtr"), violations);
        }
        if let Some(ref val) = self.cdtr_acct {
            val.validate_constraints(&format!("{path}/CdtrAcct"), violations);
        }
        if let Some(ref val) = self.ultmt_cdtr {
            val.validate_constraints(&format!("{path}/UltmtCdtr"), violations);
        }
        for (i, item) in self.instr_for_cdtr_agt.iter().enumerate() {
            item.validate_constraints(&format!("{path}/InstrForCdtrAgt[{i}]"), violations);
        }
        if let Some(ref val) = self.instr_for_dbtr_agt {
            val.validate_constraints(&format!("{path}/InstrForDbtrAgt"), violations);
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
impl crate::common::validate::Validatable for CustomerCreditTransferInitiationV11 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.grp_hdr
            .validate_constraints(&format!("{path}/GrpHdr"), violations);
        for (i, item) in self.pmt_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/PmtInf[{i}]"), violations);
        }
        for (i, item) in self.splmtry_data.iter().enumerate() {
            item.validate_constraints(&format!("{path}/SplmtryData[{i}]"), violations);
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
impl crate::common::validate::Validatable for Document {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.cstmr_cdt_trf_initn
            .validate_constraints(&format!("{path}/CstmrCdtTrfInitn"), violations);
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
impl crate::common::validate::Validatable for EquivalentAmount2 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.amt
            .validate_constraints(&format!("{path}/Amt"), violations);
        self.ccy_of_trf
            .validate_constraints(&format!("{path}/CcyOfTrf"), violations);
    }
}
impl crate::common::validate::Validatable for ExchangeRate1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.unit_ccy {
            val.validate_constraints(&format!("{path}/UnitCcy"), violations);
        }
        if let Some(ref val) = self.xchg_rate {
            val.validate_constraints(&format!("{path}/XchgRate"), violations);
        }
        if let Some(ref val) = self.rate_tp {
            val.validate_constraints(&format!("{path}/RateTp"), violations);
        }
        if let Some(ref val) = self.ctrct_id {
            val.validate_constraints(&format!("{path}/CtrctId"), violations);
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
impl crate::common::validate::Validatable for GroupHeader95 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.msg_id
            .validate_constraints(&format!("{path}/MsgId"), violations);
        self.cre_dt_tm
            .validate_constraints(&format!("{path}/CreDtTm"), violations);
        for (i, item) in self.authstn.iter().enumerate() {
            item.inner
                .validate_constraints(&format!("{path}/Authstn[{i}]"), violations);
        }
        self.nb_of_txs
            .validate_constraints(&format!("{path}/NbOfTxs"), violations);
        if let Some(ref val) = self.ctrl_sum {
            val.validate_constraints(&format!("{path}/CtrlSum"), violations);
        }
        self.initg_pty
            .validate_constraints(&format!("{path}/InitgPty"), violations);
        if let Some(ref val) = self.fwdg_agt {
            val.validate_constraints(&format!("{path}/FwdgAgt"), violations);
        }
        if let Some(ref val) = self.initn_src {
            val.validate_constraints(&format!("{path}/InitnSrc"), violations);
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
impl crate::common::validate::Validatable for InstructionForDebtorAgent1 {
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
impl crate::common::validate::Validatable for PaymentIdentification6 {
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
        if let Some(ref val) = self.uetr {
            val.validate_constraints(&format!("{path}/UETR"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentInitiationSource1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.nm
            .validate_constraints(&format!("{path}/Nm"), violations);
        if let Some(ref val) = self.prvdr {
            val.validate_constraints(&format!("{path}/Prvdr"), violations);
        }
        if let Some(ref val) = self.vrsn {
            val.validate_constraints(&format!("{path}/Vrsn"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentInstruction40 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        self.pmt_inf_id
            .validate_constraints(&format!("{path}/PmtInfId"), violations);
        self.pmt_mtd
            .validate_constraints(&format!("{path}/PmtMtd"), violations);
        if let Some(ref val) = self.reqd_advc_tp {
            val.validate_constraints(&format!("{path}/ReqdAdvcTp"), violations);
        }
        if let Some(ref val) = self.btch_bookg {
            val.validate_constraints(&format!("{path}/BtchBookg"), violations);
        }
        if let Some(ref val) = self.nb_of_txs {
            val.validate_constraints(&format!("{path}/NbOfTxs"), violations);
        }
        if let Some(ref val) = self.ctrl_sum {
            val.validate_constraints(&format!("{path}/CtrlSum"), violations);
        }
        if let Some(ref val) = self.pmt_tp_inf {
            val.validate_constraints(&format!("{path}/PmtTpInf"), violations);
        }
        self.reqd_exctn_dt
            .inner
            .validate_constraints(&format!("{path}/ReqdExctnDt"), violations);
        if let Some(ref val) = self.poolg_adjstmnt_dt {
            val.validate_constraints(&format!("{path}/PoolgAdjstmntDt"), violations);
        }
        self.dbtr
            .validate_constraints(&format!("{path}/Dbtr"), violations);
        self.dbtr_acct
            .validate_constraints(&format!("{path}/DbtrAcct"), violations);
        self.dbtr_agt
            .validate_constraints(&format!("{path}/DbtrAgt"), violations);
        if let Some(ref val) = self.dbtr_agt_acct {
            val.validate_constraints(&format!("{path}/DbtrAgtAcct"), violations);
        }
        if let Some(ref val) = self.instr_for_dbtr_agt {
            val.validate_constraints(&format!("{path}/InstrForDbtrAgt"), violations);
        }
        if let Some(ref val) = self.ultmt_dbtr {
            val.validate_constraints(&format!("{path}/UltmtDbtr"), violations);
        }
        if let Some(ref val) = self.chrg_br {
            val.validate_constraints(&format!("{path}/ChrgBr"), violations);
        }
        if let Some(ref val) = self.chrgs_acct {
            val.validate_constraints(&format!("{path}/ChrgsAcct"), violations);
        }
        if let Some(ref val) = self.chrgs_acct_agt {
            val.validate_constraints(&format!("{path}/ChrgsAcctAgt"), violations);
        }
        for (i, item) in self.cdt_trf_tx_inf.iter().enumerate() {
            item.validate_constraints(&format!("{path}/CdtTrfTxInf[{i}]"), violations);
        }
    }
}
impl crate::common::validate::Validatable for PaymentTypeInformation26 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.instr_prty {
            val.validate_constraints(&format!("{path}/InstrPrty"), violations);
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
impl crate::common::validate::IsoMessage for Document {
    fn message_type(&self) -> &'static str {
        "pain.001.001.11"
    }
    fn root_path(&self) -> &'static str {
        "/Document"
    }
}
