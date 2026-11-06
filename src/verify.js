// Verification of BlocRA-signed reports.
//
// A signed report is an envelope { report_json, attestation }. The signature
// is Ed25519 over the exact UTF-8 bytes of report_json, so nothing is
// re-serialised: check the bytes, then parse them.

import { createHash, createPublicKey, verify as cryptoVerify } from "node:crypto";
import { decodeAccountId } from "./strkey.js";

// SPKI DER prefix for a raw Ed25519 public key (RFC 8410).
const ED25519_SPKI_PREFIX = Buffer.from("302a300506032b6570032100", "hex");

function publicKeyFromAccountId(accountId) {
  const raw = decodeAccountId(accountId);
  return createPublicKey({ key: Buffer.concat([ED25519_SPKI_PREFIX, Buffer.from(raw)]), format: "der", type: "spki" });
}

/**
 * Checks an envelope against the key you trust. Returns
 * { ok, report, reasons[] }; ok is true only if every check passed.
 */
export function verifyEnvelope(envelope, trustedSigner) {
  const reasons = [];
  const att = envelope?.attestation;
  const text = envelope?.report_json;
  if (typeof text !== "string" || !att) {
    return { ok: false, report: null, reasons: ["Not a BlocRA signed report (missing report_json or attestation)"] };
  }
  if (att.algorithm !== "ed25519") reasons.push(`Unsupported signature algorithm "${att.algorithm}"`);
  if (trustedSigner && att.signer !== trustedSigner) {
    reasons.push(`Signed by ${att.signer}, not by the trusted BlocRA key ${trustedSigner}`);
  }
  const sha = createHash("sha256").update(text, "utf8").digest("hex");
  if (att.payload_sha256 && att.payload_sha256 !== sha) reasons.push("payload_sha256 does not match the report");
  let signatureOk = false;
  try {
    signatureOk = cryptoVerify(null, Buffer.from(text, "utf8"), publicKeyFromAccountId(att.signer), Buffer.from(att.signature, "hex"));
  } catch (e) {
    reasons.push(`Signature check failed: ${e.message}`);
  }
  if (!signatureOk && !reasons.some((r) => r.startsWith("Signature"))) {
    reasons.push("Signature is not valid: the report was changed or not signed by this key");
  }
  let report = null;
  try {
    report = JSON.parse(text);
  } catch {
    reasons.push("report_json is not valid JSON");
  }
  if (report && att.method_version && report.method_version && report.method_version !== att.method_version) {
    reasons.push("Method version in the report and the attestation differ");
  }
  return { ok: reasons.length === 0 && signatureOk, report, sha256: sha, reasons };
}
