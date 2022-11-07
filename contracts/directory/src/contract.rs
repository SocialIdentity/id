use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use semver::Version;

use id_types::directory::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};

use crate::error::ContractError;
use crate::state::ADMIN;
use crate::{execute, query};

pub const CONTRACT_NAME: &str = "crates.io:social-id:directory";
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
        // ownership calls
        ExecuteMsg::TransferOwnership { new_owner, blocks } => {
            execute::transfer_ownership(deps, env, info.sender, new_owner, blocks)
        }
        ExecuteMsg::AcceptOwnership {} => execute::accept_ownership(deps, env, info.sender),
        // directory calls
        ExecuteMsg::AddDirectory {
            name,
            contract,
            ens_type,
            logo,
            socials,
        } => execute::add_directory_entry(deps, info, name, contract, ens_type, logo, socials),
        ExecuteMsg::RemoveDirectory { name } => execute::remove_directory_entry(deps, info, name),
        ExecuteMsg::UpdateDirectory {
            name,
            contract,
            ens_type,
            logo,
            socials,
            new_owner,
        } => execute::update_directory_entry(
            deps, info, name, contract, ens_type, logo, socials, new_owner,
        ),
        // fee calls
        ExecuteMsg::UpdateListingFee { fee } => execute::update_listing_fee(deps, info, fee),
        ExecuteMsg::UpdateListingFeeAccount {
            fee,
            fee_account_type,
            fee_account,
        } => execute::update_listing_fee_account(deps, info, fee, fee_account_type, fee_account),
    }
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query::config(deps)?),
        QueryMsg::Admin {} => to_binary(&ADMIN.query_admin(deps)?),
        QueryMsg::Entry { name } => to_binary(&query::entry(deps, name)?),
        QueryMsg::Entries { start_after, limit } => {
            to_binary(&query::entries(deps, start_after, limit)?)
        }
        QueryMsg::EntriesContract {
            contract,
            start_after,
            limit,
        } => to_binary(&query::entries_contract(
            deps,
            contract,
            start_after,
            limit,
        )?),
        QueryMsg::EntriesOwner {
            owner,
            start_after,
            limit,
        } => to_binary(&query::entries_owner(deps, owner, start_after, limit)?),
        QueryMsg::ReverseRecord { address } => to_binary(&query::reverse_record(deps, address)?),
        QueryMsg::Resolve { name } => query::resolve(deps, name),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let old_version: Version =
        cw_utils::ensure_from_older_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // do migration stuff here

    Ok(Response::new()
        .add_attribute("contract_name", CONTRACT_NAME)
        .add_attribute("previous_contract_version", old_version.to_string())
        .add_attribute("new_contract_version", CONTRACT_VERSION))
}
