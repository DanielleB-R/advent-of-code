def matching_sum(digits, nextindex):
    total = 0
    for i in range(len(digits)):
        next_i = nextindex(i, digits)
        if digits[i] == digits[next_i]:
            total += int(digits[i])
    return total

def circular_next(i, digits):
    return (i + 1) % len(digits)

def halfway_around(i, digits):
    l = len(digits)
    return (i + l//2) % l

assert(matching_sum("1122", circular_next) == 3)
assert(matching_sum("1111", circular_next) == 4)
assert(matching_sum("1234", circular_next) == 0)

with open("input.dat") as infile:
    our_input = infile.read().strip()

print(matching_sum(our_input, circular_next))
print(matching_sum(our_input, halfway_around))
