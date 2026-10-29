//! Reusable valid and malformed wire fixtures.

/// Stable TL byte-string boundary fixtures (lengths around 254 and alignment edges).
pub fn tl_byte_fixtures() -> Vec<Vec<u8>> {
    [0_usize, 1, 3, 4, 253, 254, 255, 256]
        .into_iter()
        .map(|length| {
            (0..length)
                .map(|index| u8::try_from(index % 256).expect("remainder fits in a byte"))
                .collect()
        })
        .collect()
}

/// Malformed inputs useful for codec regression and fuzz-seed corpora.
pub fn malformed_tl_inputs() -> Vec<Vec<u8>> {
    vec![
        vec![],
        vec![255, 0, 0, 0],
        vec![254, 255, 255, 255],
        vec![1, b'x', 1, 0],
        vec![0x15, 0xc4, 0xb5, 0x1c, 0xff, 0xff, 0xff, 0xff],
    ]
}

/// Deterministic AES-IGE inputs suitable for round-trip regression tests.
/// Callers should additionally run Telegram's published end-to-end vectors.
pub mod crypto_fixtures {
    /// Deterministic 256-bit key.
    pub const KEY: [u8; 32] = [0x42; 32];
    /// Deterministic 256-bit IGE IV.
    pub const IGE_IV: [u8; 32] = [0x24; 32];
    /// Two complete plaintext blocks.
    pub const PLAINTEXT: [u8; 32] = [0x11; 32];
}
