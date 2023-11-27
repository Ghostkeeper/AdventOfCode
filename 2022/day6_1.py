# Advent of code 2022, by Ghostkeeper

import get_input_file

data = open(get_input_file.get_path(6, 1)).read()
pos = 4
while len(set(data[pos - 4:pos])) != 4:
	pos += 1

print("The first place where there are 4 unique input characters is position:", pos)