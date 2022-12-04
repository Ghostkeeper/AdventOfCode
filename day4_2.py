# Advent of code 2022, by Ghostkeeper

import get_input_file

num_overlap = 0
for line in open(get_input_file.get_path(4, 1)):
	elves = line.split(",")
	elves = [(int(elf.split("-")[0]), int(elf.split("-")[1])) for elf in elves]
	if set(range(elves[0][0], elves[0][1] + 1)).intersection(set(range(elves[1][0], elves[1][1] + 1))):
		num_overlap += 1

print(num_overlap)