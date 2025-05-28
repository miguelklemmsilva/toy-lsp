use errors::DecodeError;
use log::info;
use memchr::memmem::find;
use serde::Deserialize;
use serde::{Serialize, de::DeserializeOwned};
use std::io::{Error as IoError, ErrorKind};
use std::{
    fmt,
    str::{self, from_utf8},
};

#[cfg(test)]
mod tests;
pub mod message_codec;
mod errors;

const PREFIX: &str = "Content-Length: ";
const DELIMITER: &[u8] = b"\r\n\r\n";

#[derive(Deserialize, Debug)]
pub struct Request<P> {
    jsonrpc: String,
    id: u32,
    method: String,
    params: P
}

pub fn encode_message<T: Serialize>(msg_obj: &T) -> String {
    let payload = serde_json::to_string(msg_obj)
        .unwrap_or_else(|e| panic!("Failed to serialize message to JSON: {}", e));

    format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload)
}

pub fn decode_message<T: DeserializeOwned>(buf: &[u8]) -> Result<Request<T>, DecodeError> {
    // find the "\r\n\r\n" boundary
    let header_end = find(buf, DELIMITER).ok_or(DecodeError::MissingDelimiter)?;

    // slice out header and parse as UTF-8
    let header = from_utf8(&buf[..header_end]).map_err(|_| DecodeError::HeaderNotUtf8)?;

    // check the prefix
    if !header.starts_with(PREFIX) {
        return Err(DecodeError::MissingPrefix {
            found: header.to_string(),
        });
    }

    // grab & parse the length field
    let len_str = header[PREFIX.len()..].trim();
    let content_length = len_str
        .parse()
        .map_err(|_| DecodeError::InvalidLengthField {
            field: len_str.to_string(),
        })?;

    // make sure the buffer has enough bytes
    let body_start = header_end + 4; // skip past "\r\n\r\n"
    let available = buf.len().saturating_sub(body_start);
    if available < content_length {
        return Err(DecodeError::IncompleteBody {
            expected: content_length,
            found: available,
        });
    }

    serde_json::from_slice(&buf[body_start..body_start + content_length]).map_err(DecodeError::Json)
}

/// Try to split `data` into a complete message.
///
/// Returns:
/// - `Ok(Some((advance, &data[..advance])))` when you have a full frame.
/// - `Ok(None)` when you need more bytes.
/// - `Err(_)` if the header is malformed.
pub fn split_frame(data: &[u8]) -> Result<Option<(usize, &[u8])>, IoError> {
    // find the "\r\n\r\n" boundary
    let header_end = match find(data, DELIMITER) {
        Some(pos) => pos,
        None => return Ok(None),
    };

    // parse header as UTF-8
    let header = std::str::from_utf8(&data[..header_end])
        .map_err(|_| IoError::new(ErrorKind::InvalidData, "Header not UTF-8"))?;

    // must start with "Content-Length: "
    let len_str = header
        .strip_prefix(PREFIX)
        .ok_or_else(|| {
            IoError::new(
                ErrorKind::InvalidData,
                "Missing \"Content-Length: \" prefix",
            )
        })?
        .trim();

    // parse the length field
    let content_len: usize = len_str
        .parse()
        .map_err(|_| IoError::new(ErrorKind::InvalidData, "Invalid Content-Length"))?;

    // do we have enough bytes?
    let body_start = header_end + DELIMITER.len();
    if data.len() < body_start + content_len {
        return Ok(None);
    }

    let total = body_start + content_len;
    Ok(Some((total, &data[..total])))
}

