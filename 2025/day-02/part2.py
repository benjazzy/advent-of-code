#EXAMPLE
ranges = """11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124"""

split_ranges = ranges.split(',')
sum = 0

file = open("input.txt")

def check_number(num):
    global sum
    num = str(num)

    for character_len in range(1, int(len(num) / 2) + 1):
        # The number if chunks that match the first chunk
        num_of_matches = 1
        patterns = range(character_len, len(num), character_len)
        for pattern_num in patterns:
            first_chunk = num[:character_len]
            chunk_to_check = num[pattern_num:character_len + pattern_num]
            if first_chunk == chunk_to_check:
                num_of_matches += 1

        # If num is made up of all identical chunks then it is an invalid id.
        if num_of_matches * character_len == len(num):
            print(f"Found invalid id {num}")
            sum += int(num)
            return

for ranch in file.read().split(','):
# for ranch in split_ranges:
    split_ranch = ranch.split('-')
    for num in range(int(split_ranch[0]), int(split_ranch[1]) + 1):
        check_number(num)

print(f"Total is {sum}")

#largest
#sedond largest

#12631