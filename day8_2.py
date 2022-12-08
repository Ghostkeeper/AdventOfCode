# Advent of code 2022, by Ghostkeeper

import get_input_file

grid = open(get_input_file.get_path(8, 1)).readlines()
grid = [line.strip() for line in grid]  # Cut off the newline at the end.
grid = [[int(char) for char in line] for line in grid]
width = len(grid[0])
height = len(grid)

def count_visible(x, y, delta):
	"""
	Walk through the grid in a certain direction (delta) from a certain start position (x, y). The number of trees that
	are visible from the start position will be returned.
	:param x: The X coordinate of the start position.
	:param y: The Y coordinate of the start position.
	:param delta: The direction in which to walk, as a 2D vector.
	:return: The number of visible trees encountered while walking.
	"""
	num_visible = 0
	start_height = grid[x][y]
	while 0 <= x + delta[0] < len(grid[0]) and 0 <= y + delta[1] < len(grid):  # While we're still inside of the grid.
		x += delta[0]
		y += delta[1]
		next = grid[x][y]
		num_visible += 1  # That tree is visible.
		if next >= start_height:  # But the next ones are not?
			return num_visible
	return num_visible

highest_score = 0
for x in range(width):
	for y in range(height):
		score = 1
		for direction in [[0, 1], [0, -1], [1, 0], [-1, 0]]:
			score *= count_visible(x, y, direction)
		highest_score = max(highest_score, score)

print(highest_score)