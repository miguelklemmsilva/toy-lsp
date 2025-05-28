use core::fmt;


#[derive(Debug)]
pub enum DecodeError {
    MissingDelimiter,
    HeaderNotUtf8,
    MissingPrefix { found: String },
    InvalidLengthField { field: String },
    IncompleteBody { expected: usize, found: usize },
    Json(serde_json::Error),
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
            DecodeError::Json(err) => write!(f, "{}", err),
        }
    }
}

