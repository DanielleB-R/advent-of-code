import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

const ITERATIONS = 50;
const SAME_REGEX = /(.)\1*/g;

const iterate = (input: string): string =>
  input.replaceAll(SAME_REGEX, (s) => `${s.length}${s[0]!}`);

let current = input;

for (let i = 0; i < ITERATIONS; i++) {
  current = iterate(current);
}

console.log(current.length);
