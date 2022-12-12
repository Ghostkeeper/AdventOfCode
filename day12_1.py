# Advent of code 2022, by Ghostkeeper

import get_input_file
import grid_path

lines = open(get_input_file.get_path(12, 1)).readlines()
lines = [line.strip() for line in lines]
grid = [[ord(char) - ord("a") for char in line] for line in lines]

start_x = -1
start_y = -1
end_x = -1
end_y = -1
for x in range(len(grid[0])):
	for y in range(len(grid)):
		if lines[y][x] == "S":
			start_x = x
			start_y = y
			grid[y][x] = 0
		elif lines[y][x] == "E":
			end_x = x
			end_y = y
			grid[y][x] = 25

shortest_path = grid_path.find_path(grid, start_x, start_y, end_x, end_y)
print("The length of the shortest path to the destination is:", len(shortest_path))