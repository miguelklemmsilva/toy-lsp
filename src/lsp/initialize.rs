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

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ServerCapabilities {
    text_document_sync: Option<u16>,
    hover_provider: Option<bool>,
}

impl InitializeResult {
    pub fn new() -> Self {
        InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(1),
                hover_provider: Some(true),
            },
            server_info: Some(ServerInfo {
                name: String::from("toy-lsp"),
                version: Some(String::from("0.1.0")),
            }),
        }
    }
}
