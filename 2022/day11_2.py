# Advent of code 2022, by Ghostkeeper

import get_input_file
import monkey

input = open(get_input_file.get_path(11, 1)).read()
input = input.split("\n\n")  # Per monkey.

monkeys = []
for monkey_str in input:
	monkeys.append(monkey.Monkey(monkey_str, monkeys, worry_reduction=1))

# Calculate worry modulus to reduce huge numbers.
worry_modulus = 1
for monkey in monkeys:
	worry_modulus *= monkey.divisible_test
for monkey in monkeys:
	monkey.worry_modulus = worry_modulus

for round in range(10000):  # Simulate 10000 rounds.
	for monkey in monkeys:
		monkey.turn()

times_inspected = []
for monkey in monkeys:
	times_inspected.append(monkey.times_inspected)
times_inspected.sort()
times_inspected = list(reversed(times_inspected))

print("The total amount of monkey business is:", times_inspected[0] * times_inspected[1])