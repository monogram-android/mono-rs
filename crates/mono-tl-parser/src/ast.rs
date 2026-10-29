//! Public syntax tree.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt;

/// A half-open byte range in the original UTF-8 source.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span {
    /// First byte belonging to the item.
    pub start: usize,
    /// First byte after the item.
    pub end: usize,
}

/// Whether a combinator is a constructor or an RPC function.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Section {
    /// Constructor/type declarations.
    Types,
    /// RPC/function declarations.
    Functions,
}

/// Parsed TL source.
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Schema {
    /// Declarations in source order.
    pub declarations: Vec<Declaration>,
    /// Layer found in a `LAYER` source comment.
    pub layer: Option<u32>,
}

/// A top-level TL declaration.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Declaration {
    /// Active source section.
    pub section: Section,
    /// Immediately preceding comments.
    pub comments: Vec<String>,
    /// Declaration syntax.
    pub kind: DeclarationKind,
    /// Location in the source.
    pub span: Span,
}

/// All declaration forms in the formal TL grammar.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DeclarationKind {
    /// Constructor, function, or built-in declaration.
    Combinator(Combinator),
    /// Template/partial application.
    PartialApplication(TypeExpr),
    /// Type finalization declaration.
    Finalization {
        /// Finalization behavior.
        mode: Finalization,
        /// Affected type.
        ty: TypeExpr,
    },
}

/// Type finalization mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Finalization {
    /// No constructors before this point.
    New,
    /// No constructors after this point.
    Final,
    /// No constructors anywhere.
    Empty,
}

/// A combinator declaration.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Combinator {
    /// Namespace-qualified TL name.
    pub name: String,
    /// Explicit 32-bit number.
    pub id: Option<u32>,
    /// Fields in wire order.
    pub arguments: Vec<Argument>,
    /// Result type expression.
    pub result: TypeExpr,
    /// Built-in pseudo-declaration marker.
    pub builtin: bool,
}

/// A combinator argument.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Argument {
    /// Field name, if named.
    pub name: Option<String>,
    /// The value carried by this argument.
    pub value: ArgumentValue,
    /// Whether the field is implicit/braced.
    pub optional: bool,
    /// Whether the field is functional.
    pub bang: bool,
    /// Optional flags/natural guard.
    pub condition: Option<Condition>,
}

/// Ordinary and repeated argument forms are distinct in TL syntax.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum ArgumentValue {
    /// One value described by a type expression.
    Type(TypeExpr),
    /// A bracketed group of repeated fields.
    Repetition(Repetition),
}

/// Conditional argument guard.
#[derive(Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Condition {
    /// Guard variable.
    pub variable: String,
    /// Guard bit, when specified.
    pub bit: Option<u32>,
}

/// Repeated bracketed argument group.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Repetition {
    /// Explicit count, or `None` for the preceding natural field.
    pub multiplicity: Option<TypeExpr>,
    /// Fields in each repeated element.
    pub arguments: Vec<Argument>,
}

/// A TL type or natural-number expression.
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TypeExpr {
    /// Identifier or variable.
    Ident(String),
    /// Natural constant.
    Nat(u64),
    /// Natural-number type.
    Hash,
    /// Type or natural application.
    Apply {
        /// Applied expression.
        constructor: Box<TypeExpr>,
        /// Application operands.
        arguments: Vec<TypeExpr>,
    },
    /// Natural-number addition.
    Add {
        /// Left operand.
        left: Box<TypeExpr>,
        /// Right operand.
        right: Box<TypeExpr>,
    },
    /// Bare type.
    Bare(Box<TypeExpr>),
    /// Functional type.
    Bang(Box<TypeExpr>),
}

impl fmt::Display for TypeExpr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ident(v) => f.write_str(v),
            Self::Nat(v) => write!(f, "{v}"),
            Self::Hash => f.write_str("#"),
            Self::Add { left, right } => write!(f, "({left} + {right})"),
            Self::Bare(v) => write!(f, "%{v}"),
            Self::Bang(v) => write!(f, "!{v}"),
            Self::Apply {
                constructor,
                arguments,
            } => {
                write!(f, "{constructor}<")?;
                for (index, argument) in arguments.iter().enumerate() {
                    if index > 0 {
                        f.write_str(", ")?;
                    }
                    write!(f, "{argument}")?;
                }
                f.write_str(">")
            }
        }
    }
}
