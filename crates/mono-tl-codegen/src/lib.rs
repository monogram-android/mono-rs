//! Rust source generation from [`mono_tl_schema::Schema`].
//!
//! The generator resolves the model's expression tree. It does not recognize
//! schema-defined types by name: constructors, applications, repetitions, and
//! generic parameters are all derived from the input schema. The only named
//! mappings are caller-selectable target representations for primitive wire types.

mod documentation;
mod emitter;
mod error;
mod naming;
mod options;
mod resolver;

pub use emitter::generate;
pub use error::GenerateError;
pub use options::{Options, telegram_type_mappings};
