# Traction Report: Contract CCISQH…IPRO

| Field | Value |
| --- | --- |
| Subject | `CCISQHV7UC3NZQ5WGADCNDYCG4FRWNKPISEGPLKADC3FFYEBG7QYIPRO` |
| Kind | Smart contract |
| Chain | stellar (mainnet) |
| Period | 2026-10-31 – 2026-11-06 (UTC, 7 days) |
| Generated | 2026-11-06T13:03:21.837995398+00:00 |
| Method | traction/1 |

> **Partial data:** BlocRA holds 0.1 hours of the 7-day period (100% of ledgers in that span), collected since 2026-11-06. Figures cover only the collected time, and there is no previous period to compare with.

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
| Time covered | 0.1 hours of 7 days |

- Source: every Stellar ledger read from Horizon by BlocRA's collector and stored with its transactions and operations.
- Days are UTC calendar days; the latest day may be partial.
- A call is one invoke_host_function operation targeting the contract. Its account is the first account (G...) or smart wallet (C...) that signed an authorization entry for it, so calls sent through relayers, fee sponsors or fee bumps count for the user; calls without authorization entries count for the operation's source. Failed calls are included in calls and counted in failed.
- Declared relayers (accounts the project names as its own relayers) are left out of account counts, concentration and retention; calls they sent without user authorization are reported separately.
- Automated (bot-like) accounts made at least 1000 calls on a single day.
- New accounts were first seen during the period in all data BlocRA has collected; with less history than the period, they are marked unreliable.
- Ecosystem rank compares active accounts with every contract (or asset) active in the same period; the failure-rate median uses peers with at least 10 active accounts.

## Verification

| Field | Value |
| --- | --- |
| Signed by | `GBQ6QIHVDHFSY4CDORDELIZXRUWF72FTCRNOXUFPH7YOO3PFFMXLAVUN` (BlocRA) |
| Signature | Ed25519 over the embedded report |
| Report sha256 | `82ce0e168172cd3d32fe490ae023e09e62a2eb6ed190861744ee57c952ce2b66` |
| Signed at | 2026-11-06T13:03:21.841628029+00:00 |

Check this file with `npx blocra verify <file>`. Any edit to the figures above breaks verification.

