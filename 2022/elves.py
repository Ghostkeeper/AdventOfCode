# Advent of code 2022, by Ghostkeeper

import numpy

class Elves:
	"""
	Data structure representing a set of elves.
	"""

	def __init__(self):
		"""
		Creates an empty expedition.
		"""
		self.elves = numpy.array([], dtype=[("sum_calories", "i4")])

	def read_calories(self, fname):
		"""
		Parse a file containing elf calorie data.

		The data gets stored inside of this instance then.
		:param fname: The file to read from.
		"""
		sum_calories = []
		with open(fname) as f:
			elf_calories = -1
			for line in f.read().split("\n"):
				if line == "":
					if elf_calories >= 0:
						sum_calories.append(elf_calories)
					elf_calories = -1  # Start a new elf.
				else:
					elf_calories += int(line)
			if elf_calories >= 0:
				sum_calories.append(elf_calories)

		self.elves.resize((len(sum_calories),))
		self.elves["sum_calories"] = sum_calories