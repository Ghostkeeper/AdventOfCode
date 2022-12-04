# Advent of code 2022, by Ghostkeeper

import get_input_file

num_overlap = 0
for line in open(get_input_file.get_path(4, 1)):
	elves = line.split(",")
	elves = [(int(elf.split("-")[0]), int(elf.split("-")[1])) for elf in elves]
	elves = [set(range(elf[0], elf[1] + 1)) for elf in elves]
	if elves[0] & elves[1]:
		num_overlap += 1

print(num_overlap)