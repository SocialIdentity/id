use cw_controllers::Admin;
use cw_storage_plus::Item;
use id_types::shared::NewOwner;

pub(crate) const ADMIN_KEY: &str = "admin_000";
pub(crate) const NEW_ADMIN_KEY: &str = "new_admin_000";

pub const ADMIN: Admin = Admin::new(ADMIN_KEY);
pub const NEW_ADMIN: Item<NewOwner> = Item::new(NEW_ADMIN_KEY);
