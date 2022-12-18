# Advent of code 2022, by Ghostkeeper

import get_input_file

jets = open(get_input_file.get_path(17, 1)).read()
num_jets = len(jets)
space = []  # Space expands as needed.
rocks = [  # The rocks, in proper order of rotation.
	[[1, 1, 1, 1]],
	[[0, 1, 0],
	 [1, 1, 1],
	 [0, 1, 0]],
	[[0, 0, 1],
	 [0, 0, 1],
	 [1, 1, 1]],
	[[1],
	 [1],
	 [1],
	 [1]],
	[[1, 1],
	 [1, 1]]
]

def rock_hits(rock, rockx, rocky, dx, dy):
	"""
	Check if a rock would hit anything in the cave if it moves in a certain direction.

	The rock hits something if it is unable to move further in that direction without intersecting with a solidified
	rock or the floor.
	:param rock: The rock shape to check.
	:param rockx: The X position of the rock (within a row).
	:param rocky: The Y position of the rock (the row number).
	:param dx: The X movement vector of the rock.
	:param dy: The Y movement vector of the rock.
	:return: True if it rests, or False if it doesn't.
	"""
	height = len(rock)
	width = len(rock[0])

	# Pre-checks for the space bounds.
	if dy < 0 and rocky <= 0:
		return True  # Would hit the floor.
	if dx > 0 and rockx + width >= 7:
		return True  # Would hit the right wall.
	if dx < 0 and rockx <= 0:
		return True  # Would hit the left wall.

	for negy in range(height):
		for x in range(width):
			if rock[height - 1 - negy][x] == 1:  # This part of the rock sprite is occupied.
				if space[rocky + negy + dy][x + rockx + dx] == 1:  # Destination position is occupied.
					return True
	return False

num_rocks = 0
highest_obstacle = 0  # Position of the highest solidified rock.
jet_position = 0  # Position in the rotation of jets.
while num_rocks < 2022:
	# Create a new rock.
	rock = rocks[num_rocks % len(rocks)]
	while len(space) < highest_obstacle + 3 + len(rock):
		space.append([0, 0, 0, 0, 0, 0, 0])  # 7 wide space, currently empty.
	rocky = highest_obstacle + 3  # Rocks start 3 above the highest obstacle.
	rockx = 2  # Rocks start 2 units away from the left wall.
	num_rocks += 1

	while True:  # Until the rock solidifies.
		# Push rock with a jet.
		if jets[jet_position % num_jets] == "<":
			if not rock_hits(rock, rockx, rocky, -1, 0):
				rockx -= 1
		else:  # >
			if not rock_hits(rock, rockx, rocky, 1, 0):
				rockx += 1
		jet_position += 1  # Consumed this jet now.

		# Move rock down with gravity.
		if not rock_hits(rock, rockx, rocky, 0, -1):
			rocky -= 1
		else:  # Solidify rock.
			height = len(rock)
			for negy in range(height):
				for x in range(len(rock[0])):
					if rock[height - 1 - negy][x] == 1:  # This part of the rock sprite is occupied.
						space[rocky + negy][x + rockx] = 1  # Solidify here.
			highest_obstacle = max(highest_obstacle, rocky + height)
			break  # Continue with the next rock.

print("The total height of the tower of rocks is:", highest_obstacle)