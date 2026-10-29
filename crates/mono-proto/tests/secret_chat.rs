//! Public surface checks for generated Secret Chat layer selection.

#![cfg(feature = "secret-chat")]

use mono_proto::secret_chat::{LATEST_LAYER, SUPPORTED_LAYERS, schema_layer_for_peer};

#[test]
fn selects_the_newest_compatible_change_layer() {
    assert_eq!(schema_layer_for_peer(7), None);
    assert_eq!(schema_layer_for_peer(8), Some(8));
    assert_eq!(schema_layer_for_peer(100), Some(73));
    assert_eq!(schema_layer_for_peer(LATEST_LAYER), Some(LATEST_LAYER));
    assert_eq!(SUPPORTED_LAYERS.last().copied(), Some(LATEST_LAYER));
}

#[test]
fn generated_declaration_metadata_retains_introduction_layer() {
    use mono_proto::secret_chat::latest::{DecryptedMessageConstructor, SOURCE_URL};

    assert_eq!(
        DecryptedMessageConstructor::SCHEMA_LAYER,
        Some(LATEST_LAYER)
    );
    assert_eq!(DecryptedMessageConstructor::INTRODUCED_LAYER, Some(73));
    assert_eq!(SOURCE_URL, "https://core.telegram.org/schema/end-to-end");
}
