use log::{info, warn};
use logger::init_logging;
use lsp::{
    initialize::{InitializeParams, InitializeResponse, InitializeResult}, Request, Response
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
        let decoded_result: Result<Request<Value>, DecodeError> = decode_message(buf.as_ref());

        let decoded_message = match decoded_result {
            Ok(decoded_message) => decoded_message,
            Err(err) => {
                warn!("Error decoding message: {err}");
                // don't panic, just go to next message
                continue;
            }
        };

        handle_message(decoded_message, &mut writer).await;
        writer.flush().await?;
    }
    Ok(())
}

async fn handle_message(request: Request<Value>, writer: &mut Stdout) {
    info!("Received message with method: {}", request.method);
    match request.method.as_str() {
        "initialize" => {
            let message_params = from_value::<InitializeParams>(request.params)
                .map_err(|err| {
                    warn!("Failed to parse initialize request params {err}");
                    return;
                })
                .unwrap();

            info!("Initialized message received");
            info!(
                "Information: name: {},\n other: {:?}",
                message_params.client_info.name, message_params.client_info.version
            );
            let encoded_message = rpc::encode_message(&InitializeResponse::new(request.id));

            info!("Encoded message: {encoded_message}");
            let _ = writer
                .write_all(encoded_message.as_bytes())
                .await
                .map_err(|err| warn!("Failed to write to std io to initialize {err}"));
        }
        _ => warn!("Message type not recognized!"),
    }
}
