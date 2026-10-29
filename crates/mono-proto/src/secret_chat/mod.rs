//! Generated Secret Chat payload schemas.
//!
//! Select a module using the peer layer negotiated by the Secret Chat protocol.

/// Schema change layers reconstructed from Telegram's official end-to-end schema.
pub const SUPPORTED_LAYERS: &[u32] = &[8, 17, 20, 23, 45, 46, 66, 73, 101, 143, 144, 216];

/// Newest available Secret Chat schema layer.
pub const LATEST_LAYER: u32 = 216;

/// Select the newest generated schema not newer than the peer's layer.
#[must_use]
pub const fn schema_layer_for_peer(peer_layer: u32) -> Option<u32> {
    let mut index = 0;
    let mut selected = None;
    while index < SUPPORTED_LAYERS.len() {
        let layer = SUPPORTED_LAYERS[index];
        if layer > peer_layer {
            break;
        }
        selected = Some(layer);
        index += 1;
    }
    selected
}

/// Secret Chat payload schema at layer 8.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_8;

/// Secret Chat payload schema at layer 17.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_17;

/// Secret Chat payload schema at layer 20.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_20;

/// Secret Chat payload schema at layer 23.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_23;

/// Secret Chat payload schema at layer 45.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_45;

/// Secret Chat payload schema at layer 46.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_46;

/// Secret Chat payload schema at layer 66.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_66;

/// Secret Chat payload schema at layer 73.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_73;

/// Secret Chat payload schema at layer 101.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_101;

/// Secret Chat payload schema at layer 143.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_143;

/// Secret Chat payload schema at layer 144.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_144;

/// Secret Chat payload schema at layer 216.
#[rustfmt::skip]
#[allow(missing_docs, clippy::all, clippy::pedantic)]
pub mod layer_216;

/// Newest available Secret Chat payload schema.
pub use layer_216 as latest;
