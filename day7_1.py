# Advent of code 2022, by Ghostkeeper

import filetree
import get_input_file

terminal_output = open(get_input_file.get_path(7, 1)).read()
tree = filetree.parse_terminal(terminal_output)

def sum_small_dirs(subtree):
	sum = 0
	here_sum = 0
	for name, value in subtree.items():
		if name == "..":
			continue  # Don't go up.
		if type(value) is int:
			here_sum += value
		else:
			here_sum += filetree.get_size(value)
			sum += sum_small_dirs(value)
	if here_sum <= 100000:
		sum += here_sum
	return sum

print("The sum of all small directory sizes is:", sum_small_dirs(tree))