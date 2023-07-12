use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct State {
    pub pong_contract: Addr,
    pub ping_count: u64,
}

pub const STATE: Item<State> = Item::new("state");
pub const ADMIN: Item<Addr> = Item::new("admin");
