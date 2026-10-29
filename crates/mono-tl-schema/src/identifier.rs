//! Canonical TL declaration rendering and implicit constructor numbers.

use mono_tl_parser::{Argument, ArgumentValue, Combinator, TypeExpr};

pub(crate) fn resolve(combinator: &Combinator) -> (u32, bool) {
    match combinator.id {
        Some(id) => (id, true),
        None => (crc32fast::hash(canonical(combinator).as_bytes()), false),
    }
}

pub(crate) fn canonical(combinator: &Combinator) -> String {
    let mut parts = vec![identifier(&combinator.name)];
    if combinator.builtin {
        parts.push("?".into());
    } else {
        parts.extend(combinator.arguments.iter().map(argument));
    }
    parts.push("=".into());
    parts.push(expression(&combinator.result));
    parts.join(" ")
}

fn argument(item: &Argument) -> String {
    let mut output = String::new();
    if let Some(name) = &item.name {
        output.push_str(name);
        output.push(':');
    }
    if let Some(condition) = &item.condition {
        output.push_str(&condition.variable);
        if let Some(bit) = condition.bit {
            output.push('.');
            output.push_str(&bit.to_string());
        }
        output.push('?');
    }
    if item.bang {
        output.push('!');
    }
    match &item.value {
        ArgumentValue::Type(value) => output.push_str(&expression(value)),
        ArgumentValue::Repetition(repetition) => {
            if let Some(multiplicity) = &repetition.multiplicity {
                output.push_str(&natural(multiplicity));
                output.push('*');
            }
            output.push_str("[ ");
            output.push_str(
                &repetition
                    .arguments
                    .iter()
                    .map(argument)
                    .collect::<Vec<_>>()
                    .join(" "),
            );
            output.push_str(" ]");
        }
    }
    output
}

fn expression(value: &TypeExpr) -> String {
    match value {
        TypeExpr::Ident(name) => identifier(name),
        TypeExpr::Nat(value) => value.to_string(),
        TypeExpr::Hash => "#".into(),
        TypeExpr::Apply {
            constructor,
            arguments,
        } => std::iter::once(expression(constructor))
            .chain(arguments.iter().map(expression))
            .collect::<Vec<_>>()
            .join(" "),
        TypeExpr::Add { .. } => natural(value),
        TypeExpr::Bare(inner) => format!("%{}", expression(inner)),
        TypeExpr::Bang(inner) => format!("!{}", expression(inner)),
    }
}

fn identifier(value: &str) -> String {
    let mut characters = value.chars();
    let ordinary = characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || matches!(first, '_' | '@'))
        && characters.all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '_' | '.' | '@')
        });
    if ordinary {
        value.to_owned()
    } else {
        format!("`{value}`")
    }
}

fn natural(value: &TypeExpr) -> String {
    match value {
        TypeExpr::Add { left, right } => {
            let (left, right) = if matches!(right.as_ref(), TypeExpr::Nat(_))
                && !matches!(left.as_ref(), TypeExpr::Nat(_))
            {
                (right, left)
            } else {
                (left, right)
            };
            format!("({}+{})", natural(left), natural(right))
        }
        _ => expression(value),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mono_tl_parser::DeclarationKind;

    fn combinator(source: &str) -> Combinator {
        let schema = mono_tl_parser::parse(source).unwrap();
        let DeclarationKind::Combinator(value) = schema.declarations[0].kind.clone() else {
            panic!()
        };
        value
    }

    #[test]
    fn matches_documented_telegram_numbers() {
        let vector = combinator("vector {t:Type} # [ t ] = Vector t;");
        assert_eq!(canonical(&vector), "vector t:Type # [ t ] = Vector t");
        assert_eq!(resolve(&vector), (0x1cb5_c415, false));

        let user = combinator("user id:int first_name:string last_name:string = User;");
        assert_eq!(resolve(&user), (0xd23c_81a3, false));

        let int = combinator("int ? = Int;");
        assert_eq!(resolve(&int), (0xa850_9bda, false));
    }
}
