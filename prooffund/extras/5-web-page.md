# 5. Web page

A contract needs a front door. [web/starter.html](../web/starter.html) is a single file that talks to contracts from the browser. There's nothing to install and no build step.

## Try it

1. Open `web/starter.html` in your browser. You can double-click it.
2. Click **Say hello**.
3. Type a name and a message, then click **Sign the wall**.

The first click takes about 30 seconds, because the page is creating a test account. After that it's quick.

## Point it at your own contract

Get your contract's address:

```bash
stellar contract alias show hello
```

Open `web/starter.html` in VS Code and paste it at the top of the script:

```js
const HELLO_ID = "PASTE_YOUR_ID_HERE";
```

Save the file and refresh the page. If you changed the greeting, you'll see your own words.

## Change what the buttons do

There are only two kinds of call:

```js
// Read: free, and nothing is saved.
read(POT_ID, "prize");

// Write: a real transaction, signed by the page's test account.
write(WALL_ID, "sign", address(me.publicKey()), text("Ada"), text("gm"));
```

Pass arguments with `text()`, `address()`, `amount()` and `number()`, in the same order as your contract's function.

## Using a real wallet

The starter keeps a throwaway test key in the browser. That's fine for a demo, and never fine for real money. When you're ready for real users:

- [Stellar Wallets Kit](https://stellarwalletskit.dev) connects your app to the wallets people already use.
- [Pollar](https://docs.pollar.xyz) lets people sign in with Google, GitHub or email and gives them a wallet with no seed phrase.

## Want a typed client?

If your team uses TypeScript, this generates a client from your deployed contract. Run it from your own project, not this repo:

```bash
npx @stellar/stellar-sdk generate --contract-id YOUR_CONTRACT_ID --network testnet --rpc-url https://soroban-testnet.stellar.org --contract-name my-contract --output-dir packages/my-contract
```

Older tutorials use `stellar contract bindings typescript`. It's deprecated.
