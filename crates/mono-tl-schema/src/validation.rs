//! Semantic checks required by TL's dependent argument rules.

use crate::ModelError;
use mono_tl_parser::{Argument, ArgumentValue, DeclarationKind, TypeExpr};
use std::collections::HashMap;

#[derive(Clone, Copy, Eq, PartialEq)]
enum VariableKind {
    Natural,
    Type,
    Value,
}

pub(crate) fn validate(schema: &mono_tl_parser::Schema) -> Result<(), ModelError> {
    for declaration in &schema.declarations {
        let DeclarationKind::Combinator(combinator) = &declaration.kind else {
            continue;
        };
        let mut variables = HashMap::new();
        let mut has_natural = false;
        for argument in &combinator.arguments {
            validate_argument(&combinator.name, argument, &variables, has_natural)?;
            if let Some(name) = &argument.name {
                if variables
                    .insert(name.clone(), variable_kind(argument_type(argument)))
                    .is_some()
                {
                    return invalid(&combinator.name, format!("duplicate argument `{name}`"));
                }
            } else if argument.optional {
                return invalid(&combinator.name, "implicit arguments must be named");
            }
            if argument.optional && !argument_type(argument).is_some_and(is_implicit_type) {
                return invalid(
                    &combinator.name,
                    "implicit arguments must have type `#` or `Type`",
                );
            }
            has_natural |= variable_kind(argument_type(argument)) == VariableKind::Natural;
        }
    }
    Ok(())
}

fn validate_argument(
    name: &str,
    argument: &Argument,
    variables: &HashMap<String, VariableKind>,
    has_natural: bool,
) -> Result<(), ModelError> {
    if let Some(condition) = &argument.condition {
        if condition.bit.is_some_and(|bit| bit >= 32) {
            return invalid(
                name,
                format!(
                    "conditional bit for `{}` must be below 32",
                    condition.variable
                ),
            );
        }
        if variables.get(&condition.variable) != Some(&VariableKind::Natural) {
            return invalid(
                name,
                format!(
                    "condition references non-preceding or non-natural variable `{}`",
                    condition.variable
                ),
            );
        }
    }
    if let ArgumentValue::Repetition(repetition) = &argument.value {
        match &repetition.multiplicity {
            Some(expression) => validate_natural(name, expression, variables)?,
            None if !has_natural => {
                return invalid(
                    name,
                    "implicit repetition count has no preceding natural argument",
                );
            }
            None => {}
        }
        let mut nested = variables.clone();
        for field in &repetition.arguments {
            validate_argument(name, field, &nested, has_natural)?;
            if let Some(field_name) = &field.name {
                nested.insert(field_name.clone(), variable_kind(argument_type(field)));
            }
        }
    }
    Ok(())
}

fn validate_natural(
    name: &str,
    expression: &TypeExpr,
    variables: &HashMap<String, VariableKind>,
) -> Result<(), ModelError> {
    let variable = match expression {
        TypeExpr::Nat(_) => return Ok(()),
        TypeExpr::Add { left, right } => {
            validate_natural(name, left, variables)?;
            return validate_natural(name, right, variables);
        }
        TypeExpr::Ident(variable) => variable,
        _ => return invalid(name, "repetition multiplicity is not a natural expression"),
    };
    if variables.get(variable) == Some(&VariableKind::Natural) {
        Ok(())
    } else {
        invalid(
            name,
            format!(
                "natural expression references non-preceding or non-natural variable `{variable}`"
            ),
        )
    }
}

fn argument_type(argument: &Argument) -> Option<&TypeExpr> {
    match &argument.value {
        ArgumentValue::Type(expression) => Some(expression),
        ArgumentValue::Repetition(_) => None,
    }
}

fn variable_kind(expression: Option<&TypeExpr>) -> VariableKind {
    match expression {
        Some(TypeExpr::Hash) => VariableKind::Natural,
        Some(TypeExpr::Ident(name)) if name == "Type" => VariableKind::Type,
        _ => VariableKind::Value,
    }
}

fn is_implicit_type(expression: &TypeExpr) -> bool {
    matches!(expression, TypeExpr::Hash)
        || matches!(expression, TypeExpr::Ident(name) if name == "Type")
}

fn invalid<T>(declaration: &str, message: impl Into<String>) -> Result<T, ModelError> {
    Err(ModelError::InvalidDeclaration {
        declaration: declaration.to_owned(),
        message: message.into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_forward_conditions() {
        let schema = mono_tl_parser::parse("x value:flags.0?int flags:# = X;").unwrap();
        assert!(validate(&schema).is_err());
    }
    #[test]
    fn accepts_natural_addition() {
        let schema = mono_tl_parser::parse("x n:# values:(2 + n)*[ int ] = X;").unwrap();
        assert!(validate(&schema).is_ok());
    }
}
