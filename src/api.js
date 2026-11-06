// BlocRA API client (Node's built-in fetch, no dependencies).

const TIMEOUT_MS = 60_000;

export class ApiError extends Error {}

async function getJson(url) {
  const controller = new AbortController();
  const timer = setTimeout(() => controller.abort(), TIMEOUT_MS);
  let res;
  try {
    res = await fetch(url, { signal: controller.signal, headers: { "user-agent": "blocra-cli" } });
  } catch (e) {
    throw new ApiError(e?.name === "AbortError" ? `Timed out reaching ${new URL(url).origin}` : `Could not reach ${new URL(url).origin}: ${e.message}`);
  } finally {
    clearTimeout(timer);
  }
  const body = await res.json().catch(() => null);
  if (!res.ok || body?.success === false) {
    throw new ApiError(body?.message ?? `HTTP ${res.status} from ${new URL(url).origin}`);
  }
  return body;
}

const base = (api) => api.replace(/\/+$/, "") + "/api";

/** The key the server signs with: { signer, algorithm, method_version, chains }. */
export function fetchSigningKey(api) {
  return getJson(`${base(api)}/attestation/key`);
}

/** A signed traction report envelope: { report_json, attestation }. */
export async function fetchSignedReport(api, { subject, days, chain, relayers = [] }) {
  const params = new URLSearchParams({ subject, days: String(days), chain, signed: "true" });
  if (relayers.length) params.set("relayers", relayers.join(","));
  const body = await getJson(`${base(api)}/reports/traction?${params}`);
  return body.signed;
}
