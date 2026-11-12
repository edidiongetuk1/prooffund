#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::Address as _,
    token::{StellarAssetClient, TokenClient},
    Address, Env,
};

fn setup(env: &Env) -> (Address, Address, PrizePotClient<'_>) {
    env.mock_all_auths();
    let host = Address::generate(env);
    let usdc = env
        .register_stellar_asset_contract_v2(Address::generate(env))
        .address();
    StellarAssetClient::new(env, &usdc).mint(&host, &500);
    let pot = env.register(PrizePot, ());
    (host, usdc, PrizePotClient::new(env, &pot))
}

#[test]
fn winner_gets_the_prize() {
    let env = Env::default();
    let (host, usdc, pot) = setup(&env);
    let winner = Address::generate(&env);

    pot.fund(&host, &usdc, &500);
    assert_eq!(pot.prize(), 500);

    pot.pay_winner(&winner);
    assert_eq!(TokenClient::new(&env, &usdc).balance(&winner), 500);
    assert_eq!(pot.prize(), 0);
}

#[test]
#[should_panic]
fn cannot_pay_twice() {
    let env = Env::default();
    let (host, usdc, pot) = setup(&env);
    let winner = Address::generate(&env);

    pot.fund(&host, &usdc, &500);
    pot.pay_winner(&winner);
    pot.pay_winner(&winner);
}
