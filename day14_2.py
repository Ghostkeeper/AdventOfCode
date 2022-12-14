# Advent of code 2022, by Ghostkeeper

import get_input_file
import sand_cave

cave = sand_cave.Cave(open(get_input_file.get_path(14, 1)).read(), floor=True)
cave.simulate()

print("The amount of settled sand is:", cave.get_settled_sand_count())