use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};
use id_types::shared::BlacklistRecord;

use id_types::shared::{FeeConfig, NewOwner};

pub(crate) const ADMIN_KEY: &str = "admin_001";
pub(crate) const NEW_ADMIN_KEY: &str = "new_admin_001";

pub const ADMIN: Admin = Admin::new(ADMIN_KEY);
pub const NEW_ADMIN: Item<NewOwner> = Item::new(NEW_ADMIN_KEY);

pub const BLACKLIST_KEY: &str = "blacklist_001";
pub const BLACKLIST: Map<String, BlacklistRecord> = Map::new(BLACKLIST_KEY);

pub const FEE_KEY: &str = "fee_001";
pub const FEE: Item<FeeConfig> = Item::new(FEE_KEY);
