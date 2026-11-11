# Traction Report: Contract CDL74R…IGWA

| Field | Value |
| --- | --- |
| Subject | `CDL74RF5BLYR2YBLCCI7F5FB6TPSCLKEJUBSD2RSVWZ4YHF3VMFAIGWA` |
| Kind | Smart contract |
| Chain | stellar (mainnet) |
| Period | 2026-10-31 – 2026-11-06 (UTC, 7 days) |
| Generated | 2026-11-06T12:43:12.812772118+00:00 |
| Method | traction/1 |

> **Partial data:** BlocRA holds 3.9 hours of the 7-day period (54.7% of ledgers in that span), collected since 2026-11-06. Figures cover only the collected time, and there is no previous period to compare with.

## Scorecard

| Metric | Value | Change vs previous period |
| --- | --- | --- |
| Active accounts | 4,638 | — |
| New accounts | 4,638 (insufficient history) | — |
| Contract calls | 144,758 | — |
| Failure rate | 1.5% | — |
| Contract calls per account | 31.2 | — |

## Key findings

- 1.5% of calls failed, in line with the 0.8% median of comparable contracts.
- Ranked #1 of 146 contracts by active accounts in this period (top 1%).
- Partial data: BlocRA holds 3.9 hours of the 7-day period (54.7% of ledgers in that span). Figures cover only the collected time.

## Activity

| Day (UTC) | Active accounts | Contract calls | Failed |
| --- | --- | --- | --- |
| 2026-11-06 | 4,638 | 144,758 | 2,218 |

## User quality

| Measure | Value |
| --- | --- |
| Returning accounts | — (needs a previous period) |
| Top-10 accounts' share | 0.3% |
| Automated accounts (≥ 1,000 calls on one day) | 0 |
| Automated share of activity | 0% |

