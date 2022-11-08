use std::str::FromStr;

use cosmwasm_std::{BankMsg, CosmosMsg, DepsMut, MessageInfo, Response, StdError};
use cw20::Logo;
use id_shared::blacklist::init_blacklist;
use id_shared::fees::init_fee;
use id_shared::state::FEE;

use id_types::directory::{DirectoryRecord, EnsType, InstantiateMsg};
use id_types::shared::{FeeType, Socials};

use crate::contract::CONTRACT_NAME;
use crate::error::ContractError;
use crate::state::directory;
use id_shared::state::ADMIN;
pub fn instantiate(mut deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let admin_addr = msg
        .admin
        .map(|admin| deps.api.addr_validate(&admin))
        .transpose()?;
    ADMIN.set(deps.branch(), admin_addr)?;
    init_fee(
        deps.branch(),
        &msg.fee_account_type,
        &msg.fee_account,
        msg.fee,
    )?;

    init_blacklist(deps.branch(), msg.blacklist)?;

    Ok(Response::default())
}

// directory commands

pub fn add_directory_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    contract: String,
    ens_type: String,
    logo: Option<Logo>,
    socials: Option<Socials>,
) -> Result<Response, ContractError> {
    let ens =
        EnsType::from_str(&ens_type).map_err(|_| StdError::generic_err("Invalid ENS type"))?;

    let contract_addr = deps.api.addr_validate(&contract)?;
    let entry_exists = directory().may_load(deps.storage, name.clone())?;
    if let Some(entry) = entry_exists {
        return Err(ContractError::EntryExists { name: entry.name });
    }
    let fee_config = FEE.load(deps.storage)?;
    if fee_config.fee_account_type != FeeType::None {
        let fund_amt = info.funds.iter().find(|c| c.denom == fee_config.fee.denom);
        if let Some(fund_coin) = fund_amt {
            if fee_config.fee.amount > fund_coin.amount {
                return Err(ContractError::InsufficientFee {
                    fee: fee_config.fee,
                    supplied: fund_coin.clone(),
                });
            }
        } else if !fee_config.fee.amount.is_zero() {
            return Err(ContractError::MissingFee {
                fee: fee_config.fee,
            });
        }
    }
    let entry = DirectoryRecord {
        owner: info.sender,
        name: name.clone(),
        contract: contract_addr,
        ens_type: ens,
        logo,
        socials,
    };
    directory().save(deps.storage, name, &entry)?;

    let send_msgs = match fee_config.fee_account_type {
        FeeType::Wallet => vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: fee_config.fee_account.to_string(),
            amount: info.funds,
        })],
        FeeType::FeeSplit => {
            let msg = pfc_fee_split::fee_split_msg::ExecuteMsg::Deposit { flush: false };

            vec![msg.into_cosmos_msg(fee_config.fee_account, info.funds)?]
        }
        FeeType::None => vec![],
    };
    if send_msgs.is_empty() {
        Ok(Response::default()
            .add_attribute("action", format!("{}/add_directory_entry", CONTRACT_NAME)))
    } else {
        Ok(Response::default()
            .add_messages(send_msgs)
            .add_attribute("action", format!("{}/add_directory_entry", CONTRACT_NAME)))
    }
}

pub fn remove_directory_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let entry_exists = directory().may_load(deps.storage, name.clone())?;
    if let Some(entry) = entry_exists {
        if info.sender == entry.owner || ADMIN.is_admin(deps.as_ref(), &info.sender)? {
            directory().remove(deps.storage, name)?;

            Ok(Response::default().add_attribute(
                "action",
                format!("{}/remove_directory_entry", CONTRACT_NAME),
            ))
        } else {
            Err(ContractError::Unauthorized {
                action: "remove_directory_entry".to_string(),
                expected: entry.owner.to_string(),
                actual: info.sender.to_string(),
            })
        }
    } else {
        Err(ContractError::EntryDoesntExist { name })
    }
}

#[allow(clippy::too_many_arguments)]
pub fn update_directory_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    contract: String,
    ens_type: String,
    logo: Option<Logo>,
    socials: Option<Socials>,
    new_owner: Option<String>,
) -> Result<Response, ContractError> {
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let entry_exists = directory().may_load(deps.storage, name.clone())?;
    let is_admin = ADMIN.is_admin(deps.as_ref(), &info.sender)?;

    if let Some(mut entry) = entry_exists {
        if info.sender == entry.owner || is_admin {
            if let Some(new_owner_string) = new_owner {
                let new_owner_addr = deps.api.addr_validate(&new_owner_string)?;
                entry.owner = new_owner_addr;
            }

            let contract_addr = deps.api.addr_validate(&contract)?;

            let ens = EnsType::from_str(&ens_type)
                .map_err(|_| StdError::generic_err("Invalid ENS type"))?;
            entry.contract = contract_addr;
            entry.ens_type = ens;
            entry.logo = logo;
            entry.socials = socials;

            directory().save(deps.storage, name, &entry)?;

            Ok(Response::default().add_attribute(
                "action",
                format!("{}/update_directory_entry", CONTRACT_NAME),
            ))
        } else {
            Err(ContractError::Unauthorized {
                action: "update_directory_entry".to_string(),
                expected: entry.owner.to_string(),
                actual: info.sender.to_string(),
            })
        }
    } else {
        Err(ContractError::EntryDoesntExist { name })
    }
}
