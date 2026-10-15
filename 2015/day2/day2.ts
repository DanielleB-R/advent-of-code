import { readLines } from "../lib/input.js";

const packages: [number, number, number][] = readLines(
  /^(\d+)x(\d+)x(\d+)$/,
  ([_, h, w, l]) => [parseInt(h, 10), parseInt(w, 10), parseInt(l, 10)],
);

const totalArea = packages.reduce((sum, [h, w, l]) => {
  const side1 = h * w;
  const side2 = w * l;
  const side3 = l * h;

  const smallest = Math.min(side1, side2, side3);

  return sum + 2 * side1 + 2 * side2 + 2 * side3 + smallest;
}, 0);

console.log(totalArea);
