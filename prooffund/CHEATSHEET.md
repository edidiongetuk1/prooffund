# Cheat sheet

The Prize Pot and Hello Uyo commands here assume you ran `stellar network use testnet` and `stellar keys use alice` first, as the [extras](extras/) do. The workshop sections pass `--network testnet --source-account alice` instead.

## Commands

| I want to | Command |
|---|---|
| Use testnet by default | `stellar network use testnet` |
| Create a funded account | `stellar keys generate alice --fund` |
| Make it my default account | `stellar keys use alice` |
| See its address | `stellar keys address alice` |
| Build every contract | `stellar contract build` |
| Run a contract's tests | `cargo test -p prize-pot` |
| Deploy | `stellar contract deploy --wasm target/wasm32v1-none/release/hello_uyo.wasm --alias hello` |
| Call a function | `stellar contract invoke --id hello -- hello --to Uyo` |
| List a contract's functions | `stellar contract invoke --id hello -- --help` |
| See a contract's address | `stellar contract alias show hello` |
| Save an address as a short name | `stellar contract alias add wall --id C...` |
| Find XLM's contract address | `stellar contract id asset --asset native` |
| Start a new contract in this repo | `stellar contract init . --name my-idea` |

Everything after the `--` in `invoke` is the function you're calling and its arguments.

## Numbers

- 1 XLM is 10,000,000 stroops, and contracts count in stroops.
- 100 XLM is `1000000000`.
- Friendbot gives each new testnet account 10,000 XLM.
- An account needs at least 1 XLM to exist. You get that back if you close it.
- Each trustline sets aside another 0.5 XLM.

## Where to look

| What | Where |
|---|---|
| See any account or transaction | [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet) |
| Try things in the browser | [lab.stellar.org](https://lab.stellar.org) |
| Read the docs | [developers.stellar.org](https://developers.stellar.org) |
| Give your AI tool the real docs | [developers.stellar.org/mcp](https://developers.stellar.org/docs/build/building-with-ai) |

## Common problems

**`stellar: command not found`**: restart your terminal. If that doesn't help, repeat the CLI step in [01 · Setup](01-setup/#3-install-the-stellar-cli).

**`can't find crate for core` or a message about `wasm32v1-none`**: run `rustup target add wasm32v1-none`.

**`An identity with the name 'alice' already exists`**: you've already got an alice, so carry on. To start again, add `--overwrite`.

**Funding failed or timed out**: Friendbot may be busy with the whole room. Wait a few seconds and try again:

```bash
stellar keys fund alice
```

**`Missing signing key`**: the account you're using can't sign for what you asked. Check `--source-account`.

**`Account not found`**: the account has no XLM on testnet yet. Run `stellar keys fund alice`.

**`Error(WasmVm, InvalidAction)` or `UnreachableCodeReached`**: the contract panicked on purpose. Read the function you called and look for a `panic!` or an `unwrap()`.

**`op_no_trust`**: the receiver hasn't opened a trustline for that asset yet.

**The first build is taking forever**: that's normal. Later builds are far quicker. If you can't wait, use the files in `wasm/`.

**PowerShell shows question marks**: use Windows Terminal.

**Still stuck?** Put your red note up.
