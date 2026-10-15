interface NumericReducer {
  reduce(callback: (acc: number, val: number) => number, init: number): number;
}

export const sum = (numbers: NumericReducer): number =>
  numbers.reduce((acc, val) => acc + val, 0);

export const max = (numbers: NumericReducer): number =>
  numbers.reduce((acc, val) => (val > acc ? val : acc), -Infinity);

export const min = (numbers: NumericReducer): number =>
  numbers.reduce((acc, val) => (val < acc ? val : acc), Infinity);
