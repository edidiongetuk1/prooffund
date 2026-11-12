# 07 · Already built on Stellar

Slides 15 to 29. You don't have to build everything yourself. Other teams have already done the hard parts, and most of them work on testnet, so you can use them in your hack.

These are the 14 projects from the slides, with three things you could build with each. For everything else, see [stellar.org/ecosystem](https://stellar.org/ecosystem).

## DeFi

### Soroswap

Swap aggregator and AMM. [soroswap.finance](https://soroswap.finance) · [GitHub](https://github.com/soroswap) · Start with `npm install @soroswap/sdk`

| You could build | How |
|---|---|
| A swap button inside your dApp | Aggregator API: quote, build, sign, send |
| Pay in any token, receive USDC | An exact-out quote sent to the receiver |
| A contract that swaps on chain | Cross-contract call to the Soroswap router |

### Blend

Lending pools on Stellar. [blend.capital](https://blend.capital) · [GitHub](https://github.com/blend-capital) · Start with `npm install @blend-capital/blend-sdk`

| You could build | How |
|---|---|
| A lend-and-earn tab in your wallet | JS SDK: supply and withdraw requests |
| A live lending rates dashboard | Read pool and reserve data, no wallet needed |
| A vault that deposits into Blend | The blend-contract-sdk Rust crate |

### Aquarius

AMM and liquidity layer. [aqua.network](https://aqua.network) · [GitHub](https://github.com/AquariusDeFi) · Start with `npm install @aquariusdefi/sdk`

| You could build | How |
|---|---|
| Best-route token swaps in your app | Find Path API plus the router contract |
| Earn a fee on every swap you route | Your own integrator fee collector contract |
| An LP manager that claims rewards | SDK: deposit liquidity, claim AQUA rewards |

### DeFindex

Yield vaults for wallets and apps. [defindex.io](https://defindex.io) · [GitHub](https://github.com/defindex-io) · Start with `npm install @defindex/sdk`

| You could build | How |
|---|---|
| A savings tab for any wallet | depositToVault and getVaultAPY |
| A no-loss lottery paid from yield | getVaultBalance and withdrawFromVault |
| A contract that deposits into vaults | Vault deposit through contractimport! |

### XOXNO

Lending and swaps on Stellar. [xoxno.com](https://xoxno.com) · [GitHub](https://github.com/XOXNO) · Start with `npm install @xoxno/sdk-js`

| You could build | How |
|---|---|
| A dashboard of lending rates | The stellar-lending read client in the SDK |
| A vault that earns lending yield | The xoxno-contract-sdk Rust crate |
| In-app swaps with the best quote | Stellar aggregator quote and execute |

## Infrastructure

### Reflector

Price oracle for Stellar. [reflector.network](https://reflector.network) · [GitHub](https://github.com/reflector-network) · Start with `npm install @reflector/contract-client`

| You could build | How |
|---|---|
| A lending app that liquidates safely | Pulse lastprice() for live asset prices |
| A vault that rebalances on prices | prices() history for each asset |
| Alerts when XLM moves 5% | Flare subscriptions that call your webhook |

### OpenZeppelin

Audited contracts for Soroban. [openzeppelin.com/networks/stellar](https://openzeppelin.com/networks/stellar) · [GitHub](https://github.com/OpenZeppelin/stellar-contracts) · Start with [wizard.openzeppelin.com/stellar](https://wizard.openzeppelin.com/stellar)

| You could build | How |
|---|---|
| A stablecoin with an allowlist | Contracts Wizard: the Stablecoin type |
| An NFT collection with royalties | The non-fungible royalties extension |
| A token-voting DAO with a timelock | Governor, Votes and Timelock Controller |

### Stellar Scaffold

Full-stack dApps in one command. [stellarscaffold.org](https://stellarscaffold.org) · [GitHub](https://github.com/stellar-scaffold) · Start with `stellar scaffold init my-app`

| You could build | How |
|---|---|
| A React game with a typed client | init --template react, client generated for you |
| An NFT mint page from an OZ example | generate contract --from oz/nft-royalties |
| A wallet-connected Svelte dApp | --template svelte with the app-lib wallet helpers |

### Circle CCTP

Move native USDC on and off Stellar. [circle.com/cctp](https://circle.com/cctp) · [GitHub](https://github.com/circlefin/stellar-cctp) · Start with [developers.circle.com/cctp](https://developers.circle.com/cctp)

| You could build | How |
|---|---|
| A USDC bridge fee checker | Iris API: /v2/burn/USDC/fees/27/{domain} |
| Send testnet USDC to Base | deposit_for_burn on TokenMessengerMinter |
| A transfer status tracker | Iris API: /v2/messages/27?transactionHash= |

## Payments and apps

### Trustless Work

Stablecoin escrow as a service. [trustlesswork.com](https://trustlesswork.com) · [GitHub](https://github.com/Trustless-Work) · Start with `npm install @trustless-work/escrow`

| You could build | How |
|---|---|
| Freelance gigs paid per milestone | Multi-release escrow: approve, then release |
| Hold USDC until the order arrives | Single-release escrow: fund, then release |
| Grants with dispute resolution | Dispute and resolve with a resolver role |

### Useroutr

Any-asset checkout on Stellar. [useroutr.com](https://useroutr.com) · [GitHub](https://github.com/useroutr) · Start with `npm install @useroutr/sdk`

| You could build | How |
|---|---|
| Sell AI credits payable in any token | Funding intents and a credited webhook |
| A drop-in checkout for your store | The @useroutr/checkout-react component |
| Game top-ups settled in USDC | Hosted checkout and the balances API |

### Vellar

x402 payments for AI agents. [vellar.xyz](https://vellar.xyz) · [GitHub](https://github.com/Vellar-Wallet) · Start with [docs.vellar.xyz](https://docs.vellar.xyz)

| You could build | How |
|---|---|
| Charge USDC per API request | An x402 paywall with the Vellar facilitator |
| An agent that finds and pays APIs | Bazaar search with vellar-cli or the MCP payer |
| A passkey wallet with an agent budget | vellar-sdk smart account and a session key |

### Pollar

Payments SDK for LATAM apps. [pollar.xyz](https://pollar.xyz) · [GitHub](https://github.com/pollar-xyz) · Start with `npm install @pollar/react`

| You could build | How |
|---|---|
| Google sign-in with a Stellar wallet | PollarProvider and social-login wallets |
| Accept Pix or bank QR payments | Pollar ramps: Brazil, Bolivia, Argentina |
| Send USDC without users holding XLM | Sponsored fees and managed trustlines |

## Governance

### Tansu

On-chain governance for open source. [tansu.dev](https://tansu.dev) · [GitHub](https://github.com/Consulting-Manao/tansu) · Start with [app.tansu.dev](https://app.tansu.dev)

| You could build | How |
|---|---|
| A DAO for your hackathon team | Register a project, then vote on proposals |
| A live feed of project decisions | Read ProposalCreated and VoteCast events |
| A contract that checks a release | Cross-contract read of a project’s commit |

## Quick picker: I want my app to...

| I want to | Look at |
|---|---|
| Start with contracts and a React app together | [Scaffold Stellar](https://developers.stellar.org/docs/tools/scaffold-stellar) |
| Let people sign in with Google or email and get a wallet | [Pollar](https://docs.pollar.xyz) |
| Hold money in escrow without writing a contract | [Trustless Work](https://docs.trustlesswork.com/trustless-work) |
| Swap tokens at the best price | [Soroswap](https://docs.soroswap.finance) |
| Give users a balance that earns yield | [DeFindex](https://docs.defindex.io) |
| Lend and borrow | [Blend](https://www.blend.capital), [XOXNO Lending](https://immunefi.com/bug-bounty/xoxno/) |
| Use a price inside a contract | [Reflector](https://reflector.network) |
| Reuse contract code that's been reviewed | [OpenZeppelin Stellar Contracts](https://docs.openzeppelin.com/stellar-contracts) |
| Connect to the wallets people already use | [Stellar Wallets Kit](https://stellarwalletskit.dev) |
| Pay lots of people at once | [Stellar Disbursement Platform](https://developers.stellar.org/docs/platforms/stellar-disbursement-platform) |

## Worth a closer look

**Scaffold Stellar** gives you example contracts, generated TypeScript clients and a React front end from one command. It needs extra installs and takes a lot of memory to compile, and it needs Docker if you want a local network, so try it at home before you rely on it.

**Pollar** gives your users a wallet with no seed phrase. It covers sign-in with Google, GitHub or email, paying fees for users, and cash on and off ramps. Install `@pollar/react` and `@pollar/core`. New apps start on testnet.

**Trustless Work** is escrow as a service. You call an API and it handles the contract, with single payouts or milestones. The testnet API is at `dev.api.trustlesswork.com`, and there's a [demo app](https://demo.trustlesswork.com). You get an API key from their [dashboard](https://dapp.trustlesswork.com).

**Soroswap** finds the best price across Soroswap, Aquarius, SushiSwap, Comet and Stellar's built-in exchange, then gives you a transaction to sign. Add `?network=testnet` to use it on testnet. The free plan allows one request a second, and there's a [beginner's guide](https://docs.soroswap.finance/api/beginner-guide) that builds a swap app in the browser.

**DeFindex** comes from PaltaLabs, the team behind Soroswap. It lets apps offer a balance that earns yield by putting deposits into vaults. There's a [testnet sandbox](https://docs.defindex.io/integration-guide/sandbox).

**Blend** is Stellar's biggest lending protocol, and **Aquarius** is its biggest liquidity protocol. Most apps here that earn yield or swap tokens touch one of them.

**XOXNO** is known on MultiversX for its NFT marketplace and liquid staking. On Stellar, its product is XOXNO Lending.

**KALE** is a farming game on Stellar, where you plant, work and harvest together. It became one of the network's busiest contracts within months. It's a good reminder that the fun things get used. Read [the story of KALE](https://stellar.org/blog/ecosystem/the-story-of-kale).

---

Previous: [06 · Build faster with AI](../06-ai-tools/) · Next: [08 · Get funded](../08-funding/)
