# Advent of code 2022, by Ghostkeeper

def who_wins(their, mine):
	"""
	Who wins a round of Rock Paper Scissors?

	Their symbols are A, B or C. A is Rock, B is Paper, C is Scissors.
	My symbols are X, Y or Z. X is Rock, Y is Paper, Z is Scissors.
	:param their: The symbol they chose.
	:param mine: The symbol I chose.
	:return: 0 if I lose, 1 if it's a tie, or 2 if I win.
	"""
	if their == "A":
		if mine == "X":
			return 1
		elif mine == "Y":
			return 2
		else:
			return 0
	elif their == "B":
		if mine == "X":
			return 0
		elif mine == "Y":
			return 1
		else:
			return 2
	else:
		if mine == "X":
			return 2
		elif mine == "Y":
			return 0
		else:
			return 1