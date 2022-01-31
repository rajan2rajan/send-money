use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_instruction::SystemInstruction,
    system_program,
};

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
// pub fn transfer(from_pubkey: &Pubkey, to_pubkey: &Pubkey, lamports: u64) -> Instruction {
//     let account_metas = vec![
//         AccountMeta::new(*from_pubkey, true),
//         AccountMeta::new(*to_pubkey, false),
//     ];
//     Instruction::new_with_bincode(
//         system_program::id(),
//         &SystemInstruction::Transfer { lamports },
//         account_metas,
//     )
// }

// here we are creating a new function this is not available in rust doc
// pub fn assert_with_msg(statement: bool, err: ProgramError, msg: &str) -> ProgramResult {
//     if !statement {
//         msg!(msg);
//         Err(err)
//     } else {
//         Ok(())
//     }
// }

// example like we use that in this condtion in the form of if else condtion
// TokenInstruction::Mint { amount } => {
//     msg!("Instruction: Mint");
//     let token_account_ai = next_account_info(accounts_iter)?;
//     let mint_ai = next_account_info(accounts_iter)?;
//     let mint_authority = next_account_info(accounts_iter)?;
//     let mut token_account = TokenAccount::load(token_account_ai)?;
//     let mut mint = Mint::load(mint_ai)?;
//     assert_with_msg(
//         mint_authority.is_signer,
//         ProgramError::MissingRequiredSignature,
//         "Mint Authority must sign",
//     )?;
//     assert_with_msg(
//         mint.authority == *mint_authority.key,
//         ProgramError::MissingRequiredSignature,
//         "Mint Authority mismatch",
//     )?;
