#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
pub enum Key {
    Host,
    Token,
    Prize,
}

#[contract]
pub struct PrizePot;

#[contractimpl]
impl PrizePot {
    // The host locks the prize in the contract.
    pub fn fund(env: Env, host: Address, token: Address, prize: i128) {
        host.require_auth();
        let store = env.storage().instance();
        if store.has(&Key::Host) {
            panic!("already funded");
        }
        if prize <= 0 {
            panic!("prize must be positive");
        }

        let pot = env.current_contract_address();
        token::TokenClient::new(&env, &token).transfer(&host, &pot, &prize);

        store.set(&Key::Host, &host);
        store.set(&Key::Token, &token);
        store.set(&Key::Prize, &prize);
    }

    // Only the host can name the winner, and the prize can only be paid once.
    pub fn pay_winner(env: Env, winner: Address) {
        let store = env.storage().instance();
        let host: Address = store.get(&Key::Host).unwrap();
        host.require_auth();

        let token: Address = store.get(&Key::Token).unwrap();
        let prize: i128 = store.get(&Key::Prize).unwrap();
        store.remove(&Key::Prize);

        let pot = env.current_contract_address();
        token::TokenClient::new(&env, &token).transfer(&pot, &winner, &prize);
    }

    pub fn prize(env: Env) -> i128 {
        env.storage().instance().get(&Key::Prize).unwrap_or(0)
    }
}

mod test;
