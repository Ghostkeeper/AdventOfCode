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
		if self.tail and self.x - self.tail.x == 2:
			self.tail.right()
			if self.tail.y < self.y:
				self.tail.down()
			elif self.tail.y > self.y:
				self.tail.up()

	def left(self):
		"""
		Move the head left by 1 place.
		"""
		self.x -= 1
		if self.tail and self.tail.x - self.x == 2:
			self.tail.left()
			if self.tail.y < self.y:
				self.tail.down()
			elif self.tail.y > self.y:
				self.tail.up()

	def up(self):
		"""
		Move the head up by 1 place.
		"""
		self.y -= 1
		if self.tail and self.tail.y - self.y == 2:
			self.tail.up()
			if self.tail.x < self.x:
				self.tail.right()
			elif self.tail.x > self.x:
				self.tail.left()

	def down(self):
		"""
		Move the head down by 1 place.
		"""
		self.y += 1
		if self.tail and self.y - self.tail.y == 2:
			self.tail.down()
			if self.tail.x < self.x:
				self.tail.right()
			elif self.tail.x > self.x:
				self.tail.left()