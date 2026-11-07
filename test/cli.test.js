import { test } from "node:test";
import assert from "node:assert/strict";
import { readFileSync, writeFileSync, mkdtempSync } from "node:fs";
import { tmpdir } from "node:os";
import { join } from "node:path";
import { execFileSync } from "node:child_process";
import { generateKeyPairSync, sign, createHash } from "node:crypto";
import { decodeAccountId } from "../src/strkey.js";
import { verifyEnvelope } from "../src/verify.js";
import { renderMarkdown, parseMarkdown, renderBody } from "../src/markdown.js";

const fixture = (name) => JSON.parse(readFileSync(new URL(`./fixtures/${name}`, import.meta.url), "utf8"));
const asset = fixture("signed-asset.json");
const contract = fixture("signed-contract.json");
const SIGNER = asset.attestation.signer;
const CLI = new URL("../bin/blocra.js", import.meta.url).pathname;

test("decodes Stellar account ids and rejects typos", () => {
  assert.equal(decodeAccountId(SIGNER).length, 32);
  const typo = SIGNER.slice(0, 10) + (SIGNER[10] === "A" ? "B" : "A") + SIGNER.slice(11);
  assert.throws(() => decodeAccountId(typo), /checksum/);
  assert.throws(() => decodeAccountId("SABC"), /account id/);
});

test("accepts genuine signed reports", () => {
  for (const env of [asset, contract]) {
    const r = verifyEnvelope(env, SIGNER);
    assert.equal(r.ok, true, r.reasons.join("; "));
    assert.equal(r.report.method_version, "traction/1");
    assert.equal(r.sha256, env.attestation.payload_sha256);
  }
});

