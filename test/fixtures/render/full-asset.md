# Traction Report: USDC (issuer GA5Z…KZVN)

| Field | Value |
| --- | --- |
| Subject | `USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN` |
| Kind | Asset |
| Chain | stellar (mainnet) |
| Period | 2026-10-31 – 2026-11-06 (UTC, 30 days) |
| Generated | 2026-11-06T12:43:14.552727992+00:00 |
| Method | traction/1 |

## Scorecard

| Metric | Value | Change vs previous period |
| --- | --- | --- |
| Active accounts | 1,124 | +12.3% |
| New accounts | 3 (insufficient history) | — |
| Payments | 16,435 | -0.1% |
| Failure rate | 62.4% | 0% |
| Volume (asset units) | 31,135,693.23 | +250% |
| Payments per account | 9.1 | — |

## Key findings

- Activity is concentrated: the top 10 of 1803 accounts made 63.2% of payments.
- 4 accounts made ≥1000 payments on a single day (automated); together 44.9% of all payments.
- 67.7% of payments were sent to the sender's own account (arbitrage loops); they are excluded from volume.
- 62.4% of payments failed, well above the 1.7% median of comparable assets.
- Ranked #2 of 871 assets by active accounts in this period (top 1%).
- Partial data: BlocRA holds 3.9 hours of the 7-day period (54.7% of ledgers in that span). Figures cover only the collected time.

## Activity

| Day (UTC) | Active accounts | Payments | Failed | Volume |
| --- | --- | --- | --- | --- |
| 2026-11-06 | 1,803 | 16,435 | 10,251 | 31,135,693.23 |

## User quality

| Measure | Value |
| --- | --- |
| Returning accounts | 33.3% (333 of 1,000) |
| Top-10 accounts' share | 63.2% |
| Automated accounts (≥ 1,000 payments on one day) | 4 |
| Automated share of activity | 44.9% |
| Self-payments (excluded from volume) | <0.1% |

| Top account | Payments | Share | Automated |
| --- | --- | --- | --- |
| [GDKRLM…S6BA](https://stellar.expert/explorer/public/account/GDKRLM2DZVGQSUVXCDJLPJESNNN5TJOZJKLXWIEXURNMUBH3Q66DS6BA) | 1,000,000 | 100% | yes |
| [short](https://stellar.expert/explorer/public/account/short) | 0 | <0.1% | no |

## Asset snapshot

| Measure | Value |
| --- | --- |
| Holders | 2,528,596 |
| Supply in accounts | 209,976,150.13 |
| Supply in contracts | 114,414,429.03 |
| Supply in liquidity pools | 2,488,945.41 |

## Ecosystem position

| Measure | Value |
| --- | --- |
| Rank by active accounts | #2 of 871 |
| Median failure rate of peers | 1.7% |

## Coverage and methodology

| Measure | Value |
| --- | --- |
| Ledger range | 65,333,687 – 65,336,501 |
| Ledgers collected | 1,541 |
| Completeness | 54.7% |
| Time covered | 30 days of 30 days |

- Source: every Stellar ledger read from Horizon by BlocRA's collector and stored with its transactions and operations.
- Days are UTC calendar days; the latest day may be partial.
- Activity is payments and path payments of the asset; the account is the sender. Volume sums amounts of successful payments to other accounts, in the asset's units; payments to oneself (arbitrage loops) are counted as self-payments and left out of volume.
- Automated (bot-like) accounts made at least 1000 payments on a single day.
- New accounts were first seen during the period in all data BlocRA has collected; with less history than the period, they are marked unreliable.
- Ecosystem rank compares active accounts with every contract (or asset) active in the same period; the failure-rate median uses peers with at least 10 active accounts.

## Verification

| Field | Value |
| --- | --- |
| Signed by | `GBQ6QIHVDHFSY4CDORDELIZXRUWF72FTCRNOXUFPH7YOO3PFFMXLAVUN` (BlocRA) |
| Signature | Ed25519 over the embedded report |
| Report sha256 | `f3c59e0ccf034c68922f20ca45ea3125ff62d779cdf5cce375f5a365731d043d` |
| Signed at | 2026-11-06T12:43:14.561553122+00:00 |

Check this file with `npx blocra verify <file>`. Any edit to the figures above breaks verification.
