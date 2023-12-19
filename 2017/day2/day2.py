def maxmin_difference(l):
    return max(l) - min(l)

def even_divisor(row):
    for i in range(len(row)):
        for j in range(i+1, len(row)):
            if row[i] % row[j] == 0:
                return row[i] // row[j]
            if row[j] % row[i] == 0:
                return row[j] // row[i]
    raise ValueError

assert even_divisor([5, 9, 2, 8]) == 4
assert even_divisor([9, 4, 7 , 3]) == 3
assert even_divisor([3, 8, 6, 5]) == 2

with open("input.dat") as infile:
    our = [[int(n) for n in l.split()] for l in infile]

print(sum(maxmin_difference(row) for row in our))
print(sum(even_divisor(row) for row in our))
