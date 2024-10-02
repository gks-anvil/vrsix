use clap::Parser;

mod cli;
mod parquet;
mod sqlite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Parquet { input_file } => {
            _ = parquet::to_parquet(input_file).await;
        },
        cli::Commands::Sqlite { input_file } => {
            _ = sqlite::vcf_to_sqlite(input_file).await;
        }
    }
    Ok(())
}
