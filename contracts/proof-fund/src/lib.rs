#![no_std]
use soroban_sdk::{
    contract, contracterror, contractevent, contractimpl, contracttype, token, Address, Env,
    String, Vec,
};

// ProofFund: milestone-based community funding.
// Donors fill a campaign. The money stays locked in this contract.
// The beneficiary submits evidence for one milestone at a time, the admin approves it,
// and the contract pays out exactly that milestone's amount to the beneficiary.
// If a campaign misses its target by the deadline, donors take their money back.

const MAX_MILESTONES: u32 = 5;
const MAX_TITLE: u32 = 60;
const MAX_EVIDENCE: u32 = 120;
const TTL_THRESHOLD: u32 = 100_000;
const TTL_EXTEND_TO: u32 = 518_400;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum Error {
    AlreadyInitialized = 1,
    NotInitialized = 2,
    BadInput = 3,
    NotFound = 4,
    WrongState = 5,
    OverTarget = 6,
    DeadlinePassed = 7,
    DeadlineNotPassed = 8,
    WrongOrder = 9,
    NothingToRefund = 10,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Status {
    Funding,
    Active,
    Completed,
}

#[contracttype]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MState {
    Pending,
    Submitted,
    Released,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Milestone {
    pub title: String,
    pub amount: i128,
    pub state: MState,
    pub evidence: String,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Campaign {
    pub beneficiary: Address,
    pub title: String,
    pub target: i128,
    pub raised: i128,
    pub released: i128,
    pub deadline: u64,
    pub milestones: Vec<Milestone>,
    pub status: Status,
}

#[contracttype]
pub enum Key {
    Admin,
    Token,
    Count,
    Campaign(u32),
    Contribution(u32, Address),
}

// Events let the web page show a live feed of what happened.
#[contractevent]
pub struct Donated {
    #[topic]
    pub id: u32,
    #[topic]
    pub donor: Address,
    pub amount: i128,
}

#[contractevent]
pub struct EvidenceSubmitted {
    #[topic]
    pub id: u32,
    pub index: u32,
    pub evidence: String,
}

#[contractevent]
pub struct Released {
    #[topic]
    pub id: u32,
    pub index: u32,
    pub amount: i128,
}

#[contractevent]
pub struct Refunded {
    #[topic]
    pub id: u32,
    #[topic]
    pub donor: Address,
    pub amount: i128,
}

fn admin(env: &Env) -> Result<Address, Error> {
    env.storage().instance().get(&Key::Admin).ok_or(Error::NotInitialized)
}

fn token_client(env: &Env) -> Result<token::TokenClient<'_>, Error> {
    let token: Address = env.storage().instance().get(&Key::Token).ok_or(Error::NotInitialized)?;
    Ok(token::TokenClient::new(env, &token))
}

fn load(env: &Env, id: u32) -> Result<Campaign, Error> {
    env.storage().persistent().get(&Key::Campaign(id)).ok_or(Error::NotFound)
}

fn save(env: &Env, id: u32, campaign: &Campaign) {
    let key = Key::Campaign(id);
    env.storage().persistent().set(&key, campaign);
    env.storage().persistent().extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);
}

// The first milestone that has not been paid yet.
fn next_index(campaign: &Campaign) -> u32 {
    for i in 0..campaign.milestones.len() {
        if campaign.milestones.get(i).unwrap().state != MState::Released {
            return i;
        }
    }
    campaign.milestones.len()
}

#[contract]
pub struct ProofFund;

#[contractimpl]
impl ProofFund {
    // Set the admin and the token once. For XLM, use the native asset contract address.
    pub fn init(env: Env, admin: Address, token: Address) -> Result<(), Error> {
        if env.storage().instance().has(&Key::Admin) {
            return Err(Error::AlreadyInitialized);
        }
        admin.require_auth();
        env.storage().instance().set(&Key::Admin, &admin);
        env.storage().instance().set(&Key::Token, &token);
        env.storage().instance().set(&Key::Count, &0u32);
        Ok(())
    }

    // Only the admin creates campaigns. The target is the sum of the milestone amounts.
    // Amounts are in stroops (1 XLM = 10,000,000 stroops).
    pub fn create_campaign(
        env: Env,
        beneficiary: Address,
        title: String,
        milestone_titles: Vec<String>,
        milestone_amounts: Vec<i128>,
        deadline: u64,
    ) -> Result<u32, Error> {
        admin(&env)?.require_auth();

        let n = milestone_titles.len();
        if n == 0 || n > MAX_MILESTONES || n != milestone_amounts.len() {
            return Err(Error::BadInput);
        }
        if title.len() == 0 || title.len() > MAX_TITLE || deadline <= env.ledger().timestamp() {
            return Err(Error::BadInput);
        }

        let mut milestones: Vec<Milestone> = Vec::new(&env);
        let mut target: i128 = 0;
        for i in 0..n {
            let t = milestone_titles.get(i).unwrap();
            let amount = milestone_amounts.get(i).unwrap();
            if t.len() == 0 || t.len() > MAX_TITLE || amount <= 0 {
                return Err(Error::BadInput);
            }
            target = target.checked_add(amount).ok_or(Error::BadInput)?;
            milestones.push_back(Milestone {
                title: t,
                amount,
                state: MState::Pending,
                evidence: String::from_str(&env, ""),
            });
        }

        let id: u32 = env.storage().instance().get(&Key::Count).unwrap_or(0) + 1;
        env.storage().instance().set(&Key::Count, &id);
        save(
            &env,
            id,
            &Campaign {
                beneficiary,
                title,
                target,
                raised: 0,
                released: 0,
                deadline,
                milestones,
                status: Status::Funding,
            },
        );
        Ok(id)
    }

