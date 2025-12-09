sample = """3-5
10-14
16-20
12-18

1
5
8
11
17
32"""

fresh = 0

file = open("input.txt")
real = file.read()
split_input = real.split("\n\n")
# split_input = sample.split("\n\n")
ranges, ids = split_input[0], split_input[1]

ranges_list = []
for r in ranges.splitlines():
    print(r)
    split_range = r.split('-')
    start, end = int(split_range[0]), int(split_range[1])
    ranges_list.append((start, end))

for id in map(int, ids.splitlines()):
    for (start, end) in ranges_list:
        if start <= id <= end:
            print(f"{id} is in {start}-{end} AND IS FRESH!")
            fresh += 1
            break

print(fresh)