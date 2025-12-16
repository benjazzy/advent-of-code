tiles = """7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"""

largest_area = 0

file = open("input.txt")

lines = file.readlines()
# lines = tiles.splitlines()

def area_between(a, b):
    x = abs(a[0] - b[0]) + 1
    y = abs(a[1] - b[1]) + 1

    return x * y

def parse_coordinate(line):
    parts = line.split(',')
    x = parts[0]
    y = parts[1]

    return int(x), int(y)

for i, coordinate in enumerate(map(parse_coordinate, lines)):
    for other_coordinate in map(parse_coordinate, lines[i  + 1:]):
        area = area_between(coordinate, other_coordinate)
        if area > largest_area:
            largest_area = area

print(largest_area)