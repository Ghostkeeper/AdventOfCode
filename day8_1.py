# Advent of code 2022, by Ghostkeeper

import get_input_file

grid = open(get_input_file.get_path(8, 1)).readlines()
grid = [line.strip() for line in grid]  # Cut off the newline at the end.
grid = [[int(char) for char in line] for line in grid]
width = len(grid[0])
height = len(grid)

visible = [[False] * width for _ in range(height)]
# Around the perimiter, all trees are visible.
for x in [0, width - 1]:
	for y in range(height):
		visible[x][y] = True
for x in range(width):
	for y in [0, height - 1]:
		visible[x][y] = True

def climb(x, y, delta):
	"""
	Hillclimb through the grid in a certain direction (delta) from a certain start position (x, y). All trees that are
	visible will be marked in the visible grid.
	:param x: The X coordinate of the start position.
	:param y: The Y coordinate of the start position.
	:param delta: The direction in which to climb, as a 2D vector.
	"""
	highest = 0
	while 0 <= x + delta[0] < len(grid[0]) and 0 <= y + delta[1] < len(grid):  # While we're still inside of the grid.
		highest = max(highest, grid[x][y])
		x += delta[0]
		y += delta[1]
		next = grid[x][y]
		if next > highest:
			visible[x][y] = True  # This tree is visible!
		if next == 9:  # Maximum height. We can stop here already.
			return

# In four directions, hillclimb every row/column.
for delta in [[0, 1], [0, -1]]:
	for x in range(width - 1):
		y = 0 if delta[1] == 1 else height - 1
		climb(x, y, delta)
for delta in [[1, 0], [-1, 0]]:
	for y in range(height - 1):
		x = 0 if delta[0] == 1 else width - 1
		climb(x, y, delta)

print("The number of visible trees:", sum([sum(line) for line in visible]))