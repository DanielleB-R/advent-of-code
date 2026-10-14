import { readLines } from "../lib/input.js";

const input = readLines(
  /^(\w+): capacity (-?\d+), durability (-?\d+), flavor (-?\d+), texture (-?\d+), calories (\d+)$/,
  ([_, name, capacity, durability, flavor, texture, calories]) => ({
    name,
    capacity: parseInt(capacity, 10),
    durability: parseInt(durability, 10),
    flavor: parseInt(flavor, 10),
    texture: parseInt(texture, 10),
    calories: parseInt(calories, 10),
  })
);

type Ingredient = typeof input[0];
type IngredientScore = Omit<Ingredient, 'name'>;

const AMOUNT = 100;
const CALORIE_COUNT = 500;

if (input.length !== 4) {
  throw new Error("Solution specialized to 4 ingredients");
}

const getScore = (field: keyof IngredientScore, ...amounts: number[]): number => Math.max(
  amounts[0]*input[0][field] + amounts[1]*input[1][field] + amounts[2]*input[2][field] + amounts[3]*input[3][field],
  0
);

let bestScore = 0;
let bestCalorieConstrainedScore = 0;

for (let amount1 = 0; amount1 <= AMOUNT; amount1++) {
  for (let amount2 = 0; amount2 <= (AMOUNT - amount1); amount2++) {
    for (let amount3 = 0; amount3 <= (AMOUNT - amount1 - amount2); amount3++) {
      for (let amount4 = 0; amount4 <= (AMOUNT - amount1 - amount2 - amount3); amount4++) {
        const capacity = getScore('capacity', amount1, amount2, amount3, amount4);
        const durability = getScore('durability', amount1, amount2, amount3, amount4);
        const flavor = getScore('flavor', amount1, amount2, amount3, amount4);
        const texture = getScore('texture', amount1, amount2, amount3, amount4);
        const calories = getScore('calories', amount1, amount2, amount3, amount4);

        const score = capacity * durability * flavor * texture;
        if (score > bestScore) {
          bestScore = score;
        }

        if (calories === CALORIE_COUNT && score > bestCalorieConstrainedScore) {
          bestCalorieConstrainedScore = score;
        }
      }
    }
  }
}

console.log(bestScore);
console.log(bestCalorieConstrainedScore);
