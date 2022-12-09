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