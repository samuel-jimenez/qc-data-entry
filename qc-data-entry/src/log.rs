use std::{fs::File, path::PathBuf};

use anyhow::Result;
use simplelog::{
    ColorChoice, CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger,
};

pub fn init_logger(log_file: PathBuf) -> Result<()> {
    println!("[INFO] Logging to logfile: {}", log_file.display());
    Ok(CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Warn,
            Config::default(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            // File::create(log_file).unwrap(),
            File::options().append(true).create(true).open(log_file)?,
        ),
    ])?) //())
}
