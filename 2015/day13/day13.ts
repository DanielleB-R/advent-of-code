import { readLines } from "../lib/input.js";
import { permutationsOf } from "../lib/permutations.js";

const input: [string, number, string][] = readLines(
  /^(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+).$/,
  ([_, subject, sense, amount, object]) => [
    subject,
    (sense === "lose" ? -1 : 1) * parseInt(amount, 10),
    object,
  ],
);

type Increments = Record<string, Record<string, number>>;

const increments: Increments = {};

input.forEach(([subject, amount, object]) => {
  increments[subject] ??= {};
  increments[subject][object] = amount;
});

const maxHappinessFor = (increments: Increments): number => {
  const permutations = permutationsOf(new Set(Object.keys(increments)));

  const permutationHappiness = permutations.map((seats) => {
    let happiness = 0;
    for (let i = 0; i < seats.length - 1; i++) {
      happiness += increments[seats[i]][seats[i + 1]];
    }
    happiness += increments[seats[seats.length - 1]][seats[0]];
    for (let i = 1; i < seats.length; i++) {
      happiness += increments[seats[i]][seats[i - 1]];
    }
    happiness += increments[seats[0]][seats[seats.length - 1]];
    return happiness;
  });
  return permutationHappiness.reduce((max, val) => (val > max ? val : max));
};

console.log(maxHappinessFor(increments));

const ME = "Me";
for (const person of Object.values(increments)) {
  person[ME] = 0;
}
increments[ME] = Object.fromEntries(
  Object.keys(increments).map((name) => [name, 0]),
);

console.log(maxHappinessFor(increments));
