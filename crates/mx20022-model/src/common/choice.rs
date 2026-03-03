//! Wrapper type enabling `xs:choice` fields to work with `quick-xml` + serde.
//!
//! `quick-xml` requires that enum types used as struct field values are wrapped
//! in a struct with a `#[serde(rename = "$value")]` field.  Without the wrapper,
//! serialization of newtype enum variants fails with
//! `"cannot serialize enum newtype variant"`, and deserialization fails with
//! `"UnexpectedStart"`.
//!
//! ## XML mapping
//!
//! For an ISO 20022 struct like:
//!
//! ```xml
//! <Fr>
//!   <FIId>...</FIId>   <!-- xs:choice discriminator + content -->
//! </Fr>
//! ```
//!
//! The Rust model is:
//!
//! ```rust
//! use mx20022_model::common::ChoiceWrapper;
//!
//! #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
//! pub enum Party51Choice {
//!     #[serde(rename = "FIId")]
//!     FIId(String),           // simplified
//!     #[serde(rename = "OrgId")]
//!     OrgId(String),          // simplified
//! }
//!
//! #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
//! pub struct BusinessApplicationHeader {
//!     #[serde(rename = "Fr")]
//!     pub fr: ChoiceWrapper<Party51Choice>,
//! }
//! ```

/// A transparent wrapper that places an `xs:choice` enum as the `$value`
/// (element content) of its parent XML element.
///
/// `quick-xml` only supports serializing and deserializing enum newtype
/// variants when they appear as element content via a `$value`-renamed field.
/// This struct is that one-field wrapper.
///
/// # Access
///
/// Use the public `.inner` field to access the wrapped enum, or use
/// `ChoiceWrapper::new(value)` / `From<T>` to construct one.
///
/// ```
/// use mx20022_model::common::ChoiceWrapper;
///
/// #[derive(Debug, PartialEq)]
/// enum MyChoice { A(String), B(u32) }
///
/// let w = ChoiceWrapper::new(MyChoice::A("hello".to_owned()));
/// // Access inner value
/// match &w.inner {
///     MyChoice::A(s) => assert_eq!(s, "hello"),
///     MyChoice::B(_) => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ChoiceWrapper<T> {
    /// The wrapped xs:choice enum value.
    #[serde(rename = "$value")]
    pub inner: T,
}

impl<T> ChoiceWrapper<T> {
    /// Wrap a value.
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> From<T> for ChoiceWrapper<T> {
    fn from(inner: T) -> Self {
        Self { inner }
    }
}

impl<T> std::ops::Deref for ChoiceWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> std::ops::DerefMut for ChoiceWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq)]
    enum TestChoice {
        TypeA(String),
        TypeB(u32),
    }

    #[test]
    fn deref_reaches_inner() {
        let w = ChoiceWrapper::new(TestChoice::TypeA("x".to_owned()));
        match &*w {
            TestChoice::TypeA(s) => assert_eq!(s, "x"),
            TestChoice::TypeB(_) => panic!("wrong variant"),
        }
    }

    #[test]
    fn from_impl() {
        let w: ChoiceWrapper<TestChoice> = TestChoice::TypeB(7).into();
        assert_eq!(w.inner, TestChoice::TypeB(7));
    }

    #[test]
    fn deref_mut_updates_inner() {
        let mut w = ChoiceWrapper::new(TestChoice::TypeA("before".to_owned()));
        *w = TestChoice::TypeA("after".to_owned());
        match &w.inner {
            TestChoice::TypeA(s) => assert_eq!(s, "after"),
            TestChoice::TypeB(_) => panic!("wrong variant"),
        }
    }
}
