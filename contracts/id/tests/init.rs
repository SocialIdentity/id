use crate::common::setup_test;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::Api;
use id::query;
use id_types::id::ConfigResponse;
mod common;
#[test]

fn initializing() {
    let env = mock_env();

    let deps = setup_test(env);

    let cfg = query::config(deps.as_ref()).unwrap();
    assert_eq!(
        cfg,
        ConfigResponse {
            owner: Some(deps.api.addr_validate("john").unwrap()),
            new_owner: None,
        },
    );
}
