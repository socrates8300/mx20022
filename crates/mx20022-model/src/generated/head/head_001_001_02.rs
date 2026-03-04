/*! Generated from ISO 20022 XSD schema.
 Namespace: `urn:iso:std:iso:20022:tech:xsd:head.001.001.02`*/
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
#[serde(transparent)]
pub struct BusinessMessagePriorityCode(pub String);
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
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODate(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct ISODateTime(pub String);
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
#[serde(transparent)]
pub struct UnicodeChartsCode(pub String);
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct YesNoIndicator(pub bool);
#[allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AddressType3Choice {
    #[serde(rename = "Cd")]
    Cd(AddressType2Code),
    #[serde(rename = "Prtry")]
    Prtry(GenericIdentification30),
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
pub struct BusinessApplicationHeader5 {
    #[serde(rename = "CharSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_set: Option<UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    pub fr: crate::common::ChoiceWrapper<Party44Choice>,
    #[serde(rename = "To")]
    pub to: crate::common::ChoiceWrapper<Party44Choice>,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: Max35Text,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: Max35Text,
    #[serde(rename = "BizSvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<Max35Text>,
    #[serde(rename = "CreDt")]
    pub cre_dt: ISODateTime,
    #[serde(rename = "CpyDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<YesNoIndicator>,
    #[serde(rename = "Prty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prty: Option<BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<SignatureEnvelope>,
}
/// Builder for [`BusinessApplicationHeader5`]. Construct via [`BusinessApplicationHeader5::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BusinessApplicationHeader5Builder {
    char_set: ::std::option::Option<UnicodeChartsCode>,
    fr: ::std::option::Option<crate::common::ChoiceWrapper<Party44Choice>>,
    to: ::std::option::Option<crate::common::ChoiceWrapper<Party44Choice>>,
    biz_msg_idr: ::std::option::Option<Max35Text>,
    msg_def_idr: ::std::option::Option<Max35Text>,
    biz_svc: ::std::option::Option<Max35Text>,
    cre_dt: ::std::option::Option<ISODateTime>,
    cpy_dplct: ::std::option::Option<CopyDuplicate1Code>,
    pssbl_dplct: ::std::option::Option<YesNoIndicator>,
    prty: ::std::option::Option<BusinessMessagePriorityCode>,
    sgntr: ::std::option::Option<SignatureEnvelope>,
}
impl BusinessApplicationHeader5Builder {
    /// Set the `char_set` field.
    #[must_use]
    pub fn char_set(
        mut self,
        value: UnicodeChartsCode,
    ) -> BusinessApplicationHeader5Builder {
        self.char_set = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr` field.
    #[must_use]
    pub fn fr(
        mut self,
        value: crate::common::ChoiceWrapper<Party44Choice>,
    ) -> BusinessApplicationHeader5Builder {
        self.fr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to` field.
    #[must_use]
    pub fn to(
        mut self,
        value: crate::common::ChoiceWrapper<Party44Choice>,
    ) -> BusinessApplicationHeader5Builder {
        self.to = ::std::option::Option::Some(value);
        self
    }
    /// Set the `biz_msg_idr` field.
    #[must_use]
    pub fn biz_msg_idr(mut self, value: Max35Text) -> BusinessApplicationHeader5Builder {
        self.biz_msg_idr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_def_idr` field.
    #[must_use]
    pub fn msg_def_idr(mut self, value: Max35Text) -> BusinessApplicationHeader5Builder {
        self.msg_def_idr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `biz_svc` field.
    #[must_use]
    pub fn biz_svc(mut self, value: Max35Text) -> BusinessApplicationHeader5Builder {
        self.biz_svc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt` field.
    #[must_use]
    pub fn cre_dt(mut self, value: ISODateTime) -> BusinessApplicationHeader5Builder {
        self.cre_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cpy_dplct` field.
    #[must_use]
    pub fn cpy_dplct(
        mut self,
        value: CopyDuplicate1Code,
    ) -> BusinessApplicationHeader5Builder {
        self.cpy_dplct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pssbl_dplct` field.
    #[must_use]
    pub fn pssbl_dplct(
        mut self,
        value: YesNoIndicator,
    ) -> BusinessApplicationHeader5Builder {
        self.pssbl_dplct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prty` field.
    #[must_use]
    pub fn prty(
        mut self,
        value: BusinessMessagePriorityCode,
    ) -> BusinessApplicationHeader5Builder {
        self.prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sgntr` field.
    #[must_use]
    pub fn sgntr(
        mut self,
        value: SignatureEnvelope,
    ) -> BusinessApplicationHeader5Builder {
        self.sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
    ) -> ::std::result::Result<BusinessApplicationHeader5, crate::common::BuilderError> {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr.is_none() {
            missing.push("fr".to_owned());
        }
        if self.to.is_none() {
            missing.push("to".to_owned());
        }
        if self.biz_msg_idr.is_none() {
            missing.push("biz_msg_idr".to_owned());
        }
        if self.msg_def_idr.is_none() {
            missing.push("msg_def_idr".to_owned());
        }
        if self.cre_dt.is_none() {
            missing.push("cre_dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BusinessApplicationHeader5".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BusinessApplicationHeader5 {
            char_set: self.char_set,
            fr: self.fr.unwrap(),
            to: self.to.unwrap(),
            biz_msg_idr: self.biz_msg_idr.unwrap(),
            msg_def_idr: self.msg_def_idr.unwrap(),
            biz_svc: self.biz_svc,
            cre_dt: self.cre_dt.unwrap(),
            cpy_dplct: self.cpy_dplct,
            pssbl_dplct: self.pssbl_dplct,
            prty: self.prty,
            sgntr: self.sgntr,
        })
    }
}
impl BusinessApplicationHeader5 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BusinessApplicationHeader5Builder {
        BusinessApplicationHeader5Builder::default()
    }
}
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BusinessApplicationHeaderV02 {
    #[serde(rename = "CharSet")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub char_set: Option<UnicodeChartsCode>,
    #[serde(rename = "Fr")]
    pub fr: crate::common::ChoiceWrapper<Party44Choice>,
    #[serde(rename = "To")]
    pub to: crate::common::ChoiceWrapper<Party44Choice>,
    #[serde(rename = "BizMsgIdr")]
    pub biz_msg_idr: Max35Text,
    #[serde(rename = "MsgDefIdr")]
    pub msg_def_idr: Max35Text,
    #[serde(rename = "BizSvc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_svc: Option<Max35Text>,
    #[serde(rename = "MktPrctc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mkt_prctc: Option<ImplementationSpecification1>,
    #[serde(rename = "CreDt")]
    pub cre_dt: ISODateTime,
    #[serde(rename = "BizPrcgDt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biz_prcg_dt: Option<ISODateTime>,
    #[serde(rename = "CpyDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpy_dplct: Option<CopyDuplicate1Code>,
    #[serde(rename = "PssblDplct")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pssbl_dplct: Option<YesNoIndicator>,
    #[serde(rename = "Prty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prty: Option<BusinessMessagePriorityCode>,
    #[serde(rename = "Sgntr")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgntr: Option<SignatureEnvelope>,
    #[serde(rename = "Rltd")]
    #[serde(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub rltd: Vec<BusinessApplicationHeader5>,
}
/// Builder for [`BusinessApplicationHeaderV02`]. Construct via [`BusinessApplicationHeaderV02::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct BusinessApplicationHeaderV02Builder {
    char_set: ::std::option::Option<UnicodeChartsCode>,
    fr: ::std::option::Option<crate::common::ChoiceWrapper<Party44Choice>>,
    to: ::std::option::Option<crate::common::ChoiceWrapper<Party44Choice>>,
    biz_msg_idr: ::std::option::Option<Max35Text>,
    msg_def_idr: ::std::option::Option<Max35Text>,
    biz_svc: ::std::option::Option<Max35Text>,
    mkt_prctc: ::std::option::Option<ImplementationSpecification1>,
    cre_dt: ::std::option::Option<ISODateTime>,
    biz_prcg_dt: ::std::option::Option<ISODateTime>,
    cpy_dplct: ::std::option::Option<CopyDuplicate1Code>,
    pssbl_dplct: ::std::option::Option<YesNoIndicator>,
    prty: ::std::option::Option<BusinessMessagePriorityCode>,
    sgntr: ::std::option::Option<SignatureEnvelope>,
    rltd: ::std::vec::Vec<BusinessApplicationHeader5>,
}
impl BusinessApplicationHeaderV02Builder {
    /// Set the `char_set` field.
    #[must_use]
    pub fn char_set(
        mut self,
        value: UnicodeChartsCode,
    ) -> BusinessApplicationHeaderV02Builder {
        self.char_set = ::std::option::Option::Some(value);
        self
    }
    /// Set the `fr` field.
    #[must_use]
    pub fn fr(
        mut self,
        value: crate::common::ChoiceWrapper<Party44Choice>,
    ) -> BusinessApplicationHeaderV02Builder {
        self.fr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `to` field.
    #[must_use]
    pub fn to(
        mut self,
        value: crate::common::ChoiceWrapper<Party44Choice>,
    ) -> BusinessApplicationHeaderV02Builder {
        self.to = ::std::option::Option::Some(value);
        self
    }
    /// Set the `biz_msg_idr` field.
    #[must_use]
    pub fn biz_msg_idr(
        mut self,
        value: Max35Text,
    ) -> BusinessApplicationHeaderV02Builder {
        self.biz_msg_idr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `msg_def_idr` field.
    #[must_use]
    pub fn msg_def_idr(
        mut self,
        value: Max35Text,
    ) -> BusinessApplicationHeaderV02Builder {
        self.msg_def_idr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `biz_svc` field.
    #[must_use]
    pub fn biz_svc(mut self, value: Max35Text) -> BusinessApplicationHeaderV02Builder {
        self.biz_svc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `mkt_prctc` field.
    #[must_use]
    pub fn mkt_prctc(
        mut self,
        value: ImplementationSpecification1,
    ) -> BusinessApplicationHeaderV02Builder {
        self.mkt_prctc = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cre_dt` field.
    #[must_use]
    pub fn cre_dt(mut self, value: ISODateTime) -> BusinessApplicationHeaderV02Builder {
        self.cre_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `biz_prcg_dt` field.
    #[must_use]
    pub fn biz_prcg_dt(
        mut self,
        value: ISODateTime,
    ) -> BusinessApplicationHeaderV02Builder {
        self.biz_prcg_dt = ::std::option::Option::Some(value);
        self
    }
    /// Set the `cpy_dplct` field.
    #[must_use]
    pub fn cpy_dplct(
        mut self,
        value: CopyDuplicate1Code,
    ) -> BusinessApplicationHeaderV02Builder {
        self.cpy_dplct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `pssbl_dplct` field.
    #[must_use]
    pub fn pssbl_dplct(
        mut self,
        value: YesNoIndicator,
    ) -> BusinessApplicationHeaderV02Builder {
        self.pssbl_dplct = ::std::option::Option::Some(value);
        self
    }
    /// Set the `prty` field.
    #[must_use]
    pub fn prty(
        mut self,
        value: BusinessMessagePriorityCode,
    ) -> BusinessApplicationHeaderV02Builder {
        self.prty = ::std::option::Option::Some(value);
        self
    }
    /// Set the `sgntr` field.
    #[must_use]
    pub fn sgntr(
        mut self,
        value: SignatureEnvelope,
    ) -> BusinessApplicationHeaderV02Builder {
        self.sgntr = ::std::option::Option::Some(value);
        self
    }
    /// Set the `rltd` field (replaces any previously added items).
    #[must_use]
    pub fn rltd(
        mut self,
        value: ::std::vec::Vec<BusinessApplicationHeader5>,
    ) -> BusinessApplicationHeaderV02Builder {
        self.rltd = value;
        self
    }
    /// Append one item to the `rltd` field.
    #[must_use]
    pub fn add_rltd(
        mut self,
        value: BusinessApplicationHeader5,
    ) -> BusinessApplicationHeaderV02Builder {
        self.rltd.push(value);
        self
    }
    /// Validate required fields and construct the type.
    ///
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
        BusinessApplicationHeaderV02,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.fr.is_none() {
            missing.push("fr".to_owned());
        }
        if self.to.is_none() {
            missing.push("to".to_owned());
        }
        if self.biz_msg_idr.is_none() {
            missing.push("biz_msg_idr".to_owned());
        }
        if self.msg_def_idr.is_none() {
            missing.push("msg_def_idr".to_owned());
        }
        if self.cre_dt.is_none() {
            missing.push("cre_dt".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "BusinessApplicationHeaderV02".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(BusinessApplicationHeaderV02 {
            char_set: self.char_set,
            fr: self.fr.unwrap(),
            to: self.to.unwrap(),
            biz_msg_idr: self.biz_msg_idr.unwrap(),
            msg_def_idr: self.msg_def_idr.unwrap(),
            biz_svc: self.biz_svc,
            mkt_prctc: self.mkt_prctc,
            cre_dt: self.cre_dt.unwrap(),
            biz_prcg_dt: self.biz_prcg_dt,
            cpy_dplct: self.cpy_dplct,
            pssbl_dplct: self.pssbl_dplct,
            prty: self.prty,
            sgntr: self.sgntr,
            rltd: self.rltd,
        })
    }
}
impl BusinessApplicationHeaderV02 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> BusinessApplicationHeaderV02Builder {
        BusinessApplicationHeaderV02Builder::default()
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
pub struct ImplementationSpecification1 {
    #[serde(rename = "Regy")]
    pub regy: Max350Text,
    #[serde(rename = "Id")]
    pub id: Max2048Text,
}
/// Builder for [`ImplementationSpecification1`]. Construct via [`ImplementationSpecification1::builder()`].
#[allow(clippy::struct_field_names)]
#[derive(Default)]
pub struct ImplementationSpecification1Builder {
    regy: ::std::option::Option<Max350Text>,
    id: ::std::option::Option<Max2048Text>,
}
impl ImplementationSpecification1Builder {
    /// Set the `regy` field.
    #[must_use]
    pub fn regy(mut self, value: Max350Text) -> ImplementationSpecification1Builder {
        self.regy = ::std::option::Option::Some(value);
        self
    }
    /// Set the `id` field.
    #[must_use]
    pub fn id(mut self, value: Max2048Text) -> ImplementationSpecification1Builder {
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
        ImplementationSpecification1,
        crate::common::BuilderError,
    > {
        let mut missing: ::std::vec::Vec<::std::string::String> = ::std::vec::Vec::new();
        if self.regy.is_none() {
            missing.push("regy".to_owned());
        }
        if self.id.is_none() {
            missing.push("id".to_owned());
        }
        if !missing.is_empty() {
            return ::std::result::Result::Err(crate::common::BuilderError {
                type_name: "ImplementationSpecification1".to_owned(),
                missing_fields: missing,
            });
        }
        ::std::result::Result::Ok(ImplementationSpecification1 {
            regy: self.regy.unwrap(),
            id: self.id.unwrap(),
        })
    }
}
impl ImplementationSpecification1 {
    /// Return a new builder for this type.
    #[must_use]
    pub fn builder() -> ImplementationSpecification1Builder {
        ImplementationSpecification1Builder::default()
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
pub enum Party44Choice {
    #[serde(rename = "OrgId")]
    OrgId(PartyIdentification135),
    #[serde(rename = "FIId")]
    FIId(BranchAndFinancialInstitutionIdentification6),
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
/// Accepts content from namespace: `http://www.w3.org/2000/09/xmldsig#`
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SignatureEnvelope {
    #[serde(rename = "$value")]
    pub value: String,
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
impl crate::common::validate::Validatable for BusinessMessagePriorityCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for CopyDuplicate1Code {
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
impl crate::common::validate::Validatable for UnicodeChartsCode {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::Validatable for YesNoIndicator {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
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
impl crate::common::validate::Validatable for BusinessApplicationHeader5 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.char_set {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CharSet");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.fr.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Fr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.to.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/To");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.biz_msg_idr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BizMsgIdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.msg_def_idr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MsgDefIdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.biz_svc {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BizSvc");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.cre_dt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CreDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cpy_dplct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CpyDplct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pssbl_dplct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PssblDplct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prty {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Prty");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Sgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
    }
}
impl crate::common::validate::Validatable for BusinessApplicationHeaderV02 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        if let Some(ref val) = self.char_set {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CharSet");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.fr.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Fr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.to.inner.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/To");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.biz_msg_idr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BizMsgIdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.msg_def_idr.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MsgDefIdr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.biz_svc {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BizSvc");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.mkt_prctc {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/MktPrctc");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        {
            let snap = violations.len();
            self.cre_dt.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CreDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.biz_prcg_dt {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/BizPrcgDt");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.cpy_dplct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/CpyDplct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.pssbl_dplct {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/PssblDplct");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.prty {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Prty");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        if let Some(ref val) = self.sgntr {
            let snap = violations.len();
            val.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Sgntr");
                for v in &mut violations[snap..] {
                    v.path.insert_str(0, &pfx);
                }
            }
        }
        for (idx, elem) in self.rltd.iter().enumerate() {
            let snap = violations.len();
            elem.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Rltd[{idx}]");
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
impl crate::common::validate::Validatable for ImplementationSpecification1 {
    fn validate_constraints(
        &self,
        path: &str,
        violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {
        {
            let snap = violations.len();
            self.regy.validate_constraints("", violations);
            if violations.len() > snap {
                let pfx = format!("{path}/Regy");
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
impl crate::common::validate::Validatable for Party44Choice {
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
            Self::FIId(inner) => {
                let snap = violations.len();
                inner.validate_constraints("", violations);
                if violations.len() > snap {
                    let pfx = format!("{path}/FIId");
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
impl crate::common::validate::Validatable for SignatureEnvelope {
    fn validate_constraints(
        &self,
        _path: &str,
        _violations: &mut Vec<crate::common::validate::ConstraintViolation>,
    ) {}
}
impl crate::common::validate::IsoMessage for BusinessApplicationHeaderV02 {
    fn message_type(&self) -> &'static str {
        "head.001.001.02"
    }
    fn root_path(&self) -> &'static str {
        "/AppHdr"
    }
}
