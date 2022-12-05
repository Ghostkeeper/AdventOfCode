# Advent of code 2022, by Ghostkeeper

import cargo_stacks
import get_input_file

input = open(get_input_file.get_path(5, 1)).readlines()
stacks = cargo_stacks.read_cargo_stacks(input)

for stack in stacks:
	print(stack)