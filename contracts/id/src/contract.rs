use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;

use crate::{execute, query};
use cw2::set_contract_version;
use id_shared::ownership;
use id_shared::state::ADMIN;
use id_types::id::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use semver::Version;

pub const CONTRACT_NAME: &str = "crates.io:social-id:id";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    execute::instantiate(deps, msg)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership { new_owner, blocks } => Ok(ownership::transfer_ownership(
            deps, env, info, new_owner, blocks,
        )?),
        ExecuteMsg::AcceptOwnership {} => Ok(ownership::accept_ownership(deps, env, info)?),
        ExecuteMsg::TBD { tbd } => execute::tbd(deps, info, tbd),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
        QueryMsg::Admin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::TBD { tbd } => to_binary(&query::tbd(deps, tbd)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let old_version: Version =
        cw_utils::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do migration stuff here
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::new()
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("previous_contract_version", old_version.to_string())
        .add_attribute("new_contract_version", CONTRACT_VERSION))
}
