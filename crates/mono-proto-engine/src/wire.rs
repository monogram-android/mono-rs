//! Plain/encrypted service envelopes, containers, acknowledgements, and gzip.

use crate::Error;
use std::io::Read;

/// Maximum number of identifiers allowed in acknowledgment and state requests.
pub const MAX_SERVICE_MESSAGE_IDS: usize = 8192;
/// Boxed `msg_container` constructor number.
pub const MSG_CONTAINER_ID: u32 = 0x73f1_f8dc;
/// Boxed `msgs_ack` constructor number.
pub const MSGS_ACK_ID: u32 = 0x62d6_b459;
/// Boxed `gzip_packed` constructor number.
pub const GZIP_PACKED_ID: u32 = 0x3072_cfa1;
pub(crate) const VECTOR_ID: u32 = 0x1cb5_c415;

/// Unencrypted authorization message envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlainMessage {
    /// Client or server message identifier.
    pub message_id: i64,
    /// Boxed TL authorization body.
    pub body: Vec<u8>,
}

/// Encode the special unencrypted envelope used during authorization.
pub fn encode_plain_message(message: &PlainMessage) -> Result<Vec<u8>, Error> {
    if message.body.is_empty() || message.body.len() % 4 != 0 {
        return Err(Error::MalformedMessage(
            "plain body must be nonempty and 4-byte aligned",
        ));
    }
    let length = u32::try_from(message.body.len())
        .map_err(|_| Error::MalformedMessage("plain body exceeds 32-bit length"))?;
    let mut output = Vec::with_capacity(20 + message.body.len());
    output.extend_from_slice(&0_i64.to_le_bytes());
    output.extend_from_slice(&message.message_id.to_le_bytes());
    output.extend_from_slice(&length.to_le_bytes());
    output.extend_from_slice(&message.body);
    Ok(output)
}

/// Decode and bound an unencrypted authorization envelope.
pub fn decode_plain_message(input: &[u8], max_body_length: usize) -> Result<PlainMessage, Error> {
    if input.len() < 20 {
        return Err(Error::MalformedMessage(
            "plain envelope is shorter than 20 bytes",
        ));
    }
    if input[..8] != [0; 8] {
        return Err(Error::MalformedMessage(
            "plain envelope has nonzero auth_key_id",
        ));
    }
    let message_id = i64::from_le_bytes(input[8..16].try_into().expect("checked header"));
    let length = u32::from_le_bytes(input[16..20].try_into().expect("checked header")) as usize;
    if length == 0 || length % 4 != 0 || length > max_body_length {
        return Err(Error::MalformedMessage("invalid plain message length"));
    }
    if input.len() != 20 + length {
        return Err(Error::MalformedMessage("plain envelope length mismatch"));
    }
    Ok(PlainMessage {
        message_id,
        body: input[20..].to_vec(),
    })
}

/// One bare message nested inside `msg_container`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContainerMessage {
    /// Nested message identifier.
    pub message_id: i64,
    /// Nested sequence number.
    pub sequence: i32,
    /// Boxed TL message body.
    pub body: Vec<u8>,
}

/// Validate constraints that depend on the outer container envelope.
pub fn validate_container_for_outer(
    messages: &[ContainerMessage],
    outer_message_id: i64,
) -> Result<(), Error> {
    if messages.iter().any(|message| {
        message.message_id >= outer_message_id
            || message
                .body
                .get(..4)
                .is_some_and(|bytes| bytes == MSG_CONTAINER_ID.to_le_bytes())
    }) {
        return Err(Error::MalformedMessage(
            "nested IDs must precede the outer ID and containers cannot nest",
        ));
    }
    Ok(())
}

/// Encode a boxed MTProto message container.
pub fn encode_container(messages: &[ContainerMessage]) -> Result<Vec<u8>, Error> {
    if messages.len() > 1024 {
        return Err(Error::MalformedMessage("container count exceeds 1024"));
    }
    let mut output = Vec::new();
    output.extend_from_slice(&MSG_CONTAINER_ID.to_le_bytes());
    output.extend_from_slice(
        &u32::try_from(messages.len())
            .expect("container count is bounded to 1024")
            .to_le_bytes(),
    );
    for message in messages {
        if message.body.is_empty() || message.body.len() % 4 != 0 {
            return Err(Error::MalformedMessage(
                "container body must be nonempty and aligned",
            ));
        }
        if message
            .body
            .get(..4)
            .is_some_and(|bytes| bytes == MSG_CONTAINER_ID.to_le_bytes())
        {
            return Err(Error::MalformedMessage("message containers cannot nest"));
        }
        output.extend_from_slice(&message.message_id.to_le_bytes());
        output.extend_from_slice(&message.sequence.to_le_bytes());
        output.extend_from_slice(
            &u32::try_from(message.body.len())
                .map_err(|_| Error::MalformedMessage("container body is too large"))?
                .to_le_bytes(),
        );
        output.extend_from_slice(&message.body);
    }
    Ok(output)
}

