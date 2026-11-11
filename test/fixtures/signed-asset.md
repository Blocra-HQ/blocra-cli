# Traction Report: USDC (issuer GA5Z…KZVN)

| Field | Value |
| --- | --- |
| Subject | `USDC:GA5ZSEJYB37JRC5AVCIA5MOP4RHTM335X2KGX3IHOJAPP5RE34K4KZVN` |
| Kind | Asset |
| Chain | stellar (mainnet) |
| Period | 2026-10-31 – 2026-11-06 (UTC, 7 days) |
| Generated | 2026-11-06T12:43:14.552727992+00:00 |
| Method | traction/1 |

> **Partial data:** BlocRA holds 3.9 hours of the 7-day period (54.7% of ledgers in that span), collected since 2026-11-06. Figures cover only the collected time, and there is no previous period to compare with.

## Scorecard

| Metric | Value | Change vs previous period |
| --- | --- | --- |
| Active accounts | 1,803 | — |
| New accounts | 1,803 (insufficient history) | — |
| Payments | 16,435 | — |
| Failure rate | 62.4% | — |
| Volume (asset units) | 31,135,693.23 | — |
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
| Returning accounts | — (needs a previous period) |
| Top-10 accounts' share | 63.2% |
| Automated accounts (≥ 1,000 payments on one day) | 4 |
| Automated share of activity | 44.9% |
| Self-payments (excluded from volume) | 67.7% |

