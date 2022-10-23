use cosmwasm_std::{Coin, Deps, StdResult, Uint128};

pub fn tbd(_deps: Deps, _tbd: String) -> StdResult<Coin> {
    Ok(Coin {
        denom:"x".to_string(),
        amount: Uint128::one(),
    })
}
