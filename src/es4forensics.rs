use anyhow::Result;
mod cli;
mod index;

mod index_builder;
mod timestamp;
mod utils;
mod ecs;
mod protocol;

use cli::{Cli, Action};
use elasticsearch::auth::Credentials;
use index_builder::*;
pub (crate) use protocol::*;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let builder = create_index_builder(&cli)?;

    match &cli.action {
        Action::CreateIndex => create_index(builder).await,
        Action::Import{bulk_size} => import(builder, *bulk_size).await
    }
}

async fn create_index(builder: IndexBuilder) -> Result<()> {
    Ok(())
}

async fn import(builder: IndexBuilder, bulk_size: usize) -> Result<()> {
    Ok(())
}

fn create_index_builder(cli: &Cli) -> Result<IndexBuilder> {
    let mut builder = IndexBuilder::with_name(cli.index_name.clone())
        .with_host(cli.host.clone())
        .with_port(cli.port)
        .with_credentials(Credentials::Basic(
            cli.username.clone(),
            cli.password.clone(),
        ))
        .with_protocol(cli.protocol.clone());

    if cli.omit_certificate_validation {
        log::warn!("disabling certificate validation");
        builder = builder.without_certificate_validation();
    }

    Ok(builder)
}