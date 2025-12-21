use std::{env, time::Duration};

use dotenv::dotenv;
use solana_commitment_config::CommitmentConfig;
use solana_keypair::Keypair;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClient;
use solana_rpc_client::nonblocking::rpc_client::RpcClient;

pub struct Client {
    pub payer: Keypair,
    pub rpc: RpcClient,
    pub sub: PubsubClient,
}

impl Client {
    pub async fn new() -> Self {
        dotenv().ok();
        let json =
            env::var("PAYER_KEYPAIR").expect(&format!("{:?} not set in .env", "PAYER_KEYPAIR"));
        let bytes: Vec<u8> = serde_json::from_str(&json).expect("Invalid JSON for keypair");
        let payer = Keypair::try_from(bytes.as_slice()).expect("Invalid keypair bytes");
        let rpc_http_url = env::var("RPC_HTTP_URL").expect("RPC_HTTP_URL not set in .env");
        let rpc_wss_url = env::var("RPC_WSS_URL").expect("RPC_WSS_URL not set in .env");
        let timeout = Duration::new(20, 0);
        let commitment_config = CommitmentConfig::processed();
        let rpc = RpcClient::new_with_timeout_and_commitment(
            rpc_http_url.to_string(),
            timeout,
            commitment_config,
        );
        let sub = PubsubClient::new(rpc_wss_url.as_str())
            .await
            .expect("PubsubClient initialization failed");
        Self { payer, rpc, sub }
    }
}
