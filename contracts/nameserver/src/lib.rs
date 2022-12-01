pub mod error;
pub mod execute;
pub mod query;
pub mod state;

pub use query::config;

use crate::error::ContractError;
use cosmwasm_std::{to_binary, Empty};
use cw2::set_contract_version;
use cw721_base::Cw721Contract;
pub use cw721_base::{
    ContractError as CW721ContractError, InstantiateMsg as CW721InstantiateMsg, MintMsg,
    MinterResponse,
};
use social_id_shared::state::ADMIN;
use social_id_types::nameserver::{Extension, NameServerExecuteMsg, NameServerQueryMsg};
// Version info for migration
const CONTRACT_NAME: &str = "crates.io:id-nameserver";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub type MintExtension = Extension;

pub type NameServerContract<'a> =
    Cw721Contract<'a, Extension, Empty, NameServerExecuteMsg, NameServerQueryMsg>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, NameServerExecuteMsg>;
pub type QueryMsg = cw721_base::QueryMsg<NameServerQueryMsg>;

pub mod entry {
    use super::*;

    use crate::query::{resolve, reverse_record, verifier, verifier_public_key, verifier_table};
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use social_id_shared::{blacklist, fees, ownership};
    use social_id_types::nameserver::{InstantiateMsg, MigrateMsg};
    use semver::Version;

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        let cw721inst = CW721InstantiateMsg {
            name: msg.name.clone(),
            symbol: msg.symbol.clone(),
            minter: msg.admin.clone(),
        };
        let res = NameServerContract::default().instantiate(deps.branch(), env, info, cw721inst)?;

        execute::instantiate(res, deps.branch(), msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint(msg) => execute::mint(deps, env, info, msg),
            ExecuteMsg::Extension { msg } => match msg {
                NameServerExecuteMsg::TransferOwnership { new_owner, blocks } => {
                    ownership::transfer_ownership(deps, env, info, new_owner, blocks)
                        .map_err(ContractError::IdSharedError)
                }
                NameServerExecuteMsg::AcceptOwnership {} => {
                    ownership::accept_ownership(deps, env, info)
                        .map_err(ContractError::IdSharedError)
                }
                NameServerExecuteMsg::UpdateListingFee { fee } => {
                    fees::update_listing_fee(deps, info, fee).map_err(ContractError::IdSharedError)
                }
                NameServerExecuteMsg::UpdateListingFeeAccount {
                    fee,
                    fee_account_type,
                    fee_account,
                } => {
                    fees::update_listing_fee_account(deps, info, fee, fee_account_type, fee_account)
                        .map_err(ContractError::IdSharedError)
                }
                NameServerExecuteMsg::VerifyName { .. } => {
                    // TODO store signatures in a new table/Extension
                    todo!()
                }
                NameServerExecuteMsg::AddVerifier {
                    name,
                    wallet,
                    public_key,
                } => execute::add_verifier_entry(deps, info, name, wallet, public_key),
                NameServerExecuteMsg::RemoveVerifier { name } => {
                    execute::remove_verifier_entry(deps, info, name)
                }
                NameServerExecuteMsg::UpdateVerifier {
                    name,
                    wallet,
                    public_key,
                } => execute::update_verifier_entry(deps, info, name, wallet, public_key),
            },
            _ => NameServerContract::default()
                .execute(deps, env, info, msg)
                .map_err(ContractError::CW721Base),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Extension { msg } => match msg {
                NameServerQueryMsg::Admin {} => to_binary(&ADMIN.query_admin(deps)?),
                NameServerQueryMsg::Config {} => to_binary(&config(deps)?),
                NameServerQueryMsg::Blacklist { name } => {
                    to_binary(&blacklist::query_entry(deps, name)?)
                }
                NameServerQueryMsg::Blacklists { start_after, limit } => {
                    to_binary(&blacklist::query_entries(deps, start_after, limit)?)
                }
                NameServerQueryMsg::ReverseRecord { address } => {
                    to_binary(&reverse_record(deps, env, address)?)
                }

                NameServerQueryMsg::Resolve { name } => resolve(deps, env, name),
                NameServerQueryMsg::Verifier { name } => to_binary(&verifier(deps, name)?),
                NameServerQueryMsg::Verifiers { start_after, limit } => {
                    to_binary(&verifier_table(deps, start_after, limit)?)
                }
                NameServerQueryMsg::VerifierPublicKey {
                    public_key,
                    start_after,
                    limit,
                } => to_binary(&verifier_public_key(deps, public_key, start_after, limit)?),
            },
            _ => NameServerContract::default().query(deps, env, msg),
        }
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
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
}
