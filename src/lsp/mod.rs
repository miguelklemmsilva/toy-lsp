use serde::{Deserialize, Serialize};

pub mod initialize;

#[derive(Deserialize, Debug)]
pub struct Request<P> {
    jsonrpc: String,
    pub id: u32,
    pub method: String,
    pub params: P
}

#[derive(Serialize, Debug)]
pub struct Response {
    jsonrpc: String,
    id: u32,
    result: Option<String>,
    error:  Option<ErrorCode>
}

#[derive(Serialize, Debug)]
pub enum ErrorCode {
    ParseError = -32700
}

#[derive(Deserialize, Debug)]
pub struct Notification {
    jsonrpc: String,
    method: String
}
