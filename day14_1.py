# Advent of code 2022, by Ghostkeeper

import get_input_file

# Parsing the input.
rock_lines = open(get_input_file.get_path(14, 1)).readlines()
rock_lines = [line.strip() for line in rock_lines]
rock_lines = [line.split(" -> ") for line in rock_lines]
rock_lines = [[vertex.split(",") for vertex in line] for line in rock_lines]
rock_lines = [[(int(vertex[0]), int(vertex[1])) for vertex in line] for line in rock_lines]

# Generate a cave.
startx = 500
starty = 0
minx = min([min([vertex[0] for vertex in line]) for line in rock_lines])
miny = min([min([vertex[1] for vertex in line]) for line in rock_lines])
maxx = max([max([vertex[0] for vertex in line]) for line in rock_lines])
maxy = max([max([vertex[1] for vertex in line]) for line in rock_lines])
minx = min(minx, startx)
miny = min(miny, starty)
maxx = max(maxx, startx)
maxy = max(maxy, starty)
cave = [[0 for x in range(minx, maxx + 1)] for y in range(miny, maxy + 1)]  # 0 represents air.

# Place rocks in the cave.
for line in rock_lines:
	x = line[0][0]
	y = line[0][1]
	for vertex in line[1:]:
		dx, dy = (vertex[0] - x, vertex[1] - y)
		# And normalise to get steps of 1.
		if dx != 0:
			dx = int(dx / abs(dx))
		if dy != 0:
			dy = int(dy / abs(dy))
		cave[y - miny][x - minx] = 1  # Representing rock.
		while (x, y) != vertex:
			x += dx
			y += dy
			cave[y - miny][x - minx] = 1

flowing_sand = []
while True:
	flowing_sand.append([startx - minx, starty - miny])
	done = False
	to_remove = set()
	for i in range(len(flowing_sand)):
		sand = flowing_sand[i]
		if sand[1] + 1 >= len(cave):  # Abyss beneath.
			done = True  # But do process the other sand.
		elif cave[sand[1] + 1][sand[0]] == 0:  # Air beneath.
			sand[1] += 1
		elif cave[sand[1] + 1][sand[0] - 1] == 0:  # Air bottom left.
			sand[0] -= 1
			sand[1] += 1
		elif cave[sand[1] + 1][sand[0] - 1] == 0:  # Air bottom right.
			sand[0] += 1
			sand[1] += 1
		else:
			to_remove.add(i)
			cave[sand[1]][sand[0]] = 2  # To indicate settled sand.
	if done:
		break

settled_sand_count = sum([row.count(2) for row in cave])
print("The amount of settled sand is:", settled_sand_count)