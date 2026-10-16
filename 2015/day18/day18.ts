import { readFileSync } from "node:fs";
import * as grid from "../lib/grid.js";
import { sum } from "../lib/math.js";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const WIDTH = 100;
const HEIGHT = 100;

if (input.length !== HEIGHT || input[0].length !== WIDTH) {
  throw new Error("Wrong size grid");
}

grid.initialize(WIDTH, HEIGHT);

let state = new Uint8Array([...input.join("")].map((c) => (c === "#" ? 1 : 0)));
let target = new Uint8Array(state);

const STEPS = 100;

const NEIGHBOURS: [number, number][] = [
  [1, 0],
  [1, 1],
  [0, 1],
  [-1, 1],
  [-1, 0],
  [-1, -1],
  [0, -1],
  [1, -1],
];

const takeStep = (state: Uint8Array, target: Uint8Array) => {
  for (let x = 0; x < WIDTH; x++) {
    for (let y = 0; y < HEIGHT; y++) {
      const initial = grid.get(state, x, y);
      const activeNeighbours = sum(
        NEIGHBOURS.map(([dx, dy]) => grid.get(state, x + dx, y + dy)),
      );

      if (initial === 1) {
        target[grid.index(x, y)] =
          activeNeighbours === 2 || activeNeighbours === 3 ? 1 : 0;
      } else {
        target[grid.index(x, y)] = activeNeighbours === 3 ? 1 : 0;
      }
    }
  }
};

for (let i = 0; i < STEPS; i++) {
  takeStep(state, target);
  [state, target] = [target, state];
}

console.log(sum(state));

state = new Uint8Array([...input.join("")].map((c) => (c === "#" ? 1 : 0)));

for (let i = 0; i < STEPS; i++) {
  state[grid.index(0, 0)] = 1;
  state[grid.index(WIDTH - 1, 0)] = 1;
  state[grid.index(WIDTH - 1, HEIGHT - 1)] = 1;
  state[grid.index(0, HEIGHT - 1)] = 1;
  takeStep(state, target);
  [state, target] = [target, state];
}

state[grid.index(0, 0)] = 1;
state[grid.index(WIDTH - 1, 0)] = 1;
state[grid.index(WIDTH - 1, HEIGHT - 1)] = 1;
state[grid.index(0, HEIGHT - 1)] = 1;

console.log(sum(state));
