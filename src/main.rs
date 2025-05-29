use log::{info, warn};
use logger::init_logging;
use lsp::{
    did_open::{self, DidOpenTextDocumentParams}, initialize::{InitializeParams, InitializeResponse, InitializeResult}, Incoming, Notification, Request, Response
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

    info!("Logger started");

    while let Some(frame) = reader.next().await {
        let buf = frame?;
        let decoded_result: Result<Incoming<Value>, DecodeError> = decode_message(buf.as_ref());

        match decoded_result {
            Ok(decoded_message) => match decoded_message {
                Incoming::Request(req) => handle_request(req, &mut writer).await,
                Incoming::Notification(not) => handle_notification(not).await,
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

async fn handle_request(request: Request<Value>, writer: &mut Stdout) {
    info!("Received message with method: {}", request.method);
    match request.method.as_str() {
        "initialize" => {
            let message_params = from_value::<InitializeParams>(request.params)
                .map_err(|err| {
                    warn!("Failed to parse initialize request params {err}");
                    return;
                })
                .unwrap();

            let encoded_message = rpc::encode_message(&InitializeResponse::new(request.id));

            info!("Encoded message: {encoded_message}");
            let _ = writer
                .write_all(encoded_message.as_bytes())
                .await
                .map_err(|err| warn!("Failed to write to std io to initialize {err}"));
        }
        unrecognized => warn!("Message type not recognized! Type: {}", unrecognized),
    }
}

async fn handle_notification(notification: Notification<Value>) {
    match notification.method.as_str() {
        "textDocument/didOpen" => {
            let notification_params = from_value::<DidOpenTextDocumentParams>(notification.params)
                .map_err(|err| {
                    warn!("Failed to parse did open notificaiton params {err}");
                    return;
                })
                .unwrap();

                info!("Received notifcation params {:?}", notification_params)
        }
        unrecognized => warn!("Message type not recognized! Type: {}", unrecognized),
    }
}
