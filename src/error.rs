pub use crate {error:: Error};
#[derive(Clone, Debug)]
pub enum TokenError{
    #[error("Lamport balance below rent-exempt threshold")]
    NotRentExempt,

    #[error("account not available")]
    AccountNotAvailable,

    #[error("Owner does not match")]
    OwnerMismatch,

    #[error("Invalid instruction")]
    InvalidInstruction,

    #[error("incorrect program id")]
    IncorrectProgramId,

    #[error("Not Enough Amount send")]
    NotEnoughAmountsend,

    #[error("no data to send")]
    NoDataToSend,

}

