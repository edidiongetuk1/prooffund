# Boundless Build Day, Uyo

12 November 2026 at Teckcocoon, Uyo, with Teckcocoon and Stellar Nigeria.

Every command and every piece of code from the slides is here, so you can copy instead of typing. Each section has its own folder, in the same order as the talk, and each one has a QR code on its slide. Every code box on GitHub has a copy button at its top right.

You don't need to know Rust, and you don't need to be a developer to join in.

## Follow the slides

| Section | Slides | What you'll do |
|---|---|---|
| [01 · Setup](01-setup/) | 6–7 | Install Rust, the Wasm target and the Stellar CLI |
| [02 · Testnet account](02-account/) | 8 | Make a free test account called alice |
| [03 · Your first contract](03-hello-world/) | 9–11 | Init, build, deploy and call a hello contract |
| [04 · Store a counter](04-counter/) | 12 | A contract that remembers a number on chain |
| [05 · The rest of your dApp](05-dapp/) | 13 | A TypeScript client, a wallet and a web page |
| [06 · Build faster with AI](06-ai-tools/) | 14 | Give your coding agent the real Stellar docs |
| [07 · Already built on Stellar](07-ecosystem/) | 15–29 | 14 projects you can build on in your hack |
| [08 · Get funded](08-funding/) | 30–32 | SCF, Instawards and more ways to earn |
| [09 · Hack guide](09-hack/) | 34–36 | Rules, ideas, judging and how to submit |

Also here:

- [Cheat sheet](CHEATSHEET.md): every command, and fixes for common errors
- [Extras](extras/): sign the wall, send a payment, and the Prize Pot

## Get the code

The easiest way has nothing to install. It needs a free GitHub account:

[![Open in GitHub Codespaces](https://github.com/codespaces/badge.svg)](https://codespaces.new/boundlessfi/build-day-uyo)

It takes a few minutes to start, and it runs in your browser, so it's a good choice for an older laptop.

If you've done the [setup](01-setup/) on your own laptop:

```bash
git clone https://github.com/boundlessfi/build-day-uyo.git
```

```bash
cd build-day-uyo
```

No Git? Click the green **Code** button at the top of this page, then **Download ZIP**.

## How the day works

- Copy the commands from here. Nobody needs to type long lines from the screen.
- Scanned a QR code on your phone? Open the same section on your laptop to copy from it.
- Put your green note on your laptop when you finish a step, and your red note up when you're stuck. A mentor will come to you.
- If your neighbour is stuck, help them. It's the fastest way to learn this.
- If your laptop won't cooperate, pair with someone and keep going. We'll fix it at the break.

## Side quests

Finished early? Try these. Tell a mentor when you've done one.

1. Be the first person on [the wall](extras/1-sign-the-wall.md).
2. Find your wall signature on the [explorer](https://stellar.expert/explorer/testnet).
3. Pay three different people in the room.
4. Change the greeting in your hello contract to your own words and deploy it.
5. Add a `get_current_value` function to the counter.
6. Add a `refund` function to the [Prize Pot](extras/4-prize-pot.md).
7. Split the prize between two winners.

## What's in this repo

| Folder | What it holds |
|---|---|
| `01-setup` to `09-hack` | One guide per section of the talk |
| `extras` | Bonus activities that use the contracts below |
| `contracts/hello-uyo` | A ready-made hello contract |
| `contracts/wall` | The contract behind the wall on the big screen |
| `contracts/prize-pot` | A contract that holds a prize and pays a winner |
| `web/wall.html` | The wall page. Add `?screen` to the address for the projector view |
| `web/starter.html` | A one-file web page to start your hack from |
| `wasm/` | The three contracts, already built, for when you can't build them yourself |

Everything today runs on Stellar testnet, where the money is free and has no value. Testnet is wiped on 16 December 2026, so keep your code on GitHub.
