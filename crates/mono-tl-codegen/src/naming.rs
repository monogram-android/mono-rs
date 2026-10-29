//! Deterministic TL-to-Rust identifier conversion.

use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use mono_tl_schema::{Combinator, CombinatorKind};
use std::fmt::Write;

pub(crate) fn type_name(name: &str) -> String {
    let converted = name.replace('.', "_").to_pascal_case();
    if converted.is_empty() {
        name.as_bytes()
            .iter()
            .fold("Symbol".into(), |mut output, byte| {
                write!(output, "{byte:02X}").expect("writing to String cannot fail");
                output
            })
    } else {
        converted
    }
}

pub(crate) fn item_name(combinator: &Combinator) -> String {
    let suffix = match combinator.kind {
        CombinatorKind::Constructor => "Constructor",
        CombinatorKind::Function => "Request",
    };
    format!("{}{suffix}", type_name(&combinator.name))
}

pub(crate) fn field_name(name: &str) -> String {
    let converted = name.replace('.', "_").to_snake_case();
    if RUST_KEYWORDS.contains(&converted.as_str()) {
        format!("{converted}_")
    } else if converted.is_empty() {
        "unnamed".into()
    } else {
        converted
    }
}

pub(crate) fn constant_name(name: &str) -> String {
    let converted = name.replace('.', "_").to_shouty_snake_case();
    if converted.is_empty() {
        "UNNAMED".into()
    } else {
        converted
    }
}

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro",
    "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
];
