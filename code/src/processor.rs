use{
    borsh::{BorshDeserialize},
    solana_program::{
        account_info::AccountInfo,
        entrypoint::ProgramResult,
        pubkey::Pubkey,
    },
};

use crate::instructions::{
    create::{create, CreateNftArgs},
    mint::initialize_mint,
};

pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let args = CreateNftArgs::try_from_slice(instruction_data)?;

    create(accounts, args)?;
    initialize_mint(accounts)
}