use cosmwasm_std::{OverflowError, StdError};
use thiserror::Error;
use cw_controllers::AdminError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Overflow(#[from] OverflowError),

    #[error("{0}")]
    AdminError(#[from] AdminError),

    #[error("unauthorized: {reason}")]
    Unauthorized {
        reason: String,
    },

}

impl ContractError {
    pub fn unauthorized(reason: impl ToString) -> Self {
        Self::Unauthorized {
            reason: reason.to_string(),
        }
    }
}
