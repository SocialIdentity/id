use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use cw721_base::{ExecuteMsg, MintMsg};
use id_shared::blacklist::{init_blacklist, is_blacklisted};
use id_shared::fees::{gen_fees, init_fee};

use id_types::nameserver::{InstantiateMsg, VerifyRecord};

use crate::error::ContractError;
use crate::state::{verifiers, NameServerConfig, NAMESERVER_CONFIG};
use crate::{MintExtension, NameServerContract};
use id_shared::state::ADMIN;

pub fn instantiate(
    resp: Response,
    mut deps: DepsMut,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let admin_addr = deps.api.addr_validate(&msg.admin)?;
    ADMIN.set(deps.branch(), Some(admin_addr))?;
    init_fee(
        deps.branch(),
        &msg.fee_account_type,
        &msg.fee_account,
        msg.fee,
    )?;

    init_blacklist(deps.branch(), msg.blacklist)?;
    NAMESERVER_CONFIG.save(
        deps.storage,
        &NameServerConfig {
            verification: msg.verification,
            verification_expiry: msg.verification_expiry,
            renewal_blocks: msg.renewal_blocks,
            owners_can_burn: msg.owners_can_burn,
            owners_can_transfer: msg.owners_can_transfer,
            suffix: msg.suffix,
        },
    )?;
    Ok(resp)
}

pub fn mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    mintmsg: MintMsg<MintExtension>,
) -> Result<Response, ContractError> {
    let contract = NameServerContract::default();
    let send_msgs = gen_fees(deps.as_ref(), &info.funds)?;
    let nameserver_config = NAMESERVER_CONFIG.load(deps.storage)?;
    let mut mint_msg_modified: MintMsg<MintExtension> = mintmsg.clone();
    if mintmsg.extension.name.is_none() || mintmsg.extension.name.as_ref().unwrap().is_empty() {
        return Err(ContractError::MissingName {});
    }
    if mintmsg.token_id.is_empty() {
        mint_msg_modified.token_id = mintmsg.extension.name.unwrap();
    }
    mint_msg_modified.token_id.make_ascii_lowercase();

    is_blacklisted(deps.as_ref(), &mint_msg_modified.token_id)?;
    is_blacklisted(
        deps.as_ref(),
        &mint_msg_modified
            .extension
            .name
            .as_ref()
            .unwrap()
            .to_ascii_lowercase(),
    )?;

    mint_msg_modified.token_uri = None;
    if nameserver_config.verification {
        mint_msg_modified.extension.verified = Some(false);
        mint_msg_modified.extension.verification_expires = None;
        mint_msg_modified.extension.verification_url = None;
        mint_msg_modified.extension.verified_by = None;
        mint_msg_modified.extension.external_text = None;
        mint_msg_modified.extension.signature = None;
    }
    if let Some(renew_blocks) = nameserver_config.renewal_blocks {
        mint_msg_modified.extension.expires = Some(renew_blocks + env.block.height)
    } else {
        mint_msg_modified.extension.expires = None
    }
    mint_msg_modified.owner = info.sender.to_string();
    let fake_info = MessageInfo {
        sender: contract.minter.load(deps.storage)?,
        funds: vec![],
    };

    let resp = contract.execute(deps, env, fake_info, ExecuteMsg::Mint(mint_msg_modified))?;
    if send_msgs.is_empty() {
        Ok(resp.add_attribute("action", "mint"))
    } else {
        Ok(resp.add_messages(send_msgs).add_attribute("action", "mint"))
    }
}

// verifier  commands

pub fn add_verifier_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    wallet: String,
    public_key: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let wallet_addr = deps.api.addr_validate(&wallet)?;
    let entry_exists = verifiers().may_load(deps.storage, name.clone())?;
    if let Some(entry) = entry_exists {
        return Err(ContractError::EntryExists { name: entry.name });
    }

    let entry = VerifyRecord {
        name: name.clone(),
        wallet: wallet_addr,
        pub_key: public_key,
    };
    verifiers().save(deps.storage, name, &entry)?;

    Ok(Response::default().add_attribute("action", "add_verifier_entry"))
}

pub fn remove_verifier_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let entry_exists = verifiers().may_load(deps.storage, name.clone())?;
    if let Some(_entry) = entry_exists {
        verifiers().remove(deps.storage, name)?;

        Ok(Response::default().add_attribute("action", "remove_verifier_entry"))
    } else {
        Err(ContractError::EntryDoesntExist { name })
    }
}

pub fn update_verifier_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    wallet: String,
    public_key: String,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    if !info.funds.is_empty() {
        return Err(ContractError::NoFundsRequired {});
    }
    let entry_exists = verifiers().may_load(deps.storage, name.clone())?;

    if let Some(mut entry) = entry_exists {
        let wallet_addr = deps.api.addr_validate(&wallet)?;
        entry.wallet = wallet_addr;
        entry.name = name.clone();
        entry.pub_key = public_key;
        verifiers().save(deps.storage, name, &entry)?;

        Ok(Response::default().add_attribute("action", "update_verifier_entry"))
    } else {
        Err(ContractError::EntryDoesntExist { name })
    }
}
