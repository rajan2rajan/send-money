// pub mod utils;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    system_program,
    sysvar::{rent::Rent, Sysvar},
};
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, Default, PartialEq)]
pub struct SendingAccount {
    amount: u64,
}

pub const PREFIX: &str = "withdraw_sol";
entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("hellow world");
    let account_info_iter = &mut accounts.iter();
    let source_account_info = next_account_info(account_info_iter)?; // source account
    let dest_account_info = next_account_info(account_info_iter)?; // dest account
    let lamport = next_account_info(account_info_iter)?; // lamport send
    let send_data = next_account_info(account_info_iter)?; // program send data
    let system_program = next_account_info(account_info_iter)?; //system program (used to creat account)

    if !source_account_info.is_signer {
        return Err(ProgramError::IncorrectProgramId);
    }
    if lamport.data_is_empty() {
        return Err(ProgramError::InsufficientFunds);
    }

    let rent = Rent::get()?; //rent to be paid to store data.
    if send_data.data_is_empty() {
        // is empty vanpaxi afi new function creat garna parxa. yo function la account info mai define garako xa
        return Err(ProgramError::InsufficientFunds);
    }

    let mut sending_account = SendingAccount::try_from_slice(&send_data.data.borrow())?;
    pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, lamports: u64) -> Instruction {
        let account_metas = vec![
            AccountMeta::new(*from_pubkey, true),
            AccountMeta::new(*to_pubkey, false),
        ];
        Instruction::new_with_bincode(
            system_program::id(),
            &SystemInstruction::Transfer { lamports },
            account_metas,
        )
    }

    invoke(
        &solana_program::system_instruction::transfer(
            source_account_info.key,
            dest_account_info.key,
            lamport,
        ),
        &[
            source_account_info.clone(),
            dest_account_info.clone(),
            system_program.clone(),
        ],
    )?;
    sending_account.serialize(&mut &mut send_data.data.borrow_mut()[..])?;

    Ok(())
}
