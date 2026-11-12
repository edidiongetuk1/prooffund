# ProofFund

Milestone-based community funding on Stellar. Donors fill a campaign, the money stays locked in a Soroban contract, and each milestone is paid out only after the beneficiary shows proof and the admin approves it. If the target is missed by the deadline, donors take their money back.

Built for Boundless Build Day, Uyo. Contract: [`contracts/proof-fund`](contracts/proof-fund/src/lib.rs). Page: [`web/prooffund.html`](web/prooffund.html).

## How it works

| Function | Who | What it does |
|---|---|---|
| `init(admin, token)` | admin | One-time setup. Token is the native XLM contract |
| `create_campaign(beneficiary, title, milestone_titles, milestone_amounts, deadline)` | admin | Target is the sum of the milestones (1 to 5). Amounts in stroops |
| `donate(id, donor, amount)` | donor | Until the deadline, up to the target. At the target the campaign becomes Active |
| `submit_evidence(id, index, evidence)` | beneficiary | Next milestone only, in order |
| `release_milestone(id, index)` | admin | Pays that milestone's amount to the beneficiary, and only to them |
| `refund(id, donor)` | donor | After the deadline, only if the target was missed |
| `get_campaign`, `get_contribution`, `campaign_count` | anyone | Read-only |

Trust assumption: the admin judges the evidence off-chain. The admin cannot move money anywhere except to the beneficiary set at creation.

## Test

```bash
cargo test -p proof-fund
```

Expect `9 passed`.

## Deploy to testnet

Run these in Codespaces or after the [setup](01-setup/). Use the `alice` account from the workshop as admin.

```bash
stellar contract build
```

```bash
stellar contract deploy --wasm target/wasm32v1-none/release/proof_fund.wasm --alias proof
```

Save the native XLM address, then set up the contract (the admin is alice):

```bash
stellar contract alias add xlm --id $(stellar contract id asset --asset native)
```

```bash
stellar contract invoke --id proof -- init --admin alice --token xlm
```

Make a beneficiary account:

```bash
stellar keys generate mama --fund
```

Create the borehole campaign. 30, 40 and 30 XLM are `300000000`, `400000000` and `300000000` stroops. Set the deadline to a unix time in the future (`date -d '+1 hour' +%s`):

```bash
stellar contract invoke --id proof -- create_campaign --beneficiary mama --title "Community borehole" --milestone_titles '["Materials","Drilling","Installation"]' --milestone_amounts '["300000000","400000000","300000000"]' --deadline $(date -d '+1 hour' +%s)
```

It prints the campaign id (`1`).

## Walk through the demo

```bash
stellar contract invoke --id proof -- donate --id 1 --donor alice --amount 1000000000
```

```bash
stellar contract invoke --id proof --source-account mama -- submit_evidence --id 1 --index 0 --evidence "https://example.com/materials-photo"
```

```bash
stellar contract invoke --id proof -- release_milestone --id 1 --index 0
```

Check `mama`'s balance, then open the transaction on the [explorer](https://stellar.expert/explorer/testnet).

Try to break it: release milestone 2 before evidence, donate past the target, release as `mama`. Each fails.

**Refund demo:** create a second campaign with a deadline about 2 minutes away, donate a little, wait, then run `refund`.

## Use the web page

1. Open `web/prooffund.html` and paste your contract ID into `PROOF_ID`.
2. Open the file in a browser (or host the `web` folder).
3. To act as admin or beneficiary, paste their testnet secret key under "Switch account". Get it with `stellar keys show alice`. Testnet keys only.
