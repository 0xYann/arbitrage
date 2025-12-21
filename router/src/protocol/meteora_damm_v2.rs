use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_unchecked,
    instruction::{Account, AccountMeta, Instruction},
    log::sol_log_compute_units,
    pubkey::Pubkey,
};
use pinocchio_pubkey::from_str;

use crate::protocol::common::Protocol;

pub const ATA_OUT_INDEX: usize = 3;

#[repr(C)]
pub struct MeteoraDammV2 {
    pub ta_out_idx: u8,
}

impl Protocol for MeteoraDammV2 {
    type Disc = [u8; 8];

    const ID: u8 = 0;
    const ARG_LEN: usize = size_of::<Self>();
    const PROGRAM_ID: &Pubkey = &from_str("cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG");
    const DISC: Self::Disc = [0xf8, 0xc6, 0x9e, 0x91, 0xe1, 0x75, 0x87, 0xc8];
    const DATA_LEN: usize = 24;
    const ACCS_LEN: usize = 14;

    #[inline(always)]
    fn from_bytes(data_idx: usize, data: &[u8]) -> &Self {
        let data_view = &data[data_idx..data_idx + Self::ARG_LEN];
        unsafe { &*(data_view.as_ptr() as *const Self) }
    }

    #[inline(always)]
    fn get_ata_out_idx(&self) -> usize {
        self.ta_out_idx as usize
    }

    #[inline(always)]
    fn invoke<'a>(&self, amount: u64, acc_idx: usize, account_infos: &'a [AccountInfo]) {
        let mut data = [0u8; Self::DATA_LEN];
        data[0..8].copy_from_slice(&Self::DISC);
        data[8..16].copy_from_slice(&amount.to_le_bytes());
        data[16..24].copy_from_slice(&0u64.to_le_bytes());

        let mut account_metas: [AccountMeta; Self::ACCS_LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut accounts: [Account; Self::ACCS_LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        // // Skip program id
        let ptr = unsafe { account_infos.as_ptr().add(acc_idx + 1) };

        macro_rules! process_account {
            ($idx:expr) => {
                let account_info = unsafe { &*ptr.add($idx) };
                account_metas[$idx] = AccountMeta::from(account_info);
                accounts[$idx] = Account::from(account_info);
            };
        }

        process_account!(0);
        process_account!(1);
        process_account!(2);
        process_account!(3);
        process_account!(4);
        process_account!(5);
        process_account!(6);
        process_account!(7);
        process_account!(8);
        process_account!(9);
        process_account!(10);
        process_account!(11);
        process_account!(12);
        process_account!(13);

        let instruction = Instruction {
            program_id: Self::PROGRAM_ID,
            data: &data,
            accounts: &account_metas,
        };

        sol_log_compute_units();
        unsafe {
            invoke_unchecked(&instruction, &accounts);
        }
    }
}
