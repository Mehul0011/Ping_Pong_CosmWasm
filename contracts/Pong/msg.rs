use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
  pub admin: Addr,
}

#[cw_serde]
pub enum ExecuteMsg {
    Pong {},
    SetPingContract { ping_contract: Addr },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetCount returns the current count as a json-encoded number
    #[returns(GetPongCountResponse)]
    GetPongCount {},
}

#[cw_serde]
pub struct GetPongCountResponse {
    pub pong_count: u64,
}