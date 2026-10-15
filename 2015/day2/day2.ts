import { readLines } from "../lib/input.js";
import { sum } from "../lib/math.js";

const packages: [number, number, number][] = readLines(
  /^(\d+)x(\d+)x(\d+)$/,
  ([_, h, w, l]) => [parseInt(h, 10), parseInt(w, 10), parseInt(l, 10)],
);

const totalArea = sum(
  packages.map(([h, w, l]) => {
    const side1 = h * w;
    const side2 = w * l;
    const side3 = l * h;

    const smallest = Math.min(side1, side2, side3);

    return 2 * side1 + 2 * side2 + 2 * side3 + smallest;
  }),
);

console.log(totalArea);

const totalArea2 = sum(
  packages.map(([h, w, l]) => {
    const smallestPerimeter = 2 * (h + w + l - Math.max(h, w, l));
    const volume = h * w * l;

    return smallestPerimeter + volume;
  }),
);

console.log(totalArea2);
