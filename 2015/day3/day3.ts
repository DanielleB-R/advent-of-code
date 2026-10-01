import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

let row = 0;
let column = 0;

const seen = new Set<string>(["0:0"]);

for (const inputChar of input) {
  switch (inputChar) {
    case "^":
      column += 1;
      break;
    case "v":
      column -= 1;
      break;
    case ">":
      row += 1;
      break;
    case "<":
      row -= 1;
      break;
    default:
      throw new Error(`Unexpected char ${inputChar}`);
  }
  seen.add(`${column}:${row}`);
}

console.log(seen.size);
