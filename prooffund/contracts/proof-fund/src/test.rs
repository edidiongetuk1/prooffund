#![cfg(test)]
use super::*;
use soroban_sdk::{
    testutils::{Address as _, Ledger as _},
    token::{StellarAssetClient, TokenClient},
    vec, Address, Env, String,
};

const XLM: i128 = 10_000_000; // stroops in 1 XLM
const DEADLINE: u64 = 5_000;

struct Ctx<'a> {
    env: Env,
    admin: Address,
    beneficiary: Address,
    donor: Address,
    token: Address,
    fund: ProofFundClient<'a>,
}

fn setup<'a>() -> Ctx<'a> {
    let env = Env::default();
    env.mock_all_auths();
    env.ledger().with_mut(|l| l.timestamp = 1_000);

    let admin = Address::generate(&env);
    let beneficiary = Address::generate(&env);
    let donor = Address::generate(&env);
    let token = env.register_stellar_asset_contract_v2(Address::generate(&env)).address();
    StellarAssetClient::new(&env, &token).mint(&donor, &(1_000 * XLM));

    let id = env.register(ProofFund, ());
    let fund = ProofFundClient::new(&env, &id);
    fund.init(&admin, &token);
    Ctx { env, admin, beneficiary, donor, token, fund }
}

// The borehole example: 30 + 40 + 30 = 100 XLM. Returns the campaign id.
fn borehole(c: &Ctx) -> u32 {
    let e = &c.env;
    c.fund.create_campaign(
        &c.beneficiary,
        &String::from_str(e, "Community borehole"),
        &vec![
            e,
            String::from_str(e, "Materials"),
            String::from_str(e, "Drilling"),
            String::from_str(e, "Installation"),
        ],
        &vec![e, 30 * XLM, 40 * XLM, 30 * XLM],
        &DEADLINE,
    )
}

fn bal(c: &Ctx, who: &Address) -> i128 {
    TokenClient::new(&c.env, &c.token).balance(who)
}

fn proof(c: &Ctx, text: &str) -> String {
    String::from_str(&c.env, text)
}

#[test]
fn full_flow_pays_each_milestone() {
    let c = setup();
    let id = borehole(&c);
    assert_eq!(c.fund.get_campaign(&id).target, 100 * XLM);

    c.fund.donate(&id, &c.donor, &(100 * XLM));
    assert_eq!(c.fund.get_campaign(&id).status, Status::Active);
    assert_eq!(c.fund.get_contribution(&id, &c.donor), 100 * XLM);
    assert_eq!(bal(&c, &c.fund.address), 100 * XLM);

    for (i, paid) in [(0u32, 30 * XLM), (1, 70 * XLM), (2, 100 * XLM)] {
        c.fund.submit_evidence(&id, &i, &proof(&c, "https://example.com/photo"));
        c.fund.release_milestone(&id, &i);
        assert_eq!(bal(&c, &c.beneficiary), paid);
    }
    let done = c.fund.get_campaign(&id);
    assert_eq!(done.status, Status::Completed);
    assert_eq!(done.released, 100 * XLM);
    assert_eq!(bal(&c, &c.fund.address), 0);
}

#[test]
fn donations_cannot_pass_the_target() {
    let c = setup();
    let id = borehole(&c);
    c.fund.donate(&id, &c.donor, &(90 * XLM));
    assert_eq!(c.fund.try_donate(&id, &c.donor, &(11 * XLM)), Err(Ok(Error::OverTarget)));
    c.fund.donate(&id, &c.donor, &(10 * XLM));
    assert_eq!(c.fund.try_donate(&id, &c.donor, &XLM), Err(Ok(Error::WrongState)));
}

#[test]
fn nothing_moves_until_fully_funded() {
    let c = setup();
    let id = borehole(&c);
    c.fund.donate(&id, &c.donor, &(50 * XLM));
    assert_eq!(
        c.fund.try_submit_evidence(&id, &0, &proof(&c, "proof")),
        Err(Ok(Error::WrongState))
    );
    assert_eq!(c.fund.try_release_milestone(&id, &0), Err(Ok(Error::WrongState)));
}

#[test]
fn release_needs_evidence_and_the_right_order() {
    let c = setup();
    let id = borehole(&c);
    c.fund.donate(&id, &c.donor, &(100 * XLM));

    assert_eq!(c.fund.try_release_milestone(&id, &0), Err(Ok(Error::WrongState)));
    assert_eq!(
        c.fund.try_submit_evidence(&id, &1, &proof(&c, "proof")),
        Err(Ok(Error::WrongOrder))
    );
    assert_eq!(c.fund.try_submit_evidence(&id, &9, &proof(&c, "proof")), Err(Ok(Error::NotFound)));

    c.fund.submit_evidence(&id, &0, &proof(&c, "proof"));
    c.fund.release_milestone(&id, &0);
    assert_eq!(c.fund.try_release_milestone(&id, &0), Err(Ok(Error::WrongState)));
}

