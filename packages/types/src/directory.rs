use std::str::FromStr;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Empty};
use cw20::Logo;

use crate::shared::NewOwner;

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
    /// Add a directory entry
    AddDirectory {
        name: String,
        contract: String,
        ens_type: String,
        logo: Option<Logo>,
        socials: Option<Socials>,
    },
    RemoveDirectory {
        name: String,
    },
    UpdateDirectory {
        name: String,
        contract: String,
        ens_type: String,
        logo: Option<Logo>,
        socials: Option<Socials>,
        new_owner: Option<String>, // for admin use only
    },
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

    #[returns(DirectoryRecord)]
    Entry { name: String },

    #[returns(DirectoryResponse)]
    Entries {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(DirectoryResponse)]
    EntriesContract {
        contract: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(DirectoryResponse)]
    EntriesOwner {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    // ENS interactions
    #[returns(ENSResponse)]
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
pub struct DirectoryResponse {
    pub entries: Vec<DirectoryRecord>,
}

#[cw_serde]
pub enum FeeType {
    None,
    Wallet,
    FeeSplit,
}

impl FromStr for FeeType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "None" => Ok(FeeType::None),
            "Wallet" => Ok(FeeType::Wallet),
            "FeeSplit" => Ok(FeeType::FeeSplit),
            _ => Err(()),
        }
    }
}

impl ToString for FeeType {
    fn to_string(&self) -> String {
        match &self {
            FeeType::Wallet => String::from("Wallet"),
            FeeType::FeeSplit => String::from("FeeSplit"),
            FeeType::None => String::from("None"),
        }
    }
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
pub struct FeeConfig {
    pub fee_account_type: FeeType,
    /// Account to send fees to
    pub fee_account: Addr,
    /// Current fee rate
    pub fee: Coin,
}

#[cw_serde]
pub struct Socials {
    pub project: Option<String>,
    pub description: Option<String>,
    pub email: Option<String>,
    pub twitter: Option<String>,
    pub telegraph: Option<String>,
    pub discord: Option<String>,
    pub web: Option<String>,
    pub github: Option<String>,
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
#[cw_serde]
pub struct ENSResponse {
    pub entries: Vec<ENSRecord>,
}
#[cw_serde]
pub struct ENSRecord {
    pub name: String,
    pub contract: Addr,
    pub token_id: String,
}
