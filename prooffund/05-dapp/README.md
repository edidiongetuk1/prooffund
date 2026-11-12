# 05 · Around the contract: the rest of your dApp

Slide 13. A contract alone isn't a product. Here is what goes around it.

| Frontend | Backend |
|---|---|
| A TypeScript client for your contract | Stellar RPC to read data and send transactions |
| A wallet to sign transactions | An indexer for history and events |

## Generate a TypeScript client

First get your contract's ID:

```bash
stellar contract alias show hello_world
```

Then generate a typed client from it. Run this in your own app's folder, and paste your ID in place of `YOUR_CONTRACT_ID`:

```bash
npx @stellar/stellar-sdk generate --contract-id YOUR_CONTRACT_ID --network testnet --output-dir ./hello-client
```

This needs [Node.js](https://nodejs.org). Older tutorials use `stellar contract bindings typescript`. That command still runs, but it's deprecated.

## Connect a wallet

- [Varial](https://varialwallet.com) is Stellar's browser wallet. It used to be called Freighter.
- [Stellar Wallets Kit](https://stellarwalletskit.dev) connects your app to most Stellar wallets with one button.
- [Pollar](https://docs.pollar.xyz) lets people sign in with Google, GitHub or email and gives them a wallet with no seed phrase.

## Talk to the network

Testnet RPC: `https://soroban-testnet.stellar.org`. See [RPC providers](https://developers.stellar.org/docs/data/apis/rpc/providers) for others.

## No build step: a one-file web page

[web/starter.html](../web/starter.html) talks to contracts straight from the browser. Open it, click **Say hello**, then point it at your own contract. [The walkthrough](../extras/5-web-page.md) shows how.

## Shortcut: Stellar Scaffold

[Stellar Scaffold](https://developers.stellar.org/docs/tools/scaffold-stellar) gives you contracts, a generated client and a React front end from one command. It needs more installs and a lot of memory, so try it at home first.

---

Previous: [04 · Store a counter](../04-counter/) · Next: [06 · Build faster with AI](../06-ai-tools/)
