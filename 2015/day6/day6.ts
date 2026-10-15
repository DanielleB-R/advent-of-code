import { readLines } from "../lib/input.js";
import { sum } from "../lib/math.js";

const commands = readLines(
  /^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$/,
  ([, operation, startX, startY, endX, endY]) => ({
    operation,
    start: [parseInt(startX, 10), parseInt(startY, 10)] as const,
    end: [parseInt(endX, 10), parseInt(endY, 10)] as const,
  }),
);

type Command = (typeof commands)[0];

const HEIGHT = 1000;
const WIDTH = 1000;

const index = (x: number, y: number) => x + WIDTH * y;

type Alterations = Record<string, (x: number) => number>;

const executeCommand = (
  lights: Uint8Array,
  command: Command,
  alterations: Alterations,
) => {
  const operation = alterations[command.operation]!;

  let rowOrigin = index(command.start[0], command.start[1]);

  const height = command.end[1] - command.start[1];
  const width = command.end[0] - command.start[0];

  for (let i = 0; i <= height; i++) {
    for (let j = 0; j <= width; j++) {
      const current = rowOrigin + j;
      lights[current] = operation(lights[current]!);
    }
    rowOrigin += WIDTH;
  }
};

const lights = new Uint8Array(HEIGHT * WIDTH);
const BINARY_ALTERATION: Alterations = {
  "turn on": () => 1,
  "turn off": () => 0,
  toggle: (x) => x ^ 1,
};

commands.forEach((command) =>
  executeCommand(lights, command, BINARY_ALTERATION),
);
console.log(sum(lights));

const lights2 = new Uint8Array(HEIGHT * WIDTH);
const SPECTRUM_ALTERATION: Alterations = {
  "turn on": (x) => x + 1,
  "turn off": (x) => Math.max(x - 1, 0),
  toggle: (x) => x + 2,
};

commands.forEach((command) =>
  executeCommand(lights2, command, SPECTRUM_ALTERATION),
);

console.log(sum(lights2));
