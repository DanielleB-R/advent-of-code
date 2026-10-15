import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

const MOVES: Record<string, [number, number]> = {
  "^": [0, 1],
  v: [0, -1],
  ">": [1, 0],
  "<": [-1, 0],
};

class Position {
  x = 0;
  y = 0;

  move(input: string) {
    const move = MOVES[input];
    if (!move) {
      throw new Error(`Unexpected move ${move}`);
    }
    this.x += move[0];
    this.y += move[1];
  }

  point() {
    return `${this.x}:${this.y}`;
  }
}

const santa = new Position();
const seen = new Set<string>(["0:0"]);

for (const inputChar of input) {
  santa.move(inputChar);
  seen.add(santa.point());
}

console.log(seen.size);

const regular = new Position();
const robo = new Position();

let current = regular;

const seen2 = new Set<string>(["0:0"]);

for (const inputChar of input) {
  current.move(inputChar);
  seen2.add(current.point());

  current = current === regular ? robo : regular;
}

console.log(seen2.size);
