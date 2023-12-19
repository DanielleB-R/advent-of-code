def find_biggest(banks):
    max_val = 0
    max_index = -1
    for i, size in enumerate(banks):
        if size > max_val:
            max_val = size
            max_index = i
    return max_index


def length_of_cycle(banks):
    steps = 0
    seen = set()

    while tuple(banks) not in seen:
        seen.add(tuple(banks))
        index = find_biggest(banks)
        amount = banks[index]
        banks[index] = 0
        for _ in range(amount):
            index += 1
            if index >= len(banks):
                index = 0
            banks[index] += 1
        steps += 1
    return steps


assert length_of_cycle([0, 2, 7, 0]) == 5

with open("input.dat") as infile:
    banks = [int(b) for b in infile.read().split()]
print(length_of_cycle(banks))
print(length_of_cycle(banks))
