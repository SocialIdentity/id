use cosmwasm_std::{
    from_binary, to_binary, to_vec, Addr, Binary, ContractResult, Deps, Order, QueryRequest,
    StdError, StdResult, WasmQuery,
};
use cw721::TokensResponse;
use cw_storage_plus::Bound;
use cw_utils::maybe_addr;
use id_shared::state::FEE;
use id_shared::{is_separator, DEFAULT_LIMIT, MAX_LIMIT};
use id_types::directory::{ConfigResponse, DirectoryRecord, EnsType};
use id_types::shared::{ENSRecord, ENSResponse};
use sha3::{Digest, Keccak256};

use crate::state::directory;
use id_shared::state::{ADMIN, NEW_ADMIN};

pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
    let admin = ADMIN.get(deps)?;
    let new_owner = NEW_ADMIN.may_load(deps.storage)?;
    let fee_config = FEE.load(deps.storage)?;
    Ok(ConfigResponse {
        owner: admin,
        new_owner,
        fees: fee_config,
    })
}

pub fn entry(deps: Deps, name: String) -> StdResult<DirectoryRecord> {
    directory().load(deps.storage, name)
}

pub fn entries(
    deps: Deps,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<DirectoryRecord>> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    //    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_after.as_ref().map(Bound::exclusive);
    let res = directory()
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<DirectoryRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn entries_contract(
    deps: Deps,
    contract: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<DirectoryRecord>> {
    let contract_addr = deps.api.addr_validate(&contract)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);
    let res = directory()
        .idx
        .contract
        .prefix(contract_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<DirectoryRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn entries_owner(
    deps: Deps,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<ENSResponse<DirectoryRecord>> {
    let owner_addr = deps.api.addr_validate(&owner)?;
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
    let start_addr = maybe_addr(deps.api, start_after)?;
    let start = start_addr.as_ref().map(Bound::exclusive);
    let res = directory()
        .idx
        .owner
        .prefix(owner_addr)
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<DirectoryRecord>>>()?;
    Ok(ENSResponse { entries: res })
}

pub fn reverse_record(deps: Deps, address: String) -> StdResult<ENSResponse<ENSRecord>> {
    let mut responses: Vec<ENSRecord> = vec![];
    deps.api.addr_validate(&address)?;
    let providers = directory()
        .range(deps.storage, None, None, Order::Ascending)
        .map(|x| x.map(|y| y.1))
        .collect::<StdResult<Vec<DirectoryRecord>>>()?;
    for entry in providers {
        let qry_req = gen_reverse_query_msg(&entry.ens_type, &entry.contract, &address)?;
        let resp = deps.querier.raw_query(&to_vec(&qry_req)?).unwrap();
        match resp {
            ContractResult::Ok(bin) => {
                let tokens = gen_ens_record_from_reverse_query(
                    &entry.ens_type,
                    &entry.name,
                    &entry.contract,
                    bin,
                )?;
                for token in tokens {
                    responses.push(token)
                }
            }
            ContractResult::Err(_) => {}
        }
    }
    Ok(ENSResponse { entries: responses })
}
pub fn resolve(deps: Deps, name: String) -> StdResult<Binary> {
    if let Some(separator) = name.rfind(is_separator) {
        // domain_part still has separator
        let (name_part, domain_part) = name.split_at(separator);
        let domain = &domain_part[1..];
        let rec = directory().may_load(deps.storage, domain.to_string())?;
        if let Some(dnsrec) = rec {
            let token_id = gen_token_id(&dnsrec.ens_type, name_part)?;
            let qry = cw721::Cw721QueryMsg::AllNftInfo {
                token_id,
                include_expired: Some(false),
            };
            let qry_req: QueryRequest<cw721::AllNftInfoResponse<String>> =
                QueryRequest::Wasm(WasmQuery::Smart {
                    contract_addr: dnsrec.contract.to_string(),
                    msg: to_binary(&qry)?,
                });
            match deps.querier.raw_query(&to_vec(&qry_req)?).unwrap() {
                ContractResult::Ok(bin) => Ok(bin),
                ContractResult::Err(msg) => Err(StdError::generic_err(msg)),
            }
        } else {
            Err(StdError::generic_err(format!("{} not registered", domain)))
        }
    } else {
        Err(StdError::generic_err("malformed name".to_string()))
    }
}
/*pub(crate) fn is_separator(char: char) -> bool {
    char == '.' || char == '@' || char == '/'
}
*/
pub(crate) fn gen_token_id(ens: &EnsType, name: &str) -> StdResult<String> {
    match ens {
        EnsType::Lns => {
            //    let mut hasher = Sha3::keccak256();
            let mut hasher = Keccak256::new();

            hasher.update(name);
            let hash = hasher.finalize();
            Ok(hex::encode(hash))
        }
        EnsType::Cw721 => Ok(name.to_string()),
    }
}

pub(crate) fn gen_ens_record_from_reverse_query(
    _ens: &EnsType,
    name: &str,
    contract: &Addr,
    bin: Binary,
) -> StdResult<Vec<ENSRecord>> {
    let mut recs: Vec<ENSRecord> = vec![];
    let token_resp = from_binary::<TokensResponse>(&bin)?;
    for token in token_resp.tokens {
        recs.push(ENSRecord {
            name: name.to_string(),
            contract: contract.clone(),
            token_id: token,
        });
    }
    Ok(recs)
}
pub(crate) fn gen_reverse_query_msg(
    _ens: &EnsType,
    contract: &Addr,
    address: &str,
) -> StdResult<QueryRequest<TokensResponse>> {
    let qry = cw721::Cw721QueryMsg::Tokens {
        owner: address.to_string(),
        start_after: None,
        limit: None,
    };
    let qry_req: QueryRequest<TokensResponse> = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: contract.to_string(),
        msg: to_binary(&qry)?,
    });
    Ok(qry_req)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_id_generation() {
        assert_eq!(
            "583efd4da3651e5cbfa095071e4c0e55444062dace38466cb1056e555c7bd5d6",
            gen_token_id(&EnsType::Lns, "abcd1234567").unwrap()
        );
        assert_eq!(
            "abcd1234567",
            gen_token_id(&EnsType::Cw721, "abcd1234567").unwrap()
        );
    }
}
