
use std::fs::File;

use log::LevelFilter;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

pub fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
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
