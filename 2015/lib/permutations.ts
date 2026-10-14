export const permutationsOf = (keys: Set<string>): string[][] => {
  if (keys.size === 0) {
    return [];
  }

  if (keys.size === 1) {
    return [Array.from(keys)];
  }

  const permutations = [];

  for (const key of keys) {
    const keyPermutations = permutationsOf(keys.difference(new Set([key])));

    for (const keyPermutation of keyPermutations) {
      permutations.push([key, ...keyPermutation]);
    }
  }

  return permutations;
};
