# 02 · Make a testnet account

Slide 8. Testnet money is free and fake, so this is the place to break things.

## Create an account

This makes a key called `alice` and funds it with free test XLM:

```bash
stellar keys generate alice --network testnet --fund
```

See its public address. It starts with G:

```bash
stellar keys address alice
```

See everything the CLI can do: keys, contracts, networks and more:

```bash
stellar --help
```

Then open your Varial wallet and switch it to **Testnet** too.

## Testnet or mainnet?

| | Testnet | Mainnet |
|---|---|---|
| Money | Free test XLM | Real XLM |
| Use it for | Learning and testing | Live products |
| A mistake costs | Nothing | Real money |

Everything today runs on testnet. Testnet is wiped on 16 December 2026, so keep your code on GitHub.

## If something goes wrong

**`An identity with the name 'alice' already exists`**: you already have an alice, so carry on. To start again, add `--overwrite`.

**Funding failed or timed out**: the faucet is busy with the whole room. Wait a few seconds and try again:

```bash
stellar keys fund alice --network testnet
```

---

Previous: [01 · Setup](../01-setup/) · Next: [03 · Your first contract](../03-hello-world/)