    // Anyone can donate until the deadline, up to the remaining amount.
    pub fn donate(env: Env, id: u32, donor: Address, amount: i128) -> Result<(), Error> {
        donor.require_auth();
        if amount <= 0 {
            return Err(Error::BadInput);
        }
        let mut c = load(&env, id)?;
        if c.status != Status::Funding {
            return Err(Error::WrongState);
        }
        if env.ledger().timestamp() > c.deadline {
            return Err(Error::DeadlinePassed);
        }
        let raised = c.raised.checked_add(amount).ok_or(Error::BadInput)?;
        if raised > c.target {
            return Err(Error::OverTarget);
        }

        c.raised = raised;
        if c.raised == c.target {
            c.status = Status::Active;
        }
        save(&env, id, &c);

        let key = Key::Contribution(id, donor.clone());
        let before: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        env.storage().persistent().set(&key, &(before + amount));
        env.storage().persistent().extend_ttl(&key, TTL_THRESHOLD, TTL_EXTEND_TO);

        token_client(&env)?.transfer(&donor, &env.current_contract_address(), &amount);
        Donated { id, donor, amount }.publish(&env);
        Ok(())
    }

    // The beneficiary shows proof (a link or a hash) for the next milestone, in order.
    pub fn submit_evidence(env: Env, id: u32, index: u32, evidence: String) -> Result<(), Error> {
        let mut c = load(&env, id)?;
        c.beneficiary.require_auth();
        if c.status != Status::Active {
            return Err(Error::WrongState);
        }
        if evidence.len() == 0 || evidence.len() > MAX_EVIDENCE {
            return Err(Error::BadInput);
        }
        if index >= c.milestones.len() {
            return Err(Error::NotFound);
        }
        if index != next_index(&c) {
            return Err(Error::WrongOrder);
        }
        let mut m = c.milestones.get(index).unwrap();
        if m.state != MState::Pending {
            return Err(Error::WrongState);
        }
        m.state = MState::Submitted;
        m.evidence = evidence.clone();
        c.milestones.set(index, m);
        save(&env, id, &c);

        EvidenceSubmitted { id, index, evidence }.publish(&env);
        Ok(())
    }

    // The admin approves a submitted milestone. Funds go only to the beneficiary.
    pub fn release_milestone(env: Env, id: u32, index: u32) -> Result<(), Error> {
        admin(&env)?.require_auth();
        let mut c = load(&env, id)?;
        if c.status != Status::Active {
            return Err(Error::WrongState);
        }
        if index >= c.milestones.len() {
            return Err(Error::NotFound);
        }
        let mut m = c.milestones.get(index).unwrap();
        if m.state != MState::Submitted {
            return Err(Error::WrongState);
        }

        // Update the record first, then move the money.
        let amount = m.amount;
        m.state = MState::Released;
        c.milestones.set(index, m);
        c.released += amount;
        if next_index(&c) == c.milestones.len() {
            c.status = Status::Completed;
        }
        save(&env, id, &c);

        token_client(&env)?.transfer(&env.current_contract_address(), &c.beneficiary, &amount);
        Released { id, index, amount }.publish(&env);
        Ok(())
    }

    // If the target was missed, each donor can take back their own contribution.
    pub fn refund(env: Env, id: u32, donor: Address) -> Result<(), Error> {
        donor.require_auth();
        let mut c = load(&env, id)?;
        if c.status != Status::Funding {
            return Err(Error::WrongState);
        }
        if env.ledger().timestamp() <= c.deadline {
            return Err(Error::DeadlineNotPassed);
        }
        let key = Key::Contribution(id, donor.clone());
        let amount: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if amount == 0 {
            return Err(Error::NothingToRefund);
        }

        env.storage().persistent().remove(&key);
        c.raised -= amount;
        save(&env, id, &c);

        token_client(&env)?.transfer(&env.current_contract_address(), &donor, &amount);
        Refunded { id, donor, amount }.publish(&env);
        Ok(())
    }

    pub fn get_campaign(env: Env, id: u32) -> Result<Campaign, Error> {
        load(&env, id)
    }

    pub fn get_contribution(env: Env, id: u32, donor: Address) -> i128 {
        env.storage().persistent().get(&Key::Contribution(id, donor)).unwrap_or(0)
    }

    pub fn campaign_count(env: Env) -> u32 {
        env.storage().instance().get(&Key::Count).unwrap_or(0)
    }
}

mod test;
