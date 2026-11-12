#![cfg(test)]
use super::*;
use soroban_sdk::{vec, Env, String};

#[test]
fn says_hello() {
    let env = Env::default();
    let id = env.register(HelloUyo, ());
    let client = HelloUyoClient::new(&env, &id);

    let words = client.hello(&String::from_str(&env, "Uyo"));
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hello"),
            String::from_str(&env, "Uyo"),
        ]
    );
}
