use solana_program::{
    account_info::AccountInfo,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    system_program,
};
use std::error::Error;

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

fn try_from_slice(v: &[u8]) -> Result<Self, Err(ProgramError)> {
    let mut v_mut = v;
    let result = Self::deserialize(&mut v_mut)?;
    if !v_mut.is_empty() {
        return Err(ProgramError::InvalidDatas);
    }
    Ok(result)
}

pub fn create_transfer_unsigned<'a>(
    sender: &AccountInfo<'a>,
    receiver: &AccountInfo<'a>,
    system_program: &AccountInfo<'a>,
    amount: u64,
) -> ProgramResult {
    invoke(
        // we use invoke when someone will execute program for us.
        &system_instruction::transfer(sender.key, receiver.key, amount),
        &[sender.clone(), receiver.clone(), system_program.clone()],
    )
}