#[test]
fn only_admin_releases_and_only_beneficiary_submits() {
    let c = setup();
    let id = borehole(&c);
    c.fund.donate(&id, &c.donor, &(100 * XLM));

    c.fund.submit_evidence(&id, &0, &proof(&c, "proof"));
    assert!(c.env.auths().iter().any(|(who, _)| *who == c.beneficiary));
    c.fund.release_milestone(&id, &0);
    assert!(c.env.auths().iter().any(|(who, _)| *who == c.admin));

    // With no signatures provided, every protected call must fail.
    c.fund.submit_evidence(&id, &1, &proof(&c, "proof"));
    c.env.set_auths(&[]);
    assert!(c.fund.try_release_milestone(&id, &1).is_err());
    assert!(c.fund.try_submit_evidence(&id, &2, &proof(&c, "x")).is_err());
    assert!(c.fund.try_donate(&id, &c.donor, &XLM).is_err());
}

#[test]
fn failed_campaign_refunds_each_donor_once() {
    let c = setup();
    let id = borehole(&c);
    let other = Address::generate(&c.env);
    StellarAssetClient::new(&c.env, &c.token).mint(&other, &(20 * XLM));
    c.fund.donate(&id, &c.donor, &(40 * XLM));
    c.fund.donate(&id, &other, &(20 * XLM));

    assert_eq!(c.fund.try_refund(&id, &c.donor), Err(Ok(Error::DeadlineNotPassed)));

    c.env.ledger().with_mut(|l| l.timestamp = DEADLINE + 1);
    assert_eq!(c.fund.try_donate(&id, &c.donor, &XLM), Err(Ok(Error::DeadlinePassed)));
    c.fund.refund(&id, &c.donor);
    assert_eq!(bal(&c, &c.donor), 1_000 * XLM);
    assert_eq!(c.fund.get_campaign(&id).raised, 20 * XLM);
    assert_eq!(c.fund.try_refund(&id, &c.donor), Err(Ok(Error::NothingToRefund)));

    c.fund.refund(&id, &other);
    assert_eq!(bal(&c, &other), 20 * XLM);
    assert_eq!(bal(&c, &c.fund.address), 0);
}

#[test]
fn funded_campaign_cannot_be_refunded() {
    let c = setup();
    let id = borehole(&c);
    c.fund.donate(&id, &c.donor, &(100 * XLM));
    c.env.ledger().with_mut(|l| l.timestamp = DEADLINE + 1);
    assert_eq!(c.fund.try_refund(&id, &c.donor), Err(Ok(Error::WrongState)));
}

#[test]
fn bad_campaigns_are_rejected() {
    let c = setup();
    let e = &c.env;
    let name = String::from_str(e, "Test");
    let one = vec![e, String::from_str(e, "Only")];

    // No milestones, mismatched lists, zero amount, deadline in the past, too many milestones.
    let none: soroban_sdk::Vec<String> = vec![e];
    let no_amounts: soroban_sdk::Vec<i128> = vec![e];
    assert_eq!(
        c.fund.try_create_campaign(&c.beneficiary, &name, &none, &no_amounts, &DEADLINE),
        Err(Ok(Error::BadInput))
    );
    assert_eq!(
        c.fund.try_create_campaign(&c.beneficiary, &name, &one, &vec![e, XLM, XLM], &DEADLINE),
        Err(Ok(Error::BadInput))
    );
    assert_eq!(
        c.fund.try_create_campaign(&c.beneficiary, &name, &one, &vec![e, 0i128], &DEADLINE),
        Err(Ok(Error::BadInput))
    );
    assert_eq!(
        c.fund.try_create_campaign(&c.beneficiary, &name, &one, &vec![e, XLM], &500),
        Err(Ok(Error::BadInput))
    );
    let six = vec![e, name.clone(), name.clone(), name.clone(), name.clone(), name.clone(), name.clone()];
    let amounts = vec![e, XLM, XLM, XLM, XLM, XLM, XLM];
    assert_eq!(
        c.fund.try_create_campaign(&c.beneficiary, &name, &six, &amounts, &DEADLINE),
        Err(Ok(Error::BadInput))
    );
    assert_eq!(c.fund.campaign_count(), 0);
}

#[test]
fn init_runs_only_once() {
    let c = setup();
    assert_eq!(c.fund.try_init(&c.admin, &c.token), Err(Ok(Error::AlreadyInitialized)));
}
