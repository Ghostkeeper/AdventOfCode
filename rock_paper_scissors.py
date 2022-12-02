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
		if mine == "X" or mine == "A":
			return 1
		elif mine == "Y" or mine == "B":
			return 2
		else:
			return 0
	elif their == "B":
		if mine == "X" or mine == "A":
			return 0
		elif mine == "Y" or mine == "B":
			return 1
		else:
			return 2
	else:
		if mine == "X" or mine == "A":
			return 2
		elif mine == "Y" or mine == "B":
			return 0
		else:
			return 1

def my_choice(their, required_outcome):
	"""
	Find out what my symbol choice should be to get the desired outcome.

	Their symbols are A, B or C. A is Rock, B is Paper, C is Scissors.
	The required outcome is X, Y or Z. X for me losing, Y for a tie, Z for me winning.
	:param their: The symbol they chose.
	:param required_outcome: What the outcome needs to be for the round.
	:return: A if I need to choose Rock, B if I need to choose Paper, or C if I need to choose Scissors.
	"""
	symbols = ["A", "B", "C"]
	their_index = symbols.index(their)
	if required_outcome == "X":  # I need to lose.
		return symbols[(their_index + 2) % 3]
	if required_outcome == "Y":  # We need to tie.
		return symbols[their_index]
	if required_outcome == "Z":  # I need to win.
		return symbols[(their_index + 1) % 3]