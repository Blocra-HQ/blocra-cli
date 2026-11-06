// blocra.json: the project's subjects, chain, API and pinned signing key.

import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { resolve } from "node:path";

export const CONFIG_FILE = "blocra.json";

export function loadConfig(dir = process.cwd()) {
  const path = resolve(dir, CONFIG_FILE);
  if (!existsSync(path)) return null;
  try {
    return JSON.parse(readFileSync(path, "utf8"));
  } catch (e) {
    throw new Error(`${CONFIG_FILE} is not valid JSON: ${e.message}`);
  }
}

export function saveConfig(config, dir = process.cwd()) {
  const path = resolve(dir, CONFIG_FILE);
  writeFileSync(path, JSON.stringify(config, null, 2) + "\n");
  return path;
}
