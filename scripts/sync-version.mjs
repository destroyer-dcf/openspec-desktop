#!/usr/bin/env node
import fs from "node:fs";

const packageJsonPath = new URL("../package.json", import.meta.url);
const tauriConfigPath = new URL("../src-tauri/tauri.conf.json", import.meta.url);
const cargoTomlPath = new URL("../src-tauri/Cargo.toml", import.meta.url);

const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, "utf8"));
const version = packageJson.version;

if (!version) {
  throw new Error("No se encontró version en package.json");
}

const tauriConfig = JSON.parse(fs.readFileSync(tauriConfigPath, "utf8"));
tauriConfig.version = version;
fs.writeFileSync(tauriConfigPath, `${JSON.stringify(tauriConfig, null, 2)}\n`);

const cargoToml = fs.readFileSync(cargoTomlPath, "utf8");
const packageSectionRegex = /(\[package\][\s\S]*?\nversion\s*=\s*")[^"]+(")/;

if (!packageSectionRegex.test(cargoToml)) {
  throw new Error("No se encontró el campo version en [package] de Cargo.toml");
}

const updatedCargoToml = cargoToml.replace(packageSectionRegex, `$1${version}$2`);
fs.writeFileSync(cargoTomlPath, updatedCargoToml);

console.log(`Version sincronizada a ${version}`);
