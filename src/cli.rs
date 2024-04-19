use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum Language {
    Node
}

impl Default for Language {
    fn default() -> Self {
        Self::Node
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Scan development depends on the cache root path
    #[arg(short, long, value_name = "FILE")]
    pub path: Option<PathBuf>,
    /// Select the language you want to clear
    #[arg(short, long, value_enum)]
    pub lang: Language
}