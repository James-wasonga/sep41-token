#![cfg(test)]

use soroban_sdk::{testutils::Address as _, Address, Env, String};

use crate::our_token::{SibToken, SibTokenClient};

struct SetUpResult<'a> {
    env: Env,
    client: SibTokenClient<'a>,
    admin: Address,
    sender: Address,
    receiver: Address,
    spender: Address,
}

fn setup<'a>() -> SetUpResult<'a> {
    let env = Env::default();

    let admin = Address::generate(&env);
    let sender = Address::generate(&env);
    let receiver = Address::generate(&env);
    let spender = Address::generate(&env);

    let contract_id = env.register(SibToken, (&admin, 1000_i128));

    let client = SibTokenClient::new(&env, &contract_id);

    SetUpResult {
        env,
        client,
        admin,
        sender,
        receiver,
        spender,
    }
}

#[test]
fn test_name() {
    let setup_result = setup();

    let name = setup_result.client.name();

    let token_name = String::from_str(&setup_result.env, "SibToken");

    assert_eq!(name, token_name);
}

#[test]
fn test_symbol() {
    let setup_result = setup();

    let symbol = setup_result.client.symbol();

    let token_symbol = String::from_str(&setup_result.env, "SIB");

    assert_eq!(symbol, token_symbol);
}

#[test]
fn test_decimal() {
    let setup_result = setup();

    let decimal = setup_result.client.decimals();

    assert_eq!(decimal, 18);
}

#[test]
fn test_initial_supply() {
    let setup_result = setup();

    let balance = setup_result
        .client
        .balance(&setup_result.admin);

    assert_eq!(balance, 1000);
}

#[test]
fn test_transfer() {
    let setup_result = setup();

    setup_result.client.transfer(
        &setup_result.admin,
        &setup_result.receiver,
        &200,
    );

    let admin_balance = setup_result
        .client
        .balance(&setup_result.admin);

    let receiver_balance = setup_result
        .client
        .balance(&setup_result.receiver);

    assert_eq!(admin_balance, 800);

    assert_eq!(receiver_balance, 200);
}

#[test]
fn test_approve() {
    let setup_result = setup();

    setup_result.client.approve(
        &setup_result.admin,
        &setup_result.spender,
        &300,
        &100,
    );

    let allowance = setup_result.client.allowance(
        &setup_result.admin,
        &setup_result.spender,
    );

    assert_eq!(allowance, 300);
}

#[test]
fn test_transfer_from() {
    let setup_result = setup();

    setup_result.client.approve(
        &setup_result.admin,
        &setup_result.spender,
        &400,
        &100,
    );

    setup_result.client.transfer_from(
        &setup_result.spender,
        &setup_result.admin,
        &setup_result.receiver,
        &200,
    );

    let admin_balance = setup_result
        .client
        .balance(&setup_result.admin);

    let receiver_balance = setup_result
        .client
        .balance(&setup_result.receiver);

    let remaining_allowance = setup_result.client.allowance(
        &setup_result.admin,
        &setup_result.spender,
    );

    assert_eq!(admin_balance, 800);

    assert_eq!(receiver_balance, 200);

    assert_eq!(remaining_allowance, 200);
}

#[test]
fn test_burn() {
    let setup_result = setup();

    setup_result.client.burn(
        &setup_result.admin,
        &100,
    );

    let balance = setup_result
        .client
        .balance(&setup_result.admin);

    assert_eq!(balance, 900);
}

#[test]
fn test_burn_from() {
    let setup_result = setup();

    setup_result.client.approve(
        &setup_result.admin,
        &setup_result.spender,
        &300,
        &100,
    );

    setup_result.client.burn_from(
        &setup_result.spender,
        &setup_result.admin,
        &100,
    );

    let admin_balance = setup_result
        .client
        .balance(&setup_result.admin);

    let remaining_allowance = setup_result.client.allowance(
        &setup_result.admin,
        &setup_result.spender,
    );

    assert_eq!(admin_balance, 900);

    assert_eq!(remaining_allowance, 200);
}

#[test]
fn test_mint() {
    let setup_result = setup();

    setup_result.client.mint(
        &setup_result.receiver,
        &500,
    );

    let receiver_balance = setup_result
        .client
        .balance(&setup_result.receiver);

    assert_eq!(receiver_balance, 500);
}