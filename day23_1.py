# Advent of code 2022, by Ghostkeeper

import diffuse
import get_input_file

text = open(get_input_file.get_path(23, 1)).readlines()

elves = set()  # Set of (X,Y) tuples that contain an elf.
for y, line in enumerate(text):
	for x, char in enumerate(line):
		if char == "#":
			elves.add((x, y))

for i in range(10):
	elves = diffuse.diffuse(elves, i)

minx = min([elf[0] for elf in elves])
miny = min([elf[1] for elf in elves])
maxx = max([elf[0] for elf in elves])
maxy = max([elf[1] for elf in elves])
field_size = (maxx + 1 - minx) * (maxy + 1 - miny)
print("The number of empty field cells is:", field_size - len(elves))