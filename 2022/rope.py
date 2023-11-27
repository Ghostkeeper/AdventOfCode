# Advent of code 2022, by Ghostkeeper

class Rope:
	"""
	Simulates rope physics.
	"""

	def __init__(self):
		"""
		Starts the rope in the individual state.
		"""
		self.x = 0
		self.y = 0
		self.tail = None  # A different rope to act as a tail.

	def right(self):
		"""
		Move the head right by 1 place.
		"""
		self.x += 1
		if self.tail:
			self.tail.follow(self)

	def left(self):
		"""
		Move the head left by 1 place.
		"""
		self.x -= 1
		if self.tail:
			self.tail.follow(self)

	def up(self):
		"""
		Move the head up by 1 place.
		"""
		self.y -= 1
		if self.tail:
			self.tail.follow(self)

	def down(self):
		"""
		Move the head down by 1 place.
		"""
		self.y += 1
		if self.tail:
			self.tail.follow(self)

	def follow(self, rope):
		"""
		Move this rope to follow another rope.
		:param rope: The rope to follow.
		"""
		if rope.x - self.x == 2:
			if rope.y - self.y >= 1:
				self.y += 1
			elif rope.y - self.y <= -1:
				self.y -= 1
			self.x += 1
		elif rope.x - self.x == -2:
			if rope.y - self.y >= 1:
				self.y += 1
			elif rope.y - self.y <= -1:
				self.y -= 1
			self.x -= 1
		else:
			if rope.y - self.y == 2:
				self.y += 1
				self.x = rope.x
			elif rope.y - self.y == -2:
				self.y -= 1
				self.x = rope.x

		if self.tail:
			self.tail.follow(self)

	def visualise(self):
		"""
		Print a visualisation of the whole rope (this rope with all its tails).
		"""
		minx = 0
		maxx = 0
		miny = 0
		maxy = 0
		t = self
		while t:
			minx = min(minx, t.x)
			maxx = max(maxx, t.x)
			miny = min(miny, t.y)
			maxy = max(maxy, t.y)
			t = t.tail
		grid = [("." * (-minx + maxx + 1)) for _ in range(-miny + maxy + 1)]

		i = 0
		t = self
		while t:
			x = t.x - minx
			y = t.y - miny
			grid[y] = grid[y][:x] + str(i) + grid[y][x + 1:]
			i += 1
			t = t.tail

		for line in grid:
			print(line)
		print()