def follow_jumps(offsets):
    steps = 0
    pc = 0
    while pc >= 0 and pc < len(offsets):
        steps += 1
        increment = offsets[pc]
        offsets[pc] += 1 if increment < 3 else -1
        pc += increment
    return steps

assert follow_jumps([0, 3, 0, 1, -3]) == 10

with open("input.dat") as infile:
    offsets = [int(l) for l in infile]

print(follow_jumps(offsets))
