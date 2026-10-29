//! Syntax-tree conversion, validation, and documentation extraction.

use crate::{
    Argument, ArgumentValue, Combinator, CombinatorKind, Condition, Documentation, Expression,
    FORMAT_VERSION, Finalization, LayerInfo, ModelError, Schema, Source,
};
use mono_tl_parser::{DeclarationKind, Section, TypeExpr};
use std::collections::{BTreeMap, HashMap, HashSet};

/// Parse and validate source in one operation.
///
/// # Errors
///
/// Returns syntax errors and semantic conflicts such as duplicate IDs.
pub fn parse(source_text: &str, source: Source) -> Result<Schema, ModelError> {
    #[cfg(feature = "tracing")]
    tracing::debug!(
        source_length = source_text.len(),
        "Building validated TL schema"
    );
    build(&mono_tl_parser::parse(source_text)?, source)
}

/// Validate a syntax tree and build the interchange representation.
///
/// # Errors
///
/// Returns a semantic error when declarations conflict.
pub fn build(ast: &mono_tl_parser::Schema, source: Source) -> Result<Schema, ModelError> {
    #[cfg(feature = "tracing")]
    tracing::debug!(
        declaration_count = ast.declarations.len(),
        "Validating TL syntax tree"
    );
    crate::validation::validate(ast)?;
    let mut constructors = Vec::new();
    let mut functions = Vec::new();
    let mut finalizations = Vec::new();
    let mut partial_applications = Vec::new();
    let mut names = HashSet::new();
    let mut ids = HashMap::new();
    for declaration in &ast.declarations {
        match &declaration.kind {
            DeclarationKind::Combinator(c) => {
                let (id, id_explicit) = crate::identifier::resolve(c);
                let kind = if declaration.section == Section::Functions {
                    CombinatorKind::Function
                } else {
                    CombinatorKind::Constructor
                };
                let key = (declaration.section, c.name.clone());
                if !names.insert(key) {
                    return Err(ModelError::Duplicate {
                        kind: "name",
                        value: c.name.clone(),
                    });
                }
                if ids.insert(id, c.name.clone()).is_some() {
                    return Err(ModelError::Duplicate {
                        kind: "ID",
                        value: format!("0x{id:08x}"),
                    });
                }
                let documentation = documentation(&declaration.comments, &c.name, kind);
                let arguments = c
                    .arguments
                    .iter()
                    .map(|a| argument(a, &documentation.parameters))
                    .collect();
                let item = Combinator {
                    name: c.name.clone(),
                    id,
                    id_hex: format!("{id:08x}"),
                    id_explicit,
                    kind,
                    arguments,
                    result: expression(&c.result),
                    documentation,
                    layer: LayerInfo {
                        schema: ast.layer,
                        introduced: extract_introduced(&declaration.comments),
                    },
                    builtin: c.builtin,
                };
                match kind {
                    CombinatorKind::Constructor => constructors.push(item),
                    CombinatorKind::Function => functions.push(item),
                }
            }
            DeclarationKind::Finalization { mode, ty } => finalizations.push(Finalization {
                mode: format!("{mode:?}").to_ascii_lowercase(),
                r#type: expression(ty),
            }),
            DeclarationKind::PartialApplication(value) => {
                partial_applications.push(expression(value));
            }
        }
    }
    Ok(Schema {
        format_version: FORMAT_VERSION,
        layer: ast.layer,
        source,
        constructors,
        functions,
        finalizations,
        partial_applications,
    })
}

fn argument(a: &mono_tl_parser::Argument, docs: &BTreeMap<String, String>) -> Argument {
    let value = match &a.value {
        mono_tl_parser::ArgumentValue::Type(value) => ArgumentValue::Type {
            expression: expression(value),
        },
        mono_tl_parser::ArgumentValue::Repetition(repetition) => ArgumentValue::Repetition {
            multiplicity: repetition.multiplicity.as_ref().map(expression),
            arguments: repetition
                .arguments
                .iter()
                .map(|field| argument(field, docs))
                .collect(),
        },
    };
    Argument {
        name: a.name.clone(),
        value,
        implicit: a.optional,
        functional: a.bang,
        condition: a.condition.as_ref().map(|c| Condition {
            variable: c.variable.clone(),
            bit: c.bit,
        }),
        description: a.name.as_ref().and_then(|n| docs.get(n).cloned()),
    }
}

fn expression(e: &TypeExpr) -> Expression {
    match e {
        TypeExpr::Ident(name) => Expression::Ident { name: name.clone() },
        TypeExpr::Nat(value) => Expression::Nat { value: *value },
        TypeExpr::Hash => Expression::Hash,
        TypeExpr::Add { left, right } => Expression::Add {
            left: Box::new(expression(left)),
            right: Box::new(expression(right)),
        },
        TypeExpr::Apply {
            constructor,
            arguments,
        } => Expression::Apply {
            constructor: Box::new(expression(constructor)),
            arguments: arguments.iter().map(expression).collect(),
        },
        TypeExpr::Bare(inner) => Expression::Bare {
            inner: Box::new(expression(inner)),
        },
        TypeExpr::Bang(inner) => Expression::Bang {
            inner: Box::new(expression(inner)),
        },
    }
}

fn documentation(comments: &[String], name: &str, kind: CombinatorKind) -> Documentation {
    let mut description = Vec::new();
    let mut parameters = BTreeMap::new();
    let mut links = Vec::new();
    for comment in comments {
        let line = comment.trim().trim_start_matches('/').trim();
        if let Some(rest) = line.strip_prefix("@description ") {
            description.push(rest.to_owned());
        } else if let Some(rest) = line.strip_prefix("@param ") {
            if let Some((n, d)) = rest.split_once(' ') {
                parameters.insert(n.to_owned(), d.trim().to_owned());
            }
        } else if line.starts_with("@see ") {
            // Collected by the common URL pass below without becoming prose.
        } else if !line.is_empty() && !line.chars().all(|c| matches!(c, '/' | '-' | '=')) {
            description.push(line.to_owned());
        }
        links.extend(find_urls(line));
    }
    let category = match kind {
        CombinatorKind::Function => "method",
        CombinatorKind::Constructor => "constructor",
    };
    Documentation {
        description: (!description.is_empty()).then(|| description.join("\n")),
        parameters,
        official_url: Some(format!("https://core.telegram.org/{category}/{name}")),
        links,
    }
}

fn find_urls(s: &str) -> Vec<String> {
    s.split_whitespace()
        .filter_map(|w| {
            let w = w.trim_matches(|c: char| ",.;()[]<>".contains(c));
            (w.starts_with("https://") || w.starts_with("http://")).then(|| w.to_owned())
        })
        .collect()
}
fn extract_introduced(comments: &[String]) -> Option<u32> {
    comments.iter().find_map(|c| {
        c.strip_prefix("@introduced ")
            .and_then(|v| v.trim().parse().ok())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn extracts_docs_and_links() {
        let s = parse("---functions---\n// @description Sends it.\n// @param x the value\na#01020304 x:int = Bool;", Source::default()).unwrap();
        assert_eq!(
            s.functions[0].arguments[0].description.as_deref(),
            Some("the value")
        );
        assert!(
            s.functions[0]
                .documentation
                .official_url
                .as_ref()
                .unwrap()
                .contains("/method/a")
        );
    }
    #[test]
    fn rejects_duplicate_ids() {
        assert!(matches!(
            parse("a#01020304 = A; b#01020304 = B;", Source::default()),
            Err(ModelError::Duplicate { .. })
        ));
    }
}
