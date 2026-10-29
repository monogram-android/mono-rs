//! Schema-derived TL expression to Rust type resolution.

use crate::{GenerateError, naming::type_name};
use mono_tl_schema::{ArgumentValue, Combinator, Expression, Schema};
use std::collections::{BTreeMap, BTreeSet};

pub(crate) struct Resolver<'a> {
    mappings: &'a BTreeMap<String, String>,
    constructor_results: BTreeMap<&'a str, &'a Expression>,
}

impl<'a> Resolver<'a> {
    pub(crate) fn new(schema: &'a Schema, mappings: &'a BTreeMap<String, String>) -> Self {
        let constructor_results = schema
            .constructors
            .iter()
            .map(|item| (item.name.as_str(), &item.result))
            .collect();
        Self {
            mappings,
            constructor_results,
        }
    }

    pub(crate) fn render(
        &self,
        expression: &Expression,
        generics: &BTreeSet<String>,
    ) -> Result<String, GenerateError> {
        self.render_inner(expression, generics)
    }

    fn render_inner(
        &self,
        expression: &Expression,
        generics: &BTreeSet<String>,
    ) -> Result<String, GenerateError> {
        Ok(match expression {
            Expression::Ident { name } if generics.contains(&type_name(name)) => type_name(name),
            Expression::Ident { name } if self.mappings.contains_key(name) => {
                self.mappings[name].clone()
            }
            Expression::Ident { name } if self.constructor_results.contains_key(name.as_str()) => {
                return self.render_inner(self.constructor_results[name.as_str()], generics);
            }
            Expression::Ident { name } => format!("Box<{}>", type_name(name)),
            Expression::Hash | Expression::Nat { .. } | Expression::Add { .. } => self
                .mappings
                .get("#")
                .cloned()
                .unwrap_or_else(|| "u32".into()),
            Expression::Apply {
                constructor,
                arguments,
            } => {
                let head = expression_head(constructor).map(type_name).ok_or_else(|| {
                    GenerateError::InvalidExpression("application head is not an identifier".into())
                })?;
                let arguments = arguments
                    .iter()
                    .map(|argument| self.render_application_argument(argument, generics))
                    .collect::<Result<Vec<_>, _>>()?;
                if arguments.is_empty() {
                    format!("Box<{head}>")
                } else {
                    format!("Box<{head}<{}>>", arguments.join(", "))
                }
            }
            Expression::Bare { inner } | Expression::Bang { inner } => {
                return self.render_inner(inner, generics);
            }
        })
    }

    fn render_application_argument(
        &self,
        expression: &Expression,
        generics: &BTreeSet<String>,
    ) -> Result<String, GenerateError> {
        match expression {
            Expression::Nat { value } => Ok(format!("__tl::Nat<{value}>")),
            Expression::Add { left, right } => Ok(format!(
                "__tl::Add<{}, {}>",
                Self::render_natural(left, generics)?,
                Self::render_natural(right, generics)?
            )),
            _ => self.render_inner(expression, generics),
        }
    }

    fn render_natural(
        expression: &Expression,
        generics: &BTreeSet<String>,
    ) -> Result<String, GenerateError> {
        match expression {
            Expression::Ident { name } if generics.contains(&type_name(name)) => {
                Ok(type_name(name))
            }
            Expression::Nat { value } => Ok(format!("__tl::Nat<{value}>")),
            Expression::Add { left, right } => Ok(format!(
                "__tl::Add<{}, {}>",
                Self::render_natural(left, generics)?,
                Self::render_natural(right, generics)?
            )),
            _ => Err(GenerateError::InvalidExpression(
                "non-natural operand in natural-number expression".into(),
            )),
        }
    }
}

pub(crate) fn expression_head(expression: &Expression) -> Option<&str> {
    match expression {
        Expression::Ident { name } => Some(name),
        Expression::Apply { constructor, .. } => expression_head(constructor),
        Expression::Bare { inner } | Expression::Bang { inner } => expression_head(inner),
        _ => None,
    }
}

pub(crate) fn generic_parameters(combinator: &Combinator) -> BTreeSet<String> {
    combinator
        .arguments
        .iter()
        .filter(|argument| {
            argument.implicit
                && (matches!(
                    &argument.value,
                    ArgumentValue::Type { expression: Expression::Ident { name } }
                        if name == "Type"
                ) || matches!(
                    &argument.value,
                    ArgumentValue::Type {
                        expression: Expression::Hash
                    }
                ))
        })
        .filter_map(|argument| argument.name.as_ref().map(|name| type_name(name)))
        .collect()
}

/// Generic variables in the order in which they parameterize the result type.
pub(crate) fn result_generic_parameters(combinator: &Combinator) -> Vec<String> {
    let generics = generic_parameters(combinator);
    let mut ordered = Vec::new();
    collect_result_generics(&combinator.result, &generics, &mut ordered);
    ordered
}

fn collect_result_generics(
    expression: &Expression,
    generics: &BTreeSet<String>,
    ordered: &mut Vec<String>,
) {
    match expression {
        Expression::Ident { name } => {
            let name = type_name(name);
            if generics.contains(&name) && !ordered.contains(&name) {
                ordered.push(name);
            }
        }
        Expression::Apply {
            constructor,
            arguments,
        } => {
            collect_result_generics(constructor, generics, ordered);
            for argument in arguments {
                collect_result_generics(argument, generics, ordered);
            }
        }
        Expression::Add { left, right } => {
            collect_result_generics(left, generics, ordered);
            collect_result_generics(right, generics, ordered);
        }
        Expression::Bare { inner } | Expression::Bang { inner } => {
            collect_result_generics(inner, generics, ordered);
        }
        Expression::Nat { .. } | Expression::Hash => {}
    }
}
