#[cfg(not(feature = "library"))]
mod error;
pub mod contract;
pub mod execute;
pub mod query;
pub mod state;

pub use crate::error::ContractError;
