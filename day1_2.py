# Advent of code 2022, by Ghostkeeper

import elves
import get_input_file

expedition = elves.Elves()
expedition.read_calories(get_input_file.get_path(1, 1))

calories_per_elf = [sum(items) for items in expedition.elves]
calories_sorted = sorted(calories_per_elf)
print("Total calories in top 3 elves:", sum(calories_sorted[-3:]))