<!-- blocra-attestation:eyJhdHRlc3RhdGlvbiI6eyJhbGdvcml0aG0iOiJlZDI1NTE5IiwibWV0aG9kX3ZlcnNpb24iOiJ0cmFjdGlvbi8xIiwicGF5bG9hZF9zaGEyNTYiOiI4MmNlMGUxNjgxNzJjZDNkMzJmZTQ5MGFlMDIzZTA5ZTYyYTJlYjZlZDE5MDg2MTc0NGVlNTdjOTUyY2UyYjY2Iiwic2lnbmF0dXJlIjoiNzNiNmQyOWI0ZDFhZTQxNmU4OGUxODliNzFjZThiY2IyZjVmZWYyYzNjN2MyY2E2MzkwNTM1OTRhMjMxNGVlODE3NjYyMGRiYzBhOGFmNDQ4M2VlODk1MDllZjcyMTNjNjJlYzRmYjk2ODJiNzI0MzYyNGNkMWQ0NGQ3NTFiMGYiLCJzaWduZWRfYXQiOiIyMDI2LTExLTA2VDEzOjAzOjIxLjg0MTYyODAyOSswMDowMCIsInNpZ25lciI6IkdCUTZRSUhWREhGU1k0Q0RPUkRFTElaWFJVV0Y3MkZUQ1JOT1hVRlBIN1lPTzNQRkZNWExBVlVOIn0sInJlcG9ydF9qc29uIjoie1wiYXNzZXRfc25hcHNob3RcIjpudWxsLFwiY2hhaW5cIjpcInN0ZWxsYXJcIixcImNvdmVyYWdlXCI6e1wiY29sbGVjdGlvbl9zdGFydGVkXCI6XCIyMDI2LTExLTA2XCIsXCJjb21wbGV0ZW5lc3NfcGN0XCI6MTAwLjAsXCJkYXlzX2NvdmVyZWRcIjowLjAxLFwiZXhwZWN0ZWRfbGVkZ2Vyc1wiOjEwOTk2NC4wLFwiZnVsbF9wZXJpb2RcIjpmYWxzZSxcImhvdXJzX2NvdmVyZWRcIjowLjEsXCJsZWRnZXJfZnJvbVwiOjY1MzM2Njc1LFwibGVkZ2VyX3RvXCI6NjUzMzY3ODEsXCJsZWRnZXJzX2NvbGxlY3RlZFwiOjEwN30sXCJkYWlseVwiOlt7XCJhY3RpdmVfYWNjb3VudHNcIjoxNDAsXCJjYWxsc1wiOjQ4NCxcImRheVwiOlwiMjAyNi0xMS0wNlwiLFwiZmFpbGVkXCI6MjcsXCJ2b2x1bWVcIjowLjB9XSxcImRlY2xhcmVkX3JlbGF5ZXJzXCI6e1wiYWNjb3VudHNcIjpbXCJHRFlaN0xCM0ZGRklFS1JHNFFZU0ozNE9ERVFTWFRSVFRTRTVDSEVHT0lSVVBTS1dFQ0hVUTVMN1wiXSxcImFjdGl2ZVwiOjAsXCJjYWxsc193aXRob3V0X3VzZXJfYXV0aFwiOm51bGx9LFwiZWNvc3lzdGVtXCI6e1wibWVkaWFuX2ZhaWxlZF9wY3RcIjoxLjEsXCJtZXRyaWNcIjpcImFjdGl2ZV9hY2NvdW50c1wiLFwib2ZcIjo0NSxcInBlcmNlbnRpbGVcIjo5NS42LFwicmFua1wiOjN9LFwiZmluZGluZ3NcIjpbXCI1MS43JSBvZiBjYWxscyB3ZXJlIHN1Ym1pdHRlZCBieSBhIHJlbGF5ZXIgb3Igc3BvbnNvciBvbiB0aGUgdXNlcidzIGJlaGFsZjsgZWFjaCBpcyBjb3VudGVkIGZvciB0aGUgYWNjb3VudCB0aGF0IGF1dGhvcml6ZWQgaXQsIG5vdCB0aGUgcmVsYXllci5cIixcIjUuNiUgb2YgY2FsbHMgZmFpbGVkLCB3ZWxsIGFib3ZlIHRoZSAxLjElIG1lZGlhbiBvZiBjb21wYXJhYmxlIGNvbnRyYWN0cy5cIixcIlJhbmtlZCAjMyBvZiA0NSBjb250cmFjdHMgYnkgYWN0aXZlIGFjY291bnRzIGluIHRoaXMgcGVyaW9kICh0b3AgNyUpLlwiLFwiUGFydGlhbCBkYXRhOiBCbG9jUkEgaG9sZHMgMC4xIGhvdXJzIG9mIHRoZSA3LWRheSBwZXJpb2QgKDEwMC4wJSBvZiBsZWRnZXJzIGluIHRoYXQgc3BhbikuIEZpZ3VyZXMgY292ZXIgb25seSB0aGUgY29sbGVjdGVkIHRpbWUuXCJdLFwiZnVuY3Rpb25zXCI6W3tcImNhbGxzXCI6MjUwLFwiZmFpbGVkXCI6MjcsXCJmYWlsZWRfcGN0XCI6MTAuOCxcImZ1bmN0aW9uXCI6XCJiYXRjaF9wbGFudFwifSx7XCJjYWxsc1wiOjIzNCxcImZhaWxlZFwiOjAsXCJmYWlsZWRfcGN0XCI6MC4wLFwiZnVuY3Rpb25cIjpcImJhdGNoX3dvcmtcIn1dLFwiZ2VuZXJhdGVkX2F0XCI6XCIyMDI2LTExLTA2VDEzOjAzOjIxLjgzNzk5NTM5OCswMDowMFwiLFwia3Bpc1wiOntcImFjdGl2ZV9hY2NvdW50c1wiOntcImNoYW5nZV9wY3RcIjpudWxsLFwicHJldmlvdXNcIjpudWxsLFwidmFsdWVcIjoxNDAuMH0sXCJjYWxsc1wiOntcImNoYW5nZV9wY3RcIjpudWxsLFwicHJldmlvdXNcIjpudWxsLFwidmFsdWVcIjo0ODQuMH0sXCJjYWxsc19wZXJfYWNjb3VudFwiOjMuNSxcImZhaWxlZF9wY3RcIjp7XCJjaGFuZ2VfcGN0XCI6bnVsbCxcInByZXZpb3VzXCI6bnVsbCxcInZhbHVlXCI6NS42fSxcIm5ld19hY2NvdW50c1wiOntcInJlbGlhYmxlXCI6ZmFsc2UsXCJ2YWx1ZVwiOjE0MH0sXCJ2b2x1bWVcIjpudWxsfSxcIm1ldGhvZF92ZXJzaW9uXCI6XCJ0cmFjdGlvbi8xXCIsXCJtZXRob2RvbG9neVwiOltcIlNvdXJjZTogZXZlcnkgU3RlbGxhciBsZWRnZXIgcmVhZCBmcm9tIEhvcml6b24gYnkgQmxvY1JBJ3MgY29sbGVjdG9yIGFuZCBzdG9yZWQgd2l0aCBpdHMgdHJhbnNhY3Rpb25zIGFuZCBvcGVyYXRpb25zLlwiLFwiRGF5cyBhcmUgVVRDIGNhbGVuZGFyIGRheXM7IHRoZSBsYXRlc3QgZGF5IG1heSBiZSBwYXJ0aWFsLlwiLFwiQSBjYWxsIGlzIG9uZSBpbnZva2VfaG9zdF9mdW5jdGlvbiBvcGVyYXRpb24gdGFyZ2V0aW5nIHRoZSBjb250cmFjdC4gSXRzIGFjY291bnQgaXMgdGhlIGZpcnN0IGFjY291bnQgKEcuLi4pIG9yIHNtYXJ0IHdhbGxldCAoQy4uLikgdGhhdCBzaWduZWQgYW4gYXV0aG9yaXphdGlvbiBlbnRyeSBmb3IgaXQsIHNvIGNhbGxzIHNlbnQgdGhyb3VnaCByZWxheWVycywgZmVlIHNwb25zb3JzIG9yIGZlZSBidW1wcyBjb3VudCBmb3IgdGhlIHVzZXI7IGNhbGxzIHdpdGhvdXQgYXV0aG9yaXphdGlvbiBlbnRyaWVzIGNvdW50IGZvciB0aGUgb3BlcmF0aW9uJ3Mgc291cmNlLiBGYWlsZWQgY2FsbHMgYXJlIGluY2x1ZGVkIGluIGNhbGxzIGFuZCBjb3VudGVkIGluIGZhaWxlZC5cIixcIkRlY2xhcmVkIHJlbGF5ZXJzIChhY2NvdW50cyB0aGUgcHJvamVjdCBuYW1lcyBhcyBpdHMgb3duIHJlbGF5ZXJzKSBhcmUgbGVmdCBvdXQgb2YgYWNjb3VudCBjb3VudHMsIGNvbmNlbnRyYXRpb24gYW5kIHJldGVudGlvbjsgY2FsbHMgdGhleSBzZW50IHdpdGhvdXQgdXNlciBhdXRob3JpemF0aW9uIGFyZSByZXBvcnRlZCBzZXBhcmF0ZWx5LlwiLFwiQXV0b21hdGVkIChib3QtbGlrZSkgYWNjb3VudHMgbWFkZSBhdCBsZWFzdCAxMDAwIGNhbGxzIG9uIGEgc2luZ2xlIGRheS5cIixcIk5ldyBhY2NvdW50cyB3ZXJlIGZpcnN0IHNlZW4gZHVyaW5nIHRoZSBwZXJpb2QgaW4gYWxsIGRhdGEgQmxvY1JBIGhhcyBjb2xsZWN0ZWQ7IHdpdGggbGVzcyBoaXN0b3J5IHRoYW4gdGhlIHBlcmlvZCwgdGhleSBhcmUgbWFya2VkIHVucmVsaWFibGUuXCIsXCJFY29zeXN0ZW0gcmFuayBjb21wYXJlcyBhY3RpdmUgYWNjb3VudHMgd2l0aCBldmVyeSBjb250cmFjdCAob3IgYXNzZXQpIGFjdGl2ZSBpbiB0aGUgc2FtZSBwZXJpb2Q7IHRoZSBmYWlsdXJlLXJhdGUgbWVkaWFuIHVzZXMgcGVlcnMgd2l0aCBhdCBsZWFzdCAxMCBhY3RpdmUgYWNjb3VudHMuXCJdLFwibmV0d29ya1wiOlwibWFpbm5ldFwiLFwicGVyaW9kXCI6e1wiZGF5c1wiOjcsXCJmcm9tXCI6XCIyMDI2LTEwLTMxXCIsXCJoYXNfcHJldmlvdXNcIjpmYWxzZSxcInByZXZpb3VzX2Zyb21cIjpcIjIwMjYtMTAtMjRcIixcInByZXZpb3VzX3RvXCI6XCIyMDI2LTEwLTMwXCIsXCJ0b1wiOlwiMjAyNi0xMS0wNlwifSxcInF1YWxpdHlcIjp7XCJib3RfbGlrZV9hY2NvdW50c1wiOjAsXCJib3RfbGlrZV9zaGFyZV9wY3RcIjowLjAsXCJib3RfdGhyZXNob2xkX2NhbGxzX3Blcl9kYXlcIjoxMDAwLFwicmVsYXllZF9jYWxsc1wiOjI1MCxcInJlbGF5ZWRfc2hhcmVfcGN0XCI6NTEuNyxcInNlbGZfcGF5bWVudF9zaGFyZV9wY3RcIjpudWxsLFwic2VsZl9wYXltZW50c1wiOm51bGwsXCJzbWFydF93YWxsZXRfYWNjb3VudHNcIjowLFwidG9wMTBfc2hhcmVfcGN0XCI6MTAuNyxcInRvcF9hY2NvdW50c1wiOlt7XCJhY2NvdW50XCI6XCJHREtSTE0yRFpWR1FTVVZYQ0RKTFBKRVNOTk41VEpPWkpLTFhXSUVYVVJOTVVCSDNRNjZEUzZCQVwiLFwiYm90X2xpa2VcIjpmYWxzZSxcImNhbGxzXCI6NixcInNoYXJlX3BjdFwiOjEuMjR9LHtcImFjY291bnRcIjpcIkdCRTJJVTJCVFhSUVBZR1FDU0FXS0ZGR1BJVk1CWUJLUTJXUTdLVDdaTERKWkpHWTRWVk02UEVaXCIsXCJib3RfbGlrZVwiOmZhbHNlLFwiY2FsbHNcIjo2LFwic2hhcmVfcGN0XCI6MS4yNH0se1wiYWNjb3VudFwiOlwiR0RXWUxQNUlEWDJQR1VPNlhOTzJBWFhFUjVPRjJaNjI0TEQ2SDZSNVI0Rk1HWk82V1BFQldZVVRcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjUsXCJzaGFyZV9wY3RcIjoxLjAzfSx7XCJhY2NvdW50XCI6XCJHREVYMlRVWlZSSUVHVTdaSVAyUk9aTlZISVhGUkxRTEQyUU5OVlRKWkJDQ1dKWk43Uk9JMjNSUVwiLFwiYm90X2xpa2VcIjpmYWxzZSxcImNhbGxzXCI6NSxcInNoYXJlX3BjdFwiOjEuMDN9LHtcImFjY291bnRcIjpcIkdEQ1dXSUdVUkpGUk5GUVhaNlFRT1ZJMllBSlZNSEFPSkZaRkJLVkRYNVJTNUlHM0FBM1I1WU1HXCIsXCJib3RfbGlrZVwiOmZhbHNlLFwiY2FsbHNcIjo1LFwic2hhcmVfcGN0XCI6MS4wM31dfSxcInJldGVudGlvblwiOntcInByZXZpb3VzX2FjdGl2ZVwiOm51bGwsXCJyZXR1cm5pbmdfYWNjb3VudHNcIjowLFwicmV0dXJuaW5nX3BjdFwiOm51bGx9LFwic3ViamVjdFwiOntcImlkXCI6XCJDQ0lTUUhWN1VDM05aUTVXR0FEQ05EWUNHNEZSV05LUElTRUdQTEtBREMzRkZZRUJHN1FZSVBST1wiLFwia2luZFwiOlwiY29udHJhY3RcIixcImxhYmVsXCI6XCJDb250cmFjdCBDQ0lTUUjigKZJUFJPXCJ9LFwidHlwZVwiOlwidHJhY3Rpb25cIixcInZlcnNpb25cIjoxfSJ9 -->
