# Advent of code 2022, by Ghostkeeper

def mix(numbers):
	"""
	Apply a decryption mixing operation to a list of numbers.
	:param numbers: The numbers to decrypt.
	:return: The mixed numbers.
	"""
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