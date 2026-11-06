#!/usr/bin/env node
// blocra: signed, verifiable traction reports from your terminal or CI.

import { readFileSync, writeFileSync, mkdirSync } from "node:fs";
import { join, basename } from "node:path";
import { fetchSigningKey, fetchSignedReport, ApiError } from "../src/api.js";
import { loadConfig, saveConfig, CONFIG_FILE } from "../src/config.js";
import { verifyEnvelope } from "../src/verify.js";
import { renderMarkdown, parseMarkdown, renderBody, fmtInt, fmtPct } from "../src/markdown.js";

const VERSION = JSON.parse(readFileSync(new URL("../package.json", import.meta.url), "utf8")).version;

const HELP = `blocra ${VERSION}: signed, verifiable traction reports for on-chain projects

Usage
  blocra init --subject <id> [--subject <id>...] [--chain stellar] [--api <url>] [--days 30]
  blocra report [<id>...] [--days 7|30] [--out <dir>] [--format md,json]
  blocra verify <file.md|file.json>... [--key <G...>]
  blocra key

Subjects
  A smart contract id, or an asset (e.g. XLM or CODE:ISSUER on Stellar).

Options
  --api <url>     BlocRA API (or BLOCRA_API, or "api" in ${CONFIG_FILE})
  --chain <name>  Chain of the subjects (default stellar)
  --key <G...>    Signing key to trust when verifying (or BLOCRA_TRUSTED_KEY,
                  or "trusted_key" in ${CONFIG_FILE}, pinned by init)
  --days <n>      Report period in days (default 7)
  --out <dir>     Where report files are written (default .)
  --format <f>    md, json or md,json (default md,json)

Reports are computed and signed by BlocRA from on-chain data; this tool only
fetches and verifies them. Exit code 1 on any failure, so it fits CI.`;

function parseArgs(argv) {
  const args = { _: [], subject: [] };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (a === "-h" || a === "--help") args.help = true;
    else if (a === "-v" || a === "--version") args.version = true;
    else if (a.startsWith("--")) {
      const [k, inline] = a.slice(2).split("=", 2);
      const v = inline ?? argv[++i];
      if (v === undefined) throw new Error(`--${k} needs a value`);
      if (k === "subject") args.subject.push(v);
      else args[k] = v;
    } else args._.push(a);
  }
  return args;
}

const bold = (s) => (process.stdout.isTTY ? `\x1b[1m${s}\x1b[0m` : s);
const green = (s) => (process.stdout.isTTY ? `\x1b[32m${s}\x1b[0m` : s);
const red = (s) => (process.stdout.isTTY ? `\x1b[31m${s}\x1b[0m` : s);
const dim = (s) => (process.stdout.isTTY ? `\x1b[2m${s}\x1b[0m` : s);

function apiUrl(args, config) {
  const api = args.api ?? process.env.BLOCRA_API ?? config?.api;
  if (!api) throw new Error(`No BlocRA API set. Pass --api <url>, set BLOCRA_API, or run "blocra init --api <url>".`);
  return api;
}

/** The signer to trust: explicit flag/env/config, otherwise the server's key (with a warning). */
async function trustedSigner(args, config, api) {
  const pinned = args.key ?? process.env.BLOCRA_TRUSTED_KEY ?? config?.trusted_key;
  if (pinned) return pinned;
  const key = await fetchSigningKey(api);
  console.error(dim(`No pinned key; trusting the key served by ${new URL(api).host}: ${key.signer}`));
  return key.signer;
}

async function cmdInit(args) {
  const existing = loadConfig() ?? {};
  const api = args.api ?? process.env.BLOCRA_API ?? existing.api;
  if (!api) throw new Error("Pass --api <url> (the BlocRA API to fetch reports from).");
  const subjects = args.subject.length ? args.subject : existing.subjects ?? [];
  if (!subjects.length) throw new Error("Pass at least one --subject <contract id or asset>.");
  const key = await fetchSigningKey(api);
  const chain = (args.chain ?? existing.chain ?? "stellar").toLowerCase();
  if (key.chains && !key.chains.includes(chain)) {
    console.error(dim(`Note: ${new URL(api).host} reports on ${key.chains.join(", ")} today; ${chain} is not supported yet.`));
  }
  const config = {
    api,
    chain,
    subjects,
    days: Number(args.days ?? existing.days ?? 30),
    trusted_key: key.signer,
    out: args.out ?? existing.out ?? "reports",
  };
  const path = saveConfig(config);
  console.log(`${green("✓")} Wrote ${basename(path)}: ${subjects.length} subject(s) on ${chain}, ${config.days}-day reports`);
  console.log(`  Pinned BlocRA signing key ${key.signer} (${key.method_version})`);
  console.log(`  Next: ${bold("npx blocra report")}`);
}

function printSummary(report) {
  const k = report.kpis ?? {};
  const noun = report.subject?.kind === "contract" ? "calls" : "payments";
  console.log(`\n${bold(report.subject?.label ?? report.subject?.id)} ${dim(`${report.chain ?? "stellar"} · ${report.period?.from} – ${report.period?.to}`)}`);
  console.log(
    `  ${fmtInt(k.active_accounts?.value)} active accounts · ${fmtInt(k.calls?.value)} ${noun} · ` +
      `${fmtPct(k.failed_pct?.value)} failed · rank #${report.ecosystem?.rank ?? "—"} of ${report.ecosystem?.of ?? "—"}`,
  );
  for (const f of report.findings ?? []) console.log(`  - ${f}`);
}

