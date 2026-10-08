import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

type Operation = (x: number, y: number) => number;
type Gate = [Operation, string | number, string | number];

type Signal = number | string | Gate;

const OPERATIONS: Record<string, Operation> = {
  AND: (x, y) => x & y,
  OR: (x, y) => x | y,
  XOR: (x, y) => (x ^ y) & 0xffff,
  LSHIFT: (x, y) => (x << y) & 0xffff,
  RSHIFT: (x, y) => x >> y,
};

const GENERAL_REGEX = /^(.*) -> (\w+)$/;

const NUMBER_REGEX = /^\d+$/;
const BINARY_REGEX = /^(.*) (AND|OR|LSHIFT|RSHIFT) (.*)$/;
const NOT_REGEX = /^NOT (.*)$/;

const process_source = (input: string): string | number => {
  return NUMBER_REGEX.test(input) ? parseInt(input, 10) : input;
};

const signals: Record<string, Signal> = {};

for (const gate of input) {
  const generalMatch = GENERAL_REGEX.exec(gate);

  if (!generalMatch) {
    throw new Error(`Invalid gate ${gate}`);
  }

  const source = generalMatch[1]!;
  const dest = generalMatch[2]!;

  const numberMatch = NUMBER_REGEX.exec(source);

  if (numberMatch) {
    signals[dest] = parseInt(numberMatch[0], 10);
    continue;
  }

  const not_match = NOT_REGEX.exec(source);
  if (not_match) {
    signals[dest] = [OPERATIONS.XOR!, not_match[1]!, 0xffff];
    continue;
  }

  const binary_match = BINARY_REGEX.exec(source);
  if (binary_match) {
    const operation = OPERATIONS[binary_match[2]!]!;
    const src1 = process_source(binary_match[1]!);
    const src2 = process_source(binary_match[3]!);

    signals[dest] = [operation, src1, src2];
    continue;
  }

  signals[dest] = source;
}

const signals2 = { ...signals };

const resolveSignals = (s: Record<string, Signal>) => {
  const resolveSignal = (value: string | number): number | null => {
    if (typeof value === "number") {
      return value;
    }

    if (typeof s[value] === "number") {
      return s[value];
    }

    return null;
  };

  while (Object.values(s).some((sig) => typeof sig !== "number")) {
    for (const signal of Object.keys(s)) {
      const signalValue = s[signal]!;

      if (typeof signalValue === "number") {
        continue;
      }

      if (typeof signalValue === "string") {
        const value = resolveSignal(signalValue);
        if (value !== null) {
          s[signal] = value;
        }
        continue;
      }

      let [op, src1, src2] = signalValue;

      const src1Value = resolveSignal(src1);
      const src2Value = resolveSignal(src2);

      if (src1Value === null || src2Value === null) {
        continue;
      }

      s[signal] = op(src1Value, src2Value);
    }
  }
};

resolveSignals(signals);

console.log(signals["a"]);

console.log(signals["b"], signals2["b"]);

signals2["b"] = signals["a"]!;

console.log(signals["b"], signals2["b"], signals2["a"], signals2["lx"]);

resolveSignals(signals2);

console.log(signals2["a"]);
