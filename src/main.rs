use log::{info, warn};
use logger::init_logging;
use lsp::{
    Incoming, Notification, Request, Response,
    did_change::DidChangeTextDocumentParams,
    did_open::{self, DidOpenTextDocumentParams},
    hover::{HoverParams, HoverResponse},
    initialize::{InitializeParams, InitializeResult},
    state::State,
};
use rpc::{decode_message, errors::DecodeError, message_codec::MessageCodec};
use serde::Serialize;
use serde_json::{Value, from_value};
use tokio::io::{AsyncWriteExt, Stdout, stdin, stdout};
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;

mod logger;
mod lsp;
mod rpc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = FramedRead::new(stdin(), MessageCodec); // yields BytesMut frames
    let mut writer = stdout();
    let _ = init_logging();
    let mut state = State::new();

    info!("Logger started");

    while let Some(frame) = reader.next().await {
        let buf = frame?;
        let decoded_result: Result<Incoming<Value>, DecodeError> = decode_message(buf.as_ref());

        match decoded_result {
            Ok(decoded_message) => match decoded_message {
                Incoming::Request(req) => handle_request(req, &mut state, &mut writer).await,
                Incoming::Notification(not) => handle_notification(not, &mut state).await,
            },
            Err(err) => {
                warn!("Error decoding message: {err}");
                // don't panic, just go to next message
                continue;
            }
        };

        writer.flush().await?;
    }
    Ok(())
}

async fn handle_request(request: Request<Value>, state: &mut State, writer: &mut Stdout) {
    info!("Received message with method: {}", request.method);
    match request.method.as_str() {
        "initialize" => {
            let _ = from_value::<InitializeParams>(request.params)
                .map_err(|err| {
                    warn!("Failed to parse initialize request params {err}");
                    return;
                })
                .unwrap();

            let message = Response::new(request.id, InitializeResult::new());
            writer_response(writer, message).await;
        }
        "textDocument/hover" => {
            let hover_params = from_value::<HoverParams>(request.params)
                .map_err(|err| {
                    warn!("Failed to parse hover request params {err}");
                    return;
                })
                .unwrap();

            let message =
                Response::new(request.id, state.hover(hover_params));

            writer_response(writer, message).await;
        }
        unrecognized => warn!("Request type not recognized! Type: {}", unrecognized),
    }
}

async fn handle_notification(notification: Notification<Value>, state: &mut State) {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            let did_open_params = from_value::<DidOpenTextDocumentParams>(notification.params)
                .map_err(|err| {
                    warn!("Failed to parse did open notification params {err}");
                    return;
                })
                .unwrap();

            info!("Received notifcation params {:?}", did_open_params);
            state.open_document(
                did_open_params.text_document.uri,
                did_open_params.text_document.text,
            );
        }
        "textDocument/didChange" => {
            let did_change_params = from_value::<DidChangeTextDocumentParams>(notification.params)
                .map_err(|err| {
                    warn!("Failed to parse did open notification params {err}");
                    return;
                })
                .unwrap();
            let uri = did_change_params.text_document.uri;
            for change in did_change_params.content_changes.iter() {
                state.update_document(uri.clone(), change.text.clone());
            }
        }
        "initialized" => {
            info!("toy-lsp recognized!")
        }
        unrecognized => warn!("Notification type not recognized! Type: {}", unrecognized),
    }
}

async fn writer_response<T: Serialize>(writer: &mut Stdout, message: T) {
    let encoded_message = rpc::encode_message(&message);
    let _ = writer
        .write_all(encoded_message.as_bytes())
        .await
        .map_err(|err| warn!("Failed to write to std io to initialize {err}"));
}