async function cmdReport(args) {
  const config = loadConfig();
  const api = apiUrl(args, config);
  const subjects = args._.length ? args._ : config?.subjects ?? [];
  if (!subjects.length) throw new Error(`Name a subject (blocra report <id>) or run "blocra init" first.`);
  const days = Number(args.days ?? config?.days ?? 7);
  const chain = (args.chain ?? config?.chain ?? "stellar").toLowerCase();
  const formats = String(args.format ?? "md,json").split(",").map((f) => f.trim());
  const outDir = args.out ?? config?.out ?? ".";
  const signer = await trustedSigner(args, config, api);
  mkdirSync(outDir, { recursive: true });

  let failed = 0;
  for (const subject of subjects) {
    try {
      const envelope = await fetchSignedReport(api, { subject, days, chain });
      const check = verifyEnvelope(envelope, signer);
      if (!check.ok) throw new Error(`Received report failed verification: ${check.reasons.join("; ")}`);
      const r = check.report;
      const name = `blocra-${chain}-${String(r.subject.id).split(":")[0].slice(0, 12).toLowerCase()}-${r.period.to}-${days}d`;
      const written = [];
      if (formats.includes("json")) {
        const p = join(outDir, `${name}.json`);
        writeFileSync(p, JSON.stringify(envelope, null, 2) + "\n");
        written.push(p);
      }
      if (formats.includes("md")) {
        const p = join(outDir, `${name}.md`);
        writeFileSync(p, renderMarkdown(envelope));
        written.push(p);
      }
      printSummary(r);
      console.log(`  ${green("✓")} signed by ${envelope.attestation.signer.slice(0, 8)}… · sha256 ${check.sha256.slice(0, 16)}…`);
      for (const p of written) console.log(`  → ${p}`);
    } catch (e) {
      failed++;
      console.error(`${red("✗")} ${subject}: ${e.message}`);
    }
  }
  if (failed) process.exitCode = 1;
}

async function cmdVerify(args) {
  if (!args._.length) throw new Error("Name the report file(s) to verify.");
  const config = loadConfig();
  let signer = args.key ?? process.env.BLOCRA_TRUSTED_KEY ?? config?.trusted_key;
  let failed = 0;
  for (const file of args._) {
    const text = readFileSync(file, "utf8");
    let envelope;
    let bodyCheck = null;
    if (file.endsWith(".md")) {
      const parsed = parseMarkdown(text);
      envelope = parsed.envelope;
      if (envelope) bodyCheck = parsed.body;
    } else {
      try {
        envelope = JSON.parse(text);
      } catch {
        envelope = null;
      }
    }
    if (!envelope) {
      failed++;
      console.log(`${red("✗ FAIL")} ${file}: no BlocRA signature found`);
      continue;
    }
    if (!signer) signer = await trustedSigner(args, config, apiUrl(args, config));
    const check = verifyEnvelope(envelope, signer);
    if (check.ok && bodyCheck !== null) {
      const expected = renderBody(check.report, envelope.attestation).replace(/\n$/, "");
      if (bodyCheck.replace(/\n+$/, "") !== expected) {
        check.ok = false;
        check.reasons.push("The Markdown text was edited after signing (it no longer matches the signed report)");
      }
    }
    if (check.ok) {
      const r = check.report;
      console.log(`${green("✓ VALID")} ${file}`);
      console.log(`  ${r.subject?.label} · ${r.chain ?? "stellar"} · ${r.period?.from} – ${r.period?.to} · ${r.method_version}`);
      console.log(`  Signed by ${envelope.attestation.signer} at ${envelope.attestation.signed_at}`);
      console.log(`  sha256 ${check.sha256}`);
    } else {
      failed++;
      console.log(`${red("✗ FAIL")} ${file}`);
      for (const reason of check.reasons) console.log(`  - ${reason}`);
    }
  }
  if (failed) process.exitCode = 1;
}

async function cmdKey(args) {
  const config = loadConfig();
  const key = await fetchSigningKey(apiUrl(args, config));
  console.log(`${key.signer}  ${dim(`${key.algorithm} · ${key.method_version} · chains: ${(key.chains ?? []).join(", ")}`)}`);
}

async function main() {
  const args = parseArgs(process.argv.slice(2));
  if (args.version) return console.log(VERSION);
  const [command, ...rest] = args._;
  args._ = rest;
  if (args.help || !command || command === "help") return console.log(HELP);
  const commands = { init: cmdInit, report: cmdReport, verify: cmdVerify, key: cmdKey };
  const run = commands[command];
  if (!run) throw new Error(`Unknown command "${command}". Run blocra --help.`);
  await run(args);
}

main().catch((e) => {
  console.error(`${red("error")} ${e instanceof ApiError ? e.message : e.message ?? e}`);
  process.exitCode = 1;
});
