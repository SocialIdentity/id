mod common;

use crate::common::setup_test;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::Api;
use social_id_contract::query;
use social_id_shared::error::IdSharedError;
use social_id_types::id::ConfigResponse;
use social_id_types::shared::NewOwner;
use social_id_shared::ownership::{accept_ownership, transfer_ownership};

#[test]
fn transferring_ownership() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    // only owner can propose ownership transferrs
    {
        let err = transfer_ownership(
            deps.as_mut(),
            env.clone(),
            mock_info("plastic", &[]),
            "plastic".into(),
            500,
        )
            .unwrap_err();
        match err {
            IdSharedError::AdminError { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
    }

    // tne owner properly proposes an ownership transfer
    {
        transfer_ownership(
            deps.as_mut(),
            env.clone(),
            mock_info("john", &[]),
            "jake".into(),
            500,
        )
            .unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg.new_owner,
            Some(NewOwner {
                new_owner: deps.api.addr_validate("jake").unwrap(),
                block_height: env.block.height + 500,
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
            accept_ownership(deps.as_mut(), env.clone(), mock_info("plastic", &[]))
                .unwrap_err();
        match err {
            IdSharedError::NoPendingOwnerChanges { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    transfer_ownership(
        deps.as_mut(),
        env.clone(),
        mock_info("john", &[]),
        "jake".into(),
        1000,
    )
        .unwrap();

    // only the pending owner can accept ownership
    {
        let err =
            accept_ownership(deps.as_mut(), env.clone(), mock_info("pumpkin", &[]))
                .unwrap_err();
        match err {
            IdSharedError::Unauthorized { .. } => {}
            _ => assert!(false, "{}", err),
        }
    }

    // the pending owner properly accepts ownership
    {
        accept_ownership(deps.as_mut(), env, mock_info("jake", &[])).unwrap();

        let cfg = query::config(deps.as_ref()).unwrap();
        assert_eq!(
            cfg,
            ConfigResponse {
                owner: Some(deps.api.addr_validate("jake").unwrap()),
                new_owner: None,
            },
        );
    }
}
