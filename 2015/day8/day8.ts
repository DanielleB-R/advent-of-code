import { readFileSync } from "node:fs";
import { describe } from "node:test";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const deEscape = (raw: string): string => {
  return raw
    .substring(1, raw.length - 1)
    .replaceAll("\\\\", "\\")
    .replaceAll('\\"', '"')
    .replaceAll(/\\x[a-fA-F\d]{2}/g, "q");
};

const difference = input.reduce((sum, raw) => {
  return sum + (raw.length - deEscape(raw).length);
}, 0);

console.log(difference);

const reEscape = (raw: string): string => {
  return `"${raw.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
};

const difference2 = input.reduce((sum, raw) => {
  return sum + (reEscape(raw).length - raw.length);
}, 0);

console.log(difference2);
