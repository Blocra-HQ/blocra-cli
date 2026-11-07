# blocra

[![CI](https://github.com/Blocra-HQ/blocra-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Blocra-HQ/blocra-cli/actions/workflows/ci.yml)
![Node](https://img.shields.io/badge/node-%3E%3D18-339933?logo=node.js)
![Dependencies](https://img.shields.io/badge/dependencies-0-brightgreen)
![License](https://img.shields.io/badge/license-MIT-green)

**Signed, verifiable traction reports for on-chain projects, from your terminal or CI.**

Grant programs, investors and partners ask for usage numbers, and screenshots or self-reported
figures are easy to doubt. `blocra` fetches a traction report for your contracts or assets that
[BlocRA](https://github.com/Blocra-HQ) computes from on-chain data and signs, and lets anyone verify
a copy in one command.

```bash
npx blocra init --api https://<blocra-api> --subject <your contract id>
npx blocra report
npx blocra verify reports/*.md
```

Chains: **Stellar** today (Soroban contracts and Stellar assets); the CLI and API take a `--chain`
option so more chains plug in without changes.

---

## How it works

1. **BlocRA computes the report** from every ledger it collects: active, new and returning accounts,
   calls or payments, failure rate, how concentrated activity is and how much comes from automated
   accounts, function usage, and rank among all contracts or assets in the same period.
2. **BlocRA signs it** with its Ed25519 key. The signature covers the exact report bytes.
3. **`blocra` only fetches and verifies.** Nothing is computed on your machine, so the numbers
   can't be adjusted before they reach a reviewer; editing a single figure breaks verification.

```
 your repo / CI ──► blocra report ──► BlocRA API ──► signed report
                                                        │
 reviewer ─────────► blocra verify ◄── report.md ◄──────┘
                     (signature + pinned BlocRA key + text unchanged)
```

A Markdown report carries its signed data in an HTML comment at the end, so the one file is enough:
`blocra verify` checks the signature, checks it was made by the BlocRA key you trust, then re-renders
the report from the signed data and compares it with the file's text.

## Commands

### `blocra init`

Writes `blocra.json` in the current directory and pins BlocRA's signing key (trust on first use).

```bash
npx blocra init --api https://<blocra-api> \
  --subject CDL74RF5BLYR2YBLCCI7F5FB6TPSCLKEJUBSD2RSVWZ4YHF3VMFAIGWA \
  --subject USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN \
  --days 30
```

```json
{
  "api": "https://<blocra-api>",
  "chain": "stellar",
  "subjects": ["CDL74RF5…", "USDC:GA5Z…"],
  "days": 30,
  "trusted_key": "GBQ6QIHV…",
  "out": "reports"
}
```

**Relayers.** Users of apps that submit transactions through a relayer, fee sponsor or smart wallet
are found automatically: BlocRA counts each contract call for the account that signed its
authorization, not the account that submitted it. Pass `--relayer <account>` (repeatable) only for
relayers that sign everything themselves; their calls are then reported separately instead of
counting as one very active "user".

### `blocra report [subject...]`

Fetches a signed report per subject (from `blocra.json` unless given), verifies it, prints a summary
and writes `<name>.md` and `<name>.json`.

```
Contract CDL74R…IGWA stellar · 2026-10-31 – 2026-11-06
  4,638 active accounts · 144,758 calls · 1.5% failed · rank #1 of 146
  - 1.5% of calls failed, in line with the 0.8% median of comparable contracts.
  - Ranked #1 of 146 contracts by active accounts in this period (top 1%).
  ✓ signed by GBQ6QIHV… · sha256 332782de86613850…
  → reports/blocra-stellar-cdl74rf5blyr-2026-11-06-7d.json
  → reports/blocra-stellar-cdl74rf5blyr-2026-11-06-7d.md
```

| Option | Default | |
|--------|---------|---|
| `--days <n>` | `7` (or `days` in `blocra.json`) | Report period |
| `--chain <name>` | `stellar` | Chain of the subjects |
| `--out <dir>` | `.` (or `out`) | Output directory |
| `--format <f>` | `md,json` | `md`, `json` or both |

### `blocra verify <file...>`

Verifies `.md` or `.json` reports. Exit code `0` only if every file is valid.

```
✓ VALID reports/blocra-stellar-usdc-2026-11-06-7d.md
  USDC (issuer GA5Z…KZVN) · stellar · 2026-10-31 – 2026-11-06 · traction/1
  Signed by GBQ6QIHVDHFSY4CDORDELIZXRUWF72FTCRNOXUFPH7YOO3PFFMXLAVUN at 2026-11-06T12:43:14Z
  sha256 f3c59e0ccf034c68922f20ca45ea3125ff62d779cdf5cce375f5a365731d043d

✗ FAIL edited.md
  - The Markdown text was edited after signing (it no longer matches the signed report)
```

The key to trust comes from `--key`, `BLOCRA_TRUSTED_KEY` or `trusted_key` in `blocra.json`;
otherwise BlocRA's official keys built into the package (`src/defaults.js`, which keeps past keys so
older reports still verify); otherwise the key the API serves, with a notice. A reviewer who received
a report needs no setup: `npx blocra verify report.md`.

### `blocra key`

Prints the API's signing key, methodology version and supported chains.

## Using the website

Reports exported from the BlocRA website (`/reports`) are the same files `blocra report` writes:
**Verifiable report (.md)** is this CLI's Markdown with the signed envelope embedded, and **Signed
data (.json)** is the envelope. Both verify with `npx blocra verify <file>`. The website's PDF is a
rendering for reading and is not verifiable on its own; ask for the `.md` or `.json`. The website's
`/verify-report` page runs the same checks in the browser (WebCrypto Ed25519).

The website uses this package's renderer rather than its own: `blocra/markdown` (`renderBody`,
`renderMarkdown`, `parseMarkdown`, `bodyMatchesReport`) and `blocra/strkey` have no Node-only
dependencies, so they run in browsers unchanged.

```js
import { renderMarkdown, parseMarkdown } from "blocra/markdown";
```

## GitHub Action

Publish a fresh, verified report every month:

```yaml
# .github/workflows/traction.yml
name: Traction report
on:
  schedule:
    - cron: "0 6 1 * *"
  workflow_dispatch:
permissions:
  contents: write
jobs:
  report:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: Blocra-HQ/blocra-cli@v0
        with:
          days: 30          # api, subjects and trusted key come from blocra.json
      - run: |
          git config user.name "blocra-bot"
          git config user.email "blocra-bot@users.noreply.github.com"
          git add reports && git commit -m "Monthly traction report" && git push || echo "No changes"
```

## Report contents

Scorecard (with change against the previous period), key findings, daily activity, user quality
(returning accounts, top-10 share, automated accounts, share of calls sent through relayers,
smart-wallet users, self-payments for assets), contract functions
or asset holders and supply, ecosystem rank, coverage and methodology, and the signature. Every
figure comes from BlocRA's API; the findings are fixed rules over those figures, not model output.

## Trust and limitations

- **What a valid signature proves:** BlocRA produced exactly this report, with the stated
  methodology version, at the stated time. It does not make BlocRA's data infallible; coverage
  (ledgers collected, completeness, time covered) is stated in every report, and partial data is
  flagged.
- **Pin the key.** Verifying against whatever key a server returns only proves consistency with that
  server; `init` pins the key and reviewers should use BlocRA's published one.
- **Zero dependencies.** The package uses only Node's built-in `fetch` and `crypto`, and never touches
  private keys.
- **Roadmap:** `blocra publish` (record the report's sha256
  in BlocRA's on-chain registry), PDF output, README badge, and more chains.

## Development

```bash
npm test            # node:test, no dependencies
node bin/blocra.js --help
```

`src/markdown.js`, `src/strkey.js` and `src/defaults.js` are copied into blocra-frontend
(`src/lib/blocra/`); after changing them, run `node scripts/sync-blocra.mjs --write` there
(its `npm run check:blocra` fails until the copies match).

## License

[MIT](LICENSE)