| Top account | Contract calls | Share | Automated |
| --- | --- | --- | --- |
| [GDWJYQ…75ET](https://stellar.expert/explorer/public/account/GDWJYQ4GJYZGQEF5EYWD34MWLTEVMBHPXU37BXCEQOO4DOF6LIWW75ET) | 49 | <0.1% | no |
| [GDVVRO…5AGQ](https://stellar.expert/explorer/public/account/GDVVROJPF45WZ7VPHX25JFCHT5I7JL5HXERDEVOR6OJ56YFDPOAU5AGQ) | 49 | <0.1% | no |
| [GDOYGB…6CPV](https://stellar.expert/explorer/public/account/GDOYGB464EHQXUPBZ6P73EA4MKVDXDKDICXBO6RSK3YVPT5Z7ZBT6CPV) | 49 | <0.1% | no |
| [GDAJPX…F3JI](https://stellar.expert/explorer/public/account/GDAJPX7NZR7YMMTH4ZWD2SKMDWOGMSFR7J3IVOS235CKC2RLNHURF3JI) | 49 | <0.1% | no |
| [GCBMJ4…SR3H](https://stellar.expert/explorer/public/account/GCBMJ4JTFA26LGRKRM5CFDRXN645RJEKEH5DLGF5XOR6ZCRBCCNTSR3H) | 49 | <0.1% | no |

## Contract functions

| Function | Calls | Failure rate |
| --- | --- | --- |
| `plant` | 81,796 | 0.1% |
| `work` | 62,764 | 3.4% |
| `harvest` | 198 | 3% |

## Ecosystem position

| Measure | Value |
| --- | --- |
| Rank by active accounts | #1 of 146 |
| Median failure rate of peers | 0.8% |

## Coverage and methodology

| Measure | Value |
| --- | --- |
| Ledger range | 65,333,687 – 65,336,501 |
| Ledgers collected | 1,541 |
| Completeness | 54.7% |
| Time covered | 3.9 hours of 7 days |

- Source: every Stellar ledger read from Horizon by BlocRA's collector and stored with its transactions and operations.
- Days are UTC calendar days; the latest day may be partial.
- A call is one invoke_host_function operation targeting the contract; the account is the operation's source. Failed calls (from failed transactions) are included in calls and counted in failed.
- Automated (bot-like) accounts made at least 1000 calls on a single day.
- New accounts were first seen during the period in all data BlocRA has collected; with less history than the period, they are marked unreliable.
- Ecosystem rank compares active accounts with every contract (or asset) active in the same period; the failure-rate median uses peers with at least 10 active accounts.

## Verification

| Field | Value |
| --- | --- |
| Signed by | `GBQ6QIHVDHFSY4CDORDELIZXRUWF72FTCRNOXUFPH7YOO3PFFMXLAVUN` (BlocRA) |
| Signature | Ed25519 over the embedded report |
| Report sha256 | `332782de86613850a50fc61fbd8bc466a70f714032573edd40d1b01cde28f9cb` |
| Signed at | 2026-11-06T12:43:12.815059566+00:00 |

Check this file with `npx blocra verify <file>`. Any edit to the figures above breaks verification.

<!-- blocra-attestation:eyJhdHRlc3RhdGlvbiI6eyJhbGdvcml0aG0iOiJlZDI1NTE5IiwibWV0aG9kX3ZlcnNpb24iOiJ0cmFjdGlvbi8xIiwicGF5bG9hZF9zaGEyNTYiOiIzMzI3ODJkZTg2NjEzODUwYTUwZmM2MWZiZDhiYzQ2NmE3MGY3MTQwMzI1NzNlZGQ0MGQxYjAxY2RlMjhmOWNiIiwic2lnbmF0dXJlIjoiNTdjNzg5MTk2MGNiYWRhZGE2MTZjZDc1MDY4ZjI0NmE1MzM2OWRjOTI1ZWRkYjM0ZWVhYjdkZmU0OWFiMmFmODYwMWQ4MWUyMWI3YTVkMjZjYTI4MWVhZmJjYTlhOGRjZjQ5M2VjMTcxNjE4NzVmNWE0MGE2N2RlMjgyMzZkMGYiLCJzaWduZWRfYXQiOiIyMDI2LTExLTA2VDEyOjQzOjEyLjgxNTA1OTU2NiswMDowMCIsInNpZ25lciI6IkdCUTZRSUhWREhGU1k0Q0RPUkRFTElaWFJVV0Y3MkZUQ1JOT1hVRlBIN1lPTzNQRkZNWExBVlVOIn0sInJlcG9ydF9qc29uIjoie1wiYXNzZXRfc25hcHNob3RcIjpudWxsLFwiY2hhaW5cIjpcInN0ZWxsYXJcIixcImNvdmVyYWdlXCI6e1wiY29sbGVjdGlvbl9zdGFydGVkXCI6XCIyMDI2LTExLTA2XCIsXCJjb21wbGV0ZW5lc3NfcGN0XCI6NTQuNyxcImRheXNfY292ZXJlZFwiOjAuMTYsXCJleHBlY3RlZF9sZWRnZXJzXCI6MTA5OTY0LjAsXCJmdWxsX3BlcmlvZFwiOmZhbHNlLFwiaG91cnNfY292ZXJlZFwiOjMuOSxcImxlZGdlcl9mcm9tXCI6NjUzMzM2ODcsXCJsZWRnZXJfdG9cIjo2NTMzNjUwMSxcImxlZGdlcnNfY29sbGVjdGVkXCI6MTU0MX0sXCJkYWlseVwiOlt7XCJhY3RpdmVfYWNjb3VudHNcIjo0NjM4LFwiY2FsbHNcIjoxNDQ3NTgsXCJkYXlcIjpcIjIwMjYtMTEtMDZcIixcImZhaWxlZFwiOjIyMTgsXCJ2b2x1bWVcIjowLjB9XSxcImVjb3N5c3RlbVwiOntcIm1lZGlhbl9mYWlsZWRfcGN0XCI6MC44LFwibWV0cmljXCI6XCJhY3RpdmVfYWNjb3VudHNcIixcIm9mXCI6MTQ2LFwicGVyY2VudGlsZVwiOjEwMC4wLFwicmFua1wiOjF9LFwiZmluZGluZ3NcIjpbXCIxLjUlIG9mIGNhbGxzIGZhaWxlZCwgaW4gbGluZSB3aXRoIHRoZSAwLjglIG1lZGlhbiBvZiBjb21wYXJhYmxlIGNvbnRyYWN0cy5cIixcIlJhbmtlZCAjMSBvZiAxNDYgY29udHJhY3RzIGJ5IGFjdGl2ZSBhY2NvdW50cyBpbiB0aGlzIHBlcmlvZCAodG9wIDElKS5cIixcIlBhcnRpYWwgZGF0YTogQmxvY1JBIGhvbGRzIDMuOSBob3VycyBvZiB0aGUgNy1kYXkgcGVyaW9kICg1NC43JSBvZiBsZWRnZXJzIGluIHRoYXQgc3BhbikuIEZpZ3VyZXMgY292ZXIgb25seSB0aGUgY29sbGVjdGVkIHRpbWUuXCJdLFwiZnVuY3Rpb25zXCI6W3tcImNhbGxzXCI6ODE3OTYsXCJmYWlsZWRcIjoxMDcsXCJmYWlsZWRfcGN0XCI6MC4xLFwiZnVuY3Rpb25cIjpcInBsYW50XCJ9LHtcImNhbGxzXCI6NjI3NjQsXCJmYWlsZWRcIjoyMTA1LFwiZmFpbGVkX3BjdFwiOjMuNCxcImZ1bmN0aW9uXCI6XCJ3b3JrXCJ9LHtcImNhbGxzXCI6MTk4LFwiZmFpbGVkXCI6NixcImZhaWxlZF9wY3RcIjozLjAsXCJmdW5jdGlvblwiOlwiaGFydmVzdFwifV0sXCJnZW5lcmF0ZWRfYXRcIjpcIjIwMjYtMTEtMDZUMTI6NDM6MTIuODEyNzcyMTE4KzAwOjAwXCIsXCJrcGlzXCI6e1wiYWN0aXZlX2FjY291bnRzXCI6e1wiY2hhbmdlX3BjdFwiOm51bGwsXCJwcmV2aW91c1wiOm51bGwsXCJ2YWx1ZVwiOjQ2MzguMH0sXCJjYWxsc1wiOntcImNoYW5nZV9wY3RcIjpudWxsLFwicHJldmlvdXNcIjpudWxsLFwidmFsdWVcIjoxNDQ3NTguMH0sXCJjYWxsc19wZXJfYWNjb3VudFwiOjMxLjIsXCJmYWlsZWRfcGN0XCI6e1wiY2hhbmdlX3BjdFwiOm51bGwsXCJwcmV2aW91c1wiOm51bGwsXCJ2YWx1ZVwiOjEuNX0sXCJuZXdfYWNjb3VudHNcIjp7XCJyZWxpYWJsZVwiOmZhbHNlLFwidmFsdWVcIjo0NjM4fSxcInZvbHVtZVwiOm51bGx9LFwibWV0aG9kX3ZlcnNpb25cIjpcInRyYWN0aW9uLzFcIixcIm1ldGhvZG9sb2d5XCI6W1wiU291cmNlOiBldmVyeSBTdGVsbGFyIGxlZGdlciByZWFkIGZyb20gSG9yaXpvbiBieSBCbG9jUkEncyBjb2xsZWN0b3IgYW5kIHN0b3JlZCB3aXRoIGl0cyB0cmFuc2FjdGlvbnMgYW5kIG9wZXJhdGlvbnMuXCIsXCJEYXlzIGFyZSBVVEMgY2FsZW5kYXIgZGF5czsgdGhlIGxhdGVzdCBkYXkgbWF5IGJlIHBhcnRpYWwuXCIsXCJBIGNhbGwgaXMgb25lIGludm9rZV9ob3N0X2Z1bmN0aW9uIG9wZXJhdGlvbiB0YXJnZXRpbmcgdGhlIGNvbnRyYWN0OyB0aGUgYWNjb3VudCBpcyB0aGUgb3BlcmF0aW9uJ3Mgc291cmNlLiBGYWlsZWQgY2FsbHMgKGZyb20gZmFpbGVkIHRyYW5zYWN0aW9ucykgYXJlIGluY2x1ZGVkIGluIGNhbGxzIGFuZCBjb3VudGVkIGluIGZhaWxlZC5cIixcIkF1dG9tYXRlZCAoYm90LWxpa2UpIGFjY291bnRzIG1hZGUgYXQgbGVhc3QgMTAwMCBjYWxscyBvbiBhIHNpbmdsZSBkYXkuXCIsXCJOZXcgYWNjb3VudHMgd2VyZSBmaXJzdCBzZWVuIGR1cmluZyB0aGUgcGVyaW9kIGluIGFsbCBkYXRhIEJsb2NSQSBoYXMgY29sbGVjdGVkOyB3aXRoIGxlc3MgaGlzdG9yeSB0aGFuIHRoZSBwZXJpb2QsIHRoZXkgYXJlIG1hcmtlZCB1bnJlbGlhYmxlLlwiLFwiRWNvc3lzdGVtIHJhbmsgY29tcGFyZXMgYWN0aXZlIGFjY291bnRzIHdpdGggZXZlcnkgY29udHJhY3QgKG9yIGFzc2V0KSBhY3RpdmUgaW4gdGhlIHNhbWUgcGVyaW9kOyB0aGUgZmFpbHVyZS1yYXRlIG1lZGlhbiB1c2VzIHBlZXJzIHdpdGggYXQgbGVhc3QgMTAgYWN0aXZlIGFjY291bnRzLlwiXSxcIm5ldHdvcmtcIjpcIm1haW5uZXRcIixcInBlcmlvZFwiOntcImRheXNcIjo3LFwiZnJvbVwiOlwiMjAyNi0xMC0zMVwiLFwiaGFzX3ByZXZpb3VzXCI6ZmFsc2UsXCJwcmV2aW91c19mcm9tXCI6XCIyMDI2LTEwLTI0XCIsXCJwcmV2aW91c190b1wiOlwiMjAyNi0xMC0zMFwiLFwidG9cIjpcIjIwMjYtMTEtMDZcIn0sXCJxdWFsaXR5XCI6e1wiYm90X2xpa2VfYWNjb3VudHNcIjowLFwiYm90X2xpa2Vfc2hhcmVfcGN0XCI6MC4wLFwiYm90X3RocmVzaG9sZF9jYWxsc19wZXJfZGF5XCI6MTAwMCxcInNlbGZfcGF5bWVudF9zaGFyZV9wY3RcIjpudWxsLFwic2VsZl9wYXltZW50c1wiOm51bGwsXCJ0b3AxMF9zaGFyZV9wY3RcIjowLjMsXCJ0b3BfYWNjb3VudHNcIjpbe1wiYWNjb3VudFwiOlwiR0RXSllRNEdKWVpHUUVGNUVZV0QzNE1XTFRFVk1CSFBYVTM3QlhDRVFPTzRET0Y2TElXVzc1RVRcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjQ5LFwic2hhcmVfcGN0XCI6MC4wM30se1wiYWNjb3VudFwiOlwiR0RWVlJPSlBGNDVXWjdWUEhYMjVKRkNIVDVJN0pMNUhYRVJERVZPUjZPSjU2WUZEUE9BVTVBR1FcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjQ5LFwic2hhcmVfcGN0XCI6MC4wM30se1wiYWNjb3VudFwiOlwiR0RPWUdCNDY0RUhRWFVQQlo2UDczRUE0TUtWRFhES0RJQ1hCTzZSU0szWVZQVDVaN1pCVDZDUFZcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjQ5LFwic2hhcmVfcGN0XCI6MC4wM30se1wiYWNjb3VudFwiOlwiR0RBSlBYN05aUjdZTU1USDRaV0QyU0tNRFdPR01TRlI3SjNJVk9TMjM1Q0tDMlJMTkhVUkYzSklcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjQ5LFwic2hhcmVfcGN0XCI6MC4wM30se1wiYWNjb3VudFwiOlwiR0NCTUo0SlRGQTI2TEdSS1JNNUNGRFJYTjY0NVJKRUtFSDVETEdGNVhPUjZaQ1JCQ0NOVFNSM0hcIixcImJvdF9saWtlXCI6ZmFsc2UsXCJjYWxsc1wiOjQ5LFwic2hhcmVfcGN0XCI6MC4wM31dfSxcInJldGVudGlvblwiOntcInByZXZpb3VzX2FjdGl2ZVwiOm51bGwsXCJyZXR1cm5pbmdfYWNjb3VudHNcIjowLFwicmV0dXJuaW5nX3BjdFwiOm51bGx9LFwic3ViamVjdFwiOntcImlkXCI6XCJDREw3NFJGNUJMWVIyWUJMQ0NJN0Y1RkI2VFBTQ0xLRUpVQlNEMlJTVldaNFlIRjNWTUZBSUdXQVwiLFwia2luZFwiOlwiY29udHJhY3RcIixcImxhYmVsXCI6XCJDb250cmFjdCBDREw3NFLigKZJR1dBXCJ9LFwidHlwZVwiOlwidHJhY3Rpb25cIixcInZlcnNpb25cIjoxfSJ9 -->
