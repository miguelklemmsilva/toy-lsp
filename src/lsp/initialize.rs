use serde::{Deserialize, Serialize};

use super::{ErrorCode, Response};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeParams {
    pub client_info: ClientInfo,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    pub name: String,
    pub version: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResult {
    capabilities: ServerCapabilities,
    server_info: Option<ServerInfo>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    name: String,
    version: Option<String>,
}

// #[derive(Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
enum TextDocumentSyncKind {
    None = 0,
    Full = 1,
    Incremental = 2,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    text_document_sync: Option<u16>,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InitializeResponse {
    jsonrpc: String,
    id: u32,
    error: Option<ErrorCode>,
    result: InitializeResult,
}

impl InitializeResponse {
    pub fn new(id: u32) -> Self {
        InitializeResponse {
            jsonrpc: String::from("2.0"),
            id,
            error: None,
            result: InitializeResult {
                capabilities: ServerCapabilities {
                    text_document_sync: Some(1),
                },
                server_info: Some(ServerInfo {
                    name: String::from("toy-lsp"),
                    version: Some(String::from("0.1.0")),
                }),
            },
        }
    }
}
