use std::fs::File;

use log::{LevelFilter, info};
use rpc::{decode_message, message_codec::MessageCodec, Request};
use serde_json::Value;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};
use tokio::io::{AsyncWriteExt, Stdout, stdin, stdout};
use tokio_stream::StreamExt;
use tokio_util::codec::FramedRead;

mod rpc;

fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
    CombinedLogger::init(vec![
        // console (stderr) with colours
        TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        // file logger
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("app.log")?,
        ),
    ])?;
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = FramedRead::new(stdin(), MessageCodec); // yields BytesMut frames
    let mut writer = stdout();
    let _ = init_logging();

    info!("Logger started");

    while let Some(frame) = reader.next().await {
        let buf = frame?;

        let result: Request<Value> = decode_message(buf.as_ref()).unwrap();
        info!("result {:?}", result);
        writer.flush().await?;
    }
    Ok(())
}

async fn handle_message(writer: &mut Stdout, method: String, contents: Request<Value>) {

}
