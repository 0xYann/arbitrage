use crate::adapters::common::{before_check, invoke_process};
use crate::error::ErrorCode;
use crate::{solfi_program, solfi_v2_program, HopAccounts};
use anchor_lang::{prelude::*, solana_program::instruction::Instruction};
use anchor_spl::token::Token;
use anchor_spl::token_interface::{TokenAccount, TokenInterface};
use arrayref::array_ref;

use super::common::DexProcessor;

const ARGS_LEN: usize = 18;

pub struct SolfiProcessor;
impl DexProcessor for SolfiProcessor {}

pub fn build_instruction<'a>(
    remaining_accounts: &'a [AccountInfo<'a>],
    amount_in: u64,
    offset: &mut usize,
    hop_accounts: &mut HopAccounts,
    hop: usize,
    proxy_swap: bool,
    owner_seeds: Option<&[&[&[u8]]]>,
) -> Instruction {
    require!(
        remaining_accounts.len() >= *offset + V2_ACCOUNTS_LEN,
        ErrorCode::InvalidAccountsLength
    );
    let mut swap_accounts = SolfiAccountV2::parse_accounts(remaining_accounts, *offset)?;
    if swap_accounts.dex_program_id.key != &solfi_v2_program::id() {
        return Err(ErrorCode::InvalidProgramId.into());
    }
    // log pool address
    swap_accounts.market.key().log();

    let (direction, user_base_token_account, user_quote_token_account) =
        if swap_accounts.swap_source_token.mint == swap_accounts.base_mint.key()
            && swap_accounts.swap_destination_token.mint == swap_accounts.quote_mint.key()
        {
            (
                0u8,
                swap_accounts.swap_source_token.clone(),
                swap_accounts.swap_destination_token.clone(),
            )
        } else if swap_accounts.swap_source_token.mint == swap_accounts.quote_mint.key()
            && swap_accounts.swap_destination_token.mint == swap_accounts.base_mint.key()
        {
            (
                1u8,
                swap_accounts.swap_destination_token.clone(),
                swap_accounts.swap_source_token.clone(),
            )
        } else {
            return Err(ErrorCode::InvalidTokenMint.into());
        };

    let mut data = Vec::with_capacity(ARGS_LEN);
    data.push(7u8); //discriminator
    data.extend_from_slice(&amount_in.to_le_bytes()); //amount_in
    data.extend_from_slice(&1u64.to_le_bytes());
    data.extend_from_slice(&direction.to_le_bytes()); //swap direction

    let accounts = vec![
        AccountMeta::new(swap_accounts.swap_authority_pubkey.key(), true),
        AccountMeta::new(swap_accounts.market.key(), false),
        AccountMeta::new_readonly(swap_accounts.oracle.key(), false),
        AccountMeta::new_readonly(swap_accounts.global_config_account.key(), false),
        AccountMeta::new(swap_accounts.base_vault.key(), false),
        AccountMeta::new(swap_accounts.quote_vault.key(), false),
        AccountMeta::new(user_base_token_account.key(), false),
        AccountMeta::new(user_quote_token_account.key(), false),
        AccountMeta::new_readonly(swap_accounts.base_mint.key(), false),
        AccountMeta::new_readonly(swap_accounts.quote_mint.key(), false),
        AccountMeta::new_readonly(swap_accounts.base_token_program.key(), false),
        AccountMeta::new_readonly(swap_accounts.quote_token_program.key(), false),
        AccountMeta::new_readonly(swap_accounts.instruction_sysvar.key(), false),
    ];

    Instruction {
        program_id: swap_accounts.dex_program_id.key(),
        accounts,
        data,
    }
}
