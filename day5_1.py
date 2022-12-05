# Advent of code 2022, by Ghostkeeper

import cargo_stacks
import get_input_file

input = open(get_input_file.get_path(5, 1)).readlines()
stacks = cargo_stacks.read_cargo_stacks(input)

while not input[0].startswith("move "):  # Find where the instructions start.
	del input[0]

# Execute the instructions.
for instruction in input:
	instruction = instruction.split(" ")
	how_many = int(instruction[1])
	source = int(instruction[3]) - 1
	destination = int(instruction[5]) - 1
	for _ in range(how_many):  # Repeat N times.
		item = stacks[source].pop()
		stacks[destination].append(item)

# Formulate the output.
output = ""
for stack in stacks:
	output += stack[-1]
print("The top item on each stack is:", output)