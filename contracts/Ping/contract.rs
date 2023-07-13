#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr};
use cw2::set_contract_version;

use crate::msg::{ExecuteMsg, GetPingCountResponse, InstantiateMsg, QueryMsg};
use crate::state::{State, ADMIN, STATE};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:ping-pong";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {

    // pong_contract will be the 0 address
    let state = State {
        pong_contract: Addr::unchecked(""),
        ping_count: 0,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;
    let admin = msg.admin;

    ADMIN.save(deps.storage, &admin)?;
    Ok(Response::new())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::Ping {} => execute::ping(deps),
        ExecuteMsg::SetPongContract { pong_contract } => execute::set_pong_contract(deps, pong_contract),
    }
}

pub mod execute {

    use cosmwasm_std::WasmMsg;

    use super::*;

    pub fn ping(deps: DepsMut) -> StdResult<Response> {
        let state = STATE.load(deps.storage)?;

        let pong_contract = state.pong_contract;

        let pong_response = WasmMsg::Execute {
            contract_addr: pong_contract.to_string(),
            msg: to_binary("F#ck, this shit works!!")?,
            funds: vec![],
        };

        print!("We got the shit back, the {:?}th time", pong_response);

        // assert!(pong_response == state.ping_count);

        // if let Ok(Some(_)) == pong_response {
        //     todo!()
        //     // if response !=
        //     // return Err(StdError::generic_err("Invalid pong response"))
        // }
        // else  {
        //     return Err(StdError::generic_err("No pong response"))
        // }

        STATE.update(deps.storage, |mut state| -> StdResult<_> {
            state.ping_count += 1;
            Ok(state)
        })?;

        Ok(Response::new().add_attribute("action", "increment"))
    }

    pub fn set_pong_contract(deps: DepsMut, pong_contract: Addr) -> StdResult<Response>{
        let state = STATE.load(deps.storage).unwrap();
        
        let pong_state = State {
            ping_count: state.ping_count, 
            pong_contract
        };

        // print in debug mode
        print!("Welcome the shitter");

        STATE.save(deps.storage, &pong_state)?;
        Ok(Response::new())
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetPingCount {} => to_binary(&query::get_ping_count(deps)?),
    }
}

pub mod query {
    use super::*;
    pub fn get_ping_count(deps: Deps) -> StdResult<GetPingCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetPingCountResponse {
            ping_count: state.ping_count,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            admin: Addr::unchecked("admin"),
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        print!("hoola hoolla");
        
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPingCount {}).unwrap();
        let value: GetPingCountResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.ping_count);

        // it worked, let's query the state
        // let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        // let value: GetPingCountResponse = from_binary(&res).unwrap();
        // assert_eq!(17, value.count);
    }

    #[test]
    fn set_pong_contract() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            admin: Addr::unchecked("admin"),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let pong_contract = Addr::unchecked("pong_contract");
        let msg = ExecuteMsg::SetPongContract { pong_contract };
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPingCount {}).unwrap();
        let value: GetPingCountResponse = from_binary(&res).unwrap();
        assert_eq!(0, value.ping_count);
    }
    
    #[test]
    fn ping() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            admin: Addr::unchecked("admin"),
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let pong_contract = Addr::unchecked("pong_contract");
        let msg = ExecuteMsg::SetPongContract { pong_contract };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::Ping {};
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.attributes.len());
        assert_eq!(res.attributes[0].key, "action");
        assert_eq!(res.attributes[0].value, "increment");

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetPingCount {}).unwrap();
        let value: GetPingCountResponse = from_binary(&res).unwrap();
        assert_eq!(1, value.ping_count);
    }
}
