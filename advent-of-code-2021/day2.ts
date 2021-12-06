import * as fs from "fs";
import * as z from "myzod";
import { readAndParseLines } from "./lib";

enum Instruction {
  Forward = "forward",
  Down = "down",
  Up = "up",
}

const Command = z.object({
  inst: z.enum(Instruction),
  arg: z.number(),
});

type Command = z.Infer<typeof Command>;

const commands = readAndParseLines("input2.dat", (line: string): Command => {
  const pieces = line.split(" ");
  return Command.parse({
    inst: pieces[0],
    arg: parseInt(pieces[1], 10),
  });
});

type State = {
  readonly horizontal: number;
  readonly depth: number;
};

const finalState = commands.reduce(
  (state: State, { inst, arg }: Command): State => {
    switch (inst) {
      case Instruction.Forward:
        return { ...state, horizontal: state.horizontal + arg };
      case Instruction.Down:
        return { ...state, depth: state.depth + arg };
      case Instruction.Up:
        return { ...state, depth: state.depth - arg };
    }
  },
  { horizontal: 0, depth: 0 }
);

console.log(finalState, finalState.horizontal * finalState.depth);

type State2 = State & {
  readonly aim: number;
};

const finalState2 = commands.reduce(
  (state: State2, { inst, arg }: Command): State2 => {
    switch (inst) {
      case Instruction.Forward:
        return {
          ...state,
          horizontal: state.horizontal + arg,
          depth: state.depth + state.aim * arg,
        };
      case Instruction.Down:
        return { ...state, aim: state.aim + arg };
      case Instruction.Up:
        return { ...state, aim: state.aim - arg };
    }
  },
  { horizontal: 0, depth: 0, aim: 0 }
);

console.log(finalState2, finalState2.horizontal * finalState2.depth);
