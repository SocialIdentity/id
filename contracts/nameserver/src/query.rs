use cosmwasm_std::{Binary, Deps, Env, Order, StdResult};

use cw721_base::QueryMsg;
use cw_storage_plus::Bound;
use social_id_shared::{DEFAULT_LIMIT, MAX_LIMIT};

use social_id_shared::state::{ADMIN, FEE, NEW_ADMIN};

use crate::state::{verifiers, NAMESERVER_CONFIG};
use crate::NameServerContract;
use social_id_types::nameserver::{ConfigResponse, Extension, VerifyRecord};
use social_id_types::shared::{ENSRecord, ENSResponse};

pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
    let admin = ADMIN.get(deps)?;
    let new_owner = NEW_ADMIN.may_load(deps.storage)?;
    let fee_config = FEE.load(deps.storage)?;
    let nameserver_config = NAMESERVER_CONFIG.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: admin,
        new_owner,
        fees: fee_config,
        renewal_blocks: nameserver_config.renewal_blocks,
        owners_can_burn: nameserver_config.owners_can_burn,
        owners_can_transfer: nameserver_config.owners_can_transfer,
        verification: nameserver_config.verification,
        verification_expiry: nameserver_config.verification_expiry,
        suffix: nameserver_config.suffix,
    })
}

pub fn verifier(deps: Deps, name: String) -> StdResult<VerifyRecord> {
    verifiers().load(deps.storage, name)
}

pub fn verifier_table(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<VerifyRecord>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_after.as_ref().map(Bound::exclusive);
    let res = verifiers()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<VerifyRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn verifier_public_key(
    deps: Deps,
    public_key: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<VerifyRecord>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;

    let start = start_after.as_ref().map(Bound::exclusive);
    let res = verifiers()
        .idx
        .pubkey
        .prefix(public_key)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<VerifyRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn reverse_record(deps: Deps, env: Env, address: String) -> StdResult<ENSResponse<ENSRecord>> {
    let mut responses: Vec<ENSRecord> = vec![];

    let addr_valid = deps.api.addr_validate(&address)?;

    let recs: StdResult<Vec<_>> = NameServerContract::default()
        .tokens
        .idx
        .owner
        .prefix(addr_valid)
        .range(deps.storage, None, None, Order::Ascending)
        .take(MAX_LIMIT as usize)
        .collect();
    for x in recs? {
        let meta: Extension = x.1.extension;

        if meta.verified.unwrap_or(false)
            && meta.verification_expires.unwrap_or(env.block.height + 1) > env.block.height
            && meta.expires.unwrap_or(env.block.height + 1) > env.block.height
        {
            responses.push(ENSRecord {
                name: x.0.to_string(),
                contract: env.contract.address.clone(),
                token_id: x.0.to_string(),
            })
        }
    }

    Ok(ENSResponse { entries: responses })
}
pub fn resolve(deps: Deps, env: Env, name: String) -> StdResult<Binary> {
    NameServerContract::default().query(
        deps,
        env,
        QueryMsg::AllNftInfo {
            token_id: name,
            include_expired: Some(false),
        },
    )
}
