use crate::{
    error::TokenError
};
use borsh::{BorshSerialize, BorshDeserialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    decode_error::DecodeError,
    entrypoint::ProgramResult,
    instruction,
    msg,
    program::{invoke, invoke_signed},
    program_error::{PrintProgramError, ProgramError},
    pubkey::Pubkey,
    system_program,
    sysvar::{clock::Clock, fees::Fees, rent::Rent, Sysvar},
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct SendingAccount{
    amount:u64,
}
pub struct Processor {}
impl Processor {

    pub fn send_amount(
        program_id: &Pubkey, // Public key of the account the hello world program was loaded into
        accounts: &[AccountInfo], // The account to say hello to
        _input: &[u8])-> ProgramResult{

    let account_info_iter=& mut accounts.iter();
    let source_account_info=next_account_info(account_info_iter)?; // source account
    let dest_account_info=next_account_info(account_info_iter)?;// dest account
    let lamport=next_account_info(account_info_iter)?;// lamport send
    let send_data=next_account_info(account_info_iter)?; // program send data
    let system_program=next_account_info(account_info_iter)?;  //system program (used to creat account)

    if !source_account_info.is_signer {
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = Rent::get()?; //rent to be paid to store data.
    // if lamport==0{
    //     return Err(TokenError::NotEnoughAmountsend);    // is empty ko concept 
    // }
    if send_data.data_is_empty(){  // is empty vanpaxi afi new function creat garna parxa. yo function la account info mai define garako xa
        return Err(TokenError::NoDataToSend);
    }
    
    let mut sending_account = SendingAccount::try_from_slice(&accounts.data.borrow())?;

    
    
    sending_account.serialize(&mut &mut accounts.data.borrow_mut()[..])?;

    Ok(())
}

    
}
