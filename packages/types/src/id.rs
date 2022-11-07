use crate::shared::NewOwner;
use cosmwasm_schema::{cw_serde, QueryResponses};
#[allow(unused_imports)]
use cosmwasm_std::Coin;
use cosmwasm_std::{Addr, Empty};
#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is updatable
    pub admin: Option<String>,
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

    TBD {
        tbd: String,
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

    #[returns(Coin)]
    TBD { tbd: String },
}

pub type MigrateMsg = Empty;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<Addr>,
    pub new_owner: Option<NewOwner>,
}
