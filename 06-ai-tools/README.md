# 06 · Build faster with AI

Slide 14. AI tools know old Stellar code. Stellar Raven gives your coding agent the current docs and live ecosystem data.

Raven has two tools:

| Tool | What it does |
|---|---|
| `search` | Finds Stellar docs and ecosystem info |
| `execute` | Runs queries on live ecosystem data |

## Claude Code

```bash
claude mcp add --transport http stellar-raven "https://raven.stellar.org/mcp"
```

Then type `/mcp` in Claude Code, choose **Authenticate**, and sign in.

## Codex

```bash
codex mcp add stellar-raven --url "https://raven.stellar.org/mcp"
```

```bash
codex mcp login stellar-raven
```

## Cursor, VS Code, Claude desktop

Add a remote MCP server with this URL:

```text
https://raven.stellar.org/mcp
```

## Stellar Skills

Skills teach your agent Stellar best practices. In Claude Code:

```text
/plugin marketplace add stellar/stellar-dev-skill
```

More options, including `llms.txt`, are in [Building with AI](https://developers.stellar.org/docs/build/building-with-ai).

## Try it

Ask your agent to add a `reset` function to the counter from [04](../04-counter/), with a test. AI is welcome in the hack, but you must be able to explain your code.

---

Previous: [05 · The rest of your dApp](../05-dapp/) · Next: [07 · Already built on Stellar](../07-ecosystem/)
