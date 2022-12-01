use cosmwasm_std::testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{Empty, Env, OwnedDeps};

use social_id_contract::contract;
use social_id_types::id::InstantiateMsg;

pub fn setup_test(env: Env) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    contract::instantiate(
        deps.as_mut(),
        env,
        mock_info("john", &[]),
        InstantiateMsg {
            admin: Some("john".into()),
        },
    )
    .unwrap();

    deps
}
