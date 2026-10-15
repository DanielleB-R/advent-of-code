import { readFileSync } from "node:fs";
import { sum } from "../lib/math.js";

const instructions = readFileSync("input.txt", "utf8").trim();

const offset = (inst: string) => (inst === "(" ? 1 : -1);

console.log(sum(Array.from(instructions).map(offset)));

let floor = 0;

console.log(
  Array.from(instructions).findIndex((inst) => {
    floor += offset(inst);
    return floor < 0;
  }) + 1,
);
