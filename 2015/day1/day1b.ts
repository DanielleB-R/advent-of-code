import { readFileSync } from "node:fs";

const instructions = readFileSync('input.txt', 'utf8');

let floor = 0;
let index = 1;
for (const instruction of instructions) {
  if (instruction == "(") {
    floor += 1;
  } else {
    floor -= 1;
  }

  if (floor < 0) {
    break;
  }
  
  index +=1;
}

console.log(index);
