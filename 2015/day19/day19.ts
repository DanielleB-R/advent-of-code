import { readWithCoda } from "../lib/input.js";

const [transformations, initial] = readWithCoda(
  /^(\w+) => (\w+)$/,
  ([, from, to]) => [from, to] as const,
);

const groupedTrans: Record<string, string[]> = {};

transformations.forEach(([from, to]) => {
  groupedTrans[from] ??= [];
  groupedTrans[from].push(to);
});

const applyOneRound = (
  initialStrings: Set<string>,
  groupedTrans: Record<string, string[]>,
): Set<string> => {
  const results = new Set<string>();

  for (const initial of initialStrings) {
    for (const [from, toOptions] of Object.entries(groupedTrans)) {
      const pieces = initial.split(from);

      for (let i = 1; i < pieces.length; i++) {
        const start = pieces.slice(0, i).join(from);
        const end = pieces.slice(i).join(from);
        for (const to of toOptions) {
          const candidate = start + to + end;
          if (!initialStrings.has(candidate)) {
            results.add(candidate);
          }
        }
      }
    }
  }
  return results;
};

const results = applyOneRound(new Set<string>([initial]), groupedTrans);

console.log(results.size);

const groupedReverse: Record<string, string[]> = {};

transformations.forEach(([from, to]) => {
  groupedReverse[to] ??= [];
  groupedReverse[to].push(from);
});

let results2 = new Set<string>([initial]);
let rounds = 0;

while (!results2.has("e")) {
  const possibilities = applyOneRound(results2, groupedReverse);
  const orderedPossibilities = Array.from(possibilities).sort(
    (a, b) => a.length - b.length,
  );
  results2 = new Set(orderedPossibilities.slice(0, 1));
  rounds += 1;
}

console.log(rounds);
