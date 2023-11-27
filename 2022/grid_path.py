# Advent of code 2022, by Ghostkeeper

import math
import queue

def estimate(from_x, from_y, to_x, to_y):
	"""
	Estimate the distance from one point in the grid to another.
	:param from_x: The X coordinate of the start position.
	:param from_y: The Y coordinate of the start position.
	:param to_x: The X coordinate of the end position.
	:param to_y: The Y coordinate of the end position.
	:return: An estimate of how far it'll be to get to the destination, as a float.
	"""
	dx = to_x - from_x
	dy = to_y - from_y
	return math.sqrt(dx * dx + dy * dy)

def get_around(grid, x, y):
	if x > 0:
		yield x - 1, y
	if x < len(grid[0]) - 1:
		yield x + 1, y
	if y > 0:
		yield x, y - 1
	if y < len(grid) - 1:
		yield x, y + 1

def find_path(grid, start_x, start_y, end_x, end_y, max_climb = 1):
	"""
	Find the shortest path through a grid from start to end.

	The path is allowed to climb by at most max_climb with each step.

	This implementation uses A* to find the shortest path.
	:param grid: A height map grid to search through.
	:param start_x: The X coordinate of the start position.
	:param start_y: The Y coordinate of the start position.
	:param end_x: The X coordinate of the end position.
	:param end_y: The Y coordinate of the end position.
	:param max_climb: The maximum incline by which we can climb each step. There is no minimum, nor fall damage.
	:return: The shortest path from start to end, as a list of tuples of x,y coordinates. The start of the path is not
	included, but the end is.
	"""
	open = queue.PriorityQueue()  # Queue of coordinates yet to be processed. Each coordinate is a tuple of (estimate, x, y).
	open.put((grid[start_y][start_x], start_x, start_y))
	open_set = {(start_x, start_y)}
	from_where = {}  # For each cell, what coordinate we came from to reconstruct the shortest path.
	to_start = [[float("inf") for _ in grid] for _ in grid[0]]
	to_start[start_x][start_y] = 0
	total_dist = [[float("inf") for _ in grid] for _ in grid[0]]
	total_dist[start_x][start_y] = estimate(start_x, start_y, end_x, end_y)

	while open_set:
		_, x, y = open.get()
		open_set.remove((x, y))

		if x == end_x and y == end_y:
			# Found the shortest path! Reconstruct it with from_where.
			path = [(end_x, end_y)]
			while x != start_x or y != start_y:
				x, y = from_where[(x, y)]
				path.append((x, y))
			path.pop()  # Don't include the start position.
			path = list(reversed(path))
			return path

		for neighbour_x, neighbour_y in get_around(grid, x, y):
			if grid[neighbour_y][neighbour_x] - grid[y][x] > max_climb:  # To steep to climb here.
				continue

			dist_so_far = to_start[x][y] + 1
			if dist_so_far < to_start[neighbour_x][neighbour_y]:  # Better approached from this direction.
				to_start[neighbour_x][neighbour_y] = dist_so_far
				total_dist[neighbour_x][neighbour_y] = dist_so_far + estimate(neighbour_x, neighbour_y, end_x, end_y)
				from_where[(neighbour_x, neighbour_y)] = x, y
				if (x, y) not in open_set:
					open.put((total_dist[neighbour_x][neighbour_y], neighbour_x, neighbour_y))
					open_set.add((neighbour_x, neighbour_y))

	raise Exception("No path was found to the destination!")