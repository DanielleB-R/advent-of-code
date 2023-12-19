from collections import Counter

RIGHT = 0
UP = 1
LEFT = 2
DOWN = 3

def turn(direction):
    return (direction + 1) & 0x03

def spiral_seq():
    current = 1
    x = 0
    y = 0
    direction = RIGHT
    side_length = 1
    side_count = 0
    second_side = False
    yield (x, y)

    while True:
        current += 1
        # first advance
        if direction == RIGHT:
            x += 1
        elif direction == UP:
            y += 1
        elif direction == LEFT:
            x -= 1
        elif direction == DOWN:
            y -= 1

        # then adjust the direction
        side_count += 1
        if side_count == side_length:
            direction = turn(direction)
            if second_side:
                second_side = False
                side_length += 1
            else:
                second_side = True
            side_count = 0

        yield (x, y)


def spiral_coords(n):
    gen = spiral_seq()
    [_ for _ in range(n-1) if next(gen) and False]
    return next(gen)


def taxi_distance(n):
    x, y = spiral_coords(n)
    return abs(x) + abs(y)


assert taxi_distance(1) == 0
assert taxi_distance(12) == 3
assert taxi_distance(23) == 2
assert taxi_distance(1024) == 31

print(taxi_distance(265149))

def environment(x, y):
    return [(x+1, y), (x+1, y+1), (x, y+1), (x-1, y+1), (x-1, y), (x-1, y-1), (x, y-1), (x+1, y-1)]

def spiral_store(max_value):
    coord_map = Counter()

    for x, y in spiral_seq():
        total = sum(coord_map[coord] for coord in environment(x, y))
        if total == 0:
            total = 1

        coord_map[(x, y)] = total

        if total > max_value:
            return total


print(spiral_store(265149))
