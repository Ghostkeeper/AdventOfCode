# Advent of code 2022, by Ghostkeeper

import get_input_file
import snafu

lines = open(get_input_file.get_path(25, 1)).readlines()
lines = [line.strip() for line in lines]

decimal_sum = 0
for line in lines:
	decimal_sum += snafu.snafu_to_decimal(line)

print("The sum of fuel amounts in SNAFU numbers is:", snafu.decimal_to_snafu(decimal_sum))