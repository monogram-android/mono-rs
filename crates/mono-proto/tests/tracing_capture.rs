//! Capture regressions for safe and explicitly dangerous codec tracing.

#![cfg(feature = "tracing")]

use mono_proto::codec::{Decoder, Encoder, Error, Limits};
use std::sync::{Arc, Mutex};
use tracing::{
    Event, Id, Metadata, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Record},
};

const BYTE_SENTINEL: &[u8] = &[13, 37, 199, 241];
const BYTE_SENTINEL_DEBUG: &str = "[13, 37, 199, 241]";

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
fn raw_byte_redaction_is_feature_controlled_and_codec_contract_is_stable() {
    let mut baseline = Encoder::new();
    baseline.write_raw(BYTE_SENTINEL);
    let baseline_debug = format!("{baseline:?}");
    let baseline_bytes = baseline.into_bytes();

    let capture = Capture::default();
    let mut traced = Encoder::new();
    tracing::subscriber::with_default(capture.clone(), || traced.write_raw(BYTE_SENTINEL));
    let output = capture.output();

    assert_eq!(traced.as_slice(), baseline_bytes);
    assert_eq!(format!("{traced:?}"), baseline_debug);

    #[cfg(feature = "dangerous-unredacted-logging")]
    {
        assert!(
            output.contains(BYTE_SENTINEL_DEBUG),
            "capture was: {output}"
        );
        assert!(
            output.contains("dangerous_unredacted_logging=true"),
            "capture was: {output}"
        );
    }

    #[cfg(not(feature = "dangerous-unredacted-logging"))]
    {
        assert!(output.contains("value=[REDACTED]"), "capture was: {output}");
        assert!(
            !output.contains(BYTE_SENTINEL_DEBUG),
            "capture was: {output}"
        );
    }
}

#[test]
fn codec_error_debug_and_display_are_unchanged_with_a_subscriber() {
    fn invalid_marker() -> Error {
        let mut decoder = Decoder::new(&[255, 0, 0, 0], Limits::default()).unwrap();
        decoder.read_bytes().unwrap_err()
    }

    let baseline = invalid_marker();
    let capture = Capture::default();
    let traced = tracing::subscriber::with_default(capture, invalid_marker);

    assert_eq!(traced, baseline);
    assert_eq!(format!("{traced:?}"), format!("{baseline:?}"));
    assert_eq!(traced.to_string(), baseline.to_string());
}
