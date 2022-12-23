# Advent of code 2022, by Ghostkeeper

import diffuse
import get_input_file

text = open(get_input_file.get_path(23, 1)).readlines()

elves = set()  # Set of (X,Y) tuples that contain an elf.
for y, line in enumerate(text):
	for x, char in enumerate(line):
		if char == "#":
			elves.add((x, y))

rounds = 0
while True:
	new_elves = diffuse.diffuse(elves, rounds)
	rounds += 1
	if new_elves == elves:
		break  # No elf moved. Stop here!
	else:
		elves = new_elves

print("The first round where no elf moves is:", rounds)