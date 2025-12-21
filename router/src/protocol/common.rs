use pinocchio::{
    account_info::AccountInfo,
    instruction::{Account, AccountMeta},
    log::sol_log_64,
    pubkey::Pubkey,
};
use pinocchio_token::state::TokenAccount;

pub trait Protocol {
    type Disc;

    const ID: u8;
    const ARG_LEN: usize;
    const PROGRAM_ID: &'static Pubkey;
    const DISC: Self::Disc;
    const DATA_LEN: usize;
    const ACCS_LEN: usize;

    fn from_bytes(data_idx: usize, data: &[u8]) -> &Self;
    fn get_ata_out_idx(&self) -> usize;
    fn invoke<'a>(&self, amount: u64, acc_idx: usize, account_infos: &'a [AccountInfo]);

    fn process<'a>(
        amount: u64,
        data_idx: usize,
        acc_idx: usize,
        data: &'a [u8],
        account_infos: &'a [AccountInfo],
    ) -> (u64, usize, usize) {
        let arg = Self::from_bytes(data_idx, data);

        let balance_account = unsafe { account_infos.get_unchecked(arg.get_ata_out_idx()) };
        let balance_ptr =
            unsafe { balance_account.borrow_data_unchecked().as_ptr().add(64) as *const u64 };

        let balance_before = unsafe { *balance_ptr };
        arg.invoke(amount, acc_idx, account_infos);
        let balance_after = unsafe { *balance_ptr };

        (
            balance_after - balance_before,
            data_idx + Self::ARG_LEN,
            acc_idx + 1 + Self::ACCS_LEN,
        )
    }
}
