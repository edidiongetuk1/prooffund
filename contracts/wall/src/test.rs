#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env, String};

fn text(env: &Env, words: &str) -> String {
    String::from_str(env, words)
}

#[test]
fn numbers_go_up() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Wall, ());
    let wall = WallClient::new(&env, &id);

    let first = wall.sign(&Address::generate(&env), &text(&env, "Ada"), &text(&env, "gm"));
    let second = wall.sign(&Address::generate(&env), &text(&env, "Bassey"), &text(&env, "gm"));

    assert_eq!(first, 1);
    assert_eq!(second, 2);
    assert_eq!(wall.count(), 2);
}

#[test]
#[should_panic]
fn name_cannot_be_empty() {
    let env = Env::default();
    env.mock_all_auths();
    let id = env.register(Wall, ());
    let wall = WallClient::new(&env, &id);

    wall.sign(&Address::generate(&env), &text(&env, ""), &text(&env, "gm"));
}
