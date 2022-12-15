# Advent of code 2022, by Ghostkeeper

import get_input_file
import re

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

def merge_ranges(ranges):
	"""
	Marge a list of ranges, producing a minimal new list of ranges that don't have overlaps.
	:param ranges: The ranges to merge. Each range is a tuple of a lower bound (inclusive) and an upper bound
	(exclusive).
	:return: A list of ranges that has no overlaps but represents the same area.
	"""
	output = []  # Keep the output sorted by upper bound, so that we can always just try joining with the last one without checking the whole list.
	for range in sorted(ranges):  # Sort by lower bound.
		if len(output) == 0:
			output.append(range)
			continue
		# Check if this (the leftmost unprocessed range) overlaps with the rightmost processed range.
		rightmost = output[-1]
		if range[0] <= rightmost[1]:  # Also allow joining if they are exactly adjacent. Reduces output size!
			output[-1] = rightmost[0], max(rightmost[1], range[1])
		else:
			output.append(range)
	return output

scan_row = 2000000
text = open(get_input_file.get_path(15, 1)).read()
sensors = []
beacons = set()
for match in re.finditer(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", text):
	sensorx, sensory, beaconx, beacony = match.groups()
	sensors.append(Sensor(int(sensorx), int(sensory), int(beaconx), int(beacony)))
	beacons.add((int(beaconx), int(beacony)))

ranges = []
for sensor in sensors:
	ranges.append(sensor.get_bounds_at_row(scan_row))
ranges = merge_ranges(ranges)  # Merge overlapping ranges.

num_impossible_places = 0
for range in ranges:
	num_impossible_places += range[1] - range[0]
for beacon in beacons:
	if beacon[1] == scan_row:
		num_impossible_places -= 1

print("The number of places where no beacon could be is:", num_impossible_places)