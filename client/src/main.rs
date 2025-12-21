use std::sync::Arc;

use anyhow::Result;
use tracing::Level;

use crate::{cache::Cache, client::Client, stream::get_latest_blockhash_spinner};

pub mod cache;
pub mod client;
pub mod stream;

// TODO: config threads and tasks
#[tokio::main]
async fn main() -> Result<()> {
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    tracing::info!("Initializing client");
    let clients = Arc::new(Client::new().await);

    tracing::info!("Initializing cache");
    let cache = Arc::new(Cache::new(25));

    tracing::info!("Starting blockhash spinner");
    let clients_clone = Arc::clone(&clients);
    let cache_clone = Arc::clone(&cache);
    let get_latest_blockhash_spinner = tokio::spawn(async move {
        if let Err(e) = get_latest_blockhash_spinner(&clients_clone, &cache_clone).await {
            eprintln!("Error in get_latest_blockhash_spinner: {:?}", e);
        }
    });

    get_latest_blockhash_spinner.await;

    Ok(())
}