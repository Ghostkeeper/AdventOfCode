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
			self.tail.x += 1
			self.tail.y = self.y

	def left(self):
		"""
		Move the head left by 1 place.
		"""
		self.x -= 1
		if self.tail and self.tail.x - self.x == 2:
			self.tail.x -= 1
			self.tail.y = self.y

	def up(self):
		"""
		Move the head up by 1 place.
		"""
		self.y -= 1
		if self.tail and self.tail.y - self.y == 2:
			self.tail.y -= 1
			self.tail.x = self.x

	def down(self):
		"""
		Move the head down by 1 place.
		"""
		self.y += 1
		if self.tail and self.y - self.tail.y == 2:
			self.tail.y += 1
			self.tail.x = self.x