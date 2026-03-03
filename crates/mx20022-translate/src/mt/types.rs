//! Core types representing the structure of a parsed SWIFT MT message.

/// A complete parsed SWIFT MT message composed of up to five blocks.
///
/// # Example
///
/// ```rust
/// use mx20022_translate::mt::parser::parse;
///
/// let raw = "{1:F01BANKBEBBAXXX0000000000}{2:I103BANKDEFFXXXXN}{3:}{4:\n:20:REF\n:23B:CRED\n:32A:230615EUR1000,00\n:50K:ACME\n:59:SMITH\n:71A:SHA\n-}{5:{CHK:ABC123}}";
/// let msg = parse(raw).unwrap();
/// assert_eq!(msg.message_type(), Some("103"));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct MtMessage {
    /// Block 1: Basic Header (required by spec but optional here for robustness).
    pub block1: Option<Block1>,
    /// Block 2: Application Header.
    pub block2: Option<Block2>,
    /// Block 3: User Header (tag/value pairs).
    pub block3: Option<Block3>,
    /// Block 4: Text / message body.
    pub block4: Block4,
    /// Block 5: Trailer (tag/value pairs).
    pub block5: Option<Block5>,
}

impl MtMessage {
    /// Returns the MT message type string (e.g., `"103"`, `"202"`, `"940"`)
    /// derived from Block 2.
    pub fn message_type(&self) -> Option<&str> {
        match &self.block2 {
            Some(Block2::Input(b)) => Some(b.message_type.as_str()),
            Some(Block2::Output(b)) => Some(b.message_type.as_str()),
            None => None,
        }
    }
}

/// Block 1: Basic Header Block.
///
/// Format: `{1:F01BANKBEBBAXXX0000000000}`
#[derive(Debug, Clone, PartialEq)]
pub struct Block1 {
    /// Application ID: `F` (FIN), `A` (GPA), or `L` (GPA).
    pub app_id: char,
    /// Service ID — two characters, e.g. `"01"`.
    pub service_id: String,
    /// Logical Terminal address — 12 characters (BIC8 + logical terminal + branch).
    pub lt_address: String,
    /// Session number — 4 digits.
    pub session_number: String,
    /// Sequence number — 6 digits.
    pub sequence_number: String,
}

/// Block 2: Application Header Block.
///
/// Two variants depending on whether the message is being sent (Input) or
/// has been received (Output).
#[derive(Debug, Clone, PartialEq)]
pub enum Block2 {
    /// Message sent by the user to SWIFT.
    Input(Block2Input),
    /// Message received from SWIFT.
    Output(Block2Output),
}

/// Block 2 Input variant.
///
/// Format: `I{message_type}{destination}{priority}`
#[derive(Debug, Clone, PartialEq)]
pub struct Block2Input {
    /// Three-digit MT message type, e.g. `"103"`.
    pub message_type: String,
    /// 12-character destination LT address.
    pub destination: String,
    /// Message priority: `S` (System), `N` (Normal), `U` (Urgent).
    pub priority: Option<char>,
    /// Optional delivery monitoring code.
    pub delivery_monitoring: Option<char>,
    /// Optional obsolescence period.
    pub obsolescence_period: Option<String>,
}

/// Block 2 Output variant.
///
/// Format: `O{message_type}{input_time}{MIR}{output_date}{output_time}{priority}`
#[derive(Debug, Clone, PartialEq)]
pub struct Block2Output {
    /// Three-digit MT message type, e.g. `"103"`.
    pub message_type: String,
    /// Input time — four digits `HHMM`.
    pub input_time: String,
    /// MIR date component — six digits `YYMMDD`.
    pub input_date: String,
    /// Full 28-character MIR: `date(6) + LT(12) + session(4) + sequence(6)`.
    pub mir: String,
    /// Output date — six digits `YYMMDD`.
    pub output_date: String,
    /// Output time — four digits `HHMM`.
    pub output_time: String,
    /// Message priority.
    pub priority: Option<char>,
}

/// Block 3: User Header Block.
///
/// Stores tag/value pairs in insertion order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block3 {
    /// Ordered list of `(tag, value)` pairs.
    pub tags: Vec<(String, String)>,
}

impl Block3 {
    /// Returns the value of the first tag matching `tag`, or `None`.
    pub fn get(&self, tag: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(t, _)| t == tag)
            .map(|(_, v)| v.as_str())
    }
}

/// Block 4: Text Block — the message body.
#[derive(Debug, Clone, PartialEq)]
pub struct Block4 {
    /// Ordered list of tagged fields extracted from the text block.
    pub fields: Vec<TagField>,
}

impl Block4 {
    /// Returns the first field whose tag matches `tag`, or `None`.
    pub fn get(&self, tag: &str) -> Option<&TagField> {
        self.fields.iter().find(|f| f.tag == tag)
    }

    /// Returns all fields whose tag matches `tag`.
    pub fn get_all(&self, tag: &str) -> Vec<&TagField> {
        self.fields.iter().filter(|f| f.tag == tag).collect()
    }
}

/// A single tagged field inside Block 4.
#[derive(Debug, Clone, PartialEq)]
pub struct TagField {
    /// The field tag, e.g. `"20"`, `"32A"`, `"50K"`.
    pub tag: String,
    /// The field value, potentially spanning multiple lines (joined with `\n`).
    pub value: String,
}

/// Block 5: Trailer Block.
///
/// Stores tag/value pairs in insertion order.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Block5 {
    /// Ordered list of `(tag, value)` pairs.
    pub tags: Vec<(String, String)>,
}

impl Block5 {
    /// Returns the value of the first tag matching `tag`, or `None`.
    pub fn get(&self, tag: &str) -> Option<&str> {
        self.tags
            .iter()
            .find(|(t, _)| t == tag)
            .map(|(_, v)| v.as_str())
    }
}
