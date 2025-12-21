#![cfg_attr(not(test), no_std)]

use core::u8;

use crate::protocol::{common::Protocol, meteora_damm_v2::MeteoraDammV2, solfi_v2::SolFiV2};
use pinocchio::{
    account_info::AccountInfo, no_allocator, nostd_panic_handler, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

pub mod protocol;

// TODO: lazy_program_entrypoint?
program_entrypoint!(process_instruction);
no_allocator!();
nostd_panic_handler!();

pinocchio_pubkey::declare_id!("HtB4N4tBriiQ2nYUg1p1GYwx9cnJruRcLEt7nM1vnmPp");

#[inline(always)]
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8], // amount_in - 5 protocol disciminants - protocol data
) -> ProgramResult {
    let mut amount = unsafe { *(data.as_ptr() as *const u64) };
    let mut data_idx = 13;
    let mut acc_idx = 0;

    let ptr = data.as_ptr();
    for i in 8..13 {
        match unsafe { *ptr.add(i) } {
            MeteoraDammV2::ID => {
                (amount, data_idx, acc_idx) =
                    MeteoraDammV2::process(amount, data_idx, acc_idx, data, accounts);
            }
            SolFiV2::ID => {
                (amount, data_idx, acc_idx) =
                    SolFiV2::process(amount, data_idx, acc_idx, data, accounts);
            }
            _ => break,
        }
    }

    if amount < unsafe { *(data.as_ptr() as *const u64) } {
        return Err(ProgramError::Custom(0));
    }

    Ok(())
}
