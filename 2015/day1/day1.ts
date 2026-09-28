import { readFileSync } from "node:fs";

const instructions = readFileSync('input.txt', 'utf8');

let floor = 0;
for (const instruction of instructions) {
  if (instruction == "(") {
    floor += 1;
  } else {
    floor -= 1;
  }
}

console.log(floor);
