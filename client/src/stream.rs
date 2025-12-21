use std::sync::Arc;

use anyhow::Result;
use solana_commitment_config::CommitmentConfig;

use crate::{cache::Cache, client::Client};

pub async fn get_latest_blockhash_spinner(clients: &Arc<Client>, cache: &Arc<Cache>) -> Result<()> {
    loop {
        let hash = clients
            .rpc
            .get_latest_blockhash_with_commitment(CommitmentConfig::confirmed())
            .await
            .unwrap_or_default()
            .0;
        cache.latest_blockhash.store(Arc::new(hash));
        tracing::info!("Current blockhash: {:?}", hash);
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

// async fn stream_account_update(
//     clients: &Arc<Clients>,
//     cache: &Arc<Cache>,
//     account: &Pubkey,
// ) -> Result<()> {
//     let config = Some(RpcAccountInfoConfig {
//         encoding: Some(UiAccountEncoding::Base64),
//         data_slice: None,
//         commitment: Some(CommitmentConfig::processed()),
//         min_context_slot: None,
//     });
//     let (mut stream, _unsubscribe) = clients.sub.account_subscribe(&account, config).await?;
//     while let Some(event) = stream.next().await {
//         // tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
//         // let now = Utc::now();
//         // let block_time = clients
//         //     .rpc
//         //     .get_block_time(event.context.slot)
//         //     .await
//         //     .unwrap();
//         // let naive = NaiveDateTime::from_timestamp_opt(block_time, 0).unwrap();
//         // let datetime: DateTime<Utc> = DateTime::<Utc>::from_utc(naive, Utc);
//         // println!("MU - Emitted at  {:?}", datetime);
//         // println!("MU - Received at {:?}", now);
//         // println!("MU - Slot  {:?}", event.context.slot);

//         let decoded_data = event
//             .value
//             .account
//             .data
//             .decode()
//             .expect("Failed to decode account data");
//         let payload = decoded_data
//             .get(8..)
//             .expect("Account data too short to skip discriminator");
//         let pool_state =
//             PoolState::deserialize(&mut payload.as_ref()).expect("Failed to deserialize PoolState");
//         let key = Pubkey::from_str_const(event.value.pubkey.as_str());
//         let slot = event.context.slot;
//         match cache.state.entry(key) {
//             Entry::Occupied(e) => {
//                 let cell: Arc<ArcSwap<State>> = Arc::clone(e.get());
//                 drop(e);
//                 let cur = cell.load();
//                 if cur.last_updated < slot {
//                     cell.store(Arc::new(State {
//                         pool_state,
//                         last_updated: slot,
//                     }));
//                 }
//             }
//             Entry::Vacant(e) => {
//                 let cell = Arc::new(ArcSwap::from_pointee(State {
//                     pool_state,
//                     last_updated: slot,
//                 }));
//                 e.insert(cell);
//             }
//         }
//     }
//     Ok(())
// }
