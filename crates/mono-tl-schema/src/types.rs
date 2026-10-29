//! Stable interchange data types and errors.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use thiserror::Error;

/// Version of the JSON interchange contract.
pub const FORMAT_VERSION: u32 = 1;

/// A complete schema suitable for generators in any language.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Schema {
    /// Interchange format version (independent of Telegram's layer).
    pub format_version: u32,
    /// Telegram API layer advertised by the source.
    pub layer: Option<u32>,
    /// Provenance supplied by the caller.
    pub source: Source,
    /// Constructors, in source order.
    pub constructors: Vec<Combinator>,
    /// RPC methods, in source order.
    pub functions: Vec<Combinator>,
    /// Finalization declarations.
    pub finalizations: Vec<Finalization>,
    /// Template/partial application declarations.
    pub partial_applications: Vec<Expression>,
}

/// Input provenance.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Source {
    /// Human-readable file or URL.
    pub name: String,
    /// Upstream URL, when known.
    pub url: Option<String>,
}

/// A constructor or function.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Combinator {
    /// Namespace-qualified TL name.
    pub name: String,
    /// Resolved constructor number (explicit or canonical CRC32).
    pub id: u32,
    /// Hex form convenient for languages with awkward unsigned integers.
    pub id_hex: String,
    /// Whether the source supplied the number instead of relying on canonical CRC32.
    pub id_explicit: bool,
    /// Constructor or function.
    pub kind: CombinatorKind,
    /// Ordered arguments.
    pub arguments: Vec<Argument>,
    /// Result expression.
    pub result: Expression,
    /// Structured documentation.
    pub documentation: Documentation,
    /// Layer known for this declaration. Current flat schemas only identify the
    /// containing layer and cannot prove an earlier introduction layer.
    pub layer: LayerInfo,
    /// Built-in pseudo-declaration.
    pub builtin: bool,
}

/// Kind of combinator.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CombinatorKind {
    /// Data constructor.
    Constructor,
    /// RPC function.
    Function,
}

/// Layer metadata with explicit semantics.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct LayerInfo {
    /// Layer of the containing schema.
    pub schema: Option<u32>,
    /// First layer, only when the input explicitly provides it.
    pub introduced: Option<u32>,
}

/// One argument.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Argument {
    /// TL field name.
    pub name: Option<String>,
    /// Ordinary value or repeated field group.
    pub value: ArgumentValue,
    /// Implicit/braced generic argument.
    pub implicit: bool,
    /// Functional/exclamation argument.
    pub functional: bool,
    /// Optional bit guard.
    pub condition: Option<Condition>,
    /// Documentation derived from `@param`.
    pub description: Option<String>,
}

/// The mutually exclusive forms of a combinator argument.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ArgumentValue {
    /// One value with a TL type expression.
    Type {
        /// Parsed expression; never a schema-name placeholder.
        expression: Expression,
    },
    /// A bracketed group repeated according to a natural expression.
    Repetition {
        /// Count expression, or the preceding natural argument when omitted.
        multiplicity: Option<Expression>,
        /// Fields in one repeated element.
        arguments: Vec<Argument>,
    },
}

/// Optional-field condition.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Condition {
    /// Flags/natural argument name.
    pub variable: String,
    /// Optional bit number.
    pub bit: Option<u32>,
}

/// Language-neutral expression tree.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum Expression {
    /// An identifier.
    Ident {
        /// Namespace-qualified TL spelling.
        name: String,
    },
    /// Natural constant.
    Nat {
        /// Constant value.
        value: u64,
    },
    /// The natural-number type `#`.
    Hash,
    /// Natural addition supported by the TL formal grammar.
    Add {
        /// Left operand.
        left: Box<Expression>,
        /// Right operand.
        right: Box<Expression>,
    },
    /// Applied type expression.
    Apply {
        /// Applied constructor/type.
        constructor: Box<Expression>,
        /// Application arguments.
        arguments: Vec<Expression>,
    },
    /// Bare type expression.
    Bare {
        /// Wrapped expression.
        inner: Box<Expression>,
    },
    /// Functional type expression.
    Bang {
        /// Wrapped expression.
        inner: Box<Expression>,
    },
}

/// Documentation and cross-language official links.
#[derive(Clone, Debug, Default, Serialize, Deserialize, JsonSchema)]
pub struct Documentation {
    /// Prose description.
    pub description: Option<String>,
    /// Named parameter descriptions.
    pub parameters: BTreeMap<String, String>,
    /// Official Telegram documentation page inferred from the declaration kind.
    pub official_url: Option<String>,
    /// Other URLs found in comments.
    pub links: Vec<String>,
}

/// `New`, `Final`, or `Empty` type declaration.
#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema)]
pub struct Finalization {
    /// Lowercase finalization mode.
    pub mode: String,
    /// Affected type.
    pub r#type: Expression,
}

/// Semantic validation failure.
#[derive(Debug, Error)]
pub enum ModelError {
    /// Parser failure.
    #[error(transparent)]
    Parse(#[from] mono_tl_parser::ParseError),
    /// A name or ID occurs more than once in the same namespace/kind.
    #[error("Duplicate {kind} {value}")]
    Duplicate {
        /// Duplicate category.
        kind: &'static str,
        /// Conflicting name/ID.
        value: String,
    },
    /// A declaration violates TL's dependent-type scoping rules.
    #[error("Invalid declaration {declaration}: {message}")]
    InvalidDeclaration {
        /// Namespace-qualified declaration name.
        declaration: String,
        /// Explanation of the violated rule.
        message: String,
    },
}
