import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

const ITERATIONS = 50;
const SAME_REGEX = /(.)\1*/g;

const decompose = (input: string): string[] => [...input.matchAll(SAME_REGEX)].map(([s]) => s);

const describe = (input: string[]): string[] => input.map((part) => `${part.length}${part[0]!}`);

let current = input;

for (let i = 0; i < ITERATIONS; i++) {
        current = describe(decompose(current)).join("");
}

console.log(current.length);
