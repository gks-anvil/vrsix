use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Load {
        #[arg(value_name = "INPUT")]
        input_file: PathBuf,
    },
    FetchId {
        #[arg(value_name = "VRS_ID")]
        vrs_id: String,
    },
    FetchRange {
        #[arg(value_name = "chr")]
        chr: String,

        #[arg(value_name = "start")]
        start: i64,

        #[arg(value_name = "end")]
        end: i64
    }
}
