import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim().split("\n");

const AUTO_NAUGHTY_SUBSTRINGS = ["ab", "cd", "pq", "xy"];
const THREE_VOWEL_REGEX = /[aeiou].*[aeiou].*[aeiou]/;
const DOUBLE_LETTER_REGEX = /(.)\1/;

const nice = input
  .filter((str) => !AUTO_NAUGHTY_SUBSTRINGS.some((ss) => str.includes(ss)))
  .filter((str) => THREE_VOWEL_REGEX.test(str))
  .filter((str) => DOUBLE_LETTER_REGEX.test(str));

console.log(nice.length);

const DOUBLED_DOUBLE_REGEX = /(..).*\1/;
const DOUBLED_WITH_INTERLOPER_REGEX = /(.).\1/;

const nice2 = input
  .filter((str) => DOUBLED_DOUBLE_REGEX.test(str))
  .filter((str) => DOUBLED_WITH_INTERLOPER_REGEX.test(str));

console.log(nice2.length);
