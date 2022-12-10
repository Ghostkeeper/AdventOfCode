# Advent of code 2022, by Ghostkeeper

import get_input_file

commands = open(get_input_file.get_path(10, 1)).readlines()
commands = [command.strip() for command in commands]

interesting_cycles = {20, 60, 100, 140, 180, 220}
sum_interesting_signals = 0

x = 1
cycle = 0
for command in commands:
	if command == "noop":
		if cycle in interesting_cycles:
			sum_interesting_signals += x * cycle
		cycle += 1
	else:
		parts = command.split(" ")
		delta = int(parts[1])
		if cycle in interesting_cycles or cycle + 1 in interesting_cycles:
			sum_interesting_signals += x * cycle
		x += delta
		cycle += 2

print("The sum of the interestng signal strengths is:", sum_interesting_signals)