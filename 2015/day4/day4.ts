import { readFileSync } from "node:fs";
import { createHash } from "node:crypto";

const input = readFileSync("input.txt", "utf8").trim();

const hashRound = (counter: number): string => {
  const hash = createHash("md5");

  hash.update(`${input}${counter}`);

  return hash.digest("hex");
};

let counter = 0;

while (!hashRound(counter).startsWith("00000")) {
  counter++;
}

console.log(counter);

while (!hashRound(counter).startsWith("000000")) {
  counter++;
}

console.log(counter);
