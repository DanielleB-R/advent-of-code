import { readLines } from "../lib/input.ts";

const sues = readLines(
  /^Sue (\d+): (\w+): (\d+), (\w+): (\d+), (\w+): (\d+)$/,
  ([, id, k1, v1, k2, v2, k3, v3]) => ({
    id,
    characteristics: Object.fromEntries([[k1, parseInt(v1, 10)], [k2, parseInt(v2, 10)], [k3, parseInt(v3, 10)]])
  })
);

const matchingCharacteristics = `children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1`;

const reference = Object.fromEntries(matchingCharacteristics.split("\n").map((line) => {const pieces = line.split(": "); return [pieces[0], parseInt(pieces[1]!, 10)]}));

const foundSue = sues.find(({characteristics}) => Object.entries(characteristics).every(([k, v]) => reference[k] === v));

console.log(foundSue!.id);

const GT_KEYS = ["cats", "trees"];
const LT_KEYS = ["pomeranians", "goldfish"];

const foundSue2 = sues.find(({characteristics}) => Object.entries(characteristics).every(([k, v]) => {
  if (GT_KEYS.includes(k)) {
    return v > reference[k];
  }
  if (LT_KEYS.includes(k)) {
    return v < reference[k];
  }
  return v === reference[k];
}))

console.log(foundSue2!.id);
