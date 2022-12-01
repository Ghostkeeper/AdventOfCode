# Advent of code 2022, by Ghostkeeper

import elves
import get_input_file

expedition = elves.Elves()
expedition.read_calories(get_input_file.get_path(1, 1))

print("Highest calories in a single elf:", max(expedition.elves["sum_calories"]))