use pinocchio::{
    account_info::AccountInfo,
    cpi::invoke_unchecked,
    instruction::{Account, AccountMeta, Instruction},
    pubkey::Pubkey,
};
use pinocchio_pubkey::from_str;

use crate::protocol::common::Protocol;

pub const ATA_BASE_INDEX: usize = 6;
pub const ATA_QUOTE_INDEX: usize = 7;

#[repr(C)]
pub struct SolFiV2 {
    pub ta_out_idx: u8,
    pub quote_to_base: u8,
}

impl Protocol for SolFiV2 {
    type Disc = u8;

    const ID: u8 = 1;
    const ARG_LEN: usize = size_of::<Self>();
    const PROGRAM_ID: &Pubkey = &from_str("SV2EYYJyRz2YhfXwXnhNAevDEui5Q6yrfyo13WtupPF");
    const DISC: Self::Disc = 7u8;
    const DATA_LEN: usize = 18;
    const ACCS_LEN: usize = 13;

    #[inline(always)]
    fn from_bytes(data_idx: usize, data: &[u8]) -> &Self {
        // let data_view = &data[data_idx..data_idx + Self::ARG_LEN];
        unsafe { &*(data.as_ptr().add(data_idx) as *const Self) }
    }

    #[inline(always)]
    fn get_ata_out_idx(&self) -> usize {
        self.ta_out_idx as usize
    }

    #[inline(always)]
    fn invoke<'a>(&self, amount: u64, acc_idx: usize, account_infos: &'a [AccountInfo]) {
        let mut data = [0u8; Self::DATA_LEN];
        data[0] = Self::DISC;
        data[1..9].copy_from_slice(&amount.to_le_bytes());
        data[9..17].copy_from_slice(&0u64.to_le_bytes());
        data[17..18].copy_from_slice(&self.quote_to_base.to_le_bytes());

        let mut account_metas: [AccountMeta; Self::ACCS_LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };
        let mut accounts: [Account; Self::ACCS_LEN] =
            unsafe { core::mem::MaybeUninit::uninit().assume_init() };

        // Skip program id
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

        let instruction = Instruction {
            program_id: Self::PROGRAM_ID,
            data: &data,
            accounts: &account_metas,
        };

        unsafe {
            invoke_unchecked(&instruction, &accounts);
        }
    }
}