test("rejects an edited report", () => {
  const env = structuredClone(asset);
  env.report_json = env.report_json.replace(/"active_accounts":\{"change_pct":null,"previous":null,"value":(\d+)/, (_m, v) => `"active_accounts":{"change_pct":null,"previous":null,"value":${Number(v) * 10}`);
  assert.notEqual(env.report_json, asset.report_json);
  const r = verifyEnvelope(env, SIGNER);
  assert.equal(r.ok, false);
});

test("rejects a report re-signed by another key, even with a consistent hash", () => {
  const env = structuredClone(asset);
  const { privateKey } = generateKeyPairSync("ed25519");
  env.report_json = env.report_json.replace('"findings":[', '"findings":["All good.",');
  env.attestation.signature = sign(null, Buffer.from(env.report_json), privateKey).toString("hex");
  env.attestation.payload_sha256 = createHash("sha256").update(env.report_json).digest("hex");
  const r = verifyEnvelope(env, SIGNER);
  assert.equal(r.ok, false);
});

test("rejects a genuine report when a different key is trusted", () => {
  const other = "GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3";
  const r = verifyEnvelope(asset, other);
  assert.equal(r.ok, false);
  assert.match(r.reasons.join(" "), /not by the trusted BlocRA key/);
});

test("Markdown round-trips and embeds the envelope", () => {
  for (const env of [asset, contract]) {
    const md = renderMarkdown(env);
    assert.doesNotMatch(md, /NaN|undefined|\[object/);
    for (const section of ["## Scorecard", "## Key findings", "## Activity", "## User quality", "## Ecosystem position", "## Coverage and methodology", "## Verification"]) {
      assert.ok(md.includes(section), section);
    }
    const { body, envelope } = parseMarkdown(md);
    assert.deepEqual(envelope, env);
    assert.equal(body.replace(/\n+$/, ""), renderBody(JSON.parse(env.report_json), env.attestation).replace(/\n$/, ""));
  }
});

function cli(args, cwd) {
  try {
    return { code: 0, out: execFileSync("node", [CLI, ...args], { cwd, encoding: "utf8", env: { ...process.env, NO_COLOR: "1" } }) };
  } catch (e) {
    return { code: e.status, out: `${e.stdout}${e.stderr}` };
  }
}

test("blocra verify passes a genuine Markdown report and fails an edited one", () => {
  const dir = mkdtempSync(join(tmpdir(), "blocra-"));
  const good = join(dir, "good.md");
  writeFileSync(good, renderMarkdown(asset));
  const ok = cli(["verify", good, "--key", SIGNER], dir);
  assert.equal(ok.code, 0, ok.out);
  assert.match(ok.out, /VALID/);

  const bad = join(dir, "bad.md");
  writeFileSync(bad, renderMarkdown(asset).replace(/\| Active accounts \| ([\d,]+)/, "| Active accounts | 999,999"));
  const res = cli(["verify", bad, "--key", SIGNER], dir);
  assert.equal(res.code, 1);
  assert.match(res.out, /edited after signing/);
});

test("blocra verify fails files without a signature", () => {
  const dir = mkdtempSync(join(tmpdir(), "blocra-"));
  writeFileSync(join(dir, "plain.md"), "# Our traction\n\n1,000,000 users\n");
  const res = cli(["verify", "plain.md", "--key", SIGNER], dir);
  assert.equal(res.code, 1);
  assert.match(res.out, /no BlocRA signature/);
});

test("relayed contract reports show relayer figures and still verify", () => {
  const relayed = fixture("signed-relayed.json");
  const r = verifyEnvelope(relayed, SIGNER);
  assert.equal(r.ok, true, r.reasons.join("; "));
  assert.ok(r.report.quality.relayed_calls > 0);
  const md = renderMarkdown(relayed);
  assert.match(md, /Calls submitted by a relayer or sponsor/);
  assert.match(md, /Declared relayers \(excluded from account counts\)/);
  assert.doesNotMatch(md, /NaN|undefined/);
});

test("trusts any key in a list of official keys", () => {
  assert.equal(verifyEnvelope(asset, ["GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3", SIGNER]).ok, true);
  assert.equal(verifyEnvelope(asset, ["GDQNY3PBOJOKYZSRMK2S7LHHGWZIUISD4QORETLMXEWXBI7KFZZMKTL3"]).ok, false);
});

test("base64 embed/parse is identical with and without Buffer (browser path)", async () => {
  const md = await import("../src/markdown.js");
  const samples = [asset, contract, fixture("signed-relayed.json")];
  const withBuffer = samples.map((env) => md.renderMarkdown(env));
  const text = "Ünïcödé — ≥ 😀 " + "x".repeat(70000);
  const b64 = md.utf8ToBase64(text);
  const saved = Object.getOwnPropertyDescriptor(globalThis, "Buffer");
  delete globalThis.Buffer;
  try {
    assert.equal(typeof Buffer, "undefined");
    assert.equal(md.utf8ToBase64(text), b64);
    assert.equal(md.base64ToUtf8(b64), text);
    samples.forEach((env, i) => {
      const out = md.renderMarkdown(env);
      assert.equal(out, withBuffer[i]);
      const { body, envelope } = md.parseMarkdown(out);
      assert.deepEqual(envelope, env);
      assert.ok(md.bodyMatchesReport(body, JSON.parse(env.report_json), env.attestation));
    });
  } finally {
    Object.defineProperty(globalThis, "Buffer", saved);
  }
  assert.equal(md.base64ToUtf8(b64), text);
});

test("package exports the renderer for the website", async () => {
  const pkg = JSON.parse(readFileSync(new URL("../package.json", import.meta.url), "utf8"));
  assert.equal(pkg.exports["./markdown"], "./src/markdown.js");
  assert.equal(pkg.bin.blocra, "bin/blocra.js");
  const viaExports = await import("blocra/markdown");
  assert.equal(typeof viaExports.renderMarkdown, "function");
  assert.equal(typeof viaExports.parseMarkdown, "function");
  assert.equal(typeof viaExports.renderBody, "function");
});
