import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

class Position {
  row: number;
  column: number;

  constructor() {
    this.row = 0;
    this.column = 0;
  }

  north() {
    this.column += 1;
  }

  south() {
    this.column -= 1;
  }

  east() {
    this.row += 1;
  }

  west() {
    this.row -= 1;
  }

  point() {
    return `${this.column}:${this.row}`;
  }
}

const regular = new Position();
const robo = new Position();

let current = regular;

const seen = new Set<string>(["0:0"]);

for (const inputChar of input) {
  switch (inputChar) {
    case "^":
      current.north();
      break;
    case "v":
      current.south();
      break;
    case ">":
      current.east();
      break;
    case "<":
      current.west();
      break;
    default:
      throw new Error(`Unexpected char ${inputChar}`);
  }
  seen.add(current.point());

  current = current === regular ? robo : regular;
}

console.log(seen.size);
