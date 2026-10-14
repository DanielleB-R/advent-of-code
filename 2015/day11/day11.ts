import { readFileSync } from "node:fs";

const input = readFileSync("input.txt", "utf8").trim();

type Password = number[];

const charIndex = (char: string): number =>
  char.charCodeAt(0) - "a".charCodeAt(0);

const readIn = (input: string): Password => {
  const password: Password = [];
  for (const char of input) {
    password.push(charIndex(char));
  }

  password.reverse();

  return password;
};

const FORBIDDEN_LETTERS = ["i", "o", "l"].map(charIndex);

const increment = (password: Password) => {
  for (let i = 0; i < password.length; i++) {
    password[i] += 1;
    if (FORBIDDEN_LETTERS.includes(password[i])) {
      password[i] += 1;
    }
    if (password[i] >= 26) {
      password[i] = 0;
    } else {
      break;
    }
  }
};

const hasRun = (password: Password): boolean => {
  for (let i = 0; i < password.length - 2; i++) {
    if (
      password[i] === password[i + 1] + 1 &&
      password[i + 1] === password[i + 2] + 1
    ) {
      return true;
    }
  }
  return false;
};

const hasPairs = (password: Password): boolean => {
  let pairsSeen = 0;
  for (let i = 0; i < password.length - 1; i++) {
    if (password[i] === password[i + 1]) {
      pairsSeen += 1;
      i += 1;
    }
  }
  return pairsSeen >= 2;
};

const nextPassword = (password: Password): Password => {
  const newPassword = [...password];
  while (true) {
    increment(newPassword);
    if (hasPairs(newPassword) && hasRun(newPassword)) {
      return newPassword;
    }
  }
};

const writeOut = (password: Password): string => {
  password.reverse();

  const chars = password.map((char) =>
    String.fromCharCode(char + "a".charCodeAt(0)),
  );

  return chars.join("");
};

const password = readIn(input);

const newPassword = nextPassword(password);

const output = writeOut(newPassword);

console.log(output);

console.log(writeOut(nextPassword(readIn(output))));
