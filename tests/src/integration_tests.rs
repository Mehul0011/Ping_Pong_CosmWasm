use cosmwasm_std::Empty;
use cosmwasm_std::{coin, Addr};
use cw_multi_test::Executor;
use cw_multi_test::{App, Contract, ContractWrapper};

// put common functions here, such as ContractWrapper functions

#[allow(dead_code)]
fn mock_app() -> App {
    App::default()
}

#[allow(dead_code)]
fn ping_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        ping::contract::execute,
        ping::contract::instantiate,
        ping::contract::query,
    );
    // .with_migrate(ping::contract::migrate);
    Box::new(contract)
}

#[allow(dead_code)]
fn pong_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        pong::contract::execute,
        pong::contract::instantiate,
        pong::contract::query,
    );
    // .with_reply(pong::contract::reply);
    Box::new(contract)
}

pub fn mint_native(app: &mut App, recipient: String, denom: String, amount: u128) {
    app.sudo(cw_multi_test::SudoMsg::Bank(
        cw_multi_test::BankSudo::Mint {
            to_address: recipient,
            amount: vec![coin(amount, denom)],
        },
    ))
    .unwrap();
}

#[test]
fn integration_test() {
    // 1.  set up mock app
    let mut router = mock_app();

    // 2.  set up contracts

    // 2.a.  set up ping contract
    let ping_code_id = router.store_code(ping_contract());
    println!("ping_code_id: {}", ping_code_id);

    // 2.b set up pong contract
    let pong_code_id = router.store_code(pong_contract());
    println!("pong_code_id: {}", pong_code_id);
    //3.  instantiate contracts
    let ping_contract_addr = router.instantiate_contract(
        ping_code_id,
        Addr::unchecked("pong_owner"),
        &ping::msg::InstantiateMsg {
            admin: Addr::unchecked("pong_owner"),
        }, // funds
        &[],
        "Contract Factory", // label
        None,               // code admin (for migration)
    ).unwrap();

    let _res = router
    .execute_contract(
        Addr::unchecked("ping owner"),
        ping_contract_addr.clone(),
        &ping::msg::ExecuteMsg::DeployPongContract { pong_code_id },
        &[], // funds
    );

    // let pong_contract_addr = router
    //     .instantiate_contract(
    //         pong_code_id,
    //         Addr::unchecked("pong_owner"),
    //         &pong::msg::InstantiateMsg {
    //             admin: Addr::unchecked("pong_owner"),
    //         }, // funds
    //         &[],
    //         "Contract Factory", // label
    //         None,               // code admin (for migration)
    //     )
    //     .unwrap();



    // 4. execute setPongcontract on the ping contract
    // let res = router
    //     .execute_contract(
    //         Addr::unchecked("pong_owner"),
    //         ping_contract_addr.clone(),
    //         &ping::msg::ExecuteMsg::SetPongContract { pong_contract: pong_contract_addr.clone() },
    //         &[], // funds
    //     )
    //     .unwrap();
    // println!("res of setting the pong address on ping: {:?}", res);

    // let _res = router
    //     .execute_contract(
    //         Addr::unchecked("pong_owner"),
    //         pong_contract_addr,
    //         &pong::msg::ExecuteMsg::SetPingContract { ping_contract: ping_contract_addr.clone()},
    //         &[], // funds
    //     )
    //     .unwrap();


    // execute fn ping on ping contract
    let _res = router
        .execute_contract(
            Addr::unchecked("pong_owner"),
            ping_contract_addr.clone(),
            &ping::msg::ExecuteMsg::Ping {},
            &[], // funds
        )
        .unwrap();

    // // execute fn pong on pong contract
    // let _res = router
    //     .execute_contract(
    //         Addr::unchecked("pong_owner"),
    //         pong_contract_addr,
    //         &pong::msg::ExecuteMsg::Pong {},
    //         &[], // funds
    //     )
    //     .unwrap();
    
}
