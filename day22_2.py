# Advent of code 2022, by Ghostkeeper

import get_input_file
import re

text = open(get_input_file.get_path(22, 1)).readlines()
instructions = text[-1].strip()
map = text[:-2]  # Remove last 2 lines from the input to get the map.
map = [line[:-1] for line in map]  # Remove newlines. Don't fully strip, since we want to reserve the spaces.

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
		for _ in range(distance):
			# Check the 14 possible edges of the map to tell if we exceed them and where to go if we do.
			if 50 <= x < 100 and y == 0 and direction == 3:
				cx = 0
				cy = x - 50 + 150
				cd = 0
			elif 100 <= x < 150 and y == 0 and direction == 3:
				cx = x - 100
				cy = 199
				cd = 3
			elif x == 149 and 0 <= y < 50 and direction == 0:
				cx = 99
				cy = 149 - y
				cd = 2
			elif 100 <= x < 150 and y == 49 and direction == 1:
				cx = 99
				cy = x - 50
				cd = 2
			elif x == 99 and 50 <= y < 100 and direction == 0:
				cx = y + 50
				cy = 49
				cd = 3
			elif x == 99 and 100 <= y < 150 and direction == 0:
				cx = 149
				cy = 49 - (y - 100)
				cd = 2
			elif 50 <= x < 100 and y == 149 and direction == 1:
				cx = 49
				cy = x + 100
				cd = 2
			elif x == 49 and 150 <= y < 200 and direction == 0:
				cx = y - 100
				cy = 149
				cd = 3
			elif 0 <= x < 50 and y == 199 and direction == 1:
				cx = x + 100
				cy = 0
				cd = 1
			elif x == 0 and 150 <= y < 200 and direction == 2:
				cx = 50 + y - 150
				cy = 0
				cd = 1
			elif x == 0 and 100 <= y < 150 and direction == 2:
				cx = 50
				cy = 49 - (y - 100)
				cd = 0
			elif 0 <= x < 50 and y == 100 and direction == 3:
				cx = 50
				cy = x + 50
				cd = 0
			elif x == 50 and 50 <= y < 100 and direction == 2:
				cx = y - 50
				cy = 100
				cd = 1
			elif x == 50 and 0 <= y < 50 and direction == 2:
				cx = 0
				cy = 149 - y
				cd = 0
			else:
				dx, dy = delta[direction]
				cx = x + dx
				cy = y + dy
				cd = direction
			print(x, y, direction, cx, cy)
			if cx < 0 or cy < 0 or cy >= len(map) or cx >= len(map[cy]) or map[cy][cx] == " ":
				raise Exception(f"Out of bounds! Here be dragons. {cx},{cy}")
			if map[cy][cx] == "#":
				break  # Stop movement here.
			else:
				x = cx
				y = cy
				direction = cd  # Only need to change direction if we actually move.

password = (y + 1) * 1000 + (x + 1) * 4 + direction
print("The final password is:", password)