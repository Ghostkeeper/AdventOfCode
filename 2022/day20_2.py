# Advent of code 2022, by Ghostkeeper

import decrypt_mix
import get_input_file

numbers = open(get_input_file.get_path(20, 1)).readlines()
numbers = [int(number.strip()) for number in numbers]

# Apply the decryption key.
decryption_key = 811589153
numbers = [number * decryption_key for number in numbers]
numbers = decrypt_mix.mix(numbers, 10)

start = numbers.index(0)
coordinate_sum = numbers[(start + 1000) % len(numbers)] + numbers[(start + 2000) % len(numbers)] + numbers[(start + 3000) % len(numbers)]
print("The sum of the grove coordinates is:", coordinate_sum)