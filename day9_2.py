# Advent of code 2022, by Ghostkeeper

import get_input_file
import rope

commands = open(get_input_file.get_path(9, 1)).readlines()
commands = [command.strip() for command in commands]

r = rope.Rope()
tail = r
for i in range(9):  # 10 pieces, including the r piece above.
	tail.tail = rope.Rope()
	tail = tail.tail

directions = {
	"R": r.right,
	"L": r.left,
	"U": r.up,
	"D": r.down
}

tail_positions = {(0, 0)}  # Track all unique positions of the tail.
for command in commands:
	direction = directions[command[0]]
	distance = int(command.split(" ")[1])
	for _ in range(distance):  # Repeat N times.
		direction()
		tail_positions.add((tail.x, tail.y))

print("Number of unique positions for the tail:", len(tail_positions))