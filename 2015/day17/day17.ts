import { readLines } from "../lib/input.js";
import { min } from "../lib/math.js";

const containers = readLines(/^\d+$/, ([size]) => parseInt(size, 10));

const TOTAL_SIZE = 150;

const combinationsOf = (sizes: number[], remaining: number): number[][] => {
  if (remaining === 0) {
    return [[]];
  }

  if (sizes.length === 0 || remaining < 0) {
    return [];
  }

  let [head, ...rest] = sizes;

  return combinationsOf(rest, remaining - head)
    .map((containers) => [head, ...containers])
    .concat(combinationsOf(rest, remaining));
};

const combinations = combinationsOf(containers, TOTAL_SIZE);

console.log(combinations.length);

const minLength = min(combinations.map((c) => c.length));

console.log(combinations.filter((c) => c.length === minLength).length);
