use crate::state::{ADMIN, BLACKLIST};
use cosmwasm_std::{Deps, DepsMut, MessageInfo, Order, Response, StdResult};
use cw_storage_plus::Bound;
use std::collections::HashSet;

use crate::error::IdSharedError;
use crate::{DEFAULT_LIMIT, MAX_LIMIT};
use id_types::directory::BlacklistRecord;
use id_types::shared::ENSResponse;

pub fn init_blacklist(deps: DepsMut, blacklist: Vec<BlacklistRecord>) -> Result<(), IdSharedError> {
    let dupe_check: HashSet<String> = blacklist.iter().map(|v| v.name.clone()).collect();
    if dupe_check.len() != blacklist.len() {
        return Err(IdSharedError::BlacklistNotUnique {});
    }
    for rec in blacklist {
        BLACKLIST.save(deps.storage, rec.name.clone(), &rec)?
    }
    Ok(())
}
pub fn add_blacklist_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
    reason: Option<String>,
) -> Result<Response, IdSharedError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }

    let entry_exists = BLACKLIST.may_load(deps.storage, name.clone())?;
    if let Some(entry) = entry_exists {
        return Err(IdSharedError::BlacklistEntryExists { name: entry.name });
    }

    let entry = BlacklistRecord {
        name: name.clone(),
        reason,
    };
    BLACKLIST.save(deps.storage, name, &entry)?;

    Ok(Response::default().add_attribute("action", "add_blacklist_entry"))
}

pub fn remove_blacklist_entry(
    deps: DepsMut,
    info: MessageInfo,
    name: String,
) -> Result<Response, IdSharedError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }

    let entry_exists = BLACKLIST.may_load(deps.storage, name.clone())?;
    if let Some(_entry) = entry_exists {
        BLACKLIST.remove(deps.storage, name);

        Ok(Response::default().add_attribute("action", "remove_blacklist_entry"))
    } else {
        Err(IdSharedError::BlacklistEntryDoesntExist { name })
    }
}

pub fn query_entry(deps: Deps, name: String) -> StdResult<BlacklistRecord> {
    BLACKLIST.load(deps.storage, name)
}

pub fn query_entries(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<BlacklistRecord>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_after.as_ref().map(Bound::exclusive);
    let res = BLACKLIST
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<BlacklistRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn is_blacklisted(deps: Deps, name: &str) -> Result<(), IdSharedError> {
    if let Some(blacklist) = BLACKLIST.may_load(deps.storage, name.to_string())? {
        Err(IdSharedError::Blacklisted {
            name: blacklist.name,
            reason: blacklist
                .reason
                .unwrap_or_else(|| "-no reason supplied-".to_string()),
        })
    } else {
        Ok(())
    }
}
