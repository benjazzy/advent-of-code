#EXAMPLE
ranges = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"""

split_ranges = ranges.split(',')
sum = 0

file = open("input.txt")

# Iterate through each set of numbers
for ranch in file.read().split(','):
# for ranch in split_ranges:
    split_ranch = ranch.split('-')

    for num in range(int(split_ranch[0]), int(split_ranch[1]) + 1):
        # Check if is valid
        num = str(num)

        # Skip any numbers that are an odd number of digits
        if len(num) %  2 != 0:
            continue

        first_half = num[int(len(num) / 2):]
        second_half = num[:int(len(num) / 2)]

        if first_half == second_half:
            print(f"Found invalid id {num}")
            sum += int(num)

print(f"Total is {sum}")