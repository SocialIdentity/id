use cosmwasm_schema::cw_serde;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};
use social_id_types::nameserver::VerifyRecord;

pub(crate) const VERIFIER_KEY: &str = "verifier_001";
pub(crate) const VERIFIER_OWNER_KEY: &str = "verifier_pubkey_001";
pub(crate) const NAMESERVER_CONFIG_KEY: &str = "nameserver_config_001";

pub const NAMESERVER_CONFIG: Item<NameServerConfig> = Item::new(NAMESERVER_CONFIG_KEY);

pub fn verifiers<'a>() -> IndexedMap<'a, String, VerifyRecord, VerifierIndexes<'a>> {
    IndexedMap::new(
        VERIFIER_KEY,
        VerifierIndexes {
            pubkey: MultiIndex::new(verify_pubkey_idx, VERIFIER_KEY, VERIFIER_OWNER_KEY),
        },
    )
}

pub struct VerifierIndexes<'a>
where
//T: Serialize + DeserializeOwned + Clone,
{
    pub pubkey: MultiIndex<'a, String, VerifyRecord, String>,
}

impl<'a> IndexList<VerifyRecord> for VerifierIndexes<'a>
where
//    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<VerifyRecord>> + '_> {
        let v: Vec<&dyn Index<VerifyRecord>> = vec![&self.pubkey];
        Box::new(v.into_iter())
    }
}

pub fn verify_pubkey_idx(_pk: &[u8], d: &VerifyRecord) -> String {
    d.pub_key.clone()
}

#[cw_serde]
pub struct NameServerConfig {
    /// true if verification is required. false otherwise
    pub verification: bool,
    /// true if verification can expire
    pub verification_expiry: Option<u64>,
    /// NONE = no renewal.
    pub renewal_blocks: Option<u64>,

    /// turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_burn: bool,
    /// turn this ON to allow holders of the nft to burn their tokens
    pub owners_can_transfer: bool,
    // suffix of the name server (informational only)
    pub suffix: String,
}
