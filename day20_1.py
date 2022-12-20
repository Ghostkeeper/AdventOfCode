# Advent of code 2022, by Ghostkeeper

import get_input_file

numbers = open(get_input_file.get_path(20, 1)).readlines()
numbers = [int(number.strip()) for number in numbers]

def mix(numbers):
	indices = list(range(len(numbers)))  # Mix indices first, then look up their values to finalise them.

	for i in range(len(numbers)):
		# Find the correct index in the shuffled list.
		pos = indices.index(i)
		shift = numbers[i]  # How much to shift this number.
		new_pos = (pos + shift) % (len(numbers) - 1)
		if new_pos == 0:
			new_pos = len(numbers)  # Never get positioned at the start of the list, only at the end. They are equivalent in terms of "between X and Y".

		del indices[pos]
		indices.insert(new_pos, i)

	# Now apply the indices to shuffle the number list.
	return [numbers[i] for i in indices]

# Find the value 0.
numbers = mix(numbers)
start = numbers.index(0)
coordinate_sum = numbers[(start + 1000) % len(numbers)] + numbers[(start + 2000) % len(numbers)] + numbers[(start + 3000) % len(numbers)]
print("The sum of the grove coordinates is:", coordinate_sum)