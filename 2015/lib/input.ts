import { readFileSync } from "node:fs";

export const readLines = <T>(regexp: RegExp, lineMapper: (match: RegExpExecArray) => T): T[] => {
  const input = readFileSync("input.txt", "utf8").trim().split("\n");

  const result: T[] = [];

  for (const line of input) {
    const match = regexp.exec(line);

    if (!match) {
      throw new Error(`Invalid input ${line}`);
    }

    result.push(lineMapper(match));
  }
  
  return result;
}
