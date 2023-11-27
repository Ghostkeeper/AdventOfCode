# Advent of code 2022, by Ghostkeeper

def read_cargo_stacks(input):
	"""
	From the start of the input, read the cargo on the cargo stacks.
	:param input: A puzzle input, starting with cargo stacks.
	:return: A number of stacks, each a list with items in it (of a single character).
	"""
	num_stacks = int(len(input[0]) / 4)  # One space and 3 characters per stack, so divide by 4.
	stacks = [[] for _ in range(num_stacks)]  # Create empty stacks.

	for line in input:
		if line[0] == " ":  # End of stack input data.
			break
		for i in range(num_stacks):
			item = line[i * 4 + 1]
			if item == " ":  # Empty, the stack isn't this high.
				continue
			stacks[i].insert(0, item)  # Put at bottom of stack.
	return stacks