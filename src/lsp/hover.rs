use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HoverParams {
    /**
     * The text document.
     */
    pub text_document: TextDocumentIdentifier,

    /**
     * The position inside the text document.
     */
    position: Position,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /**
     * Line position in a document (zero-based).
     */
    line: u32,

    /**
     * Character offset on a line in a document (zero-based). The meaning of this
     * offset is determined by the negotiated `PositionEncodingKind`.
     *
     * If the character value is greater than the line length it defaults back
     * to the line length.
     */
    character: u32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    pub uri: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HoverResponse {
    contents: String,
}

impl HoverResponse {
    pub fn new(contents: String) -> Self {
        HoverResponse { contents }
    }
}
