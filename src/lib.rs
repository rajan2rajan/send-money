
pub mod Processor;
pub mod error;
pub mod utils;
use crate::{
    processor::Processor,
    error::TokenError
};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey,
    program_error::PrintProgramError,
};

pub const PREFIX:&str="withdraw_sol";
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = Processor::process(program_id, accounts, data) {
        // catch the error so we can print it
        error.print::<TokenError>();
        return Err(error);
    }
    Ok(())
}