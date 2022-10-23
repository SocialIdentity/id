use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Coin, Empty};

#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is updatable
    pub admin: Option<String>,
}

#[cw_serde]
pub enum ExecuteMsg {
    UpdateAdmin {
        admin: Option<String>,
    },
    TBD {
        tbd: String,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Admin
    #[returns(Option < String >)]
    Admin {},

    #[returns(Coin)]
    TBD {
        tbd: String,
    },
}

pub type MigrateMsg = Empty;
