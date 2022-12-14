use cosmwasm_std::testing::{mock_dependencies, mock_info, MockApi, MockQuerier, MockStorage};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Empty, Env, OwnedDeps};

use social_directory::contract;
use social_id_types::directory::InstantiateMsg;
use social_id_types::shared::BlacklistRecord;

pub const ADMIN_NAME: &str = "john";
#[allow(dead_code)]
pub const REGULAR_USER_NAME: &str = "jake";
pub const FEE_WALLET_NAME: &str = "fee_wallet";
#[allow(dead_code)]
pub const ABC_CONTRACT_ADDRESS: &str = "abc_contract";
#[allow(dead_code)]
pub const FOO_CONTRACT_ADDRESS: &str = "foo_contract";

pub fn fee_coin() -> Coin {
    Coin::new(5u128, "udenom")
}
#[allow(dead_code)]
pub fn default_fee_msg() -> CosmosMsg {
    CosmosMsg::Bank(BankMsg::Send {
        to_address: FEE_WALLET_NAME.into(),
        amount: vec![fee_coin()],
    })
}
pub fn setup_test(env: Env) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    contract::instantiate(
        deps.as_mut(),
        env,
        mock_info(ADMIN_NAME, &[]),
        InstantiateMsg {
            admin: Some(ADMIN_NAME.into()),
            fee: fee_coin(),

            fee_account_type: "Wallet".to_string(),
            fee_account: FEE_WALLET_NAME.to_string(),
            blacklist: vec![
                BlacklistRecord {
                    name: "banned".to_string(),
                    reason: None,
                },
                BlacklistRecord {
                    name: "2ban".to_string(),
                    reason: Some("reason".to_string()),
                },
            ],
        },
    )
    .unwrap();

    deps
}
