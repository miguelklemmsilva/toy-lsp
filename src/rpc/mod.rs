use memchr::memmem::find;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{
    fmt,
    str::{self, from_utf8},
};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub enum DecodeError {
    MissingDelimiter,
    HeaderNotUtf8,
    MissingPrefix { found: String },
    InvalidLengthField { field: String },
    IncompleteBody { expected: usize, found: usize },
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecodeError::MissingDelimiter => write!(
                f,
                "Could not find the message separator \"\\r\\n\\r\\n\" — is the header terminated properly?"
            ),
            DecodeError::HeaderNotUtf8 => write!(f, "Header bytes are not valid UTF-8"),
            DecodeError::MissingPrefix { found } => write!(
                f,
                "Expected header to start with \"Content-Length: \", but found: `{}`",
                found
            ),
            DecodeError::InvalidLengthField { field } => write!(
                f,
                "Failed to parse the Content-Length value `{}` as a positive integer",
                field
            ),
            DecodeError::IncompleteBody { expected, found } => write!(
                f,
                "Declared Content-Length is {} but only {} bytes of body data were provided",
                expected, found
            ),
        }
    }
}

pub fn encode_message<T: Serialize>(msg_obj: &T) -> String {
    let payload = serde_json::to_string(msg_obj)
        .unwrap_or_else(|e| panic!("Failed to serialize message to JSON: {}", e));

    format!("Content-Length: {}\r\n\r\n{}", payload.len(), payload)
}

pub fn decode_message<T: DeserializeOwned>(buf: &[u8]) -> Result<T, DecodeError> {
    // find the "\r\n\r\n" boundary
    let header_end = find(buf, b"\r\n\r\n").ok_or(DecodeError::MissingDelimiter)?;

    // slice out header and parse as UTF-8
    let header = from_utf8(&buf[..header_end]).map_err(|_| DecodeError::HeaderNotUtf8)?;

    // check the prefix
    const PREFIX: &str = "Content-Length: ";
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

    let parsed_content: Result<T, serde_json::Error> =
        serde_json::from_slice(&buf[body_start..body_start + content_length]);

    Ok(parsed_content.unwrap())
}
