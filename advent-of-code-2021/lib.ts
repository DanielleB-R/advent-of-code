import * as fs from "fs";

export const readLines = (filename: string): string[] =>
  fs.readFileSync(filename, "utf8").trim().split("\n");

export const readAndParseLines = <T>(
  filename: string,
  parser: (line: string) => T
): T[] => readLines(filename).map(parser);
