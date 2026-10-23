import { readFileSync } from "node:fs";

type LineMapper<T> = (match: RegExpExecArray) => T;

const scanLines = <T>(
  regexp: RegExp,
  lineMapper: LineMapper<T>,
  lines: string[],
): T[] => {
  const result: T[] = [];

  for (const line of lines) {
    const match = regexp.exec(line);

    if (!match) {
      throw new Error(`Invalid input ${line}`);
    }

    result.push(lineMapper(match));
  }

  return result;
};

export const readLines = <T>(
  regexp: RegExp,
  lineMapper: LineMapper<T>,
): T[] => {
  const input = readFileSync("input.txt", "utf8").trim().split("\n");

  return scanLines(regexp, lineMapper, input);
};

export const readWithCoda = <T>(
  regexp: RegExp,
  lineMapper: LineMapper<T>,
): [T[], string] => {
  const input = readFileSync("input.txt", "utf8").trim().split("\n\n");

  const lines = scanLines(regexp, lineMapper, input[0].split("\n"));
  return [lines, input[1]];
};
