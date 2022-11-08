use crate::error::IdSharedError;
use cosmwasm_std::DepsMut;
use cosmwasm_std::Event;
use cosmwasm_std::Response;
use cosmwasm_std::{Env, MessageInfo};

use crate::state::{ADMIN, NEW_ADMIN};
use id_types::shared::NewOwner;

pub fn transfer_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: String,
    blocks: u64,
) -> Result<Response, IdSharedError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }
    let new_admin = deps.api.addr_validate(&new_owner)?;

    let new_admin_record = NewOwner {
        new_owner: new_admin,
        block_height: env.block.height + blocks,
    };
    NEW_ADMIN.save(deps.storage, &new_admin_record)?;

    Ok(Response::new().add_attribute("action", "transfer_ownership"))
}

pub fn accept_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, IdSharedError> {
    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }
    let new_admin_record_o = NEW_ADMIN.may_load(deps.storage)?;

    if let Some(new_admin_record) = new_admin_record_o {
        if new_admin_record.new_owner != info.sender {
            return Err(IdSharedError::Unauthorized {
                action: "accept_ownership".to_string(),
                expected: new_admin_record.new_owner.to_string(),
                actual: info.sender.to_string(),
            });
        }

        if new_admin_record.block_height < env.block.height {
            return Err(IdSharedError::Unauthorized {
                action: "accept_gov_contract expired".to_string(),
                expected: format!("{}", new_admin_record.block_height),
                actual: format!("{}", env.block.height),
            });
        }

        NEW_ADMIN.remove(deps.storage);
        ADMIN.set(deps, Some(new_admin_record.new_owner))?;

        let event =
            Event::new("ownership_transferred").add_attribute("new_owner", info.sender.to_string());

        Ok(Response::new()
            .add_event(event)
            .add_attribute("action", "transfer_ownership"))
    } else {
        Err(IdSharedError::NoPendingOwnerChanges)
    }
}
