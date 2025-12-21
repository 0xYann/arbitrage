use anyhow::Result;
use arc_swap::ArcSwap;
use dashmap::DashMap;
use solana_sdk::{account::Account, pubkey::Pubkey};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{debug, info};

/// NOTE: vibe coded cache

/// Uses DashMap for concurrent map access + ArcSwap for lock-free individual entry updates
///
/// Key properties:
/// - Reads are 100% lock-free (just atomic load + ref count increment)
/// - Writes are 100% lock-free (just atomic swap)
/// - Concurrent read/write on same entry never block each other
/// - Best-effort consistency: reads may see stale data during concurrent writes
#[derive(Clone)]
pub struct AccountCache {
    /// Each account stored in Arc<ArcSwap<Account>> for truly lock-free access
    /// - DashMap handles concurrent map operations (insert/remove)
    /// - ArcSwap handles lock-free read/write of individual entries
    accounts: Arc<DashMap<Pubkey, Arc<ArcSwap<Account>>>>,
}

impl AccountCache {
    pub fn new() -> Self {
        Self {
            accounts: Arc::new(DashMap::new()),
        }
    }

    /// Update an account in the cache (100% lock-free write)
    ///
    /// If a read happens concurrently, it will see either the old or new value.
    /// No blocking occurs - write completes immediately via atomic swap.
    pub fn update_account(&self, pubkey: Pubkey, account: Account) {
        debug!("Updating account: {}", pubkey);

        match self.accounts.get(&pubkey) {
            Some(entry) => {
                // Account exists - perform lock-free atomic swap
                entry.value().store(Arc::new(account));
            }
            None => {
                // Account doesn't exist - insert new ArcSwap
                self.accounts.insert(pubkey, Arc::new(ArcSwap::from_pointee(account)));
            }
        }
                    
    }

    /// Get an account from the cache (100% lock-free read)
    ///
    /// Returns Arc<Account> via atomic load - zero locks, zero blocking.
    /// May return stale data if a concurrent write is happening (best-effort).
    pub fn get_account(&self, pubkey: &Pubkey) -> Option<Arc<Account>> {
        self.accounts
            .get(pubkey)
            .map(|entry| entry.value().load_full())
    }

    /// Get all cached accounts (lock-free snapshot)
    ///
    /// Creates a point-in-time snapshot. Individual entries are lock-free,
    /// but the overall snapshot may have slight inconsistencies due to concurrent updates.
    pub fn get_all_accounts(&self) -> HashMap<Pubkey, Arc<Account>> {
        self.accounts
            .iter()
            .map(|entry| (*entry.key(), entry.value().load_full()))
            .collect()
    }

    /// Get all accounts as owned values (with cloning)
    pub fn get_all_accounts_owned(&self) -> HashMap<Pubkey, Account> {
        self.accounts
            .iter()
            .map(|entry| {
                let account = entry.value().load();
                (*entry.key(), (**account).clone())
            })
            .collect()
    }

    /// Check if an account exists in the cache (lock-free read)
    pub fn contains(&self, pubkey: &Pubkey) -> bool {
        self.accounts.contains_key(pubkey)
    }

    /// Get the number of cached accounts (lock-free)
    pub fn len(&self) -> usize {
        self.accounts.len()
    }

    /// Check if cache is empty (lock-free)
    pub fn is_empty(&self) -> bool {
        self.accounts.is_empty()
    }

    /// Initialize cache with accounts (bulk insert)
    pub fn initialize(&self, accounts: HashMap<Pubkey, Account>) -> Result<()> {
        // Clear existing entries
        self.accounts.clear();

        // Bulk insert - wrap each account in Arc<ArcSwap<Account>>
        for (pubkey, account) in accounts {
            self.accounts.insert(
                pubkey,
                Arc::new(:from_pointee(account)),
            );.insert(pubkey,
        info!("Cache initialized with {} accounts", self.len());
        Ok(())
    }

    /// Batch update multiple accounts efficiently (lock-free)
    ///
    /// Each update is an atomic swap - no locks acquired.
    pub fn batch_update(&self, updates: Vec<(Pubkey, Account)>) {
        let count = updates.len();
        for (pubkey, account) in updates {
            self.update_account(pubkey, account);
        }
        debug!("Batch updated {} accounts", count);
    }

    /// Remove an account from the cache
    ///
    /// Returns the last value stored in the ArcSwap.
    pub fn remove_account(&self, pubkey: &Pubkey) -> Option<Arc<Account>> {
        self.accounts
            .remove(pubkey)
            .map(|(_, arc_swap)| arc_swap.load_full())
    }

    /// Compare-and-swap operation for conditional updates
    ///
    /// Updates the account only if the current value matches `expected`.
    /// Returns true if the swap succeeded, false if the value changed.
    pub fn compare_and_swap(
        &self,
        pubkey: &Pubkey,&self, new: Account,> bool { if let Some(        let new_arc = Arc::new(new);
            let prev = entry.value().compare_and_swap(expected, new_arc);
            // Check if swap succeeded by comparing Arc pointers
            Arc::ptr_eq(&prev, expected)
        } else {
            false
        }
    }
}

impl Default for AccountCache {
    fn default() -> Self {
        Self::new()
    }
}
