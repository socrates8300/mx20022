//! Hand-written shared types used across generated message modules.

mod builder;
mod choice;
pub mod validate;

pub use builder::BuilderError;
pub use choice::ChoiceWrapper;
