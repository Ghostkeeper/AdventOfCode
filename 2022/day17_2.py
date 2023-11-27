# Advent of code 2022, by Ghostkeeper

import get_input_file
import tetris_rock

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

num_rocks = 0
highest_obstacle = 0  # Position of the highest solidified rock.
jet_position = 0  # Position in the rotation of jets.
modulo_combinations = {}
height_at_rock = [0]
# First run the simulation until we detect a loop.
while True:
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
			if not tetris_rock.rock_hits(space, rock, rockx, rocky, -1, 0):
				rockx -= 1
		else:  # >
			if not tetris_rock.rock_hits(space, rock, rockx, rocky, 1, 0):
				rockx += 1
		jet_position += 1  # Consumed this jet now.

		# Move rock down with gravity.
		if not tetris_rock.rock_hits(space, rock, rockx, rocky, 0, -1):
			rocky -= 1
		else:  # Solidify rock.
			height = len(rock)
			for negy in range(height):
				for x in range(len(rock[0])):
					if rock[height - 1 - negy][x] == 1:  # This part of the rock sprite is occupied.
						space[rocky + negy][x + rockx] = 1  # Solidify here.
			highest_obstacle = max(highest_obstacle, rocky + height)
			break  # Continue with the next rock.

	height_at_rock.append(highest_obstacle)
	mod_jets = jet_position % num_jets
	mod_rocks = num_rocks % len(rocks)
	shape = 0  # Make a hash of the shape of the rocks at the top. Not 100% reliable if rocks fall around other rocks with jets though!
	for x in range(7):
		for y in range(18):  # Take the bits from a 7*9 block at the top as the shape hash. 63 bits, just fits in a fast long int.
			if len(space) < y + 14:
				break
			shape += space[-4 - y][x] << (x * 9 + y)
	if (mod_jets, mod_rocks, shape) in modulo_combinations:
		break
	else:
		modulo_combinations[(mod_jets, mod_rocks, shape)] = highest_obstacle, jet_position, num_rocks

# Calculate the offset part and the repeating part.
base_height, base_jets, base_rocks = modulo_combinations[(jet_position % num_jets, num_rocks % len(rocks), shape)]
height_per_repeat = highest_obstacle - base_height
rocks_per_repeat = num_rocks - base_rocks
jets_per_repeat = jet_position - base_jets
num_repeats = int((1000_000_000_000 - num_rocks) / rocks_per_repeat) + 1
num_rocks = num_repeats * rocks_per_repeat + base_rocks  # Should go to almost 1 trillion.
rocks_remaining = 1000_000_000_000 - num_rocks
# Now skip until just before the end.
highest_obstacle = height_per_repeat * num_repeats + base_height + (height_at_rock[base_rocks + rocks_remaining] - height_at_rock[base_rocks])

print("The total height of the tower of rocks is:", highest_obstacle)