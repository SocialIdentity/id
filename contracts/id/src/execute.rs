use cosmwasm_std::{DepsMut, MessageInfo, Response};
use id_shared::state::ADMIN;

use id_types::id::InstantiateMsg;

use crate::error::ContractError;

pub fn instantiate(mut deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let admin_addr = msg
        .admin
        .map(|admin| deps.api.addr_validate(&admin))
        .transpose()?;
    ADMIN.set(deps.branch(), admin_addr)?;
    Ok(Response::default())
}

pub fn tbd(_deps: DepsMut, _info: MessageInfo, _tbd: String) -> Result<Response, ContractError> {
    Ok(Response::default())
}
