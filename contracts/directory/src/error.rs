use cosmwasm_std::{Coin, OverflowError, StdError};
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    AdminError(#[from] AdminError),

    #[error("Directory: Missing Fee of {fee:?}")]
    MissingFee { fee: Coin },
    #[error("Directory:No Funds Required")]
    NoFundsRequired {},
    #[error("Directory: Missing Fee of {fee:?}. received {supplied:?}")]
    InsufficientFee { fee: Coin, supplied: Coin },
    #[error("Directory: entry {name:?} already exists")]
    EntryExists { name: String },
    #[error("Directory: entry {name:?} does not exist")]
    EntryDoesntExist { name: String },

    #[error(
        "Directory: Unauthorized (action: {action:?}, expected: {expected:?}, actual: {actual:?})"
    )]
    Unauthorized {
        action: String,
        expected: String,
        actual: String,
    },
    #[error("ID: No pending ownership change")]
    NoPendingOwnerChanges,
}
