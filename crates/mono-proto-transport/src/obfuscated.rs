//! Stateful client transport obfuscation.

use crate::Error;

/// Protocol marker embedded in an obfuscated transport initialization payload.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ObfuscatedProtocol {
    /// Abridged (`0xefefefef`).
    Abridged,
    /// Intermediate (`0xeeeeeeee`).
    Intermediate,
    /// Padded intermediate (`0xdddddddd`).
    PaddedIntermediate,
}

impl ObfuscatedProtocol {
    const fn identifier(self) -> u32 {
        match self {
            Self::Abridged => 0xefef_efef,
            Self::Intermediate => 0xeeee_eeee,
            Self::PaddedIntermediate => 0xdddd_dddd,
        }
    }
}

/// Stateful client-side MTProto transport obfuscation.
///
/// This is the stream primitive used for direct DC and MTProxy client
/// connections. Proxy-server routing, tags, fake TLS, configuration, and
/// connection acceptance remain the responsibility of the server.
#[derive(Debug)]
pub struct ObfuscatedStream {
    header: [u8; 64],
    outgoing: mono_proto_crypto::AesCtr,
    incoming: mono_proto_crypto::AesCtr,
}

impl ObfuscatedStream {
    /// Construct an obfuscated client stream from a securely random nonce.
    ///
    /// `dc_id` is included only for an MTProxy connection. `secret` must be the
    /// normalized 16-byte secret without a transport-selector prefix.
    pub fn new_client(
        mut nonce: [u8; 64],
        protocol: ObfuscatedProtocol,
        dc_id: Option<i16>,
        secret: Option<&[u8]>,
    ) -> Result<Self, Error> {
        let first = u32::from_le_bytes(nonce[..4].try_into().expect("fixed nonce"));
        let second = u32::from_le_bytes(nonce[4..8].try_into().expect("fixed nonce"));
        let forbidden = [
            0x4441_4548,
            0x5453_4f50,
            0x2054_4547,
            0x4954_504f,
            0x0201_0316,
            0xdddd_dddd,
            0xeeee_eeee,
        ];
        if nonce[0] == 0xef || second == 0 || forbidden.contains(&first) {
            return Err(Error::InvalidObfuscationNonce);
        }
        nonce[56..60].copy_from_slice(&protocol.identifier().to_le_bytes());
        if let Some(dc_id) = dc_id {
            nonce[60..62].copy_from_slice(&dc_id.to_le_bytes());
        }
        let mut reversed = nonce;
        reversed.reverse();
        let mut outgoing_key = nonce[8..40].to_vec();
        let mut incoming_key = reversed[8..40].to_vec();
        if let Some(secret) = secret {
            if secret.len() != 16 {
                return Err(Error::InvalidProxySecret(secret.len()));
            }
            outgoing_key.extend_from_slice(secret);
            incoming_key.extend_from_slice(secret);
            outgoing_key = mono_proto_crypto::sha256(&outgoing_key).to_vec();
            incoming_key = mono_proto_crypto::sha256(&incoming_key).to_vec();
        }
        let mut outgoing = mono_proto_crypto::AesCtr::new(&outgoing_key, &nonce[40..56])
            .map_err(|error| Error::Connection(error.to_string()))?;
        let incoming = mono_proto_crypto::AesCtr::new(&incoming_key, &reversed[40..56])
            .map_err(|error| Error::Connection(error.to_string()))?;
        let mut encrypted_nonce = nonce;
        outgoing.apply(&mut encrypted_nonce);
        let mut header = nonce;
        header[56..].copy_from_slice(&encrypted_nonce[56..]);
        Ok(Self {
            header,
            outgoing,
            incoming,
        })
    }

    /// Initialization payload sent once immediately after the TCP handshake.
    pub const fn header(&self) -> &[u8; 64] {
        &self.header
    }

    /// Encrypt the next outgoing stream bytes in place.
    pub fn encrypt(&mut self, bytes: &mut [u8]) {
        self.outgoing.apply(bytes);
    }

    /// Decrypt the next incoming stream bytes in place.
    pub fn decrypt(&mut self, bytes: &mut [u8]) {
        self.incoming.apply(bytes);
    }
}
