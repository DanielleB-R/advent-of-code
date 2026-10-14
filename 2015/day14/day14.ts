import { readLines } from "../lib/input.js";

class Reindeer {
  readonly name: string;
  readonly speed: number;
  readonly goTime: number;
  readonly restTime: number;
  distance = 0;
  mode: "go" | "rest" = "go";
  modeTime = 0;

  constructor(name: string, speed: number, goTime: number, restTime: number) {
    this.name = name;
    this.speed = speed;
    this.goTime = goTime;
    this.restTime = restTime;
  }

  tick() {
    this.modeTime++;
    if (this.mode === "go") {
      this.distance += this.speed;
      if (this.modeTime === this.goTime) {
        this.mode = "rest";
        this.modeTime = 0;
      }
    } else {
      if (this.modeTime === this.restTime) {
        this.mode = "go";
        this.modeTime = 0;
      }
    }
  }
}

const reindeer = readLines(
  /^(\w+) can fly (\d+) km\/s for (\d+) seconds, but then must rest for (\d+) seconds.$/,
  ([_, name, speed, goTime, restTime]) => {
    return new Reindeer(
      name,
      parseInt(speed, 10),
      parseInt(goTime, 10),
      parseInt(restTime, 10),
    );
  },
);

const RACE_LENGTH = 2503;

const scores = Object.fromEntries(reindeer.map(({ name }) => [name, 0]));

for (let i = 0; i < RACE_LENGTH; i++) {
  reindeer.forEach((reindeer) => reindeer.tick());
  const winningDistance = Math.max(...reindeer.map(({ distance }) => distance));
  reindeer.forEach(({ name, distance }) => {
    if (distance === winningDistance) {
      scores[name] += 1;
    }
  });
}

console.log(Math.max(...reindeer.map(({ distance }) => distance)));

console.log(Math.max(...Object.values(scores)));
