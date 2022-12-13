# Advent of code 2022, by Ghostkeeper

def compare(left, right):
	"""
	Compare two items for whether they are in the right order.
	:param left: One of the items.
	:param right: The other item.
	:return: -1 if the items are in the right order, 1 if the items are in the wrong order, or 0 if they are equal (and
	one must continue checking).
	"""
	if type(left) is int and type(right) is int:
		if left < right:
			return -1
		elif left > right:
			return 1
		else:
			return 0
	elif type(left) is int and type(right) is list:
		return compare([left], right)
	elif type(left) is list and type(right) is int:
		return compare(left, [right])
	else:  # Both list.
		for j in range(min(len(left), len(right))):
			comparison = compare(left[j], right[j])
			if comparison != 0:
				return comparison
		if len(left) < len(right):
			return -1
		elif len(left) > len(right):
			return 1
		else:
			return 0