use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;

#[cw_serde]
pub struct NewOwner {
    pub new_owner: Addr,
    pub block_height: u64,
}
