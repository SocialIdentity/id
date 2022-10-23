use cosmwasm_std::{DepsMut, MessageInfo, Response};
use crate::error::ContractError;
use crate::state::{ADMIN};
use id_types::msg::{InstantiateMsg};

pub fn instantiate(mut deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let admin_addr = msg
        .admin
        .map(|admin| deps.api.addr_validate(&admin))
        .transpose()?;
    ADMIN.set(deps.branch(), admin_addr)?;
    Ok(Response::default())
}

pub fn update_admin(
    deps: DepsMut,
    info: MessageInfo,
    admin: Option<String>,
) -> Result<Response, ContractError> {
    let validated_admin_address = if let Some(admin_s) = admin {
        Some(deps.api.addr_validate(&admin_s).unwrap())
    } else {
        None
    };
    Ok(ADMIN.execute_update_admin(deps, info, validated_admin_address)?)
}

pub fn tbd(
    _deps: DepsMut,
    _info: MessageInfo,
    _tbd: String,
) -> Result<Response, ContractError> {
    Ok(Response::default())
}
