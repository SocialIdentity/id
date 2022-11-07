use cosmwasm_std::{Addr, DepsMut, Env, Event, MessageInfo, Response};

use id_types::id::InstantiateMsg;
use id_types::shared::NewOwner;

use crate::contract::CONTRACT_NAME;
use crate::error::ContractError;
use crate::state::{ADMIN, NEW_ADMIN};

pub fn instantiate(mut deps: DepsMut, msg: InstantiateMsg) -> Result<Response, ContractError> {
    let admin_addr = msg
        .admin
        .map(|admin| deps.api.addr_validate(&admin))
        .transpose()?;
    ADMIN.set(deps.branch(), admin_addr)?;
    Ok(Response::default())
}

pub fn transfer_ownership(
    deps: DepsMut,
    env: Env,
    sender: Addr,
    new_owner: String,
    blocks: u64,
) -> Result<Response, ContractError> {
    ADMIN.assert_admin(deps.as_ref(), &sender)?;
    let new_admin = deps.api.addr_validate(&new_owner)?;

    let new_admin_record = NewOwner {
        new_owner: new_admin,
        block_height: env.block.height + blocks,
    };
    NEW_ADMIN.save(deps.storage, &new_admin_record)?;

    Ok(Response::new().add_attribute("action", format!("{}/transfer_ownership", CONTRACT_NAME)))
}

pub fn accept_ownership(deps: DepsMut, env: Env, sender: Addr) -> Result<Response, ContractError> {
    let new_admin_record_o = NEW_ADMIN.may_load(deps.storage)?;

    if let Some(new_admin_record) = new_admin_record_o {
        if new_admin_record.new_owner != sender {
            return Err(ContractError::Unauthorized {
                action: "accept_ownership".to_string(),
                expected: new_admin_record.new_owner.to_string(),
                actual: sender.to_string(),
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
            .add_attribute("new_owner", sender.to_string());

        Ok(Response::new()
            .add_event(event)
            .add_attribute("action", format!("{}/transfer_ownership", CONTRACT_NAME)))
    } else {
        Err(ContractError::NoPendingOwnerChanges)
    }
}

pub fn tbd(_deps: DepsMut, _info: MessageInfo, _tbd: String) -> Result<Response, ContractError> {
    Ok(Response::default())
}
