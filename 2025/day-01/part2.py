import math

sample_input = """L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"""

position = 50
count = 0

f = open("input1.txt")

# for x in f.readlines():
for x in sample_input.splitlines():
    value = 0

    if 'R' in x:
        #addition
        position += int(x[1:])
        count += math.floor(position / 100)
    elif 'L' in x:
        #subtraction
        value = int(x[1:])
        count += math.floor(value / 100)
        if value % 100 > position != 0:
            count += 1
        position += 100 - value
        if position % 100 == 0:
            count += 1

    position %= 100

    print(f"Command: {x}, Value: {value}, Position: {position}, Count: {count}")
# print(f"{sample_input}")