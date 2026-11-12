# 4. Prize Pot

A host promises a prize. How does anyone know the money exists?

In this contract, the host locks the prize first. When the host names a winner, the contract pays them. No person holds the money, and nobody else can take it.

Open [contracts/prize-pot/src/lib.rs](../contracts/prize-pot/src/lib.rs) and read along. It has three functions:

| Function | What it does |
|---|---|
| `fund` | The host locks the prize in the contract |
| `pay_winner` | The host names a winner and the contract pays them |
| `prize` | Anyone can check how much is locked |

## Run the tests

Tests run on your laptop against a pretend network, so they're free and quick.

```bash
cargo test -p prize-pot
```

You should see `test result: ok. 2 passed`. The first run is slow. If you're short of time, skip this and carry on.

## Put it on the network

```bash
stellar contract build
```

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/prize_pot.wasm --alias pot
```

Can't build? Use the ready-made one:

```bash
stellar contract deploy --wasm wasm/prize_pot.wasm --alias pot
```

## Lock the prize

XLM has its own contract address on testnet. Save it as `xlm`, so you don't have to paste it every time:

```bash
stellar contract alias add xlm --id CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC
```

Now lock 100 XLM. Contracts count in stroops, and 1 XLM is 10,000,000 stroops, so 100 XLM is 1000000000.

```bash
stellar contract invoke --id pot -- fund --host alice --token xlm --prize 1000000000
```

Check it:

```bash
stellar contract invoke --id pot -- prize
```

You should see `"1000000000"`. Your 100 XLM now sits in the contract.

## Try to rob the pot

Create a second account for the winner:

```bash
stellar keys generate winner --fund
```

Now be the thief. Try to pay yourself, as the winner account:

```bash
stellar contract invoke --id pot --source-account winner -- pay_winner --winner winner
```

It fails. The error says a signing key is missing, because `pay_winner` needs the host's signature and the winner doesn't have it. That one line, `host.require_auth()`, is protecting the money.

## Pay the winner

Now do it as the host:

```bash
stellar contract invoke --id pot -- pay_winner --winner winner
```

Check the winner's balance:

```bash
stellar contract invoke --id xlm -- balance --id winner
```

You should see `"101000000000"`. That's 10,100 XLM: the 10,000 from Friendbot plus the prize. Click the link in your terminal and find the transfer on the explorer.

## Try to pay twice

```bash
stellar contract invoke --id pot -- pay_winner --winner winner
```

It fails with a long, ugly error, because the prize is gone. Can you see which line in `pay_winner` makes sure of that?

Put your green note up. You've just written the core idea behind on-chain prizes.

## Finished early?

- Add a `refund` function so the host can take the prize back.
- Split the prize between two winners.
- Add a deadline, so the winner can claim if the host disappears.
- Replace the ugly error with a clear one using `#[contracterror]`.

Run `cargo test -p prize-pot` as you go. The same contract works with any token, including USDC.
