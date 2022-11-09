use std::str::FromStr;

#[allow(unused_imports)]
use crate::shared::{BlacklistRecord, ENSRecord, ENSResponse};
use crate::shared::{FeeConfig, NewOwner, Socials};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Empty};
use cw20::Logo;
#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is updatable
    pub admin: Option<String>,
    pub fee: Coin,
    /// type of account (wallet or some contract?)
    pub fee_account_type: String,
    /// Fee Account to send fees too
    pub fee_account: String,
    /// verbotten domains
    pub blacklist: Vec<BlacklistRecord>,
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

    /// Add a directory entry
    AddDirectory {
        name: String,
        contract: String,
        ens_type: String,
        logo: Option<Logo>,
        socials: Option<Socials>,
    },
    /// Remove a directory entry
    RemoveDirectory {
        name: String,
    },
    /// update a directory entry
    UpdateDirectory {
        name: String,
        contract: String,
        ens_type: String,
        logo: Option<Logo>,
        socials: Option<Socials>,
        new_owner: Option<String>, // for admin use only
    },

    /// Add a blacklist entry
    AddBlacklist {
        name: String,
        reason: Option<String>,
    },
    /// Remove a blacklist entry
    RemoveBlacklist {
        name: String,
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

    #[returns(DirectoryRecord)]
    Entry { name: String },

    #[returns(ENSResponse<DirectoryRecord>)]
    Entries {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(ENSResponse<DirectoryRecord>)]
    EntriesContract {
        contract: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(ENSResponse<DirectoryRecord>)]
    EntriesOwner {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(BlacklistRecord)]
    Blacklist { name: String },
    #[returns(ENSResponse<BlacklistRecord>)]
    Blacklists {
        start_after: Option<String>,
        limit: Option<u32>,
    },
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
pub enum EnsType {
    Lns,
    Cw721,
}

impl FromStr for EnsType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LNS" => Ok(EnsType::Lns),
            "CW721" => Ok(EnsType::Cw721),
            _ => Err(()),
        }
    }
}

impl ToString for EnsType {
    fn to_string(&self) -> String {
        match &self {
            EnsType::Lns => String::from("LNS"),
            EnsType::Cw721 => String::from("CW721"),
        }
    }
}

#[cw_serde]
pub struct DirectoryRecord {
    pub owner: Addr,
    pub name: String,
    pub contract: Addr,
    pub ens_type: EnsType,
    pub logo: Option<Logo>,
    pub socials: Option<Socials>,
}
