manifold = """.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."""

file = open("input.txt")
manifold = file.read()

def find_splitters(line):
    splits = []
    for i, char in enumerate(line):
        if char == ".":
            continue
        if char == "^":
            splits.append(i)

    return set(splits)

def next_beams(old_beams, splitters):
    new_beams = set()
    splits = 0
    shared_splits = 0
    for beam in old_beams:
        if beam in splitters:
            splits += 1
            left_beam = beam - 1
            right_beam = beam + 1
            if left_beam in new_beams:
                shared_splits += 1
            if right_beam in new_beams:
                shared_splits += 1

            new_beams.add(left_beam)
            new_beams.add(right_beam)
        else:
            new_beams.add(beam)

    return splits, shared_splits, new_beams

total_splits = 0
total_shared_splits = 0
# Current state of the beams as we iterate down
beams = set([manifold.splitlines()[0].index("S")])
for line in manifold.splitlines()[1:]:
    splits, shared_splits, new_beams = next_beams(beams, find_splitters(line))
    print(f"'{line}' had {splits} splits: {new_beams}")

    beams = new_beams
    total_splits += splits
    total_shared_splits += shared_splits

print(f"There were {total_splits} splits and {total_shared_splits} shared_splits")