# Advent of code 2022, by Ghostkeeper

import get_input_file
import re

text = open(get_input_file.get_path(22, 1)).readlines()
instructions = text[-1].strip()
map = text[:-2]  # Remove last 2 lines from the input to get the map.
map = [line[:-1] for line in map]  # Remove newlines. Don't fully strip, since we want to reserve the spaces.

# Make the map square, to make things easier.
max_width = max([len(line) for line in map])
for i, line in enumerate(map):
	line += " " * (max_width - len(line))
	map[i] = line

# Find the start position: The first valid square on the first row.
x = 0
y = 0
while map[0][x] != ".":
	x += 1
direction = 0  # Same as score: 0 for right, 1 for down, 2 for left, 3 for up.
delta = [(1, 0), (0, 1), (-1, 0), (0, -1)]

for instruction in re.finditer(r"\d+|[LR]", instructions):
	instruction = instruction.group(0)

	if instruction == "L":
		direction = (direction - 1) % 4
	elif instruction == "R":
		direction = (direction + 1) % 4

	else:
		distance = int(instruction)
		dx, dy = delta[direction]
		for _ in range(distance):
			cy = (y + dy) % len(map)
			cx = (x + dx) % len(map[cy])  # Candidate position to move towards.
			while map[cy][cx] == " ":  # Edge of map. Wrap around until we find a part of the map again.
				cy = (cy + dy) % len(map)
				cx = (cx + dx) % len(map[cy])
			if map[cy][cx] == "#":
				break  # Stop movement here.
			else:
				x = cx
				y = cy

password = (y + 1) * 1000 + (x + 1) * 4 + direction
print("The final password is:", password)