//! Capture regressions for safe and explicitly dangerous parser tracing.

#![cfg(feature = "tracing")]

use mono_tl_parser::parse;
use std::sync::{Arc, Mutex};
use tracing::{
    Event, Id, Metadata, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Record},
};

const SOURCE_SENTINEL: &str = "traceCaptureSentinel#11223344 value:int = TraceCaptureSentinel;";

#[derive(Clone, Default)]
struct Capture(Arc<Mutex<Vec<String>>>);

impl Capture {
    fn output(&self) -> String {
        self.0.lock().expect("capture lock poisoned").join("\n")
    }
}

struct FieldVisitor<'a>(&'a mut Vec<String>);

impl Visit for FieldVisitor<'_> {
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.0.push(format!("{}={value:?}", field.name()));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.0.push(format!("{}={value}", field.name()));
    }
}

impl Subscriber for Capture {
    fn enabled(&self, _metadata: &Metadata<'_>) -> bool {
        true
    }

    fn new_span(&self, _attributes: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }

    fn record(&self, _span: &Id, _values: &Record<'_>) {}

    fn record_follows_from(&self, _span: &Id, _follows: &Id) {}

    fn event(&self, event: &Event<'_>) {
        let mut fields = self.0.lock().expect("capture lock poisoned");
        event.record(&mut FieldVisitor(&mut fields));
    }

    fn enter(&self, _span: &Id) {}

    fn exit(&self, _span: &Id) {}
}

#[test]
fn schema_source_redaction_is_feature_controlled_and_parse_contract_is_stable() {
    let baseline = parse(SOURCE_SENTINEL).expect("sentinel schema must parse");
    let baseline_debug = format!("{baseline:?}");

    let capture = Capture::default();
    let traced = tracing::subscriber::with_default(capture.clone(), || parse(SOURCE_SENTINEL))
        .expect("tracing must not change parser output");
    let output = capture.output();

    assert_eq!(traced, baseline);
    assert_eq!(format!("{traced:?}"), baseline_debug);

    #[cfg(feature = "dangerous-unredacted-logging")]
    {
        assert!(output.contains(SOURCE_SENTINEL), "capture was: {output}");
        assert!(
            output.contains("dangerous_unredacted_logging=true"),
            "capture was: {output}"
        );
    }

    #[cfg(not(feature = "dangerous-unredacted-logging"))]
    {
        assert!(
            output.contains("source=[REDACTED]"),
            "capture was: {output}"
        );
        assert!(!output.contains(SOURCE_SENTINEL), "capture was: {output}");
        assert!(
            !output.contains("traceCaptureSentinel"),
            "capture was: {output}"
        );
    }
}

#[test]
fn parser_error_debug_and_display_are_unchanged_with_a_subscriber() {
    const INVALID: &str = "traceErrorSentinel#11223344 =";
    let baseline = parse(INVALID).expect_err("sentinel schema must fail");
    let capture = Capture::default();
    let traced = tracing::subscriber::with_default(capture, || parse(INVALID))
        .expect_err("tracing must not change parser errors");

    assert_eq!(traced, baseline);
    assert_eq!(format!("{traced:?}"), format!("{baseline:?}"));
    assert_eq!(traced.to_string(), baseline.to_string());
}
