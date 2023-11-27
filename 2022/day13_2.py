# Advent of code 2022, by Ghostkeeper

import compare_lists
import get_input_file
import functools

packets = open(get_input_file.get_path(13, 1)).readlines()
packets = [line for line in packets if line != "\n"]
packets = [eval(line) for line in packets]

packets.append([[2]])
packets.append([[6]])

packets = list(sorted(packets, key=functools.cmp_to_key(compare_lists.compare)))

index1 = packets.index([[2]])
index2 = packets.index([[6]])
print("The decoder key is:", (index1 + 1) * (index2 + 1))