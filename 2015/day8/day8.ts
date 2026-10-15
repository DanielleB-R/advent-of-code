import { readFileSync } from "node:fs";
import { sum } from "../lib/math.js";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const ESCAPE_MATCH_REGEX = /(\\x[a-fA-F\d]{2}|\\"|\\\\)/;

const deEscape = (raw: string): string => {
  return raw.substring(1, raw.length - 1).replaceAll(ESCAPE_MATCH_REGEX, "q");
};

console.log(sum(input.map((raw) => raw.length - deEscape(raw).length)));

const reEscape = (raw: string): string => {
  return `"${raw.replaceAll("\\", "\\\\").replaceAll('"', '\\"')}"`;
};

console.log(sum(input.map((raw) => reEscape(raw).length - raw.length)));
