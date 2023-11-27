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
	while how_many > 0:
		item = stacks[source][-how_many]  # Pick from the middle of the stack somewhere, N items down.
		del stacks[source][-how_many]  # And remove the same item.
		stacks[destination].append(item)
		how_many -= 1  # Move the next item.

# Formulate the output.
output = ""
for stack in stacks:
	output += stack[-1]
print("The top item on each stack is:", output)