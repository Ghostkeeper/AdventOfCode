# Advent of code 2022, by Ghostkeeper

import collections

def diffuse(elves, iteration_number):
	"""
	Simulate a diffusion of a set of elves for one round.
	:param elves: A set of X,Y coordinates containing elves.
	:param iteration_number: Which iteration of diffusion this is.
	:return: A new set of X,Y coordinates with the positions of these elves.
	"""
	order = ["N", "S", "W", "E"]
	rotation = iteration_number % len(order)
	order = order[rotation:] + order[:rotation]

	proposed = {}
	for elf in elves:
		if (elf[0] - 1, elf[1] - 1) not in elves and (elf[0], elf[1] - 1) not in elves \
				and (elf[0] + 1, elf[1] - 1) not in elves and (elf[0] + 1, elf[1]) not in elves \
				and (elf[0] + 1, elf[1] + 1) not in elves and (elf[0], elf[1] + 1) not in elves \
				and (elf[0] - 1, elf[1] + 1) not in elves and (elf[0] - 1, elf[1]) not in elves:
			# Elf has no other elf around it. Does nothing.
			proposed[elf] = elf
			continue

		# Follow the order to propose a movement.
		for direction in order:
			if direction == "N":
				if (elf[0] - 1, elf[1] - 1) not in elves and (elf[0], elf[1] - 1) not in elves and (elf[0] + 1, elf[1] - 1) not in elves:
					proposed[elf] = (elf[0], elf[1] - 1)
					break
			elif direction == "S":
				if (elf[0] - 1, elf[1] + 1) not in elves and (elf[0], elf[1] + 1) not in elves and (elf[0] + 1, elf[1] + 1) not in elves:
					proposed[elf] = (elf[0], elf[1] + 1)
					break
			elif direction == "W":
				if (elf[0] - 1, elf[1] - 1) not in elves and (elf[0] - 1, elf[1]) not in elves and (elf[0] - 1, elf[1] + 1) not in elves:
					proposed[elf] = (elf[0] - 1, elf[1])
					break
			else:  # "E"
				if (elf[0] + 1, elf[1] - 1) not in elves and (elf[0] + 1, elf[1]) not in elves and (elf[0] + 1, elf[1] + 1) not in elves:
					proposed[elf] = (elf[0] + 1, elf[1])
					break

		# Otherwise, stand still.
		if elf not in proposed:
			proposed[elf] = elf

	# Check for each elf whether their proposed direction would be duplicate.
	num_proposed = collections.defaultdict(int)
	for elf in elves:
		num_proposed[proposed[elf]] += 1

	# Execute their movement only if the proposition is not duplicate.
	new_elves = set()
	for elf in elves:
		if num_proposed[proposed[elf]] > 1:
			new_elves.add(elf)  # Stays still.
		else:
			new_elves.add(proposed[elf])

	return new_elves

def visualise(elves):
	minx = min([elf[0] for elf in elves])
	miny = min([elf[1] for elf in elves])
	maxx = max([elf[0] for elf in elves])
	maxy = max([elf[1] for elf in elves])

	for y in range(miny, maxy + 1):
		line = ""
		for x in range(minx, maxx + 1):
			if (x, y) in elves:
				line += "#"
			else:
				line += "."
		print(line)