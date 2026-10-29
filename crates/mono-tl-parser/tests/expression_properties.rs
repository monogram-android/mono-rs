//! Property and snapshot coverage for the public parser boundary.

use mono_tl_parser::{ArgumentValue, DeclarationKind, TypeExpr, parse};
use proptest::prelude::*;

proptest! {
    #[test]
    fn arbitrary_utf8_never_panics(input in any::<String>()) {
        let result = std::panic::catch_unwind(|| parse(&input));
        prop_assert!(result.is_ok());
    }

    #[test]
    fn natural_addition_depth_is_preserved(constants in prop::collection::vec(0_u16..1000, 1..16)) {
        let expression = constants.iter().map(u16::to_string).collect::<Vec<_>>().join(" + ");
        let schema = parse(&format!("value n:# fields:({expression})*[ item:int ] = Value;")).unwrap();
        let DeclarationKind::Combinator(combinator) = &schema.declarations[0].kind else {
            unreachable!()
        };
        let ArgumentValue::Repetition(repetition) = &combinator.arguments[1].value else {
            unreachable!()
        };
        let mut additions = 0;
        let mut current = repetition.multiplicity.as_ref().unwrap();
        while let TypeExpr::Add { right, .. } = current {
            additions += 1;
            current = right;
        }
        prop_assert_eq!(additions, constants.len().saturating_sub(1));
    }
}

#[test]
#[allow(clippy::too_many_lines)] // The inline snapshot intentionally shows the complete nested AST.
fn dependent_expression_ast_snapshot() {
    let schema = parse(
        "matrix {cell:Type} {rows cols:#} values:(rows + 1)*[ row:cols*[item:cell] ] = Matrix cell rows cols;",
    )
    .unwrap();
    insta::assert_debug_snapshot!(schema, @r###"
    Schema {
        declarations: [
            Declaration {
                section: Types,
                comments: [],
                kind: Combinator(
                    Combinator {
                        name: "matrix",
                        id: None,
                        arguments: [
                            Argument {
                                name: Some(
                                    "cell",
                                ),
                                value: Type(
                                    Ident(
                                        "Type",
                                    ),
                                ),
                                optional: true,
                                bang: false,
                                condition: None,
                            },
                            Argument {
                                name: Some(
                                    "rows",
                                ),
                                value: Type(
                                    Hash,
                                ),
                                optional: true,
                                bang: false,
                                condition: None,
                            },
                            Argument {
                                name: Some(
                                    "cols",
                                ),
                                value: Type(
                                    Hash,
                                ),
                                optional: true,
                                bang: false,
                                condition: None,
                            },
                            Argument {
                                name: Some(
                                    "values",
                                ),
                                value: Repetition(
                                    Repetition {
                                        multiplicity: Some(
                                            Add {
                                                left: Ident(
                                                    "rows",
                                                ),
                                                right: Nat(
                                                    1,
                                                ),
                                            },
                                        ),
                                        arguments: [
                                            Argument {
                                                name: Some(
                                                    "row",
                                                ),
                                                value: Repetition(
                                                    Repetition {
                                                        multiplicity: Some(
                                                            Ident(
                                                                "cols",
                                                            ),
                                                        ),
                                                        arguments: [
                                                            Argument {
                                                                name: Some(
                                                                    "item",
                                                                ),
                                                                value: Type(
                                                                    Ident(
                                                                        "cell",
                                                                    ),
                                                                ),
                                                                optional: false,
                                                                bang: false,
                                                                condition: None,
                                                            },
                                                        ],
                                                    },
                                                ),
                                                optional: false,
                                                bang: false,
                                                condition: None,
                                            },
                                        ],
                                    },
                                ),
                                optional: false,
                                bang: false,
                                condition: None,
                            },
                        ],
                        result: Apply {
                            constructor: Ident(
                                "Matrix",
                            ),
                            arguments: [
                                Ident(
                                    "cell",
                                ),
                                Ident(
                                    "rows",
                                ),
                                Ident(
                                    "cols",
                                ),
                            ],
                        },
                        builtin: false,
                    },
                ),
                span: Span {
                    start: 0,
                    end: 100,
                },
            },
        ],
        layer: None,
    }
    "###);
}
