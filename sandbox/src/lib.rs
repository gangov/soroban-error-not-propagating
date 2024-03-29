#![no_std]

mod token;
mod test;

use soroban_sdk::{contract, contractimpl, contractmeta, Address, Env};

// Metadata that is added on to the WASM custom section
contractmeta!(key = "Description", val = "Error when propagating panic msgs");

pub trait ErrorPropagationTrait {
    fn transfers(e: Env, from: Address, to: Address, token: Address, amount: i128);
}

#[contract]
struct ErrorPropagation;

#[contractimpl]
impl ErrorPropagationTrait for ErrorPropagation {
    fn transfers(e: Env, from: Address, to: Address, token: Address, amount: i128) {
        let token_client = token::Client::new(&e, &token);
        token_client.transfer(&from, &to, &amount);
    }
}
