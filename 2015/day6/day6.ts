import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const HEIGHT = 1000;
const WIDTH = 1000;

const index = (x: number, y: number) => x + WIDTH * y;

type Alterations = Record<string, (x: number) => number>;

const COMMAND_REGEX =
  /^(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)$/;

const executeCommand = (
  lights: Uint8Array,
  command: string,
  alterations: Alterations,
) => {
  const match = COMMAND_REGEX.exec(command);

  if (!match) {
    throw new Error("Bad command ${command}");
  }

  const start_x = parseInt(match[2]!, 10);
  const start_y = parseInt(match[3]!, 10);

  const stop_x = parseInt(match[4]!, 10);
  const stop_y = parseInt(match[5]!, 10);

  const operation = alterations[match[1]!]!;
  let row_origin = index(start_x, start_y);
  for (let i = 0; i < stop_y - start_y + 1; i++) {
    for (let j = 0; j < stop_x - start_x + 1; j++) {
      const current = row_origin + j;
      lights[current] = operation(lights[current]!);
    }
    row_origin += WIDTH;
  }
};

const lights = new Uint8Array(HEIGHT * WIDTH);
const BINARY_ALTERATION: Alterations = {
  "turn on": () => 1,
  "turn off": () => 0,
  toggle: (x) => x ^ 1,
};

for (const command of input) {
  executeCommand(lights, command, BINARY_ALTERATION);
}

console.log(lights.reduce((sum, x) => sum + x, 0));

const lights2 = new Uint8Array(HEIGHT * WIDTH);
const SPECTRUM_ALTERATION: Alterations = {
  "turn on": (x) => x + 1,
  "turn off": (x) => Math.max(x - 1, 0),
  toggle: (x) => x + 2,
};

for (const command of input) {
  executeCommand(lights2, command, SPECTRUM_ALTERATION);
}

console.log(lights2.reduce((sum, x) => sum + x, 0));
