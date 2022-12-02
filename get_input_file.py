# Advent of code 2022, by Ghostkeeper

import os.path

def get_path(day, exercise, index = 0):
	"""
	Get the file path towards an input file.
	:param day: The day of the exercise (1-25)
	:param exercise: The exercise index (1-2)
	:param index: If the exercise has multiple input files, the index to the file you want.
	:return: A path to a file.
	"""
	inputs_dir = os.path.join(os.path.dirname(__file__), "inputs")
	result = os.path.join(inputs_dir, f"day{day}_{exercise}")
	if index != 0:
		result += f"_{index}"
	result += ".txt"
	return result