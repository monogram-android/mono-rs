//! Caller-controlled target-language policy.

use std::collections::BTreeMap;

/// Rust representations for primitive types predefined by Telegram's TL wire format.
///
/// This policy is deliberately separate from expression resolution. Callers
/// targeting another ABI can edit, extend, or replace the returned map. Types
/// declared by a schema are discovered from their result expressions instead.
#[must_use]
pub fn telegram_type_mappings() -> BTreeMap<String, String> {
    [
        ("#", "u32"),
        ("int", "i32"),
        ("Int", "i32"),
        ("long", "i64"),
        ("Long", "i64"),
        ("double", "f64"),
        ("Double", "f64"),
        ("int128", "bnum::types::I128"),
        ("Int128", "bnum::types::I128"),
        ("int256", "bnum::types::I256"),
        ("Int256", "bnum::types::I256"),
        ("int512", "bnum::types::I512"),
        ("Int512", "bnum::types::I512"),
        ("string", "String"),
        ("String", "String"),
        ("bytes", "Vec<u8>"),
        ("Bytes", "Vec<u8>"),
    ]
    .into_iter()
    .map(|(tl, rust)| (tl.to_owned(), rust.to_owned()))
    .collect()
}

/// Code generation options.
#[derive(Clone, Debug)]
pub struct Options {
    /// Module-level documentation line.
    pub module_documentation: String,
    /// Rust representations for formally predefined or application-provided
    /// external TL types. Schema-defined aggregate types never require entries.
    pub type_mappings: BTreeMap<String, String>,
    /// Rust items (typically imports) emitted before generated declarations.
    pub prelude: Vec<String>,
    /// Emit implementations against `crate::codec` for generated declarations.
    /// Enable this when generating the canonical `mono-proto` crate.
    pub emit_codec: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            module_documentation: "Generated Telegram TL declarations.".into(),
            type_mappings: telegram_type_mappings(),
            prelude: Vec::new(),
            emit_codec: false,
        }
    }
}
