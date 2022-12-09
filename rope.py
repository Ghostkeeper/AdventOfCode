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
		self.tail_x = 0
		self.tail_y = 0

	def right(self):
		"""
		Move the head right by 1 place.
		"""
		self.x += 1
		if self.x - self.tail_x == 2:
			self.tail_x += 1
			self.tail_y = self.y

	def left(self):
		"""
		Move the head left by 1 place.
		"""
		self.x -= 1
		if self.tail_x - self.x == 2:
			self.tail_x -= 1
			self.tail_y = self.y

	def up(self):
		"""
		Move the head up by 1 place.
		"""
		self.y -= 1
		if self.tail_y - self.y == 2:
			self.tail_y -= 1
			self.tail_x = self.x

	def down(self):
		"""
		Move the head down by 1 place.
		"""
		self.y += 1
		if self.y - self.tail_y == 2:
			self.tail_y += 1
			self.tail_x = self.x