use cosmwasm_std::{
    testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage},
    Api, Coin, Empty, Env, OwnedDeps,
};

use directory::{contract, execute, query, ContractError};
use id_types::directory::{ConfigResponse, FeeConfig, FeeType, InstantiateMsg};
use id_types::shared::NewOwner;

fn setup_test(env: Env) -> OwnedDeps<MockStorage, MockApi, MockQuerier, Empty> {
    let mut deps = mock_dependencies();

    contract::instantiate(
        deps.as_mut(),
        env,
        mock_info("john", &[]),
        InstantiateMsg {
            admin: Some("john".into()),
            fee: Coin::new(5u128, "udenom"),

            fee_account_type: "Wallet".to_string(),
            fee_account: "fee_wallet".to_string(),
        },
    )
    .unwrap();

    deps
}

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

#[test]
fn transferring_ownership() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    // only owner can propose ownership transferrs
    {
        let err = execute::transfer_ownership(
            deps.as_mut(),
            env.clone(),
            mock_info("plastic", &[]).sender,
            "plastic".into(),
            500,
        )
        .unwrap_err();
        match err {
            ContractError::AdminError { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
    }

    // tne owner properly proposes an ownership transfer
    {
        execute::transfer_ownership(
            deps.as_mut(),
            env.clone(),
            mock_info("john", &[]).sender,
            "jake".into(),
            500,
        )
        .unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg.new_owner,
            Some(NewOwner {
                new_owner: deps.api.addr_validate("jake").unwrap(),
                block_height: env.block.height + 500
            })
        );
    }
}

#[test]
fn accepting_ownership() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    // attempt to accept ownership when there isn't a pending ownership transfer yet
    {
        let err =
            execute::accept_ownership(deps.as_mut(), env.clone(), mock_info("plastic", &[]).sender)
                .unwrap_err();
        match err {
            ContractError::NoPendingOwnerChanges { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    execute::transfer_ownership(
        deps.as_mut(),
        env.clone(),
        mock_info("john", &[]).sender,
        "jake".into(),
        1000,
    )
    .unwrap();

    // only the pending owner can accept ownership
    {
        let err =
            execute::accept_ownership(deps.as_mut(), env.clone(), mock_info("pumpkin", &[]).sender)
                .unwrap_err();
        match err {
            ContractError::Unauthorized { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    // the pending owner properly accepts ownership
    {
        execute::accept_ownership(deps.as_mut(), env, mock_info("jake", &[]).sender).unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg,
            ConfigResponse {
                owner: Some(deps.api.addr_validate("jake").unwrap()),
                new_owner: None,

                fees: FeeConfig {
                    fee_account_type: FeeType::Wallet,
                    fee_account: deps.api.addr_validate("fee_wallet").unwrap(),
                    fee: Coin::new(5u128, "udenom")
                }
            },
        );
    }
}
