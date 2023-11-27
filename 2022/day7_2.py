# Advent of code 2022, by Ghostkeeper

import filetree
import get_input_file

terminal_output = open(get_input_file.get_path(7, 1)).read()
tree = filetree.parse_terminal(terminal_output)

total_size = filetree.get_size(tree)
required_space = total_size - 40000000

all_dir_sizes = []  # For each directory, their total size.
def list_all_dirs_size(dir):
	all_dir_sizes.append(filetree.get_size(dir))
	for name, value in dir.items():
		if name == ".." or type(value) is int:
			continue  # Only find subdirs.
		list_all_dirs_size(value)
list_all_dirs_size(tree)

all_dir_sizes.sort()
for size in all_dir_sizes:
	if size >= required_space:
		print("The size of the smallest directory we can delete is:", size)
		break