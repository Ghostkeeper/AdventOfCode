# Advent of code 2022, by Ghostkeeper

import get_input_file

lines = open(get_input_file.get_path(21, 1)).readlines()
lines = [line.strip() for line in lines]

monkeys = {}
for line in lines:
	parts = line.split(": ")
	monkeys[parts[0]] = parts[1]

root_found = False
while not root_found:  # Keep iterating over the list of monkeys until we found the value for the root monkey.
	for name, value in monkeys.items():
		try:
			value = float(value)
		except ValueError:  # Not a simple number.
			mon1 = value[0:4]
			mon2 = value[7:11]
			if type(monkeys[mon1]) is float and type(monkeys[mon2]) is float:
				if value[5] == "+":
					value = monkeys[mon1] + monkeys[mon2]
				elif value[5] == "-":
					value = monkeys[mon1] - monkeys[mon2]
				elif value[5] == "*":
					value = monkeys[mon1] * monkeys[mon2]
				elif value[5] == "/":
					value = monkeys[mon1] / monkeys[mon2]
				if name == "root":  # This was the final answer.
					root_found = True
		monkeys[name] = value

print("The value for the root monkey is:", int(monkeys["root"]))