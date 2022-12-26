# Advent of code 2022, by Ghostkeeper

import get_input_file
import math
import queue

# Parse the input grid.
lines = open(get_input_file.get_path(24, 1)).readlines()
grids = []  # 3D grid, one frame per minute of the 2D field. First dimension is T, second dimension is Y, third dimension is X.
grids.append([])
for line in lines:
	grids[0].append([])
	for char in line.strip()[1:-1]:
		if char == ".":
			grids[0][-1].append(0)
		elif char == ">":
			grids[0][-1].append(1 << 0)
		elif char == "^":
			grids[0][-1].append(1 << 1)
		elif char == "<":
			grids[0][-1].append(1 << 2)
		elif char == "v":
			grids[0][-1].append(1 << 3)
		elif char == "#":
			grids[0][-1].append(-1)
		else:
			raise Exception("Unknown input:", char)

width = len(grids[0][0])
height = len(grids[0]) - 2
cycle = width
while cycle / height != int(cycle / height):
	cycle += width

def step(grid):
	"""
	Compute new grid from the previous grid state. This moves the blizzards in the grid to their new positions.
	:param grid: A new grid with blizzards moved.
	:return: A new grid.
	"""
	new_grid = [[0 for _ in line] for line in grid]
	for y, line in enumerate(grid):
		for x, cell in enumerate(line):
			if cell == -1:
				new_grid[y][x] = -1
				continue
			if cell & (1 << 0):  # Blizzard going right.
				new_grid[y][(x + 1) % width] += 1 << 0
			if cell & (1 << 1):  # Blizzard going up.
				new_grid[(y - 2) % height + 1][x] += 1 << 1
			if cell & (1 << 2):  # Blizzard going left.
				new_grid[y][(x - 1) % width] += 1 << 2
			if cell & (1 << 3):  # Blizzard going down.
				new_grid[y % height + 1][x] += 1 << 3
	return new_grid

# Pre-simulate all stages in the blizzards cycle.
for i in range(1, cycle):
	grids.append(step(grids[-1]))

endx = width -1
endy = height + 1
def estimate(x, y):
	"""
	Estimate the distance from a point to the destination of the blizzard canyon.
	:param x: The X coordinate to estimate the distance from.
	:param y: The Y coordinate to estimate the distance from.
	:return: The estimate distance to the destination.
	"""
	dx = endx - x
	dy = endy - y
	return math.sqrt(dx * dx + dy * dy)

def get_neighbours(x, y, t):
	"""
	Find which cells we could move to from a given position.
	:param x: The X coordinate of the current position.
	:param y: The Y coordinate of the current position.
	:param t: The time frame of the current position.
	:return: A sequence of tuples of X,Y coordinates to which we could move from here.
	"""
	nextt = (t + 1) % cycle
	if grids[nextt][y][x] == 0:
		yield x, y  # We could stand still.
	if x < width - 1 and grids[nextt][y][x + 1] == 0:
		yield x + 1, y  # We could move right.
	if y > 1 and grids[nextt][y - 1][x] == 0:
		yield x, y - 1  # We could move up.
	if x > 0 and grids[nextt][y][x - 1] == 0:
		yield x - 1, y  # We could move left.
	if y < height + 1 and grids[nextt][y + 1][x] == 0:
		yield x, y + 1  # We could move down.

# Now find the shortest path to the exit using A*.
# Set-up state.
open = queue.PriorityQueue()  # Queue of coordinates yet to be processed. Each coordinate is a tuple of (estimate, x, y, t)
open.put((estimate(0, 0), 0, 0, 0))  # Start from position 0, 0 on T=0
open_set = {(0, 0, 0)}
from_where = {}  # For each cell, what coordinate we came from to reconstruct the shortest path.
to_start = [[[float("inf") for _ in grids[0][0]] for _ in grids[0]] for _ in grids]
to_start[0][0][0] = 0
total_dist = [[[float("inf") for _ in grids[0][0]] for _ in grids[0]] for _ in grids]
total_dist[0][0][0] = estimate(0, 0)

while open_set:
	_, x, y, t = open.get()
	open_set.remove((x, y, t))

	if x == endx and y == endy:
		# Found the shortest path! Reconstruct it with from_where.
		#path = [(endx, endy, t)]
		#while x != 0 or y != 0 or t != 0:
		#	x, y, t = from_where[(x, y, t)]
		#	path.append((x, y, t))
		#path.pop()  # Don't include the start position.
		print("The shortest path to the finish has length:", t)
		break

	while len(to_start) <= t + 1:
		to_start.append([[float("inf") for _ in grids[0][0]] for _ in grids[0]])
		total_dist.append([[float("inf") for _ in grids[0][0]] for _ in grids[0]])

	for neighbourx, neighboury in get_neighbours(x, y, t):
		dist_so_far = to_start[t][y][x] + 1
		if dist_so_far < to_start[t + 1][neighboury][neighbourx]:  # Better approached from this direction.
			to_start[t + 1][neighboury][neighbourx] = dist_so_far
			total_dist[t + 1][neighboury][neighbourx] = dist_so_far + estimate(neighbourx, neighboury)
			from_where[(neighbourx, neighboury, t + 1)] = x, y, t
			if (x, y, t) not in open_set:
				open.put((total_dist[t + 1][neighboury][neighbourx], neighbourx, neighboury, t + 1))
				open_set.add((neighbourx, neighboury, t + 1))