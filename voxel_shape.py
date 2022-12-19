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

	def external_surface(self):
		"""
		Get the external surface area, not including the surface area of cavities inside the shape.
		:return: The external surface area.
		"""
		# First find a voxel on the external surface. The voxel with lowest X coordinate is always on the surface.
		minx = min([voxel[0] for voxel in self.voxels])
		miny = min([voxel[1] for voxel in self.voxels])
		minz = min([voxel[2] for voxel in self.voxels])
		maxx = max([voxel[0] for voxel in self.voxels]) + 1
		maxy = max([voxel[1] for voxel in self.voxels]) + 1
		maxz = max([voxel[2] for voxel in self.voxels]) + 1
		voxel = None
		for x in range(minx, maxx):
			for y in range(miny, maxy):
				for z in range(minz, maxz):
					if (x, y, z) in self.voxels:
						voxel = x, y, z
						break
				if voxel:
					break
			if voxel:
				break

		# Start an any-first search from the starting surface.
		surface = (-1, 0, 0)  # The low-X surface is always on the outside, so start there.
		visited = set()
		open = {(voxel, surface)}
		while open:
			voxel, surface = open.pop()
			assert (voxel[0] + surface[0], voxel[1] + surface[1], voxel[2] + surface[2]) not in self.voxels
			visited.add((voxel, surface))
			surface_dim = 0  # Find the dimension at which the current surface is.
			while surface[surface_dim] == 0:
				surface_dim += 1
			for direction in [(0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)]:  # Check all four adjacent surfaces.
				dx, dy, dz = direction[surface_dim], direction[(surface_dim + 2) % 3], direction[(surface_dim + 1) % 3]  # Rotate according to surface_dim.
				diagonal = voxel[0] + dx + surface[0], voxel[1] + dy + surface[1], voxel[2] + dz + surface[2]  # First check for the diagonal voxel that looks perpendicularly over the current surface.
				if diagonal in self.voxels:  # The neighbouring voxel there is occupied.
					if (diagonal, (-dx, -dy, -dz)) not in visited:
						open.add((diagonal, (-dx, -dy, -dz)))  # Opposite to the direction we were initially checking.
				else:
					neighbour = voxel[0] + dx, voxel[1] + dy, voxel[2] + dz  # Then check the straight neighbour that could cover the surface in this direction.
					if neighbour in self.voxels:
						if (neighbour, (surface[0], surface[1], surface[2])) not in visited:
							open.add((neighbour, (surface[0], surface[1], surface[2])))  # Same as the direction we were initially checking.
					else:
						if (voxel, (dx, dy, dz)) not in visited:
							open.add((voxel, (dx, dy, dz)))

		return len(visited)