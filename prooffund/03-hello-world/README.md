# 03 · Your first contract

Slides 9 to 11. Four commands take you from an empty folder to a live contract: init, build, deploy, invoke.

You need [setup](../01-setup/) done and an [alice account](../02-account/).

## 1. Create the project

Run this in any folder except inside `build-day-uyo`:

```bash
stellar contract init hello-world
```

```bash
cd hello-world
```

## 2. Read the contract

Open `contracts/hello-world/src/lib.rs`. This is the part that matters:

```rust
#[contract]
pub struct Contract;

#[contractimpl]
impl Contract {
    pub fn hello(env: Env, to: String) -> Vec<String> {
        vec![&env, String::from_str(&env, "Hello"), to]
    }
}
```

| In the code | What it does |
|---|---|
| `#[contract]` | Marks the struct as a contract |
| `#[contractimpl]` | Exposes functions anyone can call |
| `Env` | Access to storage, events and auth |
| `String` | Text the caller passes in |

## 3. Test, build, deploy, call it

Run each one from inside `hello-world`.

**Test.** Runs the unit tests on your laptop:

```bash
cargo test
```

**Build.** Compiles the contract to a `.wasm` file:

```bash
stellar contract build
```

**Deploy.** Puts it on testnet. You get back an ID starting with C:

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/hello_world.wasm --source-account alice --network testnet --alias hello_world
```

**Invoke.** Calls `hello` on your contract:

```bash
stellar contract invoke --id hello_world --source-account alice --network testnet -- hello --to Builders
```

You should see `["Hello","Builders"]`. Put your green note up.

## 4. Find it on the explorer

Copy your contract ID and search for it on [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet). That one is yours.

Lost the ID? This prints it:

```bash
stellar contract alias show hello_world
```

## Can't build?

Use the ready-made contract from this repo. It has the same `hello` function:

```bash
curl -LO https://github.com/boundlessfi/build-day-uyo/raw/main/wasm/hello_uyo.wasm
```

```bash
stellar contract deploy --wasm hello_uyo.wasm --source-account alice --network testnet --alias hello_world
```

Then run the invoke command above.

## Make it yours

Change `"Hello"` in `lib.rs` to your own greeting, in Pidgin, Ibibio, Efik or anything else. Then build, deploy and invoke again. Your new contract gets a new ID, because a deployed contract doesn't change by itself.

---

Previous: [02 · Testnet account](../02-account/) · Next: [04 · Store a counter](../04-counter/)
