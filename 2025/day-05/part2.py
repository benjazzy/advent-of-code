sample = """2-6
3-5

1
5
8
11
17
32"""

file = open("input.txt")
real = file.read()
# split_input = real.split("\n\n")
split_input = sample.split("\n\n")
ranges = split_input[0]

fresh = 0

unique_ranges = []
for r in ranges.splitlines():
    split_range = r.split('-')
    start, end = int(split_range[0]), int(split_range[1])
    for i, (other_start, other_end) in enumerate(unique_ranges):
        if start > other_end or end < other_start:
            continue
        elif start < other_start and end <= other_end:
            end = other_start - 1
        elif start >= other_start and end > other_end:
            start = other_end + 1
        elif start <= other_start and end >= other_end:
            del unique_ranges[i]
        elif start >= other_start and end <= other_end:
            break
    else:
        unique_ranges.append((start, end))

for start, end in set(unique_ranges):
    size = (end - start) + 1
    fresh += size
print(set(unique_ranges))
print(fresh)
# print(ranges.splitlines(), "\n", set(ranges.splitlines()))

    # ranges_list.append((start, end))
    # for id in range(start, end + 1):
    #     id_list.add(id)

# start > other_end || end < other_start: (1, 2), (3, 4)
# start < other_start && end < other_end: (1, 3), (2, 4)
# start > other_start && end > other_end: (2, 4), (1, 3)
# start < other_start && end > other_end: (1, 4), (2, 3)
# start > other_start && end < other_end: (2, 3), (1, 4)
# else append start-end

