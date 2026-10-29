//! Rust documentation rendering.

use mono_tl_schema::Documentation;
use std::fmt::Write;

pub(crate) fn emit(out: &mut String, docs: &Documentation) {
    emit_with(out, docs, line);
}

pub(crate) fn emit_indented(out: &mut String, docs: &Documentation) {
    emit_with(out, docs, indented_line);
}

fn emit_with(out: &mut String, docs: &Documentation, emit_line: fn(&mut String, &str)) {
    if let Some(description) = &docs.description {
        for text in description.lines() {
            emit_line(out, text);
        }
    }
    if let Some(url) = &docs.official_url {
        emit_line(out, &format!("Official Telegram documentation: {url}."));
    }
    for url in &docs.links {
        if docs.official_url.as_ref() != Some(url) {
            emit_line(out, &format!("Related protocol documentation: {url}."));
        }
    }
}

pub(crate) fn line(out: &mut String, text: &str) {
    out.push_str("#[doc = ");
    write!(out, "{:?}", escape(text)).expect("writing to String cannot fail");
    out.push_str("]\n");
}

pub(crate) fn indented_line(out: &mut String, text: &str) {
    out.push_str("    #[doc = ");
    write!(out, "{:?}", escape(text)).expect("writing to String cannot fail");
    out.push_str("]\n");
}

fn escape(text: &str) -> String {
    let escaped = text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('[', "\\[")
        .replace(']', "\\]");
    linkify_urls(&escaped)
}

fn linkify_urls(text: &str) -> String {
    let mut output = String::with_capacity(text.len());
    let mut remaining = text;
    while let Some(start) = remaining.find("https://") {
        output.push_str(&remaining[..start]);
        let candidate = &remaining[start..];
        let mut end = candidate
            .find(char::is_whitespace)
            .unwrap_or(candidate.len());
        while end > 0 && matches!(candidate.as_bytes()[end - 1], b'.' | b',' | b';' | b')') {
            end -= 1;
        }
        if end == 0 {
            output.push_str(candidate);
            return output;
        }
        output.push('<');
        output.push_str(&candidate[..end]);
        output.push('>');
        remaining = &candidate[end..];
    }
    output.push_str(remaining);
    output
}
