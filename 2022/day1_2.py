# Advent of code 2022, by Ghostkeeper

import elves
import get_input_file
import numpy

expedition = elves.Elves()
expedition.read_calories(get_input_file.get_path(1, 1))

calories_sorted = numpy.sort(expedition.elves["sum_calories"])
print("Total calories in top 3 elves:", sum(calories_sorted[-3:]))