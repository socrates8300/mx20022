/*! Generated from ISO 20022 XSD schema.
 Namespace: `urn:iso:std:iso:20022:tech:xsd:pacs.002.001.12`*/
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
                let frac_count = value
                    .find('.')
                    .map_or(
                        0,
                        |dot| {
                            value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                        },
                    );
                let violated = frac_count > 5usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 5",
                            frac_count
                        ),
                    });
                }
            }
            {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                let violated = digit_count > 18usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                                if !(65u8..=90u8).contains(&b)
                                    && !(48u8..=57u8).contains(&b)
                                {
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
                                if !(65u8..=90u8).contains(&b)
                                    && !(48u8..=57u8).contains(&b)
                                {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<AnyBICDec2014Identifier> for String {
    fn from(v: AnyBICDec2014Identifier) -> Self {
        v.0
    }
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
                                if !(65u8..=90u8).contains(&b)
                                    && !(48u8..=57u8).contains(&b)
                                {
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
                                if !(65u8..=90u8).contains(&b)
                                    && !(48u8..=57u8).contains(&b)
                                {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<BICFIDec2014Identifier> for String {
    fn from(v: BICFIDec2014Identifier) -> Self {
        v.0
    }
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let frac_count = value
                    .find('.')
                    .map_or(
                        0,
                        |dot| {
                            value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                        },
                    );
                let violated = frac_count > 17usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 17",
                            frac_count
                        ),
                    });
                }
            }
            {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                let violated = digit_count > 18usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                            !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[1usize];
                            !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[2usize];
                            !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                        || ({
                            let b = bytes[3usize];
                            !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                                && !(48u8..=57u8).contains(&b)
                        })
                };
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::Pattern,
                        message: "value does not match pattern [a-zA-Z0-9]{4}"
                            .to_string(),
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Exact4AlphaNumericText> for String {
    fn from(v: Exact4AlphaNumericText) -> Self {
        v.0
    }
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalCashAccountType1Code> for String {
    fn from(v: ExternalCashAccountType1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 3
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalCashClearingSystem1Code(pub String);
impl TryFrom<String> for ExternalCashClearingSystem1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 3usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 3", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalCashClearingSystem1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalCashClearingSystem1Code> for String {
    fn from(v: ExternalCashClearingSystem1Code) -> Self {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 5usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 5", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
pub struct ExternalDiscountAmountType1Code(pub String);
impl TryFrom<String> for ExternalDiscountAmountType1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 35usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 35", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
pub struct ExternalPaymentGroupStatus1Code(pub String);
impl TryFrom<String> for ExternalPaymentGroupStatus1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalPaymentGroupStatus1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalPaymentGroupStatus1Code> for String {
    fn from(v: ExternalPaymentGroupStatus1Code) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 4
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ExternalPaymentTransactionStatus1Code(pub String);
impl TryFrom<String> for ExternalPaymentTransactionStatus1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalPaymentTransactionStatus1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalPaymentTransactionStatus1Code> for String {
    fn from(v: ExternalPaymentTransactionStatus1Code) -> Self {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
pub struct ExternalStatusReason1Code(pub String);
impl TryFrom<String> for ExternalStatusReason1Code {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl ExternalStatusReason1Code {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<ExternalStatusReason1Code> for String {
    fn from(v: ExternalStatusReason1Code) -> Self {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                        message: "value does not match pattern [A-Z]{2,2}[0-9]{2,2}[a-zA-Z0-9]{1,30}"
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
/// Maximum length: 1025
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max1025Text(pub String);
impl TryFrom<String> for Max1025Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 1025usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 1025", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max1025Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max1025Text> for String {
    fn from(v: Max1025Text) -> Self {
        v.0
    }
}
/// Minimum length: 1
/// Maximum length: 105
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct Max105Text(pub String);
impl TryFrom<String> for Max105Text {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 105usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 105", len
                        ),
                    });
                }
            }
        }
        Ok(Self(value))
    }
}
impl Max105Text {
    /// Construct a validated instance, checking all XSD constraints.
    #[allow(clippy::unreadable_literal)]
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max105Text> for String {
    fn from(v: Max105Text) -> Self {
        v.0
    }
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 10240usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 10240", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Max10KBinary> for String {
    fn from(v: Max10KBinary) -> Self {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 128usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 128", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 140usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 140", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 16usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 16", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 2048usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 2048", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 34usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 34", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 350usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 350", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 35usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 35", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 4usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let len = value.chars().count();
                let violated = len < 1usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MinLength,
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                    });
                }
            }
            {
                let len = value.chars().count();
                let violated = len > 70usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 70", len
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                let frac_count = value
                    .find('.')
                    .map_or(
                        0,
                        |dot| {
                            value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                        },
                    );
                let violated = frac_count > 0usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 0",
                            frac_count
                        ),
                    });
                }
            }
            {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                let violated = digit_count > 18usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
        Self::try_from(value.into())
    }
}
impl From<Number> for String {
    fn from(v: Number) -> Self {
        v.0
    }
}
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
impl TryFrom<String> for PercentageRate {
    type Error = crate::common::validate::ConstraintError;
    #[allow(clippy::unreadable_literal)]
    fn try_from(value: String) -> Result<Self, Self::Error> {
        {
            let value: &str = &value;
            {
                let frac_count = value
                    .find('.')
                    .map_or(
                        0,
                        |dot| {
                            value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                        },
                    );
                let violated = frac_count > 10usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 10",
                            frac_count
                        ),
                    });
                }
            }
            {
                let digit_count = value.chars().filter(char::is_ascii_digit).count();
                let violated = digit_count > 11usize;
                if violated {
                    return Err(crate::common::validate::ConstraintError {
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 11",
                            digit_count
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                                if !(48u8..=57u8).contains(&b) && b != 40u8 && b != 41u8
                                    && b != 43u8 && b != 45u8
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
                        }) || bytes[8usize] != 45u8
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
                        }) || bytes[13usize] != 45u8 || bytes[14usize] != 52u8
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
                        }) || bytes[18usize] != 45u8
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
                        }) || bytes[23usize] != 45u8
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
    pub fn new(
        value: impl Into<String>,
    ) -> Result<Self, crate::common::validate::ConstraintError> {
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
    orgnl_rsn: ::std::option::Option<
        crate::common::ChoiceWrapper<MandateSetupReason1Choice>,
    >,
    orgnl_trckg_days: ::std::option::Option<Exact2NumericText>,
}
impl AmendmentInformationDetails14Builder {
    /// Set the `orgnl_mndt_id` field.
    #[must_use]
    pub fn orgnl_mndt_id(
        mut self,
        value: Max35Text,
    ) -> AmendmentInformationDetails14Builder {
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
    pub fn orgnl_dbtr_acct(
        mut self,
        value: CashAccount40,
    ) -> AmendmentInformationDetails14Builder {
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
    pub fn orgnl_fnl_colltn_dt(
        mut self,
        value: ISODate,
    ) -> AmendmentInformationDetails14Builder {
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
    ) -> ::std::result::Result<
        AmendmentInformationDetails14,
        crate::common::BuilderError,
    > {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<BranchData3, crate::common::BuilderError> {
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
    id: ::std::option::Option<
        crate::common::ChoiceWrapper<AccountIdentification4Choice>,
    >,
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
    pub fn build(
        self,
    ) -> ::std::result::Result<CashAccount40, crate::common::BuilderError> {
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
    pub fn agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> Charges7Builder {
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
    pub clr_sys_id: Option<
        crate::common::ChoiceWrapper<ClearingSystemIdentification2Choice>,
    >,
    #[serde(rename = "MmbId")]
    pub mmb_id: Max35Text,
}
/// Builder for [`ClearingSystemMemberIdentification2`]. Construct via [`ClearingSystemMemberIdentification2::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ClearingSystemMemberIdentification2Builder {
    clr_sys_id: ::std::option::Option<
        crate::common::ChoiceWrapper<ClearingSystemIdentification2Choice>,
    >,
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
    pub fn mmb_id(
        mut self,
        value: Max35Text,
    ) -> ClearingSystemMemberIdentification2Builder {
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
    ) -> ::std::result::Result<
        ClearingSystemMemberIdentification2,
        crate::common::BuilderError,
    > {
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
    pub fn tp(
        mut self,
        value: MandateTypeInformation2,
    ) -> CreditTransferMandateData1Builder {
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
    pub fn dt_of_vrfctn(
        mut self,
        value: ISODateTime,
    ) -> CreditTransferMandateData1Builder {
        self.dt_of_vrfctn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `elctrnc_sgntr` field.
    #[must_use]
    pub fn elctrnc_sgntr(
        mut self,
        value: Max10KBinary,
    ) -> CreditTransferMandateData1Builder {
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
    pub fn tp(
        mut self,
        value: CreditorReferenceType2,
    ) -> CreditorReferenceInformation2Builder {
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
    ) -> ::std::result::Result<
        CreditorReferenceInformation2,
        crate::common::BuilderError,
    > {
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
    cd_or_prtry: ::std::option::Option<
        crate::common::ChoiceWrapper<CreditorReferenceType1Choice>,
    >,
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
    pub fn build(
        self,
    ) -> ::std::result::Result<DateAndPlaceOfBirth1, crate::common::BuilderError> {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<DatePeriod2, crate::common::BuilderError> {
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
    #[serde(rename = "FIToFIPmtStsRpt")]
    pub fi_to_fi_pmt_sts_rpt: FIToFIPaymentStatusReportV12,
}
/// Builder for [`Document`]. Construct via [`Document::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct DocumentBuilder {
    fi_to_fi_pmt_sts_rpt: ::std::option::Option<FIToFIPaymentStatusReportV12>,
}
impl DocumentBuilder {
    /// Set the `fi_to_fi_pmt_sts_rpt` field.
    #[must_use]
    pub fn fi_to_fi_pmt_sts_rpt(
        mut self,
        value: FIToFIPaymentStatusReportV12,
    ) -> DocumentBuilder {
        self.fi_to_fi_pmt_sts_rpt = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
        if self.fi_to_fi_pmt_sts_rpt.is_none() {
            missing.push("fi_to_fi_pmt_sts_rpt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "Document".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(Document {
            fi_to_fi_pmt_sts_rpt: self.fi_to_fi_pmt_sts_rpt.unwrap(),
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
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> DocumentAdjustment1Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<DocumentAdjustment1, crate::common::BuilderError> {
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
    ) -> ::std::result::Result<
        DocumentLineIdentification1,
        crate::common::BuilderError,
    > {
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
    pub fn add_id(
        mut self,
        value: DocumentLineIdentification1,
    ) -> DocumentLineInformation1Builder {
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
    cd_or_prtry: ::std::option::Option<
        crate::common::ChoiceWrapper<DocumentLineType1Choice>,
    >,
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
    pub fn build(
        self,
    ) -> ::std::result::Result<DocumentLineType1, crate::common::BuilderError> {
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
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> EquivalentAmount2Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ccy_of_trf` field.
    #[must_use]
    pub fn ccy_of_trf(
        mut self,
        value: ActiveOrHistoricCurrencyCode,
    ) -> EquivalentAmount2Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<EquivalentAmount2, crate::common::BuilderError> {
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
pub struct FIToFIPaymentStatusReportV12 {
    #[serde(rename = "GrpHdr")]
    pub grp_hdr: GroupHeader101,
    #[serde(rename = "OrgnlGrpInfAndSts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub orgnl_grp_inf_and_sts: Vec<OriginalGroupHeader17>,
    #[serde(rename = "TxInfAndSts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub tx_inf_and_sts: Vec<PaymentTransaction130>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`FIToFIPaymentStatusReportV12`]. Construct via [`FIToFIPaymentStatusReportV12::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct FIToFIPaymentStatusReportV12Builder {
    grp_hdr: ::std::option::Option<GroupHeader101>,
    orgnl_grp_inf_and_sts: ::std::vec::Vec<OriginalGroupHeader17>,
    tx_inf_and_sts: ::std::vec::Vec<PaymentTransaction130>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl FIToFIPaymentStatusReportV12Builder {
    /// Set the `grp_hdr` field.
    #[must_use]
    pub fn grp_hdr(
        mut self,
        value: GroupHeader101,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.grp_hdr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_grp_inf_and_sts` field (replaces any previously added items).
    #[must_use]
    pub fn orgnl_grp_inf_and_sts(
        mut self,
        value: ::std::vec::Vec<OriginalGroupHeader17>,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.orgnl_grp_inf_and_sts = value;
        self
    }
    /// Append one item to the `orgnl_grp_inf_and_sts` field.
    #[must_use]
    pub fn add_orgnl_grp_inf_and_sts(
        mut self,
        value: OriginalGroupHeader17,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.orgnl_grp_inf_and_sts.push(value);
        self
    }
    /// Set the `tx_inf_and_sts` field (replaces any previously added items).
    #[must_use]
    pub fn tx_inf_and_sts(
        mut self,
        value: ::std::vec::Vec<PaymentTransaction130>,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.tx_inf_and_sts = value;
        self
    }
    /// Append one item to the `tx_inf_and_sts` field.
    #[must_use]
    pub fn add_tx_inf_and_sts(
        mut self,
        value: PaymentTransaction130,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.tx_inf_and_sts.push(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> FIToFIPaymentStatusReportV12Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> FIToFIPaymentStatusReportV12Builder {
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
    ) -> ::std::result::Result<
        FIToFIPaymentStatusReportV12,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.grp_hdr.is_none() {
            missing.push("grp_hdr".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "FIToFIPaymentStatusReportV12".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(FIToFIPaymentStatusReportV12 {
            grp_hdr: self.grp_hdr.unwrap(),
            orgnl_grp_inf_and_sts: self.orgnl_grp_inf_and_sts,
            tx_inf_and_sts: self.tx_inf_and_sts,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl FIToFIPaymentStatusReportV12 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> FIToFIPaymentStatusReportV12Builder {
        FIToFIPaymentStatusReportV12Builder::default()
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
    pub fn lei(
        mut self,
        value: LEIIdentifier,
    ) -> FinancialInstitutionIdentification18Builder {
        self.lei = ::std::option::Option::Some(value);
        self
    }
    /// Set the `nm` field.
    #[must_use]
    pub fn nm(
        mut self,
        value: Max140Text,
    ) -> FinancialInstitutionIdentification18Builder {
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
    ) -> ::std::result::Result<
        FinancialInstitutionIdentification18,
        crate::common::BuilderError,
    > {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<FrequencyAndMoment1, crate::common::BuilderError> {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<FrequencyPeriod1, crate::common::BuilderError> {
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
    pub fn grnshmt_admstr(
        mut self,
        value: PartyIdentification135,
    ) -> Garnishment3Builder {
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
    pub fn rmtd_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> Garnishment3Builder {
        self.rmtd_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fmly_mdcl_insrnc_ind` field.
    #[must_use]
    pub fn fmly_mdcl_insrnc_ind(
        mut self,
        value: TrueFalseIndicator,
    ) -> Garnishment3Builder {
        self.fmly_mdcl_insrnc_ind = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mplyee_termntn_ind` field.
    #[must_use]
    pub fn mplyee_termntn_ind(
        mut self,
        value: TrueFalseIndicator,
    ) -> Garnishment3Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<Garnishment3, crate::common::BuilderError> {
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
    cd_or_prtry: ::std::option::Option<
        crate::common::ChoiceWrapper<GarnishmentType1Choice>,
    >,
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
    pub fn build(
        self,
    ) -> ::std::result::Result<GarnishmentType1, crate::common::BuilderError> {
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
    schme_nm: ::std::option::Option<
        crate::common::ChoiceWrapper<AccountSchemeName1Choice>,
    >,
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
    ) -> ::std::result::Result<
        GenericAccountIdentification1,
        crate::common::BuilderError,
    > {
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
    pub schme_nm: Option<
        crate::common::ChoiceWrapper<FinancialIdentificationSchemeName1Choice>,
    >,
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
    ) -> ::std::result::Result<
        GenericFinancialIdentification1,
        crate::common::BuilderError,
    > {
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
    pub fn id(
        mut self,
        value: Exact4AlphaNumericText,
    ) -> GenericIdentification30Builder {
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
    pub schme_nm: Option<
        crate::common::ChoiceWrapper<OrganisationIdentificationSchemeName1Choice>,
    >,
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
    pub fn issr(
        mut self,
        value: Max35Text,
    ) -> GenericOrganisationIdentification1Builder {
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
    ) -> ::std::result::Result<
        GenericOrganisationIdentification1,
        crate::common::BuilderError,
    > {
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
    pub schme_nm: Option<
        crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>,
    >,
    #[serde(rename = "Issr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issr: Option<Max35Text>,
}
/// Builder for [`GenericPersonIdentification1`]. Construct via [`GenericPersonIdentification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GenericPersonIdentification1Builder {
    id: ::std::option::Option<Max35Text>,
    schme_nm: ::std::option::Option<
        crate::common::ChoiceWrapper<PersonIdentificationSchemeName1Choice>,
    >,
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
    ) -> ::std::result::Result<
        GenericPersonIdentification1,
        crate::common::BuilderError,
    > {
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
pub struct GroupHeader101 {
    #[serde(rename = "MsgId")]
    pub msg_id: Max35Text,
    #[serde(rename = "CreDtTm")]
    pub cre_dt_tm: ISODateTime,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlBizQry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_biz_qry: Option<OriginalBusinessQuery1>,
}
/// Builder for [`GroupHeader101`]. Construct via [`GroupHeader101::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct GroupHeader101Builder {
    msg_id: ::std::option::Option<Max35Text>,
    cre_dt_tm: ::std::option::Option<ISODateTime>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    orgnl_biz_qry: ::std::option::Option<OriginalBusinessQuery1>,
}
impl GroupHeader101Builder {
    /// Set the `msg_id` field.
    #[must_use]
    pub fn msg_id(mut self, value: Max35Text) -> GroupHeader101Builder {
        self.msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt_tm` field.
    #[must_use]
    pub fn cre_dt_tm(mut self, value: ISODateTime) -> GroupHeader101Builder {
        self.cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> GroupHeader101Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> GroupHeader101Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_biz_qry` field.
    #[must_use]
    pub fn orgnl_biz_qry(
        mut self,
        value: OriginalBusinessQuery1,
    ) -> GroupHeader101Builder {
        self.orgnl_biz_qry = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
    ) -> ::std::result::Result<GroupHeader101, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.msg_id.is_none() {
            missing.push("msg_id".to_owned());
        }
        if self.cre_dt_tm.is_none() {
            missing.push("cre_dt_tm".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "GroupHeader101".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(GroupHeader101 {
            msg_id: self.msg_id.unwrap(),
            cre_dt_tm: self.cre_dt_tm.unwrap(),
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
            orgnl_biz_qry: self.orgnl_biz_qry,
        })
    }
}
impl GroupHeader101 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> GroupHeader101Builder {
        GroupHeader101Builder::default()
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
    pub fn amdmnt_ind(
        mut self,
        value: TrueFalseIndicator,
    ) -> MandateRelatedInformation15Builder {
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
    pub fn elctrnc_sgntr(
        mut self,
        value: Max1025Text,
    ) -> MandateRelatedInformation15Builder {
        self.elctrnc_sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `frst_colltn_dt` field.
    #[must_use]
    pub fn frst_colltn_dt(
        mut self,
        value: ISODate,
    ) -> MandateRelatedInformation15Builder {
        self.frst_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fnl_colltn_dt` field.
    #[must_use]
    pub fn fnl_colltn_dt(
        mut self,
        value: ISODate,
    ) -> MandateRelatedInformation15Builder {
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
    pub fn trckg_days(
        mut self,
        value: Exact2NumericText,
    ) -> MandateRelatedInformation15Builder {
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
    ) -> ::std::result::Result<
        MandateRelatedInformation15,
        crate::common::BuilderError,
    > {
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
    lcl_instrm: ::std::option::Option<
        crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    >,
    ctgy_purp: ::std::option::Option<
        crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    >,
    clssfctn: ::std::option::Option<
        crate::common::ChoiceWrapper<MandateClassification1Choice>,
    >,
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
pub struct NumberOfTransactionsPerStatus5 {
    #[serde(rename = "DtldNbOfTxs")]
    pub dtld_nb_of_txs: Max15NumericText,
    #[serde(rename = "DtldSts")]
    pub dtld_sts: ExternalPaymentTransactionStatus1Code,
    #[serde(rename = "DtldCtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dtld_ctrl_sum: Option<DecimalNumber>,
}
/// Builder for [`NumberOfTransactionsPerStatus5`]. Construct via [`NumberOfTransactionsPerStatus5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct NumberOfTransactionsPerStatus5Builder {
    dtld_nb_of_txs: ::std::option::Option<Max15NumericText>,
    dtld_sts: ::std::option::Option<ExternalPaymentTransactionStatus1Code>,
    dtld_ctrl_sum: ::std::option::Option<DecimalNumber>,
}
impl NumberOfTransactionsPerStatus5Builder {
    /// Set the `dtld_nb_of_txs` field.
    #[must_use]
    pub fn dtld_nb_of_txs(
        mut self,
        value: Max15NumericText,
    ) -> NumberOfTransactionsPerStatus5Builder {
        self.dtld_nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dtld_sts` field.
    #[must_use]
    pub fn dtld_sts(
        mut self,
        value: ExternalPaymentTransactionStatus1Code,
    ) -> NumberOfTransactionsPerStatus5Builder {
        self.dtld_sts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dtld_ctrl_sum` field.
    #[must_use]
    pub fn dtld_ctrl_sum(
        mut self,
        value: DecimalNumber,
    ) -> NumberOfTransactionsPerStatus5Builder {
        self.dtld_ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
        NumberOfTransactionsPerStatus5,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.dtld_nb_of_txs.is_none() {
            missing.push("dtld_nb_of_txs".to_owned());
        }
        if self.dtld_sts.is_none() {
            missing.push("dtld_sts".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "NumberOfTransactionsPerStatus5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(NumberOfTransactionsPerStatus5 {
            dtld_nb_of_txs: self.dtld_nb_of_txs.unwrap(),
            dtld_sts: self.dtld_sts.unwrap(),
            dtld_ctrl_sum: self.dtld_ctrl_sum,
        })
    }
}
impl NumberOfTransactionsPerStatus5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> NumberOfTransactionsPerStatus5Builder {
        NumberOfTransactionsPerStatus5Builder::default()
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
    ) -> ::std::result::Result<
        OrganisationIdentification29,
        crate::common::BuilderError,
    > {
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
pub struct OriginalGroupHeader17 {
    #[serde(rename = "OrgnlMsgId")]
    pub orgnl_msg_id: Max35Text,
    #[serde(rename = "OrgnlMsgNmId")]
    pub orgnl_msg_nm_id: Max35Text,
    #[serde(rename = "OrgnlCreDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_cre_dt_tm: Option<ISODateTime>,
    #[serde(rename = "OrgnlNbOfTxs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_nb_of_txs: Option<Max15NumericText>,
    #[serde(rename = "OrgnlCtrlSum")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_ctrl_sum: Option<DecimalNumber>,
    #[serde(rename = "GrpSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grp_sts: Option<ExternalPaymentGroupStatus1Code>,
    #[serde(rename = "StsRsnInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sts_rsn_inf: Vec<StatusReasonInformation12>,
    #[serde(rename = "NbOfTxsPerSts")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub nb_of_txs_per_sts: Vec<NumberOfTransactionsPerStatus5>,
}
/// Builder for [`OriginalGroupHeader17`]. Construct via [`OriginalGroupHeader17::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalGroupHeader17Builder {
    orgnl_msg_id: ::std::option::Option<Max35Text>,
    orgnl_msg_nm_id: ::std::option::Option<Max35Text>,
    orgnl_cre_dt_tm: ::std::option::Option<ISODateTime>,
    orgnl_nb_of_txs: ::std::option::Option<Max15NumericText>,
    orgnl_ctrl_sum: ::std::option::Option<DecimalNumber>,
    grp_sts: ::std::option::Option<ExternalPaymentGroupStatus1Code>,
    sts_rsn_inf: ::std::vec::Vec<StatusReasonInformation12>,
    nb_of_txs_per_sts: ::std::vec::Vec<NumberOfTransactionsPerStatus5>,
}
impl OriginalGroupHeader17Builder {
    /// Set the `orgnl_msg_id` field.
    #[must_use]
    pub fn orgnl_msg_id(mut self, value: Max35Text) -> OriginalGroupHeader17Builder {
        self.orgnl_msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_msg_nm_id` field.
    #[must_use]
    pub fn orgnl_msg_nm_id(mut self, value: Max35Text) -> OriginalGroupHeader17Builder {
        self.orgnl_msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cre_dt_tm` field.
    #[must_use]
    pub fn orgnl_cre_dt_tm(
        mut self,
        value: ISODateTime,
    ) -> OriginalGroupHeader17Builder {
        self.orgnl_cre_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_nb_of_txs` field.
    #[must_use]
    pub fn orgnl_nb_of_txs(
        mut self,
        value: Max15NumericText,
    ) -> OriginalGroupHeader17Builder {
        self.orgnl_nb_of_txs = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_ctrl_sum` field.
    #[must_use]
    pub fn orgnl_ctrl_sum(
        mut self,
        value: DecimalNumber,
    ) -> OriginalGroupHeader17Builder {
        self.orgnl_ctrl_sum = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grp_sts` field.
    #[must_use]
    pub fn grp_sts(
        mut self,
        value: ExternalPaymentGroupStatus1Code,
    ) -> OriginalGroupHeader17Builder {
        self.grp_sts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sts_rsn_inf` field (replaces any previously added items).
    #[must_use]
    pub fn sts_rsn_inf(
        mut self,
        value: ::std::vec::Vec<StatusReasonInformation12>,
    ) -> OriginalGroupHeader17Builder {
        self.sts_rsn_inf = value;
        self
    }
    /// Append one item to the `sts_rsn_inf` field.
    #[must_use]
    pub fn add_sts_rsn_inf(
        mut self,
        value: StatusReasonInformation12,
    ) -> OriginalGroupHeader17Builder {
        self.sts_rsn_inf.push(value);
        self
    }
    /// Set the `nb_of_txs_per_sts` field (replaces any previously added items).
    #[must_use]
    pub fn nb_of_txs_per_sts(
        mut self,
        value: ::std::vec::Vec<NumberOfTransactionsPerStatus5>,
    ) -> OriginalGroupHeader17Builder {
        self.nb_of_txs_per_sts = value;
        self
    }
    /// Append one item to the `nb_of_txs_per_sts` field.
    #[must_use]
    pub fn add_nb_of_txs_per_sts(
        mut self,
        value: NumberOfTransactionsPerStatus5,
    ) -> OriginalGroupHeader17Builder {
        self.nb_of_txs_per_sts.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
    ) -> ::std::result::Result<OriginalGroupHeader17, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.orgnl_msg_id.is_none() {
            missing.push("orgnl_msg_id".to_owned());
        }
        if self.orgnl_msg_nm_id.is_none() {
            missing.push("orgnl_msg_nm_id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "OriginalGroupHeader17".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(OriginalGroupHeader17 {
            orgnl_msg_id: self.orgnl_msg_id.unwrap(),
            orgnl_msg_nm_id: self.orgnl_msg_nm_id.unwrap(),
            orgnl_cre_dt_tm: self.orgnl_cre_dt_tm,
            orgnl_nb_of_txs: self.orgnl_nb_of_txs,
            orgnl_ctrl_sum: self.orgnl_ctrl_sum,
            grp_sts: self.grp_sts,
            sts_rsn_inf: self.sts_rsn_inf,
            nb_of_txs_per_sts: self.nb_of_txs_per_sts,
        })
    }
}
impl OriginalGroupHeader17 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalGroupHeader17Builder {
        OriginalGroupHeader17Builder::default()
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
    pub fn orgnl_msg_id(
        mut self,
        value: Max35Text,
    ) -> OriginalGroupInformation29Builder {
        self.orgnl_msg_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_msg_nm_id` field.
    #[must_use]
    pub fn orgnl_msg_nm_id(
        mut self,
        value: Max35Text,
    ) -> OriginalGroupInformation29Builder {
        self.orgnl_msg_nm_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_cre_dt_tm` field.
    #[must_use]
    pub fn orgnl_cre_dt_tm(
        mut self,
        value: ISODateTime,
    ) -> OriginalGroupInformation29Builder {
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
pub struct OriginalTransactionReference35 {
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
}
/// Builder for [`OriginalTransactionReference35`]. Construct via [`OriginalTransactionReference35::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct OriginalTransactionReference35Builder {
    intr_bk_sttlm_amt: ::std::option::Option<ActiveOrHistoricCurrencyAndAmount>,
    amt: ::std::option::Option<crate::common::ChoiceWrapper<AmountType4Choice>>,
    intr_bk_sttlm_dt: ::std::option::Option<ISODate>,
    reqd_colltn_dt: ::std::option::Option<ISODate>,
    reqd_exctn_dt: ::std::option::Option<
        crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    >,
    cdtr_schme_id: ::std::option::Option<PartyIdentification135>,
    sttlm_inf: ::std::option::Option<SettlementInstruction11>,
    pmt_tp_inf: ::std::option::Option<PaymentTypeInformation27>,
    pmt_mtd: ::std::option::Option<PaymentMethod4Code>,
    mndt_rltd_inf: ::std::option::Option<
        crate::common::ChoiceWrapper<MandateRelatedData2Choice>,
    >,
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
}
impl OriginalTransactionReference35Builder {
    /// Set the `intr_bk_sttlm_amt` field.
    #[must_use]
    pub fn intr_bk_sttlm_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> OriginalTransactionReference35Builder {
        self.intr_bk_sttlm_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `amt` field.
    #[must_use]
    pub fn amt(
        mut self,
        value: crate::common::ChoiceWrapper<AmountType4Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn intr_bk_sttlm_dt(
        mut self,
        value: ISODate,
    ) -> OriginalTransactionReference35Builder {
        self.intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_colltn_dt` field.
    #[must_use]
    pub fn reqd_colltn_dt(
        mut self,
        value: ISODate,
    ) -> OriginalTransactionReference35Builder {
        self.reqd_colltn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `reqd_exctn_dt` field.
    #[must_use]
    pub fn reqd_exctn_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.reqd_exctn_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_schme_id` field.
    #[must_use]
    pub fn cdtr_schme_id(
        mut self,
        value: PartyIdentification135,
    ) -> OriginalTransactionReference35Builder {
        self.cdtr_schme_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sttlm_inf` field.
    #[must_use]
    pub fn sttlm_inf(
        mut self,
        value: SettlementInstruction11,
    ) -> OriginalTransactionReference35Builder {
        self.sttlm_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_tp_inf` field.
    #[must_use]
    pub fn pmt_tp_inf(
        mut self,
        value: PaymentTypeInformation27,
    ) -> OriginalTransactionReference35Builder {
        self.pmt_tp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pmt_mtd` field.
    #[must_use]
    pub fn pmt_mtd(
        mut self,
        value: PaymentMethod4Code,
    ) -> OriginalTransactionReference35Builder {
        self.pmt_mtd = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mndt_rltd_inf` field.
    #[must_use]
    pub fn mndt_rltd_inf(
        mut self,
        value: crate::common::ChoiceWrapper<MandateRelatedData2Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.mndt_rltd_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rmt_inf` field.
    #[must_use]
    pub fn rmt_inf(
        mut self,
        value: RemittanceInformation21,
    ) -> OriginalTransactionReference35Builder {
        self.rmt_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_dbtr` field.
    #[must_use]
    pub fn ultmt_dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.ultmt_dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr` field.
    #[must_use]
    pub fn dbtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.dbtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_acct` field.
    #[must_use]
    pub fn dbtr_acct(
        mut self,
        value: CashAccount40,
    ) -> OriginalTransactionReference35Builder {
        self.dbtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt` field.
    #[must_use]
    pub fn dbtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> OriginalTransactionReference35Builder {
        self.dbtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dbtr_agt_acct` field.
    #[must_use]
    pub fn dbtr_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> OriginalTransactionReference35Builder {
        self.dbtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt` field.
    #[must_use]
    pub fn cdtr_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> OriginalTransactionReference35Builder {
        self.cdtr_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_agt_acct` field.
    #[must_use]
    pub fn cdtr_agt_acct(
        mut self,
        value: CashAccount40,
    ) -> OriginalTransactionReference35Builder {
        self.cdtr_agt_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr` field.
    #[must_use]
    pub fn cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cdtr_acct` field.
    #[must_use]
    pub fn cdtr_acct(
        mut self,
        value: CashAccount40,
    ) -> OriginalTransactionReference35Builder {
        self.cdtr_acct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ultmt_cdtr` field.
    #[must_use]
    pub fn ultmt_cdtr(
        mut self,
        value: crate::common::ChoiceWrapper<Party40Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.ultmt_cdtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `purp` field.
    #[must_use]
    pub fn purp(
        mut self,
        value: crate::common::ChoiceWrapper<Purpose2Choice>,
    ) -> OriginalTransactionReference35Builder {
        self.purp = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
        OriginalTransactionReference35,
        crate::common::BuilderError,
    > {
        ::std::result::Result::Ok(OriginalTransactionReference35 {
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
        })
    }
}
impl OriginalTransactionReference35 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> OriginalTransactionReference35Builder {
        OriginalTransactionReference35Builder::default()
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
    pub fn build(
        self,
    ) -> ::std::result::Result<OtherContact1, crate::common::BuilderError> {
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
pub struct PaymentTransaction130 {
    #[serde(rename = "StsId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sts_id: Option<Max35Text>,
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
    #[serde(rename = "TxSts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tx_sts: Option<ExternalPaymentTransactionStatus1Code>,
    #[serde(rename = "StsRsnInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sts_rsn_inf: Vec<StatusReasonInformation12>,
    #[serde(rename = "ChrgsInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub chrgs_inf: Vec<Charges7>,
    #[serde(rename = "AccptncDtTm")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accptnc_dt_tm: Option<ISODateTime>,
    #[serde(rename = "FctvIntrBkSttlmDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fctv_intr_bk_sttlm_dt: Option<
        crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    >,
    #[serde(rename = "AcctSvcrRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acct_svcr_ref: Option<Max35Text>,
    #[serde(rename = "ClrSysRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clr_sys_ref: Option<Max35Text>,
    #[serde(rename = "InstgAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instg_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "InstdAgt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instd_agt: Option<BranchAndFinancialInstitutionIdentification6>,
    #[serde(rename = "OrgnlTxRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgnl_tx_ref: Option<OriginalTransactionReference35>,
    #[serde(rename = "SplmtryData")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub splmtry_data: Vec<SupplementaryData1>,
}
/// Builder for [`PaymentTransaction130`]. Construct via [`PaymentTransaction130::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct PaymentTransaction130Builder {
    sts_id: ::std::option::Option<Max35Text>,
    orgnl_grp_inf: ::std::option::Option<OriginalGroupInformation29>,
    orgnl_instr_id: ::std::option::Option<Max35Text>,
    orgnl_end_to_end_id: ::std::option::Option<Max35Text>,
    orgnl_tx_id: ::std::option::Option<Max35Text>,
    orgnl_uetr: ::std::option::Option<UUIDv4Identifier>,
    tx_sts: ::std::option::Option<ExternalPaymentTransactionStatus1Code>,
    sts_rsn_inf: ::std::vec::Vec<StatusReasonInformation12>,
    chrgs_inf: ::std::vec::Vec<Charges7>,
    accptnc_dt_tm: ::std::option::Option<ISODateTime>,
    fctv_intr_bk_sttlm_dt: ::std::option::Option<
        crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    >,
    acct_svcr_ref: ::std::option::Option<Max35Text>,
    clr_sys_ref: ::std::option::Option<Max35Text>,
    instg_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    instd_agt: ::std::option::Option<BranchAndFinancialInstitutionIdentification6>,
    orgnl_tx_ref: ::std::option::Option<OriginalTransactionReference35>,
    splmtry_data: ::std::vec::Vec<SupplementaryData1>,
}
impl PaymentTransaction130Builder {
    /// Set the `sts_id` field.
    #[must_use]
    pub fn sts_id(mut self, value: Max35Text) -> PaymentTransaction130Builder {
        self.sts_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_grp_inf` field.
    #[must_use]
    pub fn orgnl_grp_inf(
        mut self,
        value: OriginalGroupInformation29,
    ) -> PaymentTransaction130Builder {
        self.orgnl_grp_inf = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_instr_id` field.
    #[must_use]
    pub fn orgnl_instr_id(mut self, value: Max35Text) -> PaymentTransaction130Builder {
        self.orgnl_instr_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_end_to_end_id` field.
    #[must_use]
    pub fn orgnl_end_to_end_id(
        mut self,
        value: Max35Text,
    ) -> PaymentTransaction130Builder {
        self.orgnl_end_to_end_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_tx_id` field.
    #[must_use]
    pub fn orgnl_tx_id(mut self, value: Max35Text) -> PaymentTransaction130Builder {
        self.orgnl_tx_id = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_uetr` field.
    #[must_use]
    pub fn orgnl_uetr(
        mut self,
        value: UUIDv4Identifier,
    ) -> PaymentTransaction130Builder {
        self.orgnl_uetr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `tx_sts` field.
    #[must_use]
    pub fn tx_sts(
        mut self,
        value: ExternalPaymentTransactionStatus1Code,
    ) -> PaymentTransaction130Builder {
        self.tx_sts = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sts_rsn_inf` field (replaces any previously added items).
    #[must_use]
    pub fn sts_rsn_inf(
        mut self,
        value: ::std::vec::Vec<StatusReasonInformation12>,
    ) -> PaymentTransaction130Builder {
        self.sts_rsn_inf = value;
        self
    }
    /// Append one item to the `sts_rsn_inf` field.
    #[must_use]
    pub fn add_sts_rsn_inf(
        mut self,
        value: StatusReasonInformation12,
    ) -> PaymentTransaction130Builder {
        self.sts_rsn_inf.push(value);
        self
    }
    /// Set the `chrgs_inf` field (replaces any previously added items).
    #[must_use]
    pub fn chrgs_inf(
        mut self,
        value: ::std::vec::Vec<Charges7>,
    ) -> PaymentTransaction130Builder {
        self.chrgs_inf = value;
        self
    }
    /// Append one item to the `chrgs_inf` field.
    #[must_use]
    pub fn add_chrgs_inf(mut self, value: Charges7) -> PaymentTransaction130Builder {
        self.chrgs_inf.push(value);
        self
    }
    /// Set the `accptnc_dt_tm` field.
    #[must_use]
    pub fn accptnc_dt_tm(mut self, value: ISODateTime) -> PaymentTransaction130Builder {
        self.accptnc_dt_tm = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fctv_intr_bk_sttlm_dt` field.
    #[must_use]
    pub fn fctv_intr_bk_sttlm_dt(
        mut self,
        value: crate::common::ChoiceWrapper<DateAndDateTime2Choice>,
    ) -> PaymentTransaction130Builder {
        self.fctv_intr_bk_sttlm_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `acct_svcr_ref` field.
    #[must_use]
    pub fn acct_svcr_ref(mut self, value: Max35Text) -> PaymentTransaction130Builder {
        self.acct_svcr_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_sys_ref` field.
    #[must_use]
    pub fn clr_sys_ref(mut self, value: Max35Text) -> PaymentTransaction130Builder {
        self.clr_sys_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instg_agt` field.
    #[must_use]
    pub fn instg_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentTransaction130Builder {
        self.instg_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `instd_agt` field.
    #[must_use]
    pub fn instd_agt(
        mut self,
        value: BranchAndFinancialInstitutionIdentification6,
    ) -> PaymentTransaction130Builder {
        self.instd_agt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `orgnl_tx_ref` field.
    #[must_use]
    pub fn orgnl_tx_ref(
        mut self,
        value: OriginalTransactionReference35,
    ) -> PaymentTransaction130Builder {
        self.orgnl_tx_ref = ::std::option::Option::Some(value);
        self
    }
    /// Set the `splmtry_data` field (replaces any previously added items).
    #[must_use]
    pub fn splmtry_data(
        mut self,
        value: ::std::vec::Vec<SupplementaryData1>,
    ) -> PaymentTransaction130Builder {
        self.splmtry_data = value;
        self
    }
    /// Append one item to the `splmtry_data` field.
    #[must_use]
    pub fn add_splmtry_data(
        mut self,
        value: SupplementaryData1,
    ) -> PaymentTransaction130Builder {
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
    ) -> ::std::result::Result<PaymentTransaction130, crate::common::BuilderError> {
        ::std::result::Result::Ok(PaymentTransaction130 {
            sts_id: self.sts_id,
            orgnl_grp_inf: self.orgnl_grp_inf,
            orgnl_instr_id: self.orgnl_instr_id,
            orgnl_end_to_end_id: self.orgnl_end_to_end_id,
            orgnl_tx_id: self.orgnl_tx_id,
            orgnl_uetr: self.orgnl_uetr,
            tx_sts: self.tx_sts,
            sts_rsn_inf: self.sts_rsn_inf,
            chrgs_inf: self.chrgs_inf,
            accptnc_dt_tm: self.accptnc_dt_tm,
            fctv_intr_bk_sttlm_dt: self.fctv_intr_bk_sttlm_dt,
            acct_svcr_ref: self.acct_svcr_ref,
            clr_sys_ref: self.clr_sys_ref,
            instg_agt: self.instg_agt,
            instd_agt: self.instd_agt,
            orgnl_tx_ref: self.orgnl_tx_ref,
            splmtry_data: self.splmtry_data,
        })
    }
}
impl PaymentTransaction130 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> PaymentTransaction130Builder {
        PaymentTransaction130Builder::default()
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
    lcl_instrm: ::std::option::Option<
        crate::common::ChoiceWrapper<LocalInstrument2Choice>,
    >,
    seq_tp: ::std::option::Option<SequenceType3Code>,
    ctgy_purp: ::std::option::Option<
        crate::common::ChoiceWrapper<CategoryPurpose1Choice>,
    >,
}
impl PaymentTypeInformation27Builder {
    /// Set the `instr_prty` field.
    #[must_use]
    pub fn instr_prty(
        mut self,
        value: Priority2Code,
    ) -> PaymentTypeInformation27Builder {
        self.instr_prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `clr_chanl` field.
    #[must_use]
    pub fn clr_chanl(
        mut self,
        value: ClearingChannel2Code,
    ) -> PaymentTypeInformation27Builder {
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
    pub fn seq_tp(
        mut self,
        value: SequenceType3Code,
    ) -> PaymentTypeInformation27Builder {
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
    pub fn adr_line(
        mut self,
        value: ::std::vec::Vec<Max70Text>,
    ) -> PostalAddress24Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<PostalAddress24, crate::common::BuilderError> {
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
    ) -> ::std::result::Result<
        ProxyAccountIdentification1,
        crate::common::BuilderError,
    > {
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
    pub fn tp(
        mut self,
        value: ReferredDocumentType4,
    ) -> ReferredDocumentInformation7Builder {
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
    ) -> ::std::result::Result<
        ReferredDocumentInformation7,
        crate::common::BuilderError,
    > {
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
    cd_or_prtry: ::std::option::Option<
        crate::common::ChoiceWrapper<ReferredDocumentType3Choice>,
    >,
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
    pub fn add_dscnt_apld_amt(
        mut self,
        value: DiscountAmountAndType1,
    ) -> RemittanceAmount2Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<RemittanceAmount2, crate::common::BuilderError> {
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
    pub fn add_dscnt_apld_amt(
        mut self,
        value: DiscountAmountAndType1,
    ) -> RemittanceAmount3Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<RemittanceAmount3, crate::common::BuilderError> {
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
    pub fn ustrd(
        mut self,
        value: ::std::vec::Vec<Max140Text>,
    ) -> RemittanceInformation21Builder {
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
pub enum ServiceLevel8Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalServiceLevel1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
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
    pub clr_sys: Option<
        crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>,
    >,
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
    clr_sys: ::std::option::Option<
        crate::common::ChoiceWrapper<ClearingSystemIdentification3Choice>,
    >,
    instg_rmbrsmnt_agt: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification6,
    >,
    instg_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    instd_rmbrsmnt_agt: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification6,
    >,
    instd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
    thrd_rmbrsmnt_agt: ::std::option::Option<
        BranchAndFinancialInstitutionIdentification6,
    >,
    thrd_rmbrsmnt_agt_acct: ::std::option::Option<CashAccount40>,
}
impl SettlementInstruction11Builder {
    /// Set the `sttlm_mtd` field.
    #[must_use]
    pub fn sttlm_mtd(
        mut self,
        value: SettlementMethod1Code,
    ) -> SettlementInstruction11Builder {
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
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum StatusReason6Choice {
    #[serde(rename = "Cd")]
    Cd(ExternalStatusReason1Code),
    #[serde(rename = "Prtry")]
    Prtry(Max35Text),
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StatusReasonInformation12 {
    #[serde(rename = "Orgtr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orgtr: Option<PartyIdentification135>,
    #[serde(rename = "Rsn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rsn: Option<crate::common::ChoiceWrapper<StatusReason6Choice>>,
    #[serde(rename = "AddtlInf")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub addtl_inf: Vec<Max105Text>,
}
/// Builder for [`StatusReasonInformation12`]. Construct via [`StatusReasonInformation12::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct StatusReasonInformation12Builder {
    orgtr: ::std::option::Option<PartyIdentification135>,
    rsn: ::std::option::Option<crate::common::ChoiceWrapper<StatusReason6Choice>>,
    addtl_inf: ::std::vec::Vec<Max105Text>,
}
impl StatusReasonInformation12Builder {
    /// Set the `orgtr` field.
    #[must_use]
    pub fn orgtr(
        mut self,
        value: PartyIdentification135,
    ) -> StatusReasonInformation12Builder {
        self.orgtr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rsn` field.
    #[must_use]
    pub fn rsn(
        mut self,
        value: crate::common::ChoiceWrapper<StatusReason6Choice>,
    ) -> StatusReasonInformation12Builder {
        self.rsn = ::std::option::Option::Some(value);
        self
    }
    /// Set the `addtl_inf` field (replaces any previously added items).
    #[must_use]
    pub fn addtl_inf(
        mut self,
        value: ::std::vec::Vec<Max105Text>,
    ) -> StatusReasonInformation12Builder {
        self.addtl_inf = value;
        self
    }
    /// Append one item to the `addtl_inf` field.
    #[must_use]
    pub fn add_addtl_inf(
        mut self,
        value: Max105Text,
    ) -> StatusReasonInformation12Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<StatusReasonInformation12, crate::common::BuilderError> {
        ::std::result::Result::Ok(StatusReasonInformation12 {
            orgtr: self.orgtr,
            rsn: self.rsn,
            addtl_inf: self.addtl_inf,
        })
    }
}
impl StatusReasonInformation12 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> StatusReasonInformation12Builder {
        StatusReasonInformation12Builder::default()
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
    pub fn tax_rmt(
        mut self,
        value: TaxData1,
    ) -> StructuredRemittanceInformation17Builder {
        self.tax_rmt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `grnshmt_rmt` field.
    #[must_use]
    pub fn grnshmt_rmt(
        mut self,
        value: Garnishment3,
    ) -> StructuredRemittanceInformation17Builder {
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
    ) -> ::std::result::Result<
        StructuredRemittanceInformation17,
        crate::common::BuilderError,
    > {
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
    pub fn envlp(
        mut self,
        value: SupplementaryDataEnvelope1,
    ) -> SupplementaryData1Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<SupplementaryData1, crate::common::BuilderError> {
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
    pub fn taxbl_base_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxAmount3Builder {
        self.taxbl_base_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `ttl_amt` field.
    #[must_use]
    pub fn ttl_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxAmount3Builder {
        self.ttl_amt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `dtls` field (replaces any previously added items).
    #[must_use]
    pub fn dtls(
        mut self,
        value: ::std::vec::Vec<TaxRecordDetails3>,
    ) -> TaxAmount3Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<TaxAmount3, crate::common::BuilderError> {
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
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxAmountAndType1Builder {
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
    ) -> ::std::result::Result<TaxAmountAndType1, crate::common::BuilderError> {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<TaxAuthorisation1, crate::common::BuilderError> {
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
    pub fn ttl_tax_amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxData1Builder {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<TaxPeriod3, crate::common::BuilderError> {
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
    pub fn build(
        self,
    ) -> ::std::result::Result<TaxRecord3, crate::common::BuilderError> {
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
    pub fn amt(
        mut self,
        value: ActiveOrHistoricCurrencyAndAmount,
    ) -> TaxRecordDetails3Builder {
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
    ) -> ::std::result::Result<TaxRecordDetails3, crate::common::BuilderError> {
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
impl crate::common::validate::Validatable
for ActiveOrHistoricCurrencyAndAmountSimpleType {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let value: &str = &self.0;
            let frac_count = value
                .find('.')
                .map_or(
                    0,
                    |dot| {
                        value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                    },
                );
            let violated = frac_count > 5usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 5",
                            frac_count
                        ),
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                    });
            }
        }
        {
            let value: &str = &self.0;
            let digit_count = value.chars().filter(char::is_ascii_digit).count();
            let violated = digit_count > 18usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
    ) {}
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
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                            {
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
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                            {
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
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                            {
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
                            if !(65u8..=90u8).contains(&b) && !(48u8..=57u8).contains(&b)
                            {
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
impl crate::common::validate::Validatable for ClearingChannel2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
    ) {}
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
            let frac_count = value
                .find('.')
                .map_or(
                    0,
                    |dot| {
                        value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                    },
                );
            let violated = frac_count > 17usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 17",
                            frac_count
                        ),
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                    });
            }
        }
        {
            let value: &str = &self.0;
            let digit_count = value.chars().filter(char::is_ascii_digit).count();
            let violated = digit_count > 18usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
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
    ) {}
}
impl crate::common::validate::Validatable for DocumentType6Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
                        !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[1usize];
                        !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[2usize];
                        !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
                    || ({
                        let b = bytes[3usize];
                        !(97u8..=122u8).contains(&b) && !(65u8..=90u8).contains(&b)
                            && !(48u8..=57u8).contains(&b)
                    })
            };
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: "value does not match pattern [a-zA-Z0-9]{4}"
                            .to_string(),
                        kind: crate::common::validate::ConstraintKind::Pattern,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalAccountIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 3usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 3", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 5usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 5", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable
for ExternalFinancialInstitutionIdentification1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 35usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 35", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalPaymentGroupStatus1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalPaymentTransactionStatus1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for ExternalStatusReason1Code {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
    ) {}
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
    ) {}
}
impl crate::common::validate::Validatable for ISODateTime {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for ISOYear {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: "value does not match pattern [A-Z0-9]{18,18}[0-9]{2,2}"
                            .to_string(),
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
    ) {}
}
impl crate::common::validate::Validatable for Max1025Text {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 1025usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 1025", len
                        ),
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 105usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 105", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MaxLength,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for Max10KBinary {
    #[allow(clippy::unreadable_literal)]
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 10240usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 10240", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 128usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 128", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 140usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 140", len
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 16usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 16", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 2048usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 2048", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 34usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 34", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 350usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 350", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 35usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 35", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 4usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 4", len
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
        let len = self.0.chars().count();
        {
            let violated = len < 1usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value is shorter than minimum length 1", len
                        ),
                        kind: crate::common::validate::ConstraintKind::MinLength,
                    });
            }
        }
        {
            let violated = len > 70usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum length 70", len
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
    ) {}
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
            let frac_count = value
                .find('.')
                .map_or(
                    0,
                    |dot| {
                        value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                    },
                );
            let violated = frac_count > 0usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 0",
                            frac_count
                        ),
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                    });
            }
        }
        {
            let value: &str = &self.0;
            let digit_count = value.chars().filter(char::is_ascii_digit).count();
            let violated = digit_count > 18usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 18",
                            digit_count
                        ),
                        kind: crate::common::validate::ConstraintKind::TotalDigits,
                    });
            }
        }
    }
}
impl crate::common::validate::Validatable for PaymentMethod4Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
            let frac_count = value
                .find('.')
                .map_or(
                    0,
                    |dot| {
                        value[dot + 1..].chars().filter(char::is_ascii_digit).count()
                    },
                );
            let violated = frac_count > 10usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum fraction digits 10",
                            frac_count
                        ),
                        kind: crate::common::validate::ConstraintKind::FractionDigits,
                    });
            }
        }
        {
            let value: &str = &self.0;
            let digit_count = value.chars().filter(char::is_ascii_digit).count();
            let violated = digit_count > 11usize;
            if violated {
                violations
                    .push(crate::common::validate::ConstraintViolation {
                        path: path.to_string(),
                        message: format!(
                            "{} (got {})", "value exceeds maximum total digits 11",
                            digit_count
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
                        let limit = if pos + 3usize < len { pos + 3usize } else { len };
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
                            if !(48u8..=57u8).contains(&b) && b != 40u8 && b != 41u8
                                && b != 43u8 && b != 45u8
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
                violations
                    .push(crate::common::validate::ConstraintViolation {
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
    ) {}
}
impl crate::common::validate::Validatable for Priority2Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for SequenceType3Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for SettlementMethod1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for TaxRecordPeriod1Code {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for TrueFalseIndicator {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
                    }) || bytes[8usize] != 45u8
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
                    }) || bytes[13usize] != 45u8 || bytes[14usize] != 52u8
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
                    }) || bytes[18usize] != 45u8
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
                    }) || bytes[23usize] != 45u8
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/IBAN");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Othr(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Othr");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.ccy.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/@Ccy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for AmendmentInformationDetails14 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.orgnl_mndt_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlMndtId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_schme_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCdtrSchmeId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCdtrAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_cdtr_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCdtrAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_dbtr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlDbtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlDbtrAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlDbtrAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_dbtr_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlDbtrAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_fnl_colltn_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlFnlColltnDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.orgnl_frqcy {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlFrqcy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.orgnl_rsn {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlRsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_trckg_days {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlTrckgDays");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/InstdAmt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::EqvtAmt(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/EqvtAmt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable
for BranchAndFinancialInstitutionIdentification6 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.fin_instn_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FinInstnId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.brnch_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BrnchId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.lei {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LEI");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pstl_adr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PstlAdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.tp {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ccy {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Ccy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prxy {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Prxy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for Charges7 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.agt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Agt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ClrSysId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.mmb_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MmbId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for Contact4 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.nm_prfx {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/NmPrfx");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.phne_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PhneNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.mob_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MobNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.fax_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FaxNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.email_adr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/EmailAdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.email_purp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/EmailPurp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.job_titl {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/JobTitl");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rspnsblty {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rspnsblty");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dept {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dept");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.othr.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Othr[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prefrd_mtd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PrefrdMtd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MndtId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dt_of_sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtOfSgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dt_of_vrfctn {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtOfVrfctn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.elctrnc_sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ElctrncSgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.frst_pmt_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FrstPmtDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.fnl_pmt_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FnlPmtDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.frqcy {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Frqcy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.rsn {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.r#ref {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Ref");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.cd_or_prtry.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdOrPrtry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Dt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::DtTm(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/DtTm");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.birth_dt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BirthDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prvc_of_birth {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PrvcOfBirth");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.city_of_birth.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CityOfBirth");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.ctry_of_birth.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtryOfBirth");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
        {
            let snap = violations.len();
            self.fr_dt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FrDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.to_dt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ToDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for DiscountAmountAndType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref wrapper) = self.tp {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.fi_to_fi_pmt_sts_rpt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FIToFIPmtStsRpt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for DocumentAdjustment1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdt_dbt_ind {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtDbtInd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rsn {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.addtl_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AddtlInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rltd_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RltdDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineInformation1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (idx, elem) in self.id.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.desc {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Desc");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for DocumentLineType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.cd_or_prtry.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdOrPrtry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.ccy_of_trf.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CcyOfTrf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for FIToFIPaymentStatusReportV12 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.grp_hdr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/GrpHdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.orgnl_grp_inf_and_sts.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlGrpInfAndSts[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.tx_inf_and_sts.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TxInfAndSts[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.splmtry_data.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SplmtryData[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BICFI");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.clr_sys_mmb_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ClrSysMmbId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.lei {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LEI");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pstl_adr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PstlAdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.othr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Othr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Tp");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prd(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::PtInTm(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/PtInTm");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.tp.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.pt_in_tm.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PtInTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for FrequencyPeriod1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.tp.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.cnt_per_prd.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CntPerPrd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for Garnishment3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.tp.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.grnshee {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Grnshee");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.grnshmt_admstr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/GrnshmtAdmstr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ref_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RefNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RmtdAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.fmly_mdcl_insrnc_ind {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FmlyMdclInsrncInd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.mplyee_termntn_ind {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MplyeeTermntnInd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GarnishmentType1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.cd_or_prtry.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdOrPrtry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.schme_nm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SchmeNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GenericFinancialIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.schme_nm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SchmeNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GenericIdentification30 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.issr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.schme_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SchmeNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GenericOrganisationIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.schme_nm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SchmeNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GenericPersonIdentification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.schme_nm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SchmeNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for GroupHeader101 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.msg_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MsgId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.cre_dt_tm.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CreDtTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instg_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstgAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instd_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstdAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_biz_qry {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlBizQry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for MandateRelatedData2Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::DrctDbtMndt(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/DrctDbtMndt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::CdtTrfMndt(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/CdtTrfMndt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for MandateRelatedInformation15 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.mndt_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MndtId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dt_of_sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtOfSgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.amdmnt_ind {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AmdmntInd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.amdmnt_inf_dtls {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AmdmntInfDtls");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.elctrnc_sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ElctrncSgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.frst_colltn_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FrstColltnDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.fnl_colltn_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FnlColltnDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.frqcy {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Frqcy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.rsn {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.trckg_days {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TrckgDays");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SvcLvl");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.lcl_instrm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LclInstrm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.ctgy_purp {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtgyPurp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.clssfctn {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Clssfctn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for NumberOfTransactionsPerStatus5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.dtld_nb_of_txs.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtldNbOfTxs");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.dtld_sts.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtldSts");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dtld_ctrl_sum {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtldCtrlSum");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AnyBIC");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.lei {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LEI");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.othr.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Othr[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable
for OrganisationIdentificationSchemeName1Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for OriginalBusinessQuery1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.msg_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MsgId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.msg_nm_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MsgNmId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cre_dt_tm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CreDtTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for OriginalGroupHeader17 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.orgnl_msg_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlMsgId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.orgnl_msg_nm_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlMsgNmId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_cre_dt_tm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCreDtTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_nb_of_txs {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlNbOfTxs");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_ctrl_sum {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCtrlSum");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.grp_sts {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/GrpSts");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.sts_rsn_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/StsRsnInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.nb_of_txs_per_sts.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/NbOfTxsPerSts[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for OriginalGroupInformation29 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.orgnl_msg_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlMsgId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.orgnl_msg_nm_id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlMsgNmId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_cre_dt_tm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlCreDtTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for OriginalTransactionReference35 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.intr_bk_sttlm_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/IntrBkSttlmAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.amt {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.intr_bk_sttlm_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/IntrBkSttlmDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.reqd_colltn_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ReqdColltnDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.reqd_exctn_dt {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ReqdExctnDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdtr_schme_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtrSchmeId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.sttlm_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SttlmInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pmt_tp_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PmtTpInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pmt_mtd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PmtMtd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.mndt_rltd_inf {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MndtRltdInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rmt_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RmtInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.ultmt_dbtr {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/UltmtDbtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.dbtr {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dbtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dbtr_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DbtrAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dbtr_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DbtrAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dbtr_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DbtrAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdtr_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtrAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdtr_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtrAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.cdtr {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Cdtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdtr_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtrAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.ultmt_cdtr {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/UltmtCdtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.purp {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Purp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
        {
            let snap = violations.len();
            self.chanl_tp.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ChanlTp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/OrgId");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::PrvtId(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/PrvtId");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Pty");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Agt(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Agt");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pstl_adr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PstlAdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.id {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctry_of_res {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtryOfRes");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctct_dtls {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtctDtls");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for PaymentTransaction130 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.sts_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/StsId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_grp_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlGrpInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_instr_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlInstrId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_end_to_end_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlEndToEndId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_tx_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlTxId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_uetr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlUETR");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tx_sts {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TxSts");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.sts_rsn_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/StsRsnInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.chrgs_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ChrgsInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.accptnc_dt_tm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AccptncDtTm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.fctv_intr_bk_sttlm_dt {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FctvIntrBkSttlmDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.acct_svcr_ref {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AcctSvcrRef");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.clr_sys_ref {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ClrSysRef");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instg_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstgAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instd_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstdAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.orgnl_tx_ref {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/OrgnlTxRef");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.splmtry_data.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SplmtryData[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstrPrty");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.clr_chanl {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ClrChanl");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.svc_lvl.iter().enumerate() {
            let snap = violations.len();
            elem.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SvcLvl[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.lcl_instrm {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LclInstrm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.seq_tp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SeqTp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.ctgy_purp {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtgyPurp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DtAndPlcOfBirth");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.othr.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Othr[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AdrTp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dept {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dept");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.sub_dept {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SubDept");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.strt_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/StrtNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.bldg_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BldgNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.bldg_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BldgNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.flr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Flr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pst_bx {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PstBx");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.room {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Room");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pst_cd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PstCd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.twn_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TwnNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.twn_lctn_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TwnLctnNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dstrct_nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DstrctNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctry_sub_dvsn {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtrySubDvsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctry {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Ctry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.adr_line.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AdrLine[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.id.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Id");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rltd_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RltdDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.line_dtls.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/LineDtls[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
        {
            let snap = violations.len();
            self.cd_or_prtry.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdOrPrtry");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.issr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Issr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DuePyblAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.dscnt_apld_amt.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DscntApldAmt[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtNoteAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.tax_amt.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxAmt[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.adjstmnt_amt_and_rsn.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AdjstmntAmtAndRsn[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RmtdAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DuePyblAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.dscnt_apld_amt.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DscntApldAmt[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdt_note_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtNoteAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.tax_amt.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxAmt[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.adjstmnt_amt_and_rsn.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AdjstmntAmtAndRsn[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rmtd_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RmtdAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for RemittanceInformation21 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        for (idx, elem) in self.ustrd.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Ustrd[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.strd.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Strd[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for SettlementInstruction11 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.sttlm_mtd.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SttlmMtd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.sttlm_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SttlmAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.clr_sys {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ClrSys");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instg_rmbrsmnt_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstgRmbrsmntAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instg_rmbrsmnt_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstgRmbrsmntAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstdRmbrsmntAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.instd_rmbrsmnt_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/InstdRmbrsmntAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.thrd_rmbrsmnt_agt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ThrdRmbrsmntAgt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.thrd_rmbrsmnt_agt_acct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/ThrdRmbrsmntAgtAcct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for StatusReason6Choice {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        match self {
            Self::Cd(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for StatusReasonInformation12 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.orgtr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Orgtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref wrapper) = self.rsn {
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rsn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.addtl_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AddtlInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
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
        for (idx, elem) in self.rfrd_doc_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RfrdDocInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.rfrd_doc_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RfrdDocAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cdtr_ref_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CdtrRefInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.invcr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Invcr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.invcee {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Invcee");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tax_rmt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxRmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.grnshmt_rmt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/GrnshmtRmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.addtl_rmt_inf.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AddtlRmtInf[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PlcAndNm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.envlp.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Envlp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for SupplementaryDataEnvelope1 {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for TaxAmount3 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.rate {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rate");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.taxbl_base_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxblBaseAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ttl_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TtlAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.dtls.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dtls[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            wrapper.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
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
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Cd");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
            }
            Self::Prtry(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/Prtry");
                    for v in &mut violations[snap..] {
                        v.path.insert_str(0, &pfx);
                    }
                }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Titl");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.nm {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Nm");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Cdtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dbtr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dbtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ultmt_dbtr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/UltmtDbtr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.admstn_zone {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AdmstnZone");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ref_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RefNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.mtd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Mtd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ttl_taxbl_base_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TtlTaxblBaseAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ttl_tax_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TtlTaxAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Dt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.seq_nb {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/SeqNb");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.rcrd.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rcrd[{idx}]");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.regn_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RegnId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tax_tp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxTp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.regn_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/RegnId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tax_tp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxTp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.authstn {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Authstn");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Yr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tp {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.fr_to_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FrToDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Tp");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctgy {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Ctgy");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.ctgy_dtls {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CtgyDtls");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.dbtr_sts {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/DbtrSts");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cert_id {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CertId");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.frms_cd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/FrmsCd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prd {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Prd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.tax_amt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/TaxAmt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.addtl_inf {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/AddtlInf");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
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
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Prd");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.amt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Amt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::IsoMessage for Document {
    fn message_type(&self) -> &'static str {
        "pacs.002.001.12"
    }
    fn root_path(&self) -> &'static str {
        "/Document"
    }
}
