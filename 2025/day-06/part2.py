sample = """
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  """

# to_cephalopod(["123", " 45", "  6"]) -> [356, 24, 1]

print("""                        _,--._
                      ,'      `.
              |\     / ,-.  ,-. \     /|
              )o),/ ( ( o )( o ) ) \.(o(
             /o/// /|  `-'  `-'  |\ \\\\\\o\\
            / / |\ \(   .    ,   )/ /| \ \\
            | | \o`-/    `\/'    \-'o/ | |
            \ \  `,'              `.'  / /
         \.  \ `-'  ,'|   /\   |`.  `-' /  ,/
          \`. `.__,' /   /  \   \ `.__,' ,'/
           \o\     ,'  ,'    `.  `.     /o/
            \o`---'  ,'        `.  `---'o/
             `.____,'            `.____,'
""")

def convert_to_vert(numbers):
    vert_numbers = []
    for char_index in range(len(numbers)):
        value = 0
        for num in numbers:
            digit = num[char_index]
            if digit.isdigit():
                value *= 10
                value += int(digit)
        vert_numbers.append(value)
    return vert_numbers

print(convert_to_vert(["123", " 45", "  6"]))

# "64 "
# "640"
# column_widths = []
# for each row sample:
#   for each index and num in enumerate(row):
#       if len(num) > colum_width[index]
#           colum_widths[index] = len(num)

file = open("input.txt")
# lines = file.readlines()
lines = sample.splitlines()

# Create a list of only symbols for ease of access later.
operations = lines[-1].split()
# operations = [symbol for symbol in lines()[-1].split()]
print(operations)

column_widths = [len(num) for num in lines[0].split()]
for row in lines[1:-1]:
    for i, num in enumerate(row.split()):
        if len(num) > column_widths[i]:
            column_widths[i] = len(num)

print(column_widths)

# Start our totals list out with just the values from the first row
# This is the issue with starting with zero when multiplying
totals = list(
    map(int, lines[0].split())
)

print(totals)

for line in lines[1:-1]:
    for i, num in enumerate(map(int, line.split())):
        stored = totals[i]
        op = operations[i]
        if op == '+':
            totals[i] = stored + num
        else:
            totals[i] = stored * num

answer = 0
for total in totals:
    answer += total

print(answer)

# my_list = ["hello", "world", "and", "jim"]
# mylist[1] = "world"
# mylist[1][2] = "r"
# mylist[1:3] = ["world", "and"]

# for word in mylist:
#   word = "hello"
#   word = "world"
#   word = "and"
#   word = "jim"