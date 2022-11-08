use crate::common::{
    default_fee_msg, fee_coin, setup_test, ABC_CONTRACT_ADDRESS, FOO_CONTRACT_ADDRESS,
    REGULAR_USER_NAME,
};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Api, Coin, StdError};
use cw20::Logo;
use directory::{execute, query, ContractError};
use id_types::directory::{DirectoryRecord, EnsType, Socials};

mod common;

#[test]
fn add_entry() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    // no funds
    {
        let err = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            None,
        )
        .unwrap_err();
        match err {
            ContractError::MissingFee { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
    }

    //  funds not sufficient
    {
        let err = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[Coin::new(1u128, "udenom")]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            None,
        )
        .unwrap_err();
        match err {
            ContractError::InsufficientFee { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
    }
    //  funds wrong type
    {
        let err = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[Coin::new(5u128, "uwrong")]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            None,
        )
        .unwrap_err();
        match err {
            ContractError::MissingFee { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
    }
    //  should work
    {
        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[fee_coin()]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            Some(Socials {
                project: None,
                description: None,
                email: None,
                twitter: Some("Test".into()),
                telegraph: None,
                discord: None,
                web: None,
                github: None,
            }),
        )
        .unwrap();

        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);

        let provider_abc = query::entry(deps.as_ref(), "abc".into()).unwrap();
        assert_eq!(
            DirectoryRecord {
                owner: deps.api.addr_validate(REGULAR_USER_NAME).unwrap(),
                name: "abc".to_string(),
                contract: deps.api.addr_validate(ABC_CONTRACT_ADDRESS).unwrap(),
                ens_type: EnsType::Lns,
                logo: None,
                socials: Some(Socials {
                    project: None,
                    description: None,
                    email: None,
                    twitter: Some("Test".into()),
                    telegraph: None,
                    discord: None,
                    web: None,
                    github: None
                }),
            },
            provider_abc
        );
        let err = query::entry(deps.as_ref(), "foo".into()).unwrap_err();

        match err {
            StdError::NotFound { .. } => {}
            _ => assert!(false, "{:?}", err),
        }

        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[fee_coin()]),
            "foo".to_string(),
            FOO_CONTRACT_ADDRESS.to_string(),
            EnsType::Cw721.to_string(),
            None,
            None,
        )
        .unwrap();
        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);
        let provider_foo = query::entry(deps.as_ref(), "foo".into()).unwrap();
        assert_eq!(
            DirectoryRecord {
                owner: deps.api.addr_validate("pumpkin").unwrap(),
                name: "foo".to_string(),
                contract: deps.api.addr_validate(FOO_CONTRACT_ADDRESS).unwrap(),
                ens_type: EnsType::Cw721,
                logo: None,
                socials: None,
            },
            provider_foo
        );

        let providers = query::entries(deps.as_ref(), None, None).unwrap();
        assert_eq!(2, providers.entries.len());
        let resp_abc = providers.entries.iter().find(|f| f.name == "abc").unwrap();
        assert_eq!(resp_abc, &provider_abc)
    }
}

#[test]
fn modify_entry() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    //  should work
    {
        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[fee_coin()]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            Some(Socials {
                project: None,
                description: None,
                email: None,
                twitter: Some("Test".into()),
                telegraph: None,
                discord: None,
                web: None,
                github: None,
            }),
        )
        .unwrap();

        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);

        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[fee_coin()]),
            "foo".to_string(),
            FOO_CONTRACT_ADDRESS.to_string(),
            EnsType::Cw721.to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);
        let provider_foo = query::entry(deps.as_ref(), "foo".into()).unwrap();
        assert_eq!(
            DirectoryRecord {
                owner: deps.api.addr_validate("pumpkin").unwrap(),
                name: "foo".to_string(),
                contract: deps.api.addr_validate(FOO_CONTRACT_ADDRESS).unwrap(),
                ens_type: EnsType::Cw721,
                logo: None,
                socials: None,
            },
            provider_foo
        );
        let err = execute::update_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "abc".to_string(),
            "xxx_contract".to_string(),
            EnsType::Lns.to_string(),
            Some(Logo::Url("https://logo.example.com".into())),
            None,
            None,
        )
        .unwrap_err();
        match err {
            ContractError::Unauthorized { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
        let _res = execute::update_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "foo".to_string(),
            "xxx_contract".to_string(),
            EnsType::Lns.to_string(),
            Some(Logo::Url("https://logo.example.com".into())),
            None,
            Some("james".into()),
        )
        .unwrap();

        let providers = query::entries(deps.as_ref(), None, None).unwrap();
        assert_eq!(2, providers.entries.len());
        let resp_foo = providers.entries.iter().find(|f| f.name == "foo").unwrap();
        assert_eq!(
            resp_foo,
            &DirectoryRecord {
                owner: deps.api.addr_validate("james").unwrap(),
                name: "foo".to_string(),
                contract: deps.api.addr_validate("xxx_contract").unwrap(),
                ens_type: EnsType::Lns,
                logo: Some(Logo::Url("https://logo.example.com".into())),
                socials: None,
            },
        );
    }
}

#[test]
fn remove_entry() {
    let env = mock_env();

    let mut deps = setup_test(env.clone());

    //  should work
    {
        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info(REGULAR_USER_NAME, &[fee_coin()]),
            "abc".to_string(),
            ABC_CONTRACT_ADDRESS.to_string(),
            EnsType::Lns.to_string(),
            None,
            Some(Socials {
                project: None,
                description: None,
                email: None,
                twitter: Some("Test".into()),
                telegraph: None,
                discord: None,
                web: None,
                github: None,
            }),
        )
        .unwrap();

        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);

        let res = execute::add_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[fee_coin()]),
            "foo".to_string(),
            FOO_CONTRACT_ADDRESS.to_string(),
            EnsType::Cw721.to_string(),
            None,
            None,
        )
        .unwrap();

        assert_eq!(1, res.messages.len());
        assert_eq!(default_fee_msg(), res.messages.first().unwrap().msg);
        let provider_foo = query::entry(deps.as_ref(), "foo".into()).unwrap();
        assert_eq!(
            DirectoryRecord {
                owner: deps.api.addr_validate("pumpkin").unwrap(),
                name: "foo".to_string(),
                contract: deps.api.addr_validate(FOO_CONTRACT_ADDRESS).unwrap(),
                ens_type: EnsType::Cw721,
                logo: None,
                socials: None,
            },
            provider_foo
        );
        let err = execute::remove_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "abc".to_string(),
        )
        .unwrap_err();
        match err {
            ContractError::Unauthorized { .. } => {}
            _ => assert!(false, "{:?}", err),
        }
        let err = execute::remove_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "def".to_string(),
        )
        .unwrap_err();
        match err {
            ContractError::EntryDoesntExist { .. } => {}
            _ => assert!(false, "{:?}", err),
        }

        let _res = execute::remove_directory_entry(
            deps.as_mut(),
            mock_info("pumpkin", &[]),
            "foo".to_string(),
        )
        .unwrap();

        let providers = query::entries(deps.as_ref(), None, None).unwrap();
        assert_eq!(1, providers.entries.len());
    }
}
