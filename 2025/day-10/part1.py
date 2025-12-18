sample = """[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}"""

file = open("ainput.txt")
sample = file.readlines()
# sample = sample.splitlines()

def get_indicators(line):
    lights = line.split(']')[0]
    list_new = []
    list_old = []

    for light in lights[1:]:
        list_new.append(False)
        list_old.append(light == '#')

    return list_new, list_old

def get_buttons(line):
    buttons = line.split('] ')[1]
    buttons = buttons.split(' {')[0]

    button_list = []
    for button in buttons.split():
        toggles = []
        for toggle in button[1:-1].split(','):
            toggles.append(int(toggle))
        button_list.append(toggles)

    return button_list

def press_buttons(indicator, buttons):
    new_indicators = []
    for button in buttons:
        new_indicator = indicator.copy()
        for toggle in button:
            new_indicator[toggle] = not new_indicator[toggle]
        new_indicators.append(new_indicator)
    return new_indicators

def solve_machine(machine):
    start, target = get_indicators(machine)
    indicators = [start]
    buttons = get_buttons(machine)
    press_count = 0
    while target not in indicators:
        press_count += 1
        new_indicators = []
        for indicator in indicators:
            new_indicators += press_buttons(indicator, buttons)
        indicators = new_indicators
    return press_count

least_press = 0

for i, line in enumerate(sample):
    presses = solve_machine(line)
    least_press += presses
    print(f"{i}/{len(sample)}: {presses}")

print(least_press)