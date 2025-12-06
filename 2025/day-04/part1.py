from argparse import BooleanOptionalAction

cafeteria = """..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."""

accessible_rolls = 0
file = open("input.txt")

def adjacent_cells(row, col):
    # Left and Right
    yield row, col - 1
    yield row, col + 1

    # Row below
    yield row + 1, col + 1
    yield row + 1, col
    yield row + 1, col - 1

    # Row above
    yield row - 1, col + 1
    yield row - 1, col
    yield row - 1, col - 1

def is_paper(paper):
    if paper == '@':
        return True
    else:
        return False

# shelves = cafeteria.splitlines()
shelves = file.readlines()
mask = shelves.copy()
for row in range(0, len(shelves)):
    for col in range(0, len(shelves[row])):
        paper = shelves[row][col]
        if is_paper(paper):
            surrounding_rolls = 0
            for r, c in adjacent_cells(row, col):
                if r < 0 or c < 0:
                    continue
                try:
                    if shelves[r][c] == '@':
                        surrounding_rolls += 1
                except IndexError:
                    continue

            if surrounding_rolls < 4:
                mask[row] = f"{mask[row][:col]}{surrounding_rolls}{mask[row][col + 1:]}"
                accessible_rolls += 1

print("\n".join(mask))
print(accessible_rolls)

# 1) Create shelves as a 2d array and iterate over cells by its column and row position
# 2) If the cell is not filled with paper then go to the next cell
# 3) Form a 3x3 with the selected paper roll in the center.
# 4) Check the surrounding space for other paper rolls.
# 5) If there are less than 4 paper rolls mark as valid, move to the next cell and repeat step 2

# ooo row - 1: col - 1 to col + 1
# oxo row    : col - 1 and col + 1
# ooo row + 1: col - 1 to col + 1