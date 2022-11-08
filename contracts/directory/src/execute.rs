use std::str::FromStr;

use cosmwasm_std::{
    BankMsg, Coin, CosmosMsg, DepsMut, Env, Event, MessageInfo, Response, StdError,
};
use cw20::Logo;

use id_types::directory::{DirectoryRecord, EnsType, FeeConfig, FeeType, InstantiateMsg, Socials};
use id_types::shared::NewOwner;

use crate::contract::CONTRACT_NAME;
use crate::error::ContractError;
use crate::state::{directory, ADMIN, FEE, NEW_ADMIN};

pub fn instantiate(mut deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let admin_addr = msg
        .admin
        .map(|admin| deps.api.addr_validate(&admin))
        .transpose()?;
    ADMIN.set(deps.branch(), admin_addr)?;
    let fee_type = FeeType::from_str(&msg.fee_account_type)
        .map_err(|_| StdError::generic_err("Invalid Fee type: None, Wallet or FeeSplit only"))?;

    let fee_config = FeeConfig {
        fee_account_type: fee_type,
        fee_account: deps.api.addr_validate(&msg.fee_account)?,
        fee: msg.fee,
    };
    FEE.save(deps.storage, &fee_config)?;
    Ok(Response::default())
}

pub fn transfer_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: String,
    blocks: u64,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let new_admin = deps.api.addr_validate(&new_owner)?;

    let new_admin_record = NewOwner {
        new_owner: new_admin,
        block_height: env.block.height + blocks,
    };
    NEW_ADMIN.save(deps.storage, &new_admin_record)?;

    Ok(Response::new().add_attribute("action", format!("{}/transfer_ownership", CONTRACT_NAME)))
}

pub fn accept_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let new_admin_record_o = NEW_ADMIN.may_load(deps.storage)?;

    if let Some(new_admin_record) = new_admin_record_o {
        if new_admin_record.new_owner != info.sender {
            return Err(ContractError::Unauthorized {
                action: "accept_ownership".to_string(),
                expected: new_admin_record.new_owner.to_string(),
                actual: info.sender.to_string(),
            });
        }

        if new_admin_record.block_height < env.block.height {
            return Err(ContractError::Unauthorized {
                action: "accept_gov_contract expired".to_string(),
                expected: format!("{}", new_admin_record.block_height),
                actual: format!("{}", env.block.height),
            });
        }

        NEW_ADMIN.remove(deps.storage);
        ADMIN.set(deps, Some(new_admin_record.new_owner))?;

        let event = Event::new(format!("{}/ownership_transferred", CONTRACT_NAME))
            .add_attribute("new_owner", info.sender.to_string());

        Ok(Response::new()
            .add_event(event)
            .add_attribute("action", format!("{}/transfer_ownership", CONTRACT_NAME)))
    } else {
        Err(ContractError::NoPendingOwnerChanges)
    }
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

// fee commands
pub fn update_listing_fee(
    deps: DepsMut,
    info: MessageInfo,
    fee: Coin,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let mut fee_config = FEE.load(deps.storage)?;
    fee_config.fee = fee;
    FEE.save(deps.storage, &fee_config)?;

    Ok(
        Response::default()
            .add_attribute("action", format!("{}/update_listing_fee", CONTRACT_NAME)),
    )
}
pub fn update_listing_fee_account(
    deps: DepsMut,
    info: MessageInfo,
    fee: Coin,
    fee_account_type: String,
    fee_account: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }

    let fee_type = FeeType::from_str(&fee_account_type)
        .map_err(|_| StdError::generic_err("Invalid Fee type: None, Wallet or FeeSplit only"))?;
    let fee_account_addr = deps.api.addr_validate(&fee_account)?;
    let fee_config = FeeConfig {
        fee_account_type: fee_type,
        fee_account: fee_account_addr,
        fee,
    };
    FEE.save(deps.storage, &fee_config)?;

    Ok(Response::default().add_attribute(
        "action",
        format!("{}/update_listing_fee_account", CONTRACT_NAME),
    ))
}
