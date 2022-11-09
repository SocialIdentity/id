use crate::shared::{BlacklistRecord, Socials};
#[allow(unused_imports)]
use crate::shared::{ENSRecord, ENSResponse};
use crate::shared::{FeeConfig, NewOwner};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, CustomMsg, Empty};

#[cw_serde]
pub struct InstantiateMsg {
    /// The admin is updatable
    pub admin: String,
    pub fee: Coin,
    /// type of account (wallet or some contract?)
    pub fee_account_type: String,
    /// Fee Account to send fees too
    pub fee_account: String,
    pub verification: bool,
    pub verification_keys: Vec<VerifyRecord>,
    /// verbotten names
    pub blacklist: Vec<BlacklistRecord>,
    // Name of the NFT contract
    pub name: String,
    // suffix of the name server (informational only)
    pub suffix: String,

    // Symbol of the NFT contract
    pub symbol: String,

    // turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_burn: bool,

    // turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_transfer: bool,
    ///NONE = verification doesn't expire
    pub verification_expiry: Option<u64>,
    /// NONE = no renewal.
    pub renewal_blocks: Option<u64>,
}

#[cw_serde]
pub enum NameServerExecuteMsg {
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

impl CustomMsg for NameServerExecuteMsg {}

#[cw_serde]
#[derive(QueryResponses)]
pub enum NameServerQueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    /// Admin
    #[returns(Option < String >)]
    Admin {},
    // Blacklisting
    #[returns(BlacklistRecord)]
    Blacklist { name: String },
    #[returns(ENSResponse<BlacklistRecord>)]
    Blacklists {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(VerifyRecord)]
    Verifier { name: String },
    #[returns(ENSResponse<VerifyRecord>)]
    Verifiers {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(ENSResponse<VerifyRecord>)]
    VerifierPublicKey {
        public_key: String,
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
impl CustomMsg for NameServerQueryMsg {}

pub type MigrateMsg = Empty;

#[cw_serde]
pub struct ConfigResponse {
    pub owner: Option<Addr>,
    pub new_owner: Option<NewOwner>,
    pub fees: FeeConfig,
    pub renewal_blocks: Option<u64>,
    pub verification: bool,
    pub verification_expiry: Option<u64>,
    pub owners_can_burn: bool,
    pub owners_can_transfer: bool,
    // suffix of the name server (informational only)
    pub suffix: String,
}

pub type Extension = Metadata;

#[cw_serde]
pub struct VerifyRecord {
    pub name: String,
    pub wallet: Addr,
    pub pub_key: String,
}

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub socials: Option<Socials>,
    /// block height that payment expires. NONE means no-expiry
    pub expires: Option<u64>,
    // following are used by verifier
    pub verified: Option<bool>,
    pub signature: Option<String>,
    pub verified_by: Option<Addr>,
    pub external_text: Option<String>,
    pub verification_url: Option<String>,
    /// block height that verification expires. NONE means no-expiry
    pub verification_expires: Option<u64>,
}
