# Advent of code 2022, by Ghostkeeper

def manhattan_distance(x1, y1, x2, y2):
	return abs(x2 - x1) + abs(y2 - y1)

class Sensor:
	"""
	Represents a sensor. It knows where its beacon is. And by extension, it knows a diamond-shape where other beacons
	cannot be.

	This class allows queries of that diamond shape to find the solution.
	"""

	def __init__(self, x, y, beaconx, beacony):
		"""
		Construct a new sensor.
		:param x: The X coordinate of the sensor.
		:param y: The Y coordinate of the sensor.
		:param beaconx: The X coordinate of the nearest beacon.
		:param beacony: The Y coordinate of the nearest beacon.
		"""
		self.x = x
		self.y = y
		self.range = manhattan_distance(x, y, beaconx, beacony)

	def get_bounds_at_row(self, y):
		"""
		Get the bounds of the sensor range at a certain Y coordinate.
		:param y: The Y coordinate of the row to get the bounds at.
		:return: A tuple containing the lowest X coordinate that is within the range of the sensor, and 1 + the highest
		X coordinate that is within range. If the entire row is out of range, both of these will be equal (indicating a
		width of 0).
		"""
		dy = abs(y - self.y)
		if dy > self.range:  # Out of range.
			return self.x, self.x  # Empty range.
		dx = self.range - dy
		return self.x - dx, self.x + dx + 1