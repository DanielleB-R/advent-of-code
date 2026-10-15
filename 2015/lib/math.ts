export const sum = (numbers: number[]): number => numbers.reduce((acc, val) => acc + val);

export const max = (numbers: number[]): number => numbers.reduce((acc, val) => val > acc ? val : acc, -Infinity);

export const min = (numbers: number[]): number => numbers.reduce((acc, val) => val < acc ? val : acc, Infinity);
