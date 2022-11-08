use crate::common::setup_test;
use cosmwasm_std::testing::mock_env;
use cosmwasm_std::Api;
use cosmwasm_std::Coin;
use directory::query;
use id_types::directory::ConfigResponse;
use id_types::shared::{FeeConfig, FeeType};

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
            fees: FeeConfig {
                fee_account_type: FeeType::Wallet,
                fee_account: deps.api.addr_validate("fee_wallet").unwrap(),
                fee: Coin::new(5u128, "udenom")
            }
        },
    );
}
