#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, symbol_short, Address, Env, String, Symbol};

const COUNT: Symbol = symbol_short!("COUNT");

// Events are how a contract tells the outside world that something happened.
// The web page listens for this one and puts your note on the big screen.
#[contractevent]
pub struct Signed {
    #[topic]
    pub who: Address,
    pub number: u32,
    pub name: String,
    pub message: String,
}

#[contract]
pub struct Wall;

#[contractimpl]
impl Wall {
    pub fn sign(env: Env, who: Address, name: String, message: String) -> u32 {
        who.require_auth();
        if name.len() == 0 || name.len() > 30 {
            panic!("name must be 1 to 30 characters");
        }
        if message.len() > 100 {
            panic!("message must be 100 characters or less");
        }

        let number = Self::count(env.clone()) + 1;
        env.storage().instance().set(&COUNT, &number);

        Signed { who, number, name, message }.publish(&env);
        number
    }

    pub fn count(env: Env) -> u32 {
        env.storage().instance().get(&COUNT).unwrap_or(0)
    }
}

mod test;
