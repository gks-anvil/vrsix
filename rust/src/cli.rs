use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Parquet {
        #[arg(value_name = "INPUT")]
        input_file: PathBuf,
    },
    Sqlite {
        #[arg(value_name = "INPUT")]
        input_file: PathBuf,
    }
}
