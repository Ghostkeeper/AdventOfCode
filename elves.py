# Advent of code 2022, by Ghostkeeper

from typing import List

class Elves:
	"""
	Data structure representing a set of elves.
	"""

	def __init__(self):
		"""
		Creates an empty expedition.
		"""
		# Elves are currently List[int], a list of the amount of calories for each item they carry.
		self.elves = []  # type: List[List[int]]

	def read_calories(self, fname):
		"""
		Parse a file containing elf calorie data.

		The data gets stored inside of this instance then.
		:param fname: The file to read from.
		"""
		with open(fname) as f:
			elf = []
			for line in f.read().split("\n"):
				if line == "":
					if elf:
						self.elves.append(elf)
					elf = []  # Start a new elf.
				else:
					elf.append(int(line))
			if elf:
				self.elves.append(elf)