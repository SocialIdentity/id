use crate::error::IdSharedError;
use crate::state::{ADMIN, FEE};
use cosmwasm_std::{BankMsg, Coin, CosmosMsg, Deps, DepsMut, MessageInfo, Response, StdError};

use social_id_types::shared::{FeeConfig, FeeType};
use std::str::FromStr;

pub fn init_fee(
    deps: DepsMut,
    fee_account_type: &str,
    fee_account: &str,
    fee: Coin,
) -> Result<(), IdSharedError> {
    let fee_type =
        FeeType::from_str(fee_account_type).map_err(|_| IdSharedError::InvalidFeeType {})?;
    let fee_config = FeeConfig {
        fee_account_type: fee_type,
        fee_account: deps.api.addr_validate(fee_account)?,
        fee,
    };
    FEE.save(deps.storage, &fee_config)?;
    Ok(())
}
// fee commands
pub fn update_listing_fee(
    deps: DepsMut,
    info: MessageInfo,
    fee: Coin,
) -> Result<Response, IdSharedError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }
    let mut fee_config = FEE.load(deps.storage)?;
    fee_config.fee = fee;
    FEE.save(deps.storage, &fee_config)?;

    Ok(Response::default().add_attribute("action", "update_listing_fee"))
}

pub fn update_listing_fee_account(
    deps: DepsMut,
    info: MessageInfo,
    fee: Coin,
    fee_account_type: String,
    fee_account: String,
) -> Result<Response, IdSharedError> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    if !info.funds.is_empty() {
        return Err(IdSharedError::NoFundsRequired {});
    }

    let fee_type = FeeType::from_str(&fee_account_type)
        .map_err(|_| StdError::generic_err("Invalid Fee type: None, Wallet or FeeSplit only"))?;
    let fee_account_addr = deps.api.addr_validate(&fee_account)?;
    let fee_config = FeeConfig {
        fee_account_type: fee_type,
        fee_account: fee_account_addr,
        fee,
    };
    FEE.save(deps.storage, &fee_config)?;

    Ok(Response::default().add_attribute("action", "update_listing_fee_account"))
}

pub fn gen_fees(deps: Deps, coins: &[Coin]) -> Result<Vec<CosmosMsg>, IdSharedError> {
    let fee_config = FEE.load(deps.storage)?;
    if fee_config.fee_account_type != FeeType::None {
        let fund_amt = coins.iter().find(|c| c.denom == fee_config.fee.denom);
        if let Some(fund_coin) = fund_amt {
            if fee_config.fee.amount > fund_coin.amount {
                return Err(IdSharedError::InsufficientFee {
                    fee: fee_config.fee,
                    supplied: fund_coin.clone(),
                });
            }
        } else if !fee_config.fee.amount.is_zero() {
            return Err(IdSharedError::MissingFee {
                fee: fee_config.fee,
            });
        }
    }
    let send_msgs = match fee_config.fee_account_type {
        FeeType::Wallet => vec![CosmosMsg::Bank(BankMsg::Send {
            to_address: fee_config.fee_account.to_string(),
            amount: Vec::from(coins),
        })],
        FeeType::FeeSplit => {
            let msg = pfc_fee_split::fee_split_msg::ExecuteMsg::Deposit { flush: false };

            vec![msg.into_cosmos_msg(fee_config.fee_account, Vec::from(coins))?]
        }
        FeeType::None => vec![],
    };
    Ok(send_msgs)
}
