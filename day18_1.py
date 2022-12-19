# Advent of code 2022, by Ghostkeeper

import get_input_file
import voxel_shape

lines = open(get_input_file.get_path(18, 1)).readlines()
lines = [line.strip() for line in lines]

shape = voxel_shape.VoxelShape()
for line in lines:
	x, y, z = (int(coord) for coord in line.split(","))
	shape.add(x, y, z)

print("The surface area of the shape is:", shape.surface_area)