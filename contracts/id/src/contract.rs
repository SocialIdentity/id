use cosmwasm_std::{entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::error::ContractError;
use crate::{execute, query};
use id_types::msg::{ExecuteMsg, QueryMsg, InstantiateMsg, MigrateMsg};
use cw2::{set_contract_version};
use crate::state::ADMIN;
use semver::Version;

pub const CONTRACT_NAME: &str = "crates.io:social-id";
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { admin } => execute::update_admin(deps, info, admin),
        ExecuteMsg::TBD {
            tbd,
        } => execute::tbd(deps, info, tbd),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Admin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::TBD {
            tbd,
        } => to_binary(&query::tbd(deps, tbd)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let old_version: Version = cw_utils::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;


    // do migration stuff here

    Ok(Response::new()
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("previous_contract_version", old_version.to_string())
        .add_attribute("new_contract_version", CONTRACT_VERSION))
}

