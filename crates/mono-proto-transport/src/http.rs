//! HTTP transport request and response envelopes.

use crate::common::packet;
use crate::{Error, Packet};

/// Build the HTTP transport request body and required protocol headers.
pub fn http_request(host: &str, payload: &[u8]) -> Vec<u8> {
    let head = format!(
        "POST /api HTTP/1.1\r\nHost: {host}\r\nContent-Type: application/x-www-form-urlencoded\r\nContent-Length: {}\r\nConnection: keep-alive\r\n\r\n",
        payload.len()
    );
    let mut result = head.into_bytes();
    result.extend_from_slice(payload);
    result
}

/// Extract one fixed-length HTTP response body. Chunked encoding is delegated to adapters.
pub fn http_response(input: &[u8], max_packet: usize) -> Result<Packet, Error> {
    let split = input
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
        .ok_or(Error::Http("missing header terminator"))?
        + 4;
    let header =
        std::str::from_utf8(&input[..split]).map_err(|_| Error::Http("header is not ASCII"))?;
    let status = header
        .lines()
        .next()
        .ok_or(Error::Http("missing status line"))?;
    if !status.contains(" 200 ") {
        return Err(Error::Http("non-success status"));
    }
    let length = header
        .lines()
        .find_map(|line| {
            line.split_once(':')
                .filter(|(name, _)| name.eq_ignore_ascii_case("content-length"))
                .map(|(_, value)| value.trim())
        })
        .ok_or(Error::Http("missing Content-Length"))?
        .parse::<usize>()
        .map_err(|_| Error::Http("invalid Content-Length"))?;
    packet(input, split, length, max_packet)
}
