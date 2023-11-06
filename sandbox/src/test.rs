#![cfg(test)]
extern crate std;

use crate::token;

use soroban_sdk::{testutils::Address as _, Address, Env, IntoVal};

fn create_token<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    let token = token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()));
    token.initialize(admin, &7, &"fuzzy".into_val(e), &"FZY".into_val(e));
    token
}

fn create_token_contract<'a>(e: &Env, admin: &Address) -> token::Client<'a> {
    token::Client::new(e, &e.register_stellar_asset_contract(admin.clone()))
}
/// This test is the same as the one in the token contract
/// check out: ../token/src/test.rs:198
/// https://github.com/stellar/soroban-examples/blob/main/token/src/test.rs#L198

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::random(&e);
    let user1 = Address::random(&e);
    let user2 = Address::random(&e);
    let user3 = Address::random(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::random(&e);
    let token = create_token(&e, &admin);

    token.initialize(&admin, &10, &"name".into_val(&e), &"symbol".into_val(&e));
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Env::default();
    let admin = Address::random(&e);
    let token = create_token_contract(&e, &admin);
    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1),
        &"name".into_val(&e),
        &"symbol".into_val(&e),
    );
}
