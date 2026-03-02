use std::path::PathBuf;

use clap::{Args, Parser, Subcommand};

// mod config;
/// use std::{env, fs::File};

/// [rustc] (E0277) the trait bound `Command: FromStr` is not satisfied
// #[derive(Parser, Debug, PartialEq, Eq)]
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct TopLevelArgs {
    #[arg(short, long)]
    pub verbose: bool,
    #[arg(short, long)]
    pub force: bool,

    /// Name of the person to greet
    #[arg(short, long)]
    pub name: Option<String>,

    /// Path of config file
    #[arg(short, long)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    // reqcomand: Option<Command>,
    pub reqcomand: Commandx,
    // comand: Option<String>,
}

// impl Default for Options {
//     fn default() -> Self {
//         Self {
//             verbose: false,
//             force: false,
//             name: "World".to_string(),
//             index: "index".to_string(),
//             argcom: Some("World".to_string()),
//             reqcomand: Command::Entry,
//             comand: None,
//         }
//     }
// }

// #[derive(Debug, PartialEq, Eq, Clone, Subcommand)]
// #[derive(Debug,  Subcommand)]
#[derive(Subcommand, Debug)]
pub enum Commandx {
    Entry { comand: String },
    View { comand: Option<String> },
    Get(GetArgs),
}

// #[derive(Debug,Clone, Args)]
#[derive(Args, Debug, Clone)]
pub struct GetArgs {
    comand: String,
}
