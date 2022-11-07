use cosmwasm_std::Addr;
use cw_controllers::Admin;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex};

use id_types::directory::{DirectoryRecord, FeeConfig};
use id_types::shared::NewOwner;

pub(crate) const ADMIN_KEY: &str = "admin_001";
pub(crate) const NEW_ADMIN_KEY: &str = "new_admin_001";
pub(crate) const FEE_KEY: &str = "fee_001";
pub(crate) const DIRECTORY_KEY: &str = "directory_001";
pub(crate) const DIRECTORY_OWNER_KEY: &str = "directory_owner_001";
pub(crate) const DIRECTORY_CONTRACT_KEY: &str = "directory_contract_001";

pub const ADMIN: Admin = Admin::new(ADMIN_KEY);
pub const NEW_ADMIN: Item<NewOwner> = Item::new(NEW_ADMIN_KEY);
pub const FEE: Item<FeeConfig> = Item::new(FEE_KEY);

pub fn directory<'a>() -> IndexedMap<'a, String, DirectoryRecord, DirectoryIndexes<'a>> {
    IndexedMap::new(
        DIRECTORY_KEY,
        DirectoryIndexes {
            owner: MultiIndex::new(directory_owner_idx, DIRECTORY_KEY, DIRECTORY_OWNER_KEY),
            contract: MultiIndex::new(
                directory_contract_idx,
                DIRECTORY_KEY,
                DIRECTORY_CONTRACT_KEY,
            ),
        },
    )
}

pub struct DirectoryIndexes<'a>
where
//T: Serialize + DeserializeOwned + Clone,
{
    pub owner: MultiIndex<'a, Addr, DirectoryRecord, String>,
    pub contract: MultiIndex<'a, Addr, DirectoryRecord, String>,
}

impl<'a> IndexList<DirectoryRecord> for DirectoryIndexes<'a>
where
//    T: Serialize + DeserializeOwned + Clone,
{
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<DirectoryRecord>> + '_> {
        let v: Vec<&dyn Index<DirectoryRecord>> = vec![&self.owner, &self.contract];
        Box::new(v.into_iter())
    }
}

pub fn directory_owner_idx(_pk: &[u8], d: &DirectoryRecord) -> Addr {
    d.owner.clone()
}

pub fn directory_contract_idx(_pk: &[u8], d: &DirectoryRecord) -> Addr {
    d.contract.clone()
}
