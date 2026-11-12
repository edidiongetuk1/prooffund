#![no_std]
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

#[contract]
pub struct HelloUyo;

#[contractimpl]
impl HelloUyo {
    // Change "Hello" to any greeting you like, then build and deploy again.
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}

mod test;
