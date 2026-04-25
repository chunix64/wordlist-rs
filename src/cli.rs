use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Txt,
    Json,
}

#[derive(ValueEnum, Clone)]
pub enum SortOrder {
    None,
    Asc,
    Desc,
}

impl OutputFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            OutputFormat::Txt => "txt",
            OutputFormat::Json => "json",
        }
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(required = true)]
    pub paths: Vec<PathBuf>,

    #[arg(short, long)]
    pub output: Option<PathBuf>,

    #[arg(long, value_enum, default_value = "txt")]
    pub output_format: OutputFormat,

    #[arg(short, long)]
    pub recursive: bool,

    #[arg(long, value_enum, default_value = "desc")]
    pub sort: SortOrder,
}
