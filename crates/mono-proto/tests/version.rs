//! Ensures package metadata identifies the generated API layer.

#[test]
fn package_version_contains_the_api_layer() {
    let suffix = format!("+layer.{}", mono_proto::LATEST_API_LAYER);
    assert!(
        env!("CARGO_PKG_VERSION").ends_with(&suffix),
        "generated package version must end with {suffix}"
    );
}
