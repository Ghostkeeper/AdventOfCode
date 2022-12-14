# Advent of code 2022, by Ghostkeeper

class Cave:
	"""
	Represents a cave where sand flows inside.
	"""

	def __init__(self, rock_lines):
		"""
		Constructs a cave from polynomial descriptions of the rocks.
		:param rock_lines: A string, with on each line a sequence of vertices.
		"""
		rock_lines = rock_lines.split("\n")
		rock_lines = [line.split(" -> ") for line in rock_lines]
		rock_lines = [[vertex.split(",") for vertex in line] for line in rock_lines]
		rock_lines = [[(int(vertex[0]), int(vertex[1])) for vertex in line] for line in rock_lines]

		self.startx = 500
		self.starty = 0
		minx = min([min([vertex[0] for vertex in line]) for line in rock_lines])
		miny = min([min([vertex[1] for vertex in line]) for line in rock_lines])
		maxx = max([max([vertex[0] for vertex in line]) for line in rock_lines])
		maxy = max([max([vertex[1] for vertex in line]) for line in rock_lines])
		minx = min(minx, self.startx)
		miny = min(miny, self.starty)
		maxx = max(maxx, self.startx)
		maxy = max(maxy, self.starty)
		self.startx -= minx
		self.starty -= miny
		self.cave = [[0 for x in range(minx, maxx + 1)] for y in range(miny, maxy + 1)]  # 0 represents air.

		# Place rocks in the cave.
		for line in rock_lines:
			x = line[0][0]
			y = line[0][1]
			for vertex in line[1:]:
				dx, dy = (vertex[0] - x, vertex[1] - y)
				# And normalise to get steps of 1.
				if dx != 0:
					dx = int(dx / abs(dx))
				if dy != 0:
					dy = int(dy / abs(dy))
				self.cave[y - miny][x - minx] = 1  # Representing rock.
				while (x, y) != vertex:
					x += dx
					y += dy
					self.cave[y - miny][x - minx] = 1

	def simulate(self):
		"""
		Causes sand to start flowing from the start position into the cave.

		The simulation runs until sand falls into the abyss or blocks the starting position.
		"""
		while True:
			sand = [self.startx, self.starty]
			while True:
				if sand[1] + 1 >= len(self.cave):  # Abyss beneath.
					return
				elif self.cave[sand[1] + 1][sand[0]] == 0:  # Air beneath.
					sand[1] += 1
				elif self.cave[sand[1] + 1][sand[0] - 1] == 0:  # Air bottom left.
					sand[0] -= 1
					sand[1] += 1
				elif self.cave[sand[1] + 1][sand[0] + 1] == 0:  # Air bottom right.
					sand[0] += 1
					sand[1] += 1
				else:
					self.cave[sand[1]][sand[0]] = 2  # To indicate settled sand.
					break

	def get_settled_sand_count(self):
		"""
		Get the amount of sand that has settled in the cave.
		:return: The amount of units of sand.
		"""
		return sum([row.count(2) for row in self.cave])