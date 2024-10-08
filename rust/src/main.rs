use clap::Parser;
mod load;
mod fetch;
mod cli;
mod sqlite;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = cli::Cli::parse();

    match &cli.command {
        cli::Commands::Load { input_file } => {
            _ = load::vcf_to_sqlite(input_file).await;
        },
        cli::Commands::FetchId { vrs_id } => {
            _ = fetch::fetch_by_vrs_id(vrs_id.clone()).await;
        },
        cli::Commands::FetchRange { chr, start, end } => {
            _ = fetch::fetch_by_range(chr.clone(), *start, *end).await;
        }
    }
    Ok(())
}
