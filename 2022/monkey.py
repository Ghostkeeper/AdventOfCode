# Advent of code 2022, by Ghostkeeper

class Monkey:
	def __init__(self, state_str, monkeys, worry_reduction):
		"""
		Construct a monkey.
		:param state_str: A part of the input that determines the starting state of the monkey.
		:param monkeys: The list of all monkeys.
		:param worry_reduction: The factor by which the worry is reduced after the monkey inspects the item.
		"""
		self.items = []
		self.operation = lambda old: old
		self.divisible_test = 1
		self.throw_true = 0
		self.throw_false = 0

		state_lines = state_str.split("\n")
		for line in state_lines:
			if line.startswith("  Starting items: "):
				self.items = line[len("  Starting items: "):].split(", ")
				self.items = [int(item) for item in self.items]
			elif line.startswith("  Operation: new = "):
				operation = line[len("  Operation: new = "):]
				self.operation = lambda old: eval(operation)
			elif line.startswith("  Test: divisible by "):
				self.divisible_test = int(line[len("  Test: divisible by "):])
			elif line.startswith("    If true: throw to monkey "):
				self.throw_true = int(line[len("    If true: throw to monkey "):])
			elif line.startswith("    If false: throw to monkey "):
				self.throw_false = int(line[len("    If false: throw to monkey "):])

		self.monkeys = monkeys
		self.worry_reduction = worry_reduction
		self.times_inspected = 0  # How often this monkey has inspected an item.
		self.worry_modulus = 1  # To reduce huge numbers, you can give a modulus by which to divide worry levels.

	def turn(self):
		"""
		Execute a turn of the monkey.
		"""
		while self.items:
			item = self.items[0]
			item = self.operation(item)  # Inspecting, worry level increases.
			self.times_inspected += 1
			item = int(item / self.worry_reduction)  # Worry level is reduced after inspecting.
			if self.worry_modulus > 1 and item > self.worry_modulus:
				item %= self.worry_modulus
			if item % self.divisible_test == 0:
				self.monkeys[self.throw_true].items.append(item)
			else:
				self.monkeys[self.throw_false].items.append(item)
			del self.items[0]