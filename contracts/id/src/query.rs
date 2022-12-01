use cosmwasm_std::{Coin, Deps, StdResult, Uint128};
use social_id_shared::state::{ADMIN, NEW_ADMIN};

use social_id_types::id::ConfigResponse;

pub fn config(deps: Deps) -> StdResult<ConfigResponse> {
    let admin = ADMIN.get(deps)?;
    let new_owner = NEW_ADMIN.may_load(deps.storage)?;

    Ok(ConfigResponse {
        owner: admin,
        new_owner,
    })
}

pub fn tbd(_deps: Deps, _tbd: String) -> StdResult<Coin> {
    Ok(Coin {
        denom: "x".to_string(),
        amount: Uint128::one(),
    })
}
