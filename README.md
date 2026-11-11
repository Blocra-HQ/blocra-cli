# blocra

[![CI](https://github.com/Blocra-HQ/blocra-cli/actions/workflows/ci.yml/badge.svg)](https://github.com/Blocra-HQ/blocra-cli/actions/workflows/ci.yml)
![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-green)

**Signed, verifiable traction reports for on-chain projects, from your terminal or CI.**

Grant programs, investors and partners ask for usage numbers, and screenshots or self-reported
figures are easy to doubt. `blocra` fetches a traction report for your contracts or assets that
[BlocRA](https://github.com/Blocra-HQ) computes from on-chain data and signs, and lets anyone verify
a copy in one command.

```bash
blocra init --api https://<blocra-api> --subject <your contract id>
blocra report
blocra verify reports/*.md
```

Chains: **Stellar** today (Soroban contracts and Stellar assets); the CLI and API take a `--chain`
option so more chains plug in without changes.

## Install

A single static binary, written in Rust.

```bash
cargo install --git https://github.com/Blocra-HQ/blocra-cli --locked
```

Or download a prebuilt binary for Linux, macOS or Windows from
[Releases](https://github.com/Blocra-HQ/blocra-cli/releases) (checksums in `SHA256SUMS`) and put it on
your `PATH`. In GitHub Actions, use the [Action](#github-action), which installs it for you.

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
blocra init --api https://<blocra-api> \
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
otherwise BlocRA's official keys built into the binary (`defaults.json`, which keeps past keys so
older reports still verify); otherwise the key the API serves, with a notice. A reviewer who received
a report needs no setup: `blocra verify report.md`.

### `blocra key`

Prints the API's signing key, methodology version and supported chains.

## Using the website

Reports exported from the BlocRA website (`/reports`) are the same files `blocra report` writes:
**Verifiable report (.md)** is this CLI's Markdown with the signed envelope embedded, and **Signed
data (.json)** is the envelope. Both verify with `blocra verify <file>`. The website's PDF is a
rendering for reading and is not verifiable on its own; ask for the `.md` or `.json`. The website's
`/verify-report` page runs the same checks in the browser (WebCrypto Ed25519).

The website renders reports in the browser with its own copy of the renderer. Both renderers must
produce the golden files in [`test/fixtures`](test/fixtures) byte for byte (`cargo test` here,
`npm run check:blocra` in the website), and both read the official keys from
[`defaults.json`](defaults.json).

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
- **Small and auditable.** One binary built on a handful of well-known crates (`ed25519-dalek`,
  `sha2`, `ureq` with rustls); it never touches private keys.
- **Roadmap:** `blocra publish` (record the report's sha256
  in BlocRA's on-chain registry), PDF output, README badge, and more chains.

## Development

```bash
cargo test                  # renderer parity, verification, end-to-end CLI runs
cargo run -- --help
cargo clippy --all-targets && cargo fmt --check
```

| Module | |
|--------|---|
| `src/main.rs` | Commands and argument parsing |
| `src/api.rs` | BlocRA API client |
| `src/verify.rs`, `src/strkey.rs` | Ed25519 signature checks, Stellar key decoding |
| `src/markdown.rs` | Markdown rendering and parsing |
| `src/js.rs` | JavaScript-compatible formatting, so the output matches the website's renderer |
| `src/config.rs`, `src/defaults.rs` | `blocra.json`, built-in official keys |

The report layout is shared with the website: change it in both renderers and regenerate the golden
files in [`test/fixtures`](test/fixtures) in the same change. Pushing a `v*` tag builds release
binaries for Linux, macOS and Windows.

## License

[MIT](LICENSE)
