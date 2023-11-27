# Advent of code 2022, by Ghostkeeper

import get_input_file

lines = open(get_input_file.get_path(21, 1)).readlines()
lines = [line.strip() for line in lines]

monkeys = {}
for line in lines:
	parts = line.split(": ")
	monkeys[parts[0]] = parts[1]
	if parts[0] == "root":
		monkeys["root"] = parts[1][0:4] + " = " + parts[1][7:11]
	if parts[0] == "humn":
		monkeys["humn"] = "XXXX"

# We'll build a functional value tree of operations to solve for humn.
def plus(left, right):
	return left + right
def minus(left, right):
	return left - right
def times(left, right):
	return left * right
def divided(left, right):
	return left / right
def equals(left, right):
	return left == right

# First pass, parse everything.
# Plain values are converted to float.
# Variables are kept as strings. These strings should always be length 4.
# Equations are changed into tuples of (operation, left, right).
# The unknown will be the string "XXXX"
for name, value in monkeys.items():
	try:
		value = float(value)
	except ValueError:
		if name == "humn":
			continue
		if value[5] == "+":
			value = (plus, value[0:4], value[7:11])
		elif value[5] == "-":
			value = (minus, value[0:4], value[7:11])
		elif value[5] == "*":
			value = (times, value[0:4], value[7:11])
		elif value[5] == "/":
			value = (divided, value[0:4], value[7:11])
		elif value[5] == "=":
			value = (equals, value[0:4], value[7:11])
		else:
			raise Exception(f"Unknown operation in {value}")
	monkeys[name] = value

# Now keep iterating until we can't change anything any more due to the unknown humn value.
# This will compute fixed values for anything that doesn't depend on humn.
changed = True
while changed:
	changed = False
	for name, value in monkeys.items():
		if type(value) == float:
			continue
		if name == "humn":
			continue  # Don't solve this yet until all the downward sides have been solved.
		left = value[1]
		right = value[2]
		if type(left) is str and type(monkeys[left]) is float:  # Dereference left and right if we can.
			left = monkeys[left]
			monkeys[name] = (value[0], left, right)
			changed = True
		if type(right) is str and type(monkeys[right]) is float:
			right = monkeys[right]
			monkeys[name] = (value[0], left, right)
			changed = True
		if type(left) is float and type(right) is float:  # Both parameters are float now.
			value = value[0](left, right)  # Execute the function to compute the result.
			monkeys[name] = value
			changed = True

# Now we can symbolically solve the human equation by starting at the root, and working through each equation until we find the human equation.
operator, left, right = monkeys["root"]
if type(right) is float:
	left, right = right, left

while True:
	if type(right) is str:
		right = monkeys[right]
		if right == "XXXX":  # This is the unknown, so Left must contain the result of the human equation.
			break  # Done! Left now contains the answer.
	operator, subleft, subright = right
	right = subleft if type(subright) is float else subright  # Iterate deeper into whichever is a formula.
	if operator == plus:  # Y = subleft + subright
		if type(subleft) is float:
			left = left - subleft
		else:
			left = left - subright
	elif operator == minus:  # Y = subleft - subright
		if type(subleft) is float:
			left = subleft - left
		else:
			left = left + subright
	elif operator == times:  # Y = subleft * subright
		if type(subleft) is float:
			left = left / subleft
		else:
			left = left / subright
	elif operator == divided:  # Y = subleft / subright
		if type(subleft) is float:
			left = subleft / left
		else:
			left = left * subright
	else:
		raise Exception(f"Unknown operator {operator}")

print("The solution for the human equation is:", left)