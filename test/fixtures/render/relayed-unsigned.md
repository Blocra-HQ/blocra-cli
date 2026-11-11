# Traction Report: Contract CCISQH…IPRO

| Field | Value |
| --- | --- |
| Subject | `CCISQHV7UC3NZQ5WGADCNDYCG4FRWNKPISEGPLKADC3FFYEBG7QYIPRO` |
| Kind | Smart contract |
| Chain | stellar (mainnet) |
| Period | 2026-10-31 – 2026-11-06 (UTC, 7 days) |
| Generated | 2026-11-06T13:03:21.837995398+00:00 |
| Method | traction/1 |

## Scorecard

| Metric | Value | Change vs previous period |
| --- | --- | --- |
| Active accounts | 140 | — |
| New accounts | 140 (insufficient history) | — |
| Contract calls | 484 | — |
| Failure rate | 5.6% | — |
| Contract calls per account | 3.5 | — |

## Key findings

- 51.7% of calls were submitted by a relayer or sponsor on the user's behalf; each is counted for the account that authorized it, not the relayer.
- 5.6% of calls failed, well above the 1.1% median of comparable contracts.
- Ranked #3 of 45 contracts by active accounts in this period (top 7%).
- Partial data: BlocRA holds 0.1 hours of the 7-day period (100.0% of ledgers in that span). Figures cover only the collected time.

## Activity

| Day (UTC) | Active accounts | Contract calls | Failed |
| --- | --- | --- | --- |
| 2026-11-06 | 140 | 484 | 27 |

## User quality

| Measure | Value |
| --- | --- |
| Returning accounts | — (needs a previous period) |
| Top-10 accounts' share | 10.7% |
| Automated accounts (≥ 1,000 calls on one day) | 0 |
| Automated share of activity | 0% |
| Calls submitted by a relayer or sponsor (counted for the signing user) | 51.7% (250) |
| Smart-wallet (contract) accounts | 0 |
| Declared relayers (excluded from account counts) | GDYZ7L…Q5L7 |
| Calls from declared relayers without user authorization | 0 |

| Top account | Contract calls | Share | Automated |
| --- | --- | --- | --- |
| [GDKRLM…S6BA](https://stellar.expert/explorer/public/account/GDKRLM2DZVGQSUVXCDJLPJESNNN5TJOZJKLXWIEXURNMUBH3Q66DS6BA) | 6 | 1.2% | no |
| [GBE2IU…6PEZ](https://stellar.expert/explorer/public/account/GBE2IU2BTXRQPYGQCSAWKFFGPIVMBYBKQ2WQ7KT7ZLDJZJGY4VVM6PEZ) | 6 | 1.2% | no |
| [GDWYLP…WYUT](https://stellar.expert/explorer/public/account/GDWYLP5IDX2PGUO6XNO2AXXER5OF2Z624LD6H6R5R4FMGZO6WPEBWYUT) | 5 | 1% | no |
| [GDEX2T…23RQ](https://stellar.expert/explorer/public/account/GDEX2TUZVRIEGU7ZIP2ROZNVHIXFRLQLD2QNNVTJZBCCWJZN7ROI23RQ) | 5 | 1% | no |
| [GDCWWI…5YMG](https://stellar.expert/explorer/public/account/GDCWWIGURJFRNFQXZ6QQOVI2YAJVMHAOJFZFBKVDX5RS5IG3AA3R5YMG) | 5 | 1% | no |

## Contract functions

| Function | Calls | Failure rate |
| --- | --- | --- |
| `batch_plant` | 250 | 10.8% |
| `batch_work` | 234 | 0% |

## Ecosystem position

| Measure | Value |
| --- | --- |
| Rank by active accounts | #3 of 45 |
| Median failure rate of peers | 1.1% |

## Coverage and methodology

| Measure | Value |
| --- | --- |
| Ledger range | 65,336,675 – 65,336,781 |
| Ledgers collected | 107 |
| Completeness | 100% |
| Time covered | 1 days of 7 days |

- Source: every Stellar ledger read from Horizon by BlocRA's collector and stored with its transactions and operations.
- Days are UTC calendar days; the latest day may be partial.
- A call is one invoke_host_function operation targeting the contract. Its account is the first account (G...) or smart wallet (C...) that signed an authorization entry for it, so calls sent through relayers, fee sponsors or fee bumps count for the user; calls without authorization entries count for the operation's source. Failed calls are included in calls and counted in failed.
- Declared relayers (accounts the project names as its own relayers) are left out of account counts, concentration and retention; calls they sent without user authorization are reported separately.
- Automated (bot-like) accounts made at least 1000 calls on a single day.
- New accounts were first seen during the period in all data BlocRA has collected; with less history than the period, they are marked unreliable.
- Ecosystem rank compares active accounts with every contract (or asset) active in the same period; the failure-rate median uses peers with at least 10 active accounts.

## Verification

Unsigned report.
