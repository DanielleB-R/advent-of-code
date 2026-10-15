import { readLines } from "../lib/input.js";

const packages: [number, number, number][] = readLines(
  /^(\d+)x(\d+)x(\d+)$/,
  ([_, h, w, l]) => [parseInt(h, 10), parseInt(w, 10), parseInt(l, 10)],
);

const totalArea = packages.reduce((sum, [h, w, l]) => {
  const smallestPerimeter = 2 * (h + w + l - Math.max(h, w, l));
  const volume = h * w * l;

  return sum + smallestPerimeter + volume;
}, 0);

console.log(totalArea);
