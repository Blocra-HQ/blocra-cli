// Official BlocRA endpoints and signing keys, built into the package so that
// `npx blocra verify report.md` works with no setup. Filled in for each
// release from the production deployment.
//
// OFFICIAL_KEYS lists every key BlocRA has signed reports with (newest
// first), so reports signed before a key rotation keep verifying.

export const DEFAULT_API = null; // e.g. "https://blocra-api.onrender.com"
export const OFFICIAL_KEYS = []; // e.g. ["GABC..."]
