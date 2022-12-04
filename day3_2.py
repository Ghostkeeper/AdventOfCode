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
lines = open(get_input_file.get_path(3, 1)).readlines()
for i in range(int(len(lines) / 3)):
	group = lines[i * 3:i * 3 + 3]
	group = [set(x.strip()) for x in group]
	common = group[0].intersection(group[1]).intersection(group[2])
	assert len(common) == 1
	priority_sum += priority(common.pop())

print("The sum of the priorities of all group badge items is:", priority_sum)