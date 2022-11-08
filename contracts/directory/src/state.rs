use cosmwasm_std::Addr;
use cw_storage_plus::{Index, IndexList, IndexedMap, MultiIndex};

use id_types::directory::DirectoryRecord;

pub(crate) const DIRECTORY_KEY: &str = "directory_001";
pub(crate) const DIRECTORY_OWNER_KEY: &str = "directory_owner_001";
pub(crate) const DIRECTORY_CONTRACT_KEY: &str = "directory_contract_001";

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
