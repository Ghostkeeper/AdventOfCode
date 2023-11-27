# Advent of code 2022, by Ghostkeeper

import get_input_file

def priority(char):
	"""
	Get the priority of a certain item, represented by a character.
	:param char: The item to get the priority of.
	:return: The numeric priority of that item.
	"""
	numeric = ord(char)
	if numeric > ord("Z"):
		return numeric - ord("a") + 1
	else:
		return numeric - ord("A") + 27

priority_sum = 0
for line in open(get_input_file.get_path(3, 1)):
	line = line.strip()  # Remove newline.
	first_compartment = line[:int(len(line) / 2)]
	second_compartment = line[int(len(line) / 2):]

	prev = None
	for char in first_compartment:
		if char in second_compartment:
			priority_sum += priority(char)
			break  # Don't want to find duplicates. The wrong item might be in the first compartment.

print("The total priority is", priority_sum)