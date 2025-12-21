use std::sync::{atomic::AtomicU64, Arc};

use arc_swap::ArcSwap;
use dashmap::DashMap;
use solana_hash::Hash;
use solana_pubkey::Pubkey;

#[derive(Default, Debug)]
pub struct Cache {
    pub latest_blockhash: ArcSwap<Hash>,
    pub latest_slot: AtomicU64,
    pub state: DashMap<Pubkey, Vec<u8>>,
}

#[derive(Clone, Default, Debug)]
pub struct AccountData {
    data: Vec<u8>,
    slot: u64,
    write_version: u64,
}

impl Cache {
    pub fn new(expected_accounts: usize) -> Self {
        Self {
            latest_blockhash: ArcSwap::new(Arc::new(Hash::default())),
            latest_slot: AtomicU64::new(0),
            state: DashMap::with_capacity(expected_accounts),
        }
    }

    pub fn get_account(&self, pubkey: &Pubkey) -> Option<Vec<u8>> {
        self.state.get(pubkey).map(|entry| entry.value().clone())
    }

    pub fn update_account(&self, pubkey: Pubkey, data: Vec<u8>) {
        self.state.entry(pubkey).insert(data);
    }
}
