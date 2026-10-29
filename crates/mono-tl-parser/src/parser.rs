use crate::lexer::{Token, TokenKind, lex};
use crate::{
    Argument, ArgumentValue, Combinator, Condition, Declaration, DeclarationKind, Finalization,
    ParseError, Repetition, Schema, Section, Span, TypeExpr,
};

/// Parse one complete TL source file.
///
/// # Errors
///
/// Returns a [`ParseError`] at the first malformed token or declaration.
pub fn parse(source: &str) -> Result<Schema, ParseError> {
    let tokens = lex(source)?;
    Parser {
        tokens,
        cursor: 0,
        section: Section::Types,
        comments: Vec::new(),
        layer: None,
    }
    .schema()
}

struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
    section: Section,
    comments: Vec<String>,
    layer: Option<u32>,
}
impl Parser {
    fn schema(mut self) -> Result<Schema, ParseError> {
        let mut declarations = Vec::new();
        while !matches!(self.peek(), TokenKind::Eof) {
            match self.peek().clone() {
                TokenKind::Comment(c) => {
                    self.cursor += 1;
                    if let Some(n) = parse_layer(&c) {
                        self.layer = Some(n);
                    }
                    self.comments.push(c);
                }
                TokenKind::Divider(s) => {
                    self.cursor += 1;
                    self.section = s;
                    self.comments.clear();
                }
                _ => declarations.push(self.declaration()?),
            }
        }
        Ok(Schema {
            declarations,
            layer: self.layer,
        })
    }
    fn declaration(&mut self) -> Result<Declaration, ParseError> {
        let start = self.token().span.start;
        let comments = std::mem::take(&mut self.comments);
        let kind = if let TokenKind::Ident(word) = self.peek() {
            let finalization = match word.as_str() {
                "New" => Some(Finalization::New),
                "Final" => Some(Finalization::Final),
                "Empty" => Some(Finalization::Empty),
                _ => None,
            };
            if let Some(mode) = finalization {
                self.cursor += 1;
                let ty = self.type_expr()?;
                self.symbol(';')?;
                DeclarationKind::Finalization { mode, ty }
            } else if !self.has_equals_before_semicolon() {
                let value = self.type_expr()?;
                self.symbol(';')?;
                DeclarationKind::PartialApplication(value)
            } else {
                self.combinator_or_partial()?
            }
        } else if !self.has_equals_before_semicolon() {
            let value = self.type_expr()?;
            self.symbol(';')?;
            DeclarationKind::PartialApplication(value)
        } else {
            self.combinator_or_partial()?
        };
        let end = self.tokens[self.cursor.saturating_sub(1)].span.end;
        Ok(Declaration {
            section: self.section,
            comments,
            kind,
            span: Span { start, end },
        })
    }
    fn has_equals_before_semicolon(&self) -> bool {
        self.tokens[self.cursor..]
            .iter()
            .take_while(|t| !matches!(t.kind, TokenKind::Symbol(';') | TokenKind::Eof))
            .any(|t| matches!(t.kind, TokenKind::Symbol('=')))
    }
    fn combinator_or_partial(&mut self) -> Result<DeclarationKind, ParseError> {
        let name = self.ident()?;
        let id = if let TokenKind::Hex(hex) = self.peek().clone() {
            self.cursor += 1;
            Some(
                u32::from_str_radix(&hex, 16)
                    .map_err(|_| self.error("combinator ID exceeds 32 bits"))?,
            )
        } else {
            None
        };
        if self.eat(';') {
            return Ok(DeclarationKind::PartialApplication(TypeExpr::Ident(name)));
        }
        let mut arguments = Vec::new();
        let mut builtin = false;
        while !self.is_symbol('=') {
            if self.eat('?') {
                builtin = true;
                break;
            }
            if matches!(self.peek(), TokenKind::Eof) {
                return Err(self.error("expected `=`"));
            }
            arguments.extend(self.argument()?);
        }
        self.symbol('=')?;
        let result = self.type_expr()?;
        self.symbol(';')?;
        Ok(DeclarationKind::Combinator(Combinator {
            name,
            id,
            arguments,
            result,
            builtin,
        }))
    }
    #[allow(clippy::too_many_lines)]
    fn argument(&mut self) -> Result<Vec<Argument>, ParseError> {
        if self.eat('{') {
            let mut names = vec![self.ident_optional()?];
            while !self.is_symbol(':') {
                names.push(self.ident_optional()?);
            }
            self.symbol(':')?;
            let bang = self.eat('!');
            let ty = self.type_expr()?;
            self.symbol('}')?;
            return Ok(names
                .into_iter()
                .map(|name| Argument {
                    name,
                    value: ArgumentValue::Type(ty.clone()),
                    optional: true,
                    bang,
                    condition: None,
                })
                .collect());
        }
        if self.eat('(') {
            let mut names = vec![self.ident_optional()?];
            while !self.is_symbol(':') {
                names.push(self.ident_optional()?);
            }
            self.symbol(':')?;
            let bang = self.eat('!');
            let ty = self.type_expr()?;
            self.symbol(')')?;
            return Ok(names
                .into_iter()
                .map(|name| Argument {
                    name,
                    value: ArgumentValue::Type(ty.clone()),
                    optional: false,
                    bang,
                    condition: None,
                })
                .collect());
        }
        let save = self.cursor;
        let name = if matches!(self.peek(), TokenKind::Ident(_))
            && matches!(
                self.tokens.get(self.cursor + 1).map(|t| &t.kind),
                Some(TokenKind::Symbol(':'))
            ) {
            let n = self.ident_optional()?;
            self.symbol(':')?;
            n
        } else {
            None
        };
        let mut condition = None;
        if let TokenKind::Ident(value) = self.peek().clone() {
            if value.contains('.')
                && matches!(
                    self.tokens.get(self.cursor + 1).map(|t| &t.kind),
                    Some(TokenKind::Symbol('?'))
                )
            {
                self.cursor += 1;
                self.cursor += 1;
                let (variable, bit) = value.split_once('.').expect("contains dot");
                condition = Some(Condition {
                    variable: variable.to_owned(),
                    bit: Some(
                        bit.parse()
                            .map_err(|_| self.error("invalid conditional bit"))?,
                    ),
                });
            } else if matches!(
                self.tokens.get(self.cursor + 1).map(|t| &t.kind),
                Some(TokenKind::Symbol('?'))
            ) {
                self.cursor += 2;
                condition = Some(Condition {
                    variable: value,
                    bit: None,
                });
            }
        }
        let bang = self.eat('!');
        if self.is_symbol('[') || self.looks_like_repetition() {
            let multiplicity = if self.is_symbol('[') {
                None
            } else {
                let expr = self.type_atom()?;
                self.symbol('*')?;
                Some(expr)
            };
            self.symbol('[')?;
            let mut nested = Vec::new();
            while !self.eat(']') {
                nested.extend(self.argument()?);
            }
            return Ok(vec![Argument {
                name,
                value: ArgumentValue::Repetition(Repetition {
                    multiplicity,
                    arguments: nested,
                }),
                optional: false,
                bang,
                condition,
            }]);
        }
        if name.is_none() && condition.is_none() && !bang && save == self.cursor {
            let ty = self.type_term()?;
            return Ok(vec![Argument {
                name: None,
                value: ArgumentValue::Type(ty),
                optional: false,
                bang: false,
                condition: None,
            }]);
        }
        let ty = if name.is_some() || condition.is_some() {
            self.type_expr()?
        } else {
            self.type_term()?
        };
        Ok(vec![Argument {
            name,
            value: ArgumentValue::Type(ty),
            optional: false,
            bang,
            condition,
        }])
    }
    fn type_term(&mut self) -> Result<TypeExpr, ParseError> {
        let mut base = self.type_atom()?;
        if self.eat('<') {
            let mut args = Vec::new();
            if !self.eat('>') {
                loop {
                    args.push(self.type_expr()?);
                    if self.eat('>') {
                        break;
                    }
                    self.symbol(',')?;
                }
            }
            base = TypeExpr::Apply {
                constructor: Box::new(base),
                arguments: args,
            };
        }
        Ok(base)
    }
    fn looks_like_repetition(&self) -> bool {
        if matches!(
            self.tokens.get(self.cursor + 1).map(|t| &t.kind),
            Some(TokenKind::Symbol('*'))
        ) {
            return true;
        }
        if !self.is_symbol('(') {
            return false;
        }
        let mut depth = 0_usize;
        for (offset, token) in self.tokens[self.cursor..].iter().enumerate() {
            match token.kind {
                TokenKind::Symbol('(') => depth += 1,
                TokenKind::Symbol(')') => {
                    depth -= 1;
                    if depth == 0 {
                        return matches!(
                            self.tokens
                                .get(self.cursor + offset + 1)
                                .map(|next| &next.kind),
                            Some(TokenKind::Symbol('*'))
                        );
                    }
                }
                _ => {}
            }
        }
        false
    }
    fn type_expr(&mut self) -> Result<TypeExpr, ParseError> {
        let mut base = self.type_atom()?;
        if self.eat('<') {
            let mut args = Vec::new();
            if !self.eat('>') {
                loop {
                    args.push(self.type_expr()?);
                    if self.eat('>') {
                        break;
                    }
                    self.symbol(',')?;
                }
            }
            base = TypeExpr::Apply {
                constructor: Box::new(base),
                arguments: args,
            };
        } else {
            let mut args = Vec::new();
            while self.starts_atom()
                && !matches!(self.peek(), TokenKind::Ident(_) if self.tokens.get(self.cursor + 1).is_some_and(|t| matches!(t.kind, TokenKind::Symbol(':'))) )
            {
                args.push(self.type_atom()?);
            }
            if !args.is_empty() {
                base = TypeExpr::Apply {
                    constructor: Box::new(base),
                    arguments: args,
                };
            }
        }
        while self.eat('+') {
            let right = self.type_expr()?;
            base = TypeExpr::Add {
                left: Box::new(base),
                right: Box::new(right),
            };
        }
        Ok(base)
    }
    fn type_atom(&mut self) -> Result<TypeExpr, ParseError> {
        if self.eat('%') {
            return Ok(TypeExpr::Bare(Box::new(self.type_atom()?)));
        }
        if self.eat('!') {
            return Ok(TypeExpr::Bang(Box::new(self.type_atom()?)));
        }
        if self.eat('#') {
            return Ok(TypeExpr::Hash);
        }
        if self.eat('(') {
            let value = self.type_expr()?;
            self.symbol(')')?;
            return Ok(value);
        }
        match self.peek().clone() {
            TokenKind::Ident(s) => {
                self.cursor += 1;
                Ok(TypeExpr::Ident(s))
            }
            TokenKind::Number(s) => {
                self.cursor += 1;
                Ok(TypeExpr::Nat(s.parse().map_err(|_| {
                    self.error("natural number exceeds 64 bits")
                })?))
            }
            _ => Err(self.error("expected a type expression")),
        }
    }
    fn starts_atom(&self) -> bool {
        matches!(
            self.peek(),
            TokenKind::Ident(_) | TokenKind::Number(_) | TokenKind::Symbol('%' | '!' | '#' | '(')
        )
    }
    fn ident(&mut self) -> Result<String, ParseError> {
        match self.peek().clone() {
            TokenKind::Ident(s) => {
                self.cursor += 1;
                Ok(s)
            }
            _ => Err(self.error("expected an identifier")),
        }
    }
    fn ident_optional(&mut self) -> Result<Option<String>, ParseError> {
        let s = self.ident()?;
        Ok((s != "_").then_some(s))
    }
    fn symbol(&mut self, c: char) -> Result<(), ParseError> {
        if self.eat(c) {
            Ok(())
        } else {
            Err(self.error(format!("expected `{c}`")))
        }
    }
    fn eat(&mut self, c: char) -> bool {
        if self.is_symbol(c) {
            self.cursor += 1;
            true
        } else {
            false
        }
    }
    fn is_symbol(&self, c: char) -> bool {
        self.peek() == &TokenKind::Symbol(c)
    }
    fn peek(&self) -> &TokenKind {
        &self.token().kind
    }
    fn token(&self) -> &Token {
        &self.tokens[self.cursor]
    }
    fn error(&self, message: impl Into<String>) -> ParseError {
        ParseError::new(message, self.token().span.start)
    }
}

fn parse_layer(comment: &str) -> Option<u32> {
    let upper = comment.to_ascii_uppercase();
    let rest = upper.strip_prefix("LAYER ")?;
    rest.trim().parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_current_style() {
        let s = parse("// docs\nfoo.bar#deadbeef flags:# x:flags.2?Vector<long> = foo.Baz;\n---functions---\nfoo.get q:!X = foo.Baz; // LAYER 9").unwrap();
        assert_eq!(s.layer, Some(9));
        assert_eq!(s.declarations.len(), 2);
        let DeclarationKind::Combinator(c) = &s.declarations[0].kind else {
            panic!()
        };
        assert_eq!(c.id, Some(0xdead_beef));
        assert_eq!(c.arguments[1].condition.as_ref().unwrap().bit, Some(2));
    }
    #[test]
    fn parses_formal_extensions() {
        let s = parse(
            "tuple {t:Type} {n:#} values:(2 + n)*[ value:t ] = Tuple t n; Vector int; Final Tuple;",
        )
        .unwrap();
        assert_eq!(s.declarations.len(), 3);
        let DeclarationKind::Combinator(tuple) = &s.declarations[0].kind else {
            panic!()
        };
        assert!(matches!(
            tuple.arguments[2].value,
            ArgumentValue::Repetition(_)
        ));
    }
    #[test]
    fn separates_unnamed_arguments() {
        let s = parse("couple {t:Type} int t = Couple t;").unwrap();
        let DeclarationKind::Combinator(c) = &s.declarations[0].kind else {
            panic!()
        };
        assert_eq!(c.arguments.len(), 3);
    }
    #[test]
    fn parses_whitespace_application_in_named_argument() {
        let schema = parse("wrapped value:Sequence Octet next:int = Wrapped;").unwrap();
        let DeclarationKind::Combinator(combinator) = &schema.declarations[0].kind else {
            panic!()
        };
        assert!(matches!(
            &combinator.arguments[0].value,
            ArgumentValue::Type(TypeExpr::Apply { arguments, .. }) if arguments.len() == 1
        ));
        assert_eq!(combinator.arguments[1].name.as_deref(), Some("next"));
    }
    #[test]
    fn reports_offsets() {
        assert_eq!(parse("x a:^ = X;").unwrap_err().offset(), 4);
    }
    #[test]
    fn accepts_omitted_leading_id_zeroes() {
        let schema = parse("x#123 = X;").unwrap();
        let DeclarationKind::Combinator(combinator) = &schema.declarations[0].kind else {
            panic!()
        };
        assert_eq!(combinator.id, Some(0x123));
    }
    #[test]
    fn comments_do_not_confuse_dividers() {
        assert!(parse("/* ---functions--- */ x = X;").is_ok());
    }
    #[test]
    fn parses_quoted_identifiers() {
        let schema = parse("`+` left:Int right:Int = Int;").unwrap();
        let DeclarationKind::Combinator(combinator) = &schema.declarations[0].kind else {
            panic!()
        };
        assert_eq!(combinator.name, "+");
        assert!(parse("`unterminated = Int;").is_err());
    }
}
