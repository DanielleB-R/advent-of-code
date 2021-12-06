import * as fs from "fs";

const increasesOf = (input: number[]): number => {
  let increases = 0;
  for (let i = 1; i < input.length; i++) {
    if (input[i] > input[i - 1]) {
      increases++;
    }
  }
  return increases;
};

const measurements = fs
  .readFileSync("input1.dat", "utf8")
  .split("\n")
  .map((s: string) => parseInt(s, 10));

console.log(increasesOf(measurements));

// NOTE: we run off the end of the array and don't worry because
// undefined adds as 0 so we don't see additional increases.
const windowSums = measurements.map(
  (_, i) => measurements[i] + measurements[i + 1] + measurements[i + 2]
);

console.log(increasesOf(windowSums));
