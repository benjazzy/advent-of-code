banks = """987654321111111
811111111111119
234234234234278
818181911112111"""

file = open("input.txt")
joltage = 0

for bank in file.readlines():
# for bank in banks.splitlines():
    bank = bank.strip()
    largest = 0
    index_of_largest = 0
    for i, battery in enumerate(bank[:-1]):
        if int(battery) > largest:
            largest = int(battery)
            index_of_largest = i

    second_largest = 0
    for battery in bank[index_of_largest + 1:]:
        if second_largest < int(battery):
            second_largest = int(battery)

    battery = (largest * 10) + second_largest
    joltage += battery

    print(f"{bank}: Battery {battery}, Sum {joltage}")


# Split the input into banks
# For each bank find the highest digit that is not the last
# That digit will be the first digit of the bank joltage
# Then find the highest digit after our first digit
# That will be the second digit
# Add that joltage to a total and process the next