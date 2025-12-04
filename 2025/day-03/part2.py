banks = """987654321111111
811111111111119
234234234234278
818181911112111"""

file = open("input.txt")
joltage = 0

# Returns the index of, and the value of the largest digit in a bank
def largest_battery(bank):
    largest = 0
    index_of_largest = 0
    for i, battery in enumerate(bank):
        if int(battery) > largest:
            largest = int(battery)
            index_of_largest = i

    return index_of_largest, largest

for bank in file.readlines():
# for bank in banks.splitlines():
    bank = bank.strip()
    battery = 0

    # Where to start search in the bank
    start = 0
    for num in range(0, 12):
        # Base 10 power of the current digit
        power = 11 - num

        # Index of the last digit to check + 1
        end = len(bank) - power

        i, digit = largest_battery(bank[start:end])

        start = i + start + 1
        battery += digit * (10 ** power)

    joltage += battery

    print(f"{bank}: Battery {battery}, Joltage {joltage}")