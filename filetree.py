# Advent of code 2022, by Ghostkeeper

def parse_terminal(terminal_output):
	"""
	Parses Terminal output to construct a representation of the file tree that was discovered from it.
	:param terminal_output: A string of terminal commands and their output.
	:return: A file tree, represented as a dict with file/dir names as keys, and subdicts (dirs) or ints (filesizes) as
	values. Each dir except the root also has a dict ".." to refer to its parent.
	"""
	tree = {}
	current = tree  # By reference!

	lines = terminal_output.split("\n")
	while lines:
		assert lines[0].startswith("$ ")  # We should've removed the first lines until the next command.
		if lines[0].startswith("$ cd "):
			target = lines[0][len("$ cd "):]
			if target == "/":
				current = tree
			else:
				current = current[target]
			del lines[0]
		elif lines[0] == "$ ls":
			del lines[0]
			while lines and not lines[0].startswith("$ "):
				if lines[0].startswith("dir "):  # A directory.
					dirname = lines[0][len("dir "):]
					if dirname not in current:
						current[dirname] = {
							"..": current
						}
				else:  # A file.
					file_line = lines[0].split(" ")
					current[file_line[1]] = int(file_line[0])
				del lines[0]

	return tree