/// Decode a boxed MTProto message container with allocation limits.
pub fn decode_container(
    input: &[u8],
    max_messages: usize,
    max_body_length: usize,
) -> Result<Vec<ContainerMessage>, Error> {
    if input.len() < 8
        || u32::from_le_bytes(input[..4].try_into().expect("checked header")) != MSG_CONTAINER_ID
    {
        return Err(Error::MalformedMessage("missing msg_container constructor"));
    }
    let count = u32::from_le_bytes(input[4..8].try_into().expect("checked header")) as usize;
    if count > max_messages.min(1024) {
        return Err(Error::MalformedMessage("container count exceeds limit"));
    }
    let mut offset = 8_usize;
    let mut messages = Vec::with_capacity(count);
    for _ in 0..count {
        let header_end = offset
            .checked_add(16)
            .ok_or(Error::MalformedMessage("container offset overflow"))?;
        if header_end > input.len() {
            return Err(Error::MalformedMessage(
                "truncated container message header",
            ));
        }
        let message_id =
            i64::from_le_bytes(input[offset..offset + 8].try_into().expect("fixed field"));
        let sequence = i32::from_le_bytes(
            input[offset + 8..offset + 12]
                .try_into()
                .expect("fixed field"),
        );
        let length = u32::from_le_bytes(
            input[offset + 12..header_end]
                .try_into()
                .expect("fixed field"),
        ) as usize;
        if length == 0 || length % 4 != 0 || length > max_body_length {
            return Err(Error::MalformedMessage("invalid nested message length"));
        }
        let body_end = header_end
            .checked_add(length)
            .ok_or(Error::MalformedMessage("nested message length overflow"))?;
        if body_end > input.len() {
            return Err(Error::MalformedMessage("nested message exceeds container"));
        }
        if input[header_end..body_end]
            .get(..4)
            .is_some_and(|bytes| bytes == MSG_CONTAINER_ID.to_le_bytes())
        {
            return Err(Error::MalformedMessage("message containers cannot nest"));
        }
        messages.push(ContainerMessage {
            message_id,
            sequence,
            body: input[header_end..body_end].to_vec(),
        });
        offset = body_end;
    }
    if offset != input.len() {
        return Err(Error::MalformedMessage("trailing bytes after container"));
    }
    Ok(messages)
}

/// Encode a boxed `msgs_ack` service message.
pub fn encode_acknowledgements(message_ids: &[i64]) -> Result<Vec<u8>, Error> {
    if message_ids.is_empty() || message_ids.len() > MAX_SERVICE_MESSAGE_IDS {
        return Err(Error::MalformedMessage(
            "acknowledgement count is outside 1..=8192",
        ));
    }
    let mut output = Vec::with_capacity(12 + message_ids.len() * 8);
    output.extend_from_slice(&MSGS_ACK_ID.to_le_bytes());
    output.extend_from_slice(&VECTOR_ID.to_le_bytes());
    output.extend_from_slice(
        &u32::try_from(message_ids.len())
            .expect("acknowledgement count is bounded to 8192")
            .to_le_bytes(),
    );
    for message_id in message_ids {
        output.extend_from_slice(&message_id.to_le_bytes());
    }
    Ok(output)
}

/// Decode and expand a boxed `gzip_packed` object with a strict output bound.
///
/// The returned bytes are the original boxed TL object and are required to be
/// nonempty and 4-byte aligned. Callers should dispatch them through their
/// selected generated declaration layer.
pub fn unpack_gzip_packed(input: &[u8], max_output_length: usize) -> Result<Vec<u8>, Error> {
    if input.len() < 8
        || u32::from_le_bytes(input[..4].try_into().expect("checked header")) != GZIP_PACKED_ID
    {
        return Err(Error::MalformedMessage("missing gzip_packed constructor"));
    }
    let limits = mono_proto_codec::Limits {
        max_bytes: input.len(),
        max_total_bytes: input.len() - 4,
        ..mono_proto_codec::Limits::default()
    };
    let mut decoder = mono_proto_codec::Decoder::new(&input[4..], limits)?;
    let packed = mono_proto_codec::decode_bytes(&mut decoder)?;
    decoder.finish()?;

    let read_limit = u64::try_from(max_output_length)
        .unwrap_or(u64::MAX)
        .saturating_add(1);
    let mut gzip = flate2::read::GzDecoder::new(packed.as_slice()).take(read_limit);
    let mut unpacked = Vec::new();
    gzip.read_to_end(&mut unpacked)
        .map_err(|error| Error::Compression(error.to_string()))?;
    if unpacked.len() > max_output_length {
        return Err(Error::MalformedMessage(
            "gzip_packed output exceeds configured limit",
        ));
    }
    if unpacked.is_empty() || unpacked.len() % 4 != 0 {
        return Err(Error::MalformedMessage(
            "gzip_packed object must be nonempty and 4-byte aligned",
        ));
    }
    Ok(unpacked)
}