| Top account | Payments | Share | Automated |
| --- | --- | --- | --- |
| [GC2IXX…2X7M](https://stellar.expert/explorer/public/account/GC2IXXCFV5LN7YF732GZ353J22F3G2BZWUM4U4BCSF2WYGTW4QIX2X7M) | 2,322 | 14.1% | yes |
| [GAUA7X…PNJU](https://stellar.expert/explorer/public/account/GAUA7XL5K54CC2DDGP77FJ2YBHRJLT36CPZDXWPM6MP7MANOGG77PNJU) | 2,047 | 12.5% | yes |
| [GBXQUJ…HMTP](https://stellar.expert/explorer/public/account/GBXQUJBEDX5TYLJ6D5BGJZFLYF5GZVGXLWA2ZORS5OIA7H6B5O3MHMTP) | 1,732 | 10.5% | yes |
| [GAOYFQ…PNLT](https://stellar.expert/explorer/public/account/GAOYFQYMNHR3A4Q44I6BYQSD6IJ7RSKWPJQLGJQSJ7TF2HO7A4SHPNLT) | 1,283 | 7.8% | yes |
| [GBEJMH…N4CW](https://stellar.expert/explorer/public/account/GBEJMHIMASJBIGGV5UKACNIZTQN3ZCILKZBH6W5PFI5TPCFPT5ASN4CW) | 680 | 4.1% | no |

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
| Time covered | 3.9 hours of 7 days |

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

<!-- blocra-attestation:eyJhdHRlc3RhdGlvbiI6eyJhbGdvcml0aG0iOiJlZDI1NTE5IiwibWV0aG9kX3ZlcnNpb24iOiJ0cmFjdGlvbi8xIiwicGF5bG9hZF9zaGEyNTYiOiJmM2M1OWUwY2NmMDM0YzY4OTIyZjIwY2E0NWVhMzEyNWZmNjJkNzc5Y2RmNWNjZTM3NWY1YTM2NTczMWQwNDNkIiwic2lnbmF0dXJlIjoiOGNmNzI2MmFmODA4YWJkOTY2NDFiY2FiNzViY2U3MDAxNTVmNTczMDczNDJmZThiZTIwNDc2Y2E3YTE5NWFmYTM1NzQzOGU4OWJmNGE0YjY3MjJlNjU1NDI3YmQ5ZTgzN2RlZGE3MjQ0MDJhNDlhNzY3ZGRhMjIyNjNiOTNiMDUiLCJzaWduZWRfYXQiOiIyMDI2LTExLTA2VDEyOjQzOjE0LjU2MTU1MzEyMiswMDowMCIsInNpZ25lciI6IkdCUTZRSUhWREhGU1k0Q0RPUkRFTElaWFJVV0Y3MkZUQ1JOT1hVRlBIN1lPTzNQRkZNWExBVlVOIn0sInJlcG9ydF9qc29uIjoie1wiYXNzZXRfc25hcHNob3RcIjp7XCJhc19vZlwiOlwiMjAyNi0xMS0wNlQxMjo0MzoxNC41NTI2MjIyMzArMDA6MDBcIixcImNvbnRyYWN0X2hvbGRlcnNcIjo0ODI5OSxcImhvbGRlcnNcIjoyNTI4NTk2LFwibGlxdWlkaXR5X3Bvb2xzXCI6OTU2LFwic2FjX2NvbnRyYWN0X2lkXCI6XCJDQ1c2N1RTWlYzU1NTMkhYTUJRNUpGR0NLSk5YS1pNN1VRVVdVWlBVVEhYU1RaTEVPN1NKTUk3NVwiLFwic3VwcGx5X2luX2FjY291bnRzXCI6MjA5OTc2MTUwLjEyODc1OTY4LFwic3VwcGx5X2luX2NvbnRyYWN0c1wiOjExNDQxNDQyOS4wMzQ5OTA1LFwic3VwcGx5X2luX2xpcXVpZGl0eV9wb29sc1wiOjI0ODg5NDUuNDA3MTc0NCxcInRvbWxcIjpcImh0dHBzOi8vY2lyY2xlLmNvbS8ud2VsbC1rbm93bi9zdGVsbGFyLnRvbWxcIn0sXCJjaGFpblwiOlwic3RlbGxhclwiLFwiY292ZXJhZ2VcIjp7XCJjb2xsZWN0aW9uX3N0YXJ0ZWRcIjpcIjIwMjYtMTEtMDZcIixcImNvbXBsZXRlbmVzc19wY3RcIjo1NC43LFwiZGF5c19jb3ZlcmVkXCI6MC4xNixcImV4cGVjdGVkX2xlZGdlcnNcIjoxMDk5NjQuMCxcImZ1bGxfcGVyaW9kXCI6ZmFsc2UsXCJob3Vyc19jb3ZlcmVkXCI6My45LFwibGVkZ2VyX2Zyb21cIjo2NTMzMzY4NyxcImxlZGdlcl90b1wiOjY1MzM2NTAxLFwibGVkZ2Vyc19jb2xsZWN0ZWRcIjoxNTQxfSxcImRhaWx5XCI6W3tcImFjdGl2ZV9hY2NvdW50c1wiOjE4MDMsXCJjYWxsc1wiOjE2NDM1LFwiZGF5XCI6XCIyMDI2LTExLTA2XCIsXCJmYWlsZWRcIjoxMDI1MSxcInZvbHVtZVwiOjMxMTM1NjkzLjIzNDMyN31dLFwiZWNvc3lzdGVtXCI6e1wibWVkaWFuX2ZhaWxlZF9wY3RcIjoxLjcsXCJtZXRyaWNcIjpcImFjdGl2ZV9hY2NvdW50c1wiLFwib2ZcIjo4NzEsXCJwZXJjZW50aWxlXCI6OTkuOSxcInJhbmtcIjoyfSxcImZpbmRpbmdzXCI6W1wiQWN0aXZpdHkgaXMgY29uY2VudHJhdGVkOiB0aGUgdG9wIDEwIG9mIDE4MDMgYWNjb3VudHMgbWFkZSA2My4yJSBvZiBwYXltZW50cy5cIixcIjQgYWNjb3VudHMgbWFkZSDiiaUxMDAwIHBheW1lbnRzIG9uIGEgc2luZ2xlIGRheSAoYXV0b21hdGVkKTsgdG9nZXRoZXIgNDQuOSUgb2YgYWxsIHBheW1lbnRzLlwiLFwiNjcuNyUgb2YgcGF5bWVudHMgd2VyZSBzZW50IHRvIHRoZSBzZW5kZXIncyBvd24gYWNjb3VudCAoYXJiaXRyYWdlIGxvb3BzKTsgdGhleSBhcmUgZXhjbHVkZWQgZnJvbSB2b2x1bWUuXCIsXCI2Mi40JSBvZiBwYXltZW50cyBmYWlsZWQsIHdlbGwgYWJvdmUgdGhlIDEuNyUgbWVkaWFuIG9mIGNvbXBhcmFibGUgYXNzZXRzLlwiLFwiUmFua2VkICMyIG9mIDg3MSBhc3NldHMgYnkgYWN0aXZlIGFjY291bnRzIGluIHRoaXMgcGVyaW9kICh0b3AgMSUpLlwiLFwiUGFydGlhbCBkYXRhOiBCbG9jUkEgaG9sZHMgMy45IGhvdXJzIG9mIHRoZSA3LWRheSBwZXJpb2QgKDU0LjclIG9mIGxlZGdlcnMgaW4gdGhhdCBzcGFuKS4gRmlndXJlcyBjb3ZlciBvbmx5IHRoZSBjb2xsZWN0ZWQgdGltZS5cIl0sXCJmdW5jdGlvbnNcIjpbXSxcImdlbmVyYXRlZF9hdFwiOlwiMjAyNi0xMS0wNlQxMjo0MzoxNC41NTI3Mjc5OTIrMDA6MDBcIixcImtwaXNcIjp7XCJhY3RpdmVfYWNjb3VudHNcIjp7XCJjaGFuZ2VfcGN0XCI6bnVsbCxcInByZXZpb3VzXCI6bnVsbCxcInZhbHVlXCI6MTgwMy4wfSxcImNhbGxzXCI6e1wiY2hhbmdlX3BjdFwiOm51bGwsXCJwcmV2aW91c1wiOm51bGwsXCJ2YWx1ZVwiOjE2NDM1LjB9LFwiY2FsbHNfcGVyX2FjY291bnRcIjo5LjEsXCJmYWlsZWRfcGN0XCI6e1wiY2hhbmdlX3BjdFwiOm51bGwsXCJwcmV2aW91c1wiOm51bGwsXCJ2YWx1ZVwiOjYyLjR9LFwibmV3X2FjY291bnRzXCI6e1wicmVsaWFibGVcIjpmYWxzZSxcInZhbHVlXCI6MTgwM30sXCJ2b2x1bWVcIjp7XCJjaGFuZ2VfcGN0XCI6bnVsbCxcInByZXZpb3VzXCI6bnVsbCxcInZhbHVlXCI6MzExMzU2OTMuMjM0MzI3fX0sXCJtZXRob2RfdmVyc2lvblwiOlwidHJhY3Rpb24vMVwiLFwibWV0aG9kb2xvZ3lcIjpbXCJTb3VyY2U6IGV2ZXJ5IFN0ZWxsYXIgbGVkZ2VyIHJlYWQgZnJvbSBIb3Jpem9uIGJ5IEJsb2NSQSdzIGNvbGxlY3RvciBhbmQgc3RvcmVkIHdpdGggaXRzIHRyYW5zYWN0aW9ucyBhbmQgb3BlcmF0aW9ucy5cIixcIkRheXMgYXJlIFVUQyBjYWxlbmRhciBkYXlzOyB0aGUgbGF0ZXN0IGRheSBtYXkgYmUgcGFydGlhbC5cIixcIkFjdGl2aXR5IGlzIHBheW1lbnRzIGFuZCBwYXRoIHBheW1lbnRzIG9mIHRoZSBhc3NldDsgdGhlIGFjY291bnQgaXMgdGhlIHNlbmRlci4gVm9sdW1lIHN1bXMgYW1vdW50cyBvZiBzdWNjZXNzZnVsIHBheW1lbnRzIHRvIG90aGVyIGFjY291bnRzLCBpbiB0aGUgYXNzZXQncyB1bml0czsgcGF5bWVudHMgdG8gb25lc2VsZiAoYXJiaXRyYWdlIGxvb3BzKSBhcmUgY291bnRlZCBhcyBzZWxmLXBheW1lbnRzIGFuZCBsZWZ0IG91dCBvZiB2b2x1bWUuXCIsXCJBdXRvbWF0ZWQgKGJvdC1saWtlKSBhY2NvdW50cyBtYWRlIGF0IGxlYXN0IDEwMDAgcGF5bWVudHMgb24gYSBzaW5nbGUgZGF5LlwiLFwiTmV3IGFjY291bnRzIHdlcmUgZmlyc3Qgc2VlbiBkdXJpbmcgdGhlIHBlcmlvZCBpbiBhbGwgZGF0YSBCbG9jUkEgaGFzIGNvbGxlY3RlZDsgd2l0aCBsZXNzIGhpc3RvcnkgdGhhbiB0aGUgcGVyaW9kLCB0aGV5IGFyZSBtYXJrZWQgdW5yZWxpYWJsZS5cIixcIkVjb3N5c3RlbSByYW5rIGNvbXBhcmVzIGFjdGl2ZSBhY2NvdW50cyB3aXRoIGV2ZXJ5IGNvbnRyYWN0IChvciBhc3NldCkgYWN0aXZlIGluIHRoZSBzYW1lIHBlcmlvZDsgdGhlIGZhaWx1cmUtcmF0ZSBtZWRpYW4gdXNlcyBwZWVycyB3aXRoIGF0IGxlYXN0IDEwIGFjdGl2ZSBhY2NvdW50cy5cIl0sXCJuZXR3b3JrXCI6XCJtYWlubmV0XCIsXCJwZXJpb2RcIjp7XCJkYXlzXCI6NyxcImZyb21cIjpcIjIwMjYtMTAtMzFcIixcImhhc19wcmV2aW91c1wiOmZhbHNlLFwicHJldmlvdXNfZnJvbVwiOlwiMjAyNi0xMC0yNFwiLFwicHJldmlvdXNfdG9cIjpcIjIwMjYtMTAtMzBcIixcInRvXCI6XCIyMDI2LTExLTA2XCJ9LFwicXVhbGl0eVwiOntcImJvdF9saWtlX2FjY291bnRzXCI6NCxcImJvdF9saWtlX3NoYXJlX3BjdFwiOjQ0LjksXCJib3RfdGhyZXNob2xkX2NhbGxzX3Blcl9kYXlcIjoxMDAwLFwic2VsZl9wYXltZW50X3NoYXJlX3BjdFwiOjY3LjcsXCJzZWxmX3BheW1lbnRzXCI6MTExMjcsXCJ0b3AxMF9zaGFyZV9wY3RcIjo2My4yLFwidG9wX2FjY291bnRzXCI6W3tcImFjY291bnRcIjpcIkdDMklYWENGVjVMTjdZRjczMkdaMzUzSjIyRjNHMkJaV1VNNFU0QkNTRjJXWUdUVzRRSVgyWDdNXCIsXCJib3RfbGlrZVwiOnRydWUsXCJjYWxsc1wiOjIzMjIsXCJzaGFyZV9wY3RcIjoxNC4xM30se1wiYWNjb3VudFwiOlwiR0FVQTdYTDVLNTRDQzJEREdQNzdGSjJZQkhSSkxUMzZDUFpEWFdQTTZNUDdNQU5PR0c3N1BOSlVcIixcImJvdF9saWtlXCI6dHJ1ZSxcImNhbGxzXCI6MjA0NyxcInNoYXJlX3BjdFwiOjEyLjQ2fSx7XCJhY2NvdW50XCI6XCJHQlhRVUpCRURYNVRZTEo2RDVCR0paRkxZRjVHWlZHWExXQTJaT1JTNU9JQTdINkI1TzNNSE1UUFwiLFwiYm90X2xpa2VcIjp0cnVlLFwiY2FsbHNcIjoxNzMyLFwic2hhcmVfcGN0XCI6MTAuNTR9LHtcImFjY291bnRcIjpcIkdBT1lGUVlNTkhSM0E0UTQ0STZCWVFTRDZJSjdSU0tXUEpRTEdKUVNKN1RGMkhPN0E0U0hQTkxUXCIsXCJib3RfbGlrZVwiOnRydWUsXCJjYWxsc1wiOjEyODMsXCJzaGFyZV9wY3RcIjo3LjgxfSx7XCJhY2NvdW50XCI6XCJHQkVKTUhJTUFTSkJJR0dWNVVLQUNOSVpUUU4zWkNJTEtaQkg2VzVQRkk1VFBDRlBUNUFTTjRDV1wiLFwiYm90X2xpa2VcIjpmYWxzZSxcImNhbGxzXCI6NjgwLFwic2hhcmVfcGN0XCI6NC4xNH1dfSxcInJldGVudGlvblwiOntcInByZXZpb3VzX2FjdGl2ZVwiOm51bGwsXCJyZXR1cm5pbmdfYWNjb3VudHNcIjowLFwicmV0dXJuaW5nX3BjdFwiOm51bGx9LFwic3ViamVjdFwiOntcImlkXCI6XCJVU0RDOkdBNVpTRUpZQjM3SlJDNUFWQ0lBNU1PUDRSSFRNMzM1WDJLR1gzSUhPSkFQUDVSRTM0SzRLWlZOXCIsXCJraW5kXCI6XCJhc3NldFwiLFwibGFiZWxcIjpcIlVTREMgKGlzc3VlciBHQTVa4oCmS1pWTilcIn0sXCJ0eXBlXCI6XCJ0cmFjdGlvblwiLFwidmVyc2lvblwiOjF9In0= -->
