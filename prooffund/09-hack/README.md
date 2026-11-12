# 09 · Hack guide

Slides 34 to 36.

Build something useful on Stellar. It has to work on testnet by 4:30 PM.

Three hours is short. One complete flow that works beats a big idea that doesn't.

## Prizes

| Place | Prize |
|---|---|
| 1st | 200 USDC |
| 2nd | 150 USDC |
| 3rd | 150 USDC |

## Rules

- Work alone or in a team of up to four.
- Build it today. You can start from any code in this repo.
- AI tools are welcome, but you must be able to explain your code.
- It has to run on testnet.
- Submit by 4:30 PM on [Boundless](https://www.boundlessfi.xyz) with your GitHub link, a contract ID or transaction link, and a short description.

## How we'll judge

| Weight | What we look for |
|---|---|
| 40% | It works on testnet, and you can show a live transaction |
| 25% | It solves a real problem |
| 20% | It makes good use of Stellar |
| 15% | The demo is clear |

## The afternoon

| Time | Checkpoint |
|---|---|
| 1:30 | Start |
| 2:30 | Your idea is locked |
| 3:30 | Something is running on testnet |
| 4:15 | Stop adding features and practise your demo |
| 4:30 | Submit |

## Ideas

| Idea | Start from |
|---|---|
| A savings group that collects from members and pays each one in turn | [Prize Pot](../extras/4-prize-pot.md), plus a list of members |
| Pay on delivery: the buyer locks the money until the goods arrive | Prize Pot, or the [Trustless Work](../07-ecosystem/) API |
| Milestone payments for freelancers | Prize Pot, paid in parts |
| Splitting a hostel bill or a group payment | Prize Pot, with several winners |
| A tip jar for creators | [The wall](../extras/1-sign-the-wall.md), plus a payment |
| Event tickets you can check at the door | The wall, with one entry per address |
| A guest book or shout-out wall for your community | The wall and `web/wall.html` |

## Pick a path that fits your team

**Contract and web page.** Copy a contract, change it, and use [`web/starter.html`](../web/starter.html) as your front end.

**Web only, no Rust.** Use a contract that's already live, or call a service like [Trustless Work](../07-ecosystem/) or [Soroswap](../07-ecosystem/) from JavaScript.

**Not a coder?** Your team needs you. Talk to real users, design the screens, write the pitch and lead the demo. A clear demo can beat stronger code.

## Start a new contract

```bash
stellar contract init . --name my-idea
```

That creates `contracts/my-idea`. Then build it and deploy it the same way:

```bash
stellar contract build
```

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/my_idea.wasm --alias my-idea
```

## Using AI

AI tools know old Stellar code, so give them the current docs. [06 · Build faster with AI](../06-ai-tools/) has the commands, and [Building with AI](https://developers.stellar.org/docs/build/building-with-ai) shows how to add Stellar Skills or the Stellar Docs server to your tool. You can also paste in [developers.stellar.org/llms.txt](https://developers.stellar.org/llms.txt).

## Your demo

You have three minutes, plus one for questions:

1. Say what problem you're solving and who has it.
2. Show it working live, including a real transaction.
3. Say what you'd do next.

Have a screen recording ready in case the wifi lets you down.

---

Previous: [08 · Get funded](../08-funding/) · Back to [the start](../)
