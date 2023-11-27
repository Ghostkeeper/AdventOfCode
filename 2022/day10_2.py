# Advent of code 2022, by Ghostkeeper

import get_input_file

commands = open(get_input_file.get_path(10, 1)).readlines()
commands = [command.strip() for command in commands]

x = 1
cycle = 0
output = []

def crt():
	row = int(cycle / 40)
	if row + 1 > len(output):
		# Add a new line.
		output.append("")
	if abs((cycle % 40) - x) <= 1:
		output[row] += "#"
	else:
		output[row] += "."

for command in commands:
	if command == "noop":
		cycle += 1
		crt()
	else:
		parts = command.split(" ")
		delta = int(parts[1])
		crt()
		cycle += 1
		crt()
		x += delta
		cycle += 1

print("This is the CRT output:")
for row in output:
	print(row)