# Advent of code 2022, by Ghostkeeper

def rock_hits(space, rock, rockx, rocky, dx, dy):
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