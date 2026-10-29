//! Comment-aware TL lexer.

use crate::{ParseError, Section, Span};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum TokenKind {
    Ident(String),
    Number(String),
    Hex(String),
    Symbol(char),
    Divider(Section),
    Comment(String),
    Eof,
}

#[derive(Clone, Debug)]
pub(crate) struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[allow(clippy::too_many_lines)]
pub(crate) fn lex(source: &str) -> Result<Vec<Token>, ParseError> {
    let bytes = source.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_whitespace() {
            i += 1;
            continue;
        }
        let start = i;
        if source[i..].starts_with("//") {
            i += 2;
            let body = i;
            while i < bytes.len() && bytes[i] != b'\n' {
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::Comment(source[body..i].trim().to_owned()),
                span: Span { start, end: i },
            });
            continue;
        }
        if source[i..].starts_with("/*") {
            i += 2;
            let body = i;
            let Some(relative_end) = source[i..].find("*/") else {
                return Err(ParseError::new("unterminated block comment", start));
            };
            i += relative_end;
            out.push(Token {
                kind: TokenKind::Comment(source[body..i].trim().to_owned()),
                span: Span { start, end: i + 2 },
            });
            i += 2;
            continue;
        }
        if source[i..].starts_with("---") {
            if let Some(end) = source[i + 3..].find("---") {
                let word = source[i + 3..i + 3 + end].trim();
                let section = match word {
                    "types" => Section::Types,
                    "functions" => Section::Functions,
                    _ => return Err(ParseError::new(format!("unknown section {word:?}"), start)),
                };
                i += 6 + end;
                out.push(Token {
                    kind: TokenKind::Divider(section),
                    span: Span { start, end: i },
                });
                continue;
            }
        }
        if bytes[i] == b'`' {
            i += 1;
            let body = i;
            while i < bytes.len() && bytes[i] != b'`' {
                i += 1;
            }
            if i == bytes.len() {
                return Err(ParseError::new("unterminated quoted identifier", start));
            }
            if i == body {
                return Err(ParseError::new("empty quoted identifier", start));
            }
            out.push(Token {
                kind: TokenKind::Ident(source[body..i].to_owned()),
                span: Span { start, end: i + 1 },
            });
            i += 1;
            continue;
        }
        if bytes[i].is_ascii_digit() {
            i += 1;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::Number(source[start..i].to_owned()),
                span: Span { start, end: i },
            });
            continue;
        }
        if bytes[i].is_ascii_alphabetic() || bytes[i] == b'_' || bytes[i] == b'@' {
            i += 1;
            while i < bytes.len()
                && (bytes[i].is_ascii_alphanumeric() || matches!(bytes[i], b'_' | b'.' | b'@'))
            {
                i += 1;
            }
            out.push(Token {
                kind: TokenKind::Ident(source[start..i].to_owned()),
                span: Span { start, end: i },
            });
            continue;
        }
        if bytes[i] == b'#' {
            i += 1;
            let hex_start = i;
            while i < bytes.len() && bytes[i].is_ascii_hexdigit() {
                i += 1;
            }
            let kind = if i > hex_start {
                TokenKind::Hex(source[hex_start..i].to_owned())
            } else {
                TokenKind::Symbol('#')
            };
            out.push(Token {
                kind,
                span: Span { start, end: i },
            });
            continue;
        }
        if "{}[]():;=?%!*,+<>".as_bytes().contains(&bytes[i]) {
            i += 1;
            out.push(Token {
                kind: TokenKind::Symbol(bytes[start] as char),
                span: Span { start, end: i },
            });
            continue;
        }
        return Err(ParseError::new(
            format!(
                "unexpected character {:?}",
                source[start..].chars().next().unwrap_or_default()
            ),
            start,
        ));
    }
    out.push(Token {
        kind: TokenKind::Eof,
        span: Span {
            start: source.len(),
            end: source.len(),
        },
    });
    Ok(out)
}
