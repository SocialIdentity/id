use std::str::FromStr;

use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin};

#[cw_serde]
pub struct NewOwner {
    pub new_owner: Addr,
    pub block_height: u64,
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
pub struct ENSRecord {
    pub name: String,
    pub contract: Addr,
    pub token_id: String,
}

#[cw_serde]
pub struct ENSResponse<T> {
    pub entries: Vec<T>,
}

#[cw_serde]
pub struct BlacklistRecord {
    pub name: String,
    pub reason: Option<String>,
}
