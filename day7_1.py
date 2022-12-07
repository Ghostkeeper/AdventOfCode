# Advent of code 2022, by Ghostkeeper

import filetree
import get_input_file

terminal_output = open(get_input_file.get_path(7, 1)).read()
tree = filetree.parse_terminal(terminal_output)

print(tree)