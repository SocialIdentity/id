use cosmwasm_std::{OverflowError, StdError};
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

    #[error("ID: Unauthorized (action: {action:?}, expected: {expected:?}, actual: {actual:?})")]
    Unauthorized {
        action: String,
        expected: String,
        actual: String,
    },
    #[error("ID: No pending ownership change")]
    NoPendingOwnerChanges,
}
