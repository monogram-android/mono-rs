use mono_tl_codegen::{Options, generate};
use mono_tl_schema::{Source, parse};
use reqwest::blocking::Client;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};

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

pub(super) fn run(check: bool) -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("..");
    let manifest_path = root.join("schemas/manifest.json");
    let manifest: Manifest =
        serde_json::from_slice(&std::fs::read(&manifest_path).map_err(|error| error.to_string())?)
            .map_err(|error| format!("invalid {}: {error}", manifest_path.display()))?;
    let client = Client::builder()
        .user_agent("mono-rs-tl-generator")
        .build()
        .map_err(|error| error.to_string())?;

    for layer in &manifest.layers {
        let bytes = download(&client, layer)?;
        let schema = parse(
            std::str::from_utf8(&bytes)
                .map_err(|error| format!("layer {} is not UTF-8: {error}", layer.layer))?,
            Source {
                name: format!("layer-{}.tl", layer.layer),
                url: Some("https://core.telegram.org/schema".into()),
            },
        )
        .map_err(|error| format!("layer {} parse failed: {error}", layer.layer))?;
        let code = generate(
            &schema,
            &Options {
                module_documentation: format!("Telegram API schema layer {}.", layer.layer),
                emit_codec: true,
                ..Options::default()
            },
        )
        .map_err(|error| format!("layer {} generation failed: {error}", layer.layer))?;
        let path = root.join(format!(
            "crates/mono-proto/src/layer_{}/api.rs",
            layer.layer
        ));
        write_or_check(&path, &code, check)?;
        println!(
            "{} layer {} -> {}",
            if check { "verified" } else { "generated" },
            layer.layer,
            path.display()
        );
    }
    Ok(())
}

fn download(client: &Client, layer: &Layer) -> Result<Vec<u8>, String> {
    let bytes = client
        .get(&layer.url)
        .send()
        .and_then(reqwest::blocking::Response::error_for_status)
        .map_err(|error| format!("layer {} download failed: {error}", layer.layer))?
        .bytes()
        .map_err(|error| format!("layer {} response failed: {error}", layer.layer))?
        .to_vec();
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
    Ok(bytes)
}

fn write_or_check(path: &Path, expected: &str, check: bool) -> Result<(), String> {
    if check {
        let actual = std::fs::read_to_string(path)
            .map_err(|error| format!("missing generated file {}: {error}", path.display()))?;
        if actual != expected {
            return Err(format!("generated file is stale: {}", path.display()));
        }
    } else {
        std::fs::write(path, expected)
            .map_err(|error| format!("write {} failed: {error}", path.display()))?;
    }
    Ok(())
}
