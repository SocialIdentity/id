use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Empty};

use crate::shared::{ENSRecord, ENSResponse};
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
    pub verification: bool,
    pub verification_keys: Vec<VerifyRecord>,
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
    // Fees
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
    // Verify
    /// Attempt a verification
    VerifyName {
        name: String,
        message: String,
        signature: String,
    },
    /// Add a public key that is can be used for verification
    AddVerifier {
        name: String,
        wallet: String,
        public_key: String,
    },
    /// Remove a verification signature
    RemoveVerifier {
        name: String,
    },
    /// Update a verification public key
    UpdateVerifier {
        name: String,
        wallet: String,
        public_key: String,
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

    // ENS interactions
    #[returns(ENSResponse<ENSRecord>)]
    ReverseRecord { address: String },
    // ENS interactions
    #[returns(ENSRecord)]
    Resolve { name: String },
}

pub type MigrateMsg = Empty;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<Addr>,
    pub new_owner: Option<NewOwner>,
    pub fees: FeeConfig,
}

#[cw_serde]
pub struct VerifyRecord {
    pub name: String,
    pub wallet: Addr,
    pub pub_key: Option<String>,
}
