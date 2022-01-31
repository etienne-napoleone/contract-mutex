use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: {}", message)]
    Unauthorized { message: String },

    #[error("Already locked by {}", address)]
    AlreadyLocked { address: Addr },

    #[error("Already unlocked")]
    AlreadyUnlocked,
}
