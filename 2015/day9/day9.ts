import { readLines } from "../lib/input.js";
import { permutationsOf } from "../lib/permutations.js";
import { max, min } from "../lib/math.js";

const input = readLines(
  /^(\w+) to (\w+) = (\d+)$/,
  ([, from, to, distance]) => ({
    from,
    to,
    distance: parseInt(distance, 10),
  }),
);

type Leg = (typeof input)[0];

const edgeKey = (from: string, to: string): string => `${from}-${to}`;

const buildGraph = (legs: Leg[]): [Set<string>, Map<string, number>] => {
  const locations = new Set<string>();
  const distances = new Map<string, number>();

  for (const { from, to, distance } of legs) {
    locations.add(from);
    locations.add(to);

    distances.set(edgeKey(from, to), distance);
    distances.set(edgeKey(to, from), distance);
  }

  return [locations, distances];
};

const [nodes, edges] = buildGraph(input);

const nodePermutations = permutationsOf(nodes);

const distances = nodePermutations.map((route) => {
  let distance = 0;
  for (let i = 0; i < route.length - 1; i++) {
    distance += edges.get(edgeKey(route[i]!, route[i + 1]!))!;
  }
  return distance;
});

console.log(min(distances));
console.log(max(distances));
