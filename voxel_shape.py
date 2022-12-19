# Advent of code 2022, by Ghostkeeper

class VoxelShape:
	"""
	A 3D shape consisting of voxels.
	"""

	def __init__(self):
		"""
		Creates an empty shape. Cubes can be added later.
		"""
		self.voxels = set()
		self.surface_area = 0

	def add(self, x, y, z):
		"""
		Add a new voxel cube to the shape.
		:param x: The X coordinate of the voxel.
		:param y: The Y coordinate of the voxel.
		:param z: The Z coordinate of the voxel.
		"""
		if (x, y, z) in self.voxels:
			return
		self.voxels.add((x, y, z))
		self.surface_area += 6  # Initially add the 6 sides of the cube to the area.
		if (x + 1, y, z) in self.voxels:
			self.surface_area -= 2  # Each neighbouring voxel hides 2 sides: One on the new cube and one on the old cube.
		if (x - 1, y, z) in self.voxels:
			self.surface_area -= 2
		if (x, y + 1, z) in self.voxels:
			self.surface_area -= 2
		if (x, y - 1, z) in self.voxels:
			self.surface_area -= 2
		if (x, y, z + 1) in self.voxels:
			self.surface_area -= 2
		if (x, y, z - 1) in self.voxels:
			self.surface_area -= 2