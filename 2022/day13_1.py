# Advent of code 2022, by Ghostkeeper

import compare_lists
import get_input_file

input = open(get_input_file.get_path(13, 1)).read()
pairs = input.split("\n\n")
pairs = [pair.split("\n") for pair in pairs]
pairs = [[eval(item) for item in pair] for pair in pairs]

sum_indices = 0
for i in range(len(pairs)):
	pair = pairs[i]
	result = compare_lists.compare(pair[0], pair[1])
	if result == -1:
		sum_indices += i + 1

print("The sum of the indices of proper-order pairs is:", sum_indices)