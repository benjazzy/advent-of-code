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

def find_next_roll(shelves):
    # global accessible_rolls
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
                    return row, col
                    # accessible_rolls += 1
    return None

shelves = file.readlines()
# shelves = cafeteria.splitlines()

while (coord := find_next_roll(shelves)) is not None:
    (row, col) = coord

    shelves[row] = f"{shelves[row][:col]}{'.'}{shelves[row][col + 1:]}"
    accessible_rolls += 1

print(f"All accessible rolls have been removed! We took {accessible_rolls}")