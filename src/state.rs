use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::Denom;
use cw_storage_plus::Item;

use crate::msg::WalletInfo;

pub const LP_TOKEN: Item<Addr> = Item::new("lp_token");
pub const BURN_FEE_INFO: Item<Uint128> = Item::new("config_burn_info");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Token {
    pub reserve: Uint128,
    pub denom: Denom,
}

pub const TOKEN1: Item<Token> = Item::new("token1");
pub const TOKEN2: Item<Token> = Item::new("token2");

pub const OWNER: Item<Option<Addr>> = Item::new("owner");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Fees {
    pub dev_wallet_lists: Vec<WalletInfo>,
    pub fee_percent_numerator: Uint128,
    pub fee_percent_denominator: Uint128,
}

pub const FEES: Item<Fees> = Item::new("fees");
