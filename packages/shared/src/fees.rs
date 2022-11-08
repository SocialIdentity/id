use crate::error::IdSharedError;
use crate::state::{ADMIN, FEE};
use cosmwasm_std::{Coin, DepsMut, MessageInfo, Response, StdError};

use id_types::shared::{FeeConfig, FeeType};
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
