mod common;
use crate::common::{setup_test, ADMIN_NAME, REGULAR_USER_NAME};
use cosmwasm_std::Api;
use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Coin,
};
use directory::{execute, query, ContractError};
use id_types::directory::{ConfigResponse, FeeConfig, FeeType};
use id_types::shared::NewOwner;

#[test]
fn transferring_ownership() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    // only owner can propose ownership transfers
    {
        let err = execute::transfer_ownership(
            deps.as_mut(),
            env.clone(),
            mock_info("plastic", &[]),
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
            mock_info(ADMIN_NAME, &[]),
            REGULAR_USER_NAME.into(),
            500,
        )
        .unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg.new_owner,
            Some(NewOwner {
                new_owner: deps.api.addr_validate(REGULAR_USER_NAME).unwrap(),
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
        let err = execute::accept_ownership(deps.as_mut(), env.clone(), mock_info("plastic", &[]))
            .unwrap_err();
        match err {
            ContractError::NoPendingOwnerChanges { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    execute::transfer_ownership(
        deps.as_mut(),
        env.clone(),
        mock_info(ADMIN_NAME, &[]),
        REGULAR_USER_NAME.into(),
        1000,
    )
    .unwrap();

    // only the pending owner can accept ownership
    {
        let err = execute::accept_ownership(deps.as_mut(), env.clone(), mock_info("pumpkin", &[]))
            .unwrap_err();
        match err {
            ContractError::Unauthorized { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    // the pending owner properly accepts ownership
    {
        execute::accept_ownership(deps.as_mut(), env, mock_info(REGULAR_USER_NAME, &[])).unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg,
            ConfigResponse {
                owner: Some(deps.api.addr_validate(REGULAR_USER_NAME).unwrap()),
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
