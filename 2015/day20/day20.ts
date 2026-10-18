import { readFileSync } from "node:fs";

const input = parseInt(readFileSync("input.txt", "utf8"), 10);

const scratchPad = new Uint32Array(input / 10);

for (let i = 1; i < input / 10; i++) {
  for (let j = i - 1; j < input / 10; j += i) {
    scratchPad[j] += i * 10;
  }
}

const firstHouse = scratchPad.findIndex((x) => x >= input) + 1;
console.log(firstHouse, scratchPad[firstHouse - 1]);

const scratchPad2 = new Uint32Array(input / 10);

for (let i = 1; i < input / 10; i++) {
  for (let j = 1; j <= 50; j++) {
    scratchPad2[i * j] += i * 11;
  }
}

const firstHouse2 = scratchPad2.findIndex((x) => x >= input);

console.log(firstHouse2, scratchPad2[firstHouse2 - 2]);
