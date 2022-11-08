use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Empty};

use crate::shared::{FeeConfig, NewOwner};

#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is updatable
    pub admin: Option<String>,
    pub fee: Coin,
    /// type of account (wallet or some contract?)
    pub fee_account_type: String,
    /// Fee Account to send fees too
    pub fee_account: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Transfer ownership to another account; will not take effect unless the new owner accepts with 'blocks' amount of blocks
    TransferOwnership {
        new_owner: String,
        blocks: u64,
    },
    /// Accept an ownership transfer
    AcceptOwnership {},
    // fees
    UpdateListingFee {
        fee: Coin,
    },
    UpdateListingFeeAccount {
        fee: Coin,
        /// type of account (wallet or some contract?)
        fee_account_type: String,
        /// Fee Account to send fees too
        fee_account: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    /// Admin
    #[returns(Option < String >)]
    Admin {},
}

pub type MigrateMsg = Empty;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<Addr>,
    pub new_owner: Option<NewOwner>,
    pub fees: FeeConfig,
}
