# 04 · Level up: store a counter

Slide 12. A contract that remembers a number on chain. Call it three times and it returns 1, then 2, then 3.

This follows [Storing data](https://developers.stellar.org/docs/build/smart-contracts/getting-started/storing-data) in the Stellar docs. Do it inside the `hello-world` project from [03](../03-hello-world/).

## 1. Add a second contract

```bash
stellar contract init . --name increment
```

That creates `contracts/increment`.

## 2. Paste the code

Replace everything in `contracts/increment/src/lib.rs` with this:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }
}

mod test;
```

Replace everything in `contracts/increment/src/test.rs` with this:

```rust
#![cfg(test)]
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register(IncrementContract, ());
    let client = IncrementContractClient::new(&env, &contract_id);

    assert_eq!(client.increment(), 1);
    assert_eq!(client.increment(), 2);
    assert_eq!(client.increment(), 3);
}
```

## 3. Test, build, deploy

```bash
cargo test
```

```bash
stellar contract build
```

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/increment.wasm --source-account alice --network testnet --alias increment
```

## 4. Call it three times

Run this three times:

```bash
stellar contract invoke --id increment --source-account alice --network testnet -- increment
```

It returns `1`, then `2`, then `3`. The count lives in the contract's storage on chain.

## What the code does

| Line | What it does |
|---|---|
| `symbol_short!("COUNTER")` | The key the number is saved under |
| `.get(&COUNTER).unwrap_or(0)` | Reads the number, or 0 the first time |
| `.set(&COUNTER, &count)` | Saves the new number |
| `.extend_ttl(50, 100)` | Keeps the data alive. Data that isn't extended gets archived |

## Try next

Add a `get_current_value` function that reads the count without changing it. Then try `decrement` and `reset`.

---

Previous: [03 · Your first contract](../03-hello-world/) · Next: [05 · The rest of your dApp](../05-dapp/)
