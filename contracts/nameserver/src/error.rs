use cosmwasm_std::{Coin, OverflowError, StdError};
use cw_controllers::AdminError;
use social_id_shared::error::IdSharedError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),
    #[error("{0}")]
    IdSharedError(#[from] IdSharedError),

    #[error("{0}")]
    AdminError(#[from] AdminError),
    #[error("{0}")]
    CW721Base(#[from] cw721_base::ContractError),

    #[error("nameserver: Missing Fee of {fee:?}")]
    MissingFee { fee: Coin },
    #[error("nameserver: Missing name")]
    MissingName {},
    #[error("nameservery:No Funds Required")]
    NoFundsRequired {},
    #[error("nameserver: Missing Fee of {fee:?}. received {supplied:?}")]
    InsufficientFee { fee: Coin, supplied: Coin },
    #[error("nameserver: entry {name:?} already exists")]
    EntryExists { name: String },
    #[error("nameserver: entry {name:?} does not exist")]
    EntryDoesntExist { name: String },

    #[error(
        "nameserver: Unauthorized (action: {action:?}, expected: {expected:?}, actual: {actual:?})"
    )]
    Unauthorized {
        action: String,
        expected: String,
        actual: String,
    },
}
