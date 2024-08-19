use clap::Parser;
use crate::cli::Cli;

mod migrate;
mod cli;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli=Cli::parse();
    cli.run().await?;
    Ok(())
}
