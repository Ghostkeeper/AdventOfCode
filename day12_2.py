# Advent of code 2022, by Ghostkeeper

import get_input_file
import grid_path

lines = open(get_input_file.get_path(12, 1)).readlines()
lines = [line.strip() for line in lines]
grid = [[ord(char) - ord("a") for char in line] for line in lines]

starts = []
end_x = -1
end_y = -1
for x in range(len(grid[0])):
	for y in range(len(grid)):
		if lines[y][x] == "S":
			grid[y][x] = 0
			starts.append((x, y))
		if lines[y][x] == "a":
			starts.append((x, y))
		elif lines[y][x] == "E":
			end_x = x
			end_y = y
			grid[y][x] = 25

shortest_length = 9999999999
for x, y in starts:
	try:
		shortest_path = grid_path.find_path(grid, x, y, end_x, end_y)
		shortest_length = min(shortest_length, len(shortest_path))
	except:
		pass  # Not every start point has a path to the destination.
print("The length of the shortest path from any a is:", shortest_length)