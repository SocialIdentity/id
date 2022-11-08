use cosmwasm_std::{OverflowError, StdError};
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdSharedError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    AdminError(#[from] AdminError),
    #[error("No Funds Required")]
    NoFundsRequired {},
    #[error("Invalid Fee type: None, Wallet or FeeSplit only")]
    InvalidFeeType {},
    #[error("Unauthorized (action: {action:?}, expected: {expected:?}, actual: {actual:?})")]
    Unauthorized {
        action: String,
        expected: String,
        actual: String,
    },
    #[error("No pending ownership change")]
    NoPendingOwnerChanges,
    #[error("Directory: blacklist names must be unique")]
    BlacklistNotUnique {},

    #[error("blacklist entry {name:?} already exists")]
    BlacklistEntryExists { name: String },
    #[error("blacklist entry {name:?} does not exist")]
    BlacklistEntryDoesntExist { name: String },
}
