use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Deserialize)]
struct Manifest {
    layers: Vec<Layer>,
}

#[derive(Deserialize)]
struct Layer {
    layer: u32,
    url: String,
    sha256: String,
}

pub(super) fn verify() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let manifest_path = root.join("schemas/manifest.json");
    let manifest: Manifest =
        serde_json::from_slice(&std::fs::read(&manifest_path).map_err(|error| error.to_string())?)
            .map_err(|error| format!("invalid {}: {error}", manifest_path.display()))?;
    let client = reqwest::blocking::Client::builder()
        .user_agent("mono-rs-schema-verifier")
        .build()
        .map_err(|error| error.to_string())?;

    let expected = [223, 229, 230, 231, 232];
    let actual = manifest
        .layers
        .iter()
        .map(|layer| layer.layer)
        .collect::<Vec<_>>();
    if actual != expected {
        return Err(format!("schema manifest layers differ: {actual:?}"));
    }

    for layer in &manifest.layers {
        let response = client
            .get(&layer.url)
            .send()
            .and_then(reqwest::blocking::Response::error_for_status)
            .map_err(|error| format!("layer {} download failed: {error}", layer.layer))?;
        let bytes = response
            .bytes()
            .map_err(|error| format!("layer {} response failed: {error}", layer.layer))?;
        let digest = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>();
        if digest != layer.sha256 {
            return Err(format!(
                "layer {} digest mismatch: expected {}, got {digest}",
                layer.layer, layer.sha256
            ));
        }
        if bytes.is_empty() {
            return Err(format!("layer {} schema is empty", layer.layer));
        }
        println!("verified layer {} ({})", layer.layer, layer.sha256);
    }
    Ok(())
}
