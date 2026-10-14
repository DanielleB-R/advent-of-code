import { readFileSync } from "node:fs";
import { permutationsOf } from "../lib/permutations.js";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const LINE_REGEX = /^(\w+) to (\w+) = (\d+)$/;

const edgeKey = (from: string, to: string): string => `${from}-${to}`;

const buildGraph = (legs: string[]): [Set<string>, Map<string, number>] => {
  const locations = new Set<string>();
  const distances = new Map<string, number>();

  for (const leg of legs) {
    const match = LINE_REGEX.exec(leg);

    if (!match) {
      throw new Error(`Malformed input: ${leg}`);
    }

    const from = match[1]!;
    const to = match[2]!;
    const distance = parseInt(match[3]!, 10);

    locations.add(from);
    locations.add(to);

    distances.set(edgeKey(from, to), distance);
    distances.set(edgeKey(to, from), distance);
  }

  return [locations, distances];
};

const [nodes, edges] = buildGraph(input);

const nodePermutations = permutationsOf(nodes);

let minDistance = 1e9;
let maxDistance = 0;

nodePermutations.forEach((route) => {
  let distance = 0;
  for (let i = 0; i < route.length - 1; i++) {
    distance += edges.get(edgeKey(route[i]!, route[i + 1]!))!;
  }
  if (distance < minDistance) {
    minDistance = distance;
  }
  if (distance > maxDistance) {
    maxDistance = distance;
  }
});

console.log(minDistance);

console.log(maxDistance);
