sample = """123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  """

file = open("input.txt")
lines = file.readlines()
# lines = sample.splitlines()

# Create a list of only symbols for ease of access later.
operations = lines[-1].split()
# operations = [symbol for symbol in lines()[-1].split()]
print(operations)

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