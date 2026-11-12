# 1. Sign the wall

Open this on your phone and put your name on a blockchain:

**https://boundlessfi.github.io/build-day-uyo**

Type your name and a message, then tap **Sign the wall**. Your first one takes about 30 seconds. Then look at the big screen.

Keep it friendly, because it's public and stays on testnet until the next reset.

## What just happened

1. Your phone made a new Stellar account. That's a pair of keys.
2. Friendbot, the testnet faucet, gave your account 10,000 free test XLM.
3. You signed a transaction that called a contract.
4. The contract gave you a number and announced your note to the world.
5. The screen heard the announcement and showed your note.

Nobody had to approve any of this. Tap **See your transaction** to see it on a public explorer.

The contract is about 40 lines long: [contracts/wall/src/lib.rs](../contracts/wall/src/lib.rs). By the end of the morning you'll be able to read all of it.
