use std::fs;

use mollusk_svm::{program::loader_keys::LOADER_V3, Mollusk};
use router::ID;
use serde_derive::Deserialize;
use solana_account_decoder::UiAccount;
use solana_sdk::{
    account::Account,
    message::{AccountMeta, Instruction},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
};
use spl_associated_token_account::get_associated_token_address;

#[cfg(test)]
pub mod tests {
    use mollusk_svm::instructions_sysvar;
    use mollusk_svm_bencher::MolluskComputeUnitBencher;

    use super::*;

    #[test]
    fn test() {
        let program_id = Pubkey::new_from_array(ID);

        let mut mollusk = Mollusk::new(&program_id, "../target/deploy/router");
        mollusk.add_program_with_loader_and_elf(
            &Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"),
            &LOADER_V3,
            include_bytes!("snapshot/programs/cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG.so"),
        );
        mollusk.add_program_with_loader_and_elf(
            &Pubkey::from_str_const("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF"),
            &LOADER_V3,
            include_bytes!("snapshot/programs/SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF.so"),
        );
        mollusk_svm_programs_token::token::add_program(&mut mollusk);
        mollusk.sysvars.clock.unix_timestamp = 1767360940;

        let base_mint = Pubkey::from_str_const("So11111111111111111111111111111111111111112");
        let quote_mint = Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

        let signer = Pubkey::new_unique();
        let signer_base_ta = get_associated_token_address(&signer, &base_mint);
        let signer_quote_ta = get_associated_token_address(&signer, &quote_mint);

        let mut data: Vec<u8> = Vec::new();
        // amount - 1 sol
        data.extend_from_slice(&1_000_000_000u64.to_le_bytes());
        // meteoraDammV2 discriminant
        data.push(0u8);
        // solfiV2 disciminant
        data.push(1u8);
        // remaining
        data.push(42u8);
        // remaining
        data.push(42u8);
        // remaining
        data.push(42u8);
        // meteoraDammV2 ta out index
        data.push(4);
        // solfiV2 ta out index
        data.push(22);
        // solfiV2 quote_to_base
        data.push(1);

        let accounts = vec![
            // meteoraDamm accounts
            AccountMeta::new_readonly(
                Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("8Pm2kZpnxD3hoMmt4bjStX2Pw2Z9abpbHzZxMPqxPmie"),
                false,
            ),
            AccountMeta::new(signer_base_ta, false),
            AccountMeta::new(signer_quote_ta, false),
            AccountMeta::new(
                Pubkey::from_str_const("sx8hCMCauCdbZ7sVBGSJmH7b7JmtuN8d8YwYmBpuPLH"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("8S8HjmPZr8tNNEmMj5pcqS5RN73uF6DmcUDEDaoUQ1Ei"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"),
                false,
            ),
            // solfiV2 accounts
            AccountMeta::new_readonly(
                Pubkey::from_str_const("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF"),
                false,
            ),
            AccountMeta::new(signer, true),
            AccountMeta::new(
                Pubkey::from_str_const("65ZHSArs5XxPseKQbB1B4r16vDxMWnCxHMzogDAqiDUc"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("2ny7eGyZCoeEVTkNLf5HcnJFBKkyA4p4gcrtb3b8y8ou"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("FmxXDSR9WvpJTCh738D1LEDuhMoA8geCtZgHb3isy7Dp"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("CRo8DBwrmd97DJfAnvCv96tZPL5Mktf2NZy2ZnhDer1A"),
                false,
            ),
            AccountMeta::new(
                Pubkey::from_str_const("GhFfLFSprPpfoRaWakPMmJTMJBHuz6C694jYwxy2dAic"),
                false,
            ),
            AccountMeta::new(signer_base_ta, false),
            AccountMeta::new(signer_quote_ta, false),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("So11111111111111111111111111111111111111112"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                Pubkey::from_str_const("Sysvar1nstructions1111111111111111111111111"),
                false,
            ),
        ];

        let instruction = Instruction::new_with_bytes(program_id, &data, accounts);

        let mut sim_accounts = vec![];
        sim_accounts.push((
            signer,
            Account::new(
                1 << 42,
                0,
                &Pubkey::from_str_const("11111111111111111111111111111111"),
            ),
        ));
        sim_accounts.push((
            Pubkey::from_str_const("HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC"),
            Account::new(
                0,
                0,
                &Pubkey::from_str_const("11111111111111111111111111111111"),
            ),
        ));
        sim_accounts.push((
            Pubkey::from_str_const("3rmHSu74h1ZcmAisVcWerTCiRDQbUrBKmcwptYGjHfet"),
            Account::new(
                0,
                0,
                &Pubkey::from_str_const("11111111111111111111111111111111"),
            ),
        ));
        sim_accounts.extend_from_slice(
            create_mint_and_ata_account(&signer, &base_mint, true, 9).as_slice(),
        );
        sim_accounts.extend_from_slice(
            create_mint_and_ata_account(&signer, &quote_mint, true, 6).as_slice(),
        );
        sim_accounts.extend_from_slice(get_sim_accounts().as_slice());
        sim_accounts.push(mollusk_svm_programs_token::token::keyed_account());
        sim_accounts.push(instructions_sysvar::keyed_account(
            [instruction.clone()].iter(),
        ));

        mollusk.process_instruction(&instruction, &sim_accounts);

        // MolluskComputeUnitBencher::new(mollusk)
        //     .bench(("bench0", &instruction, &sim_accounts))
        //     .must_pass(true)
        //     .out_dir("../target/benches")
        //     .execute();
    }
}

pub fn create_mint_and_ata_account(
    owner: &Pubkey,
    mint: &Pubkey,
    is_native: bool,
    decimals: u8,
) -> Vec<(Pubkey, Account)> {
    let rent = Rent::default();
    let ata_rent = rent.minimum_balance(spl_token::state::Account::LEN);
    let mut accounts = Vec::new();
    let ata = get_associated_token_address(owner, mint);
    let initial_balance = 1 << 42;
    let token_account = spl_token::state::Account {
        mint: *mint,
        owner: *owner,
        amount: initial_balance,
        delegate: None.into(),
        state: spl_token::state::AccountState::Initialized,
        is_native: if is_native { Some(ata_rent) } else { None }.into(),
        delegated_amount: 0,
        close_authority: None.into(),
    };
    let mut data = vec![0; spl_token::state::Account::LEN];
    token_account.pack_into_slice(&mut data);
    accounts.push((
        ata,
        Account {
            lamports: if is_native {
                ata_rent + initial_balance
            } else {
                ata_rent
            },
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: 0,
        }
        .into(),
    ));
    let mint_acc = spl_token::state::Mint {
        mint_authority: None.into(),
        supply: initial_balance,
        decimals,
        is_initialized: true,
        freeze_authority: None.into(),
    };
    let mut data = vec![0; spl_token::state::Mint::LEN];
    mint_acc.pack_into_slice(&mut data);
    accounts.push((
        *mint,
        Account {
            lamports: rent.minimum_balance(data.len()),
            data,
            owner: spl_token::ID,
            executable: false,
            rent_epoch: 0,
        }
        .into(),
    ));

    accounts
}

pub fn get_sim_accounts() -> Vec<(Pubkey, Account)> {
    #[derive(Deserialize)]
    struct AccountWithKey {
        pub pubkey: String,
        pub account: UiAccount,
    }

    let entries = fs::read_dir("../router/tests/snapshot/accounts").unwrap();
    let mut accounts = vec![];
    for entry in entries {
        let path = entry.unwrap().path();
        let account_with_key: AccountWithKey =
            serde_json::from_reader(fs::File::open(path).unwrap()).unwrap();
        let pubkey = Pubkey::from_str_const(&account_with_key.pubkey);
        let account = account_with_key.account.decode::<Account>().unwrap();
        accounts.push((pubkey, account));
    }

    accounts
}
