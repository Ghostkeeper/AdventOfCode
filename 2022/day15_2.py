# Advent of code 2022, by Ghostkeeper

import get_input_file
import pyclipper
import scanner
import re

text = open(get_input_file.get_path(15, 1)).read()
sensors = []
beacons = set()
for match in re.finditer(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)", text):
	sensorx, sensory, beaconx, beacony = match.groups()
	sensors.append(scanner.Sensor(int(sensorx), int(sensory), int(beaconx), int(beacony)))
	beacons.add((int(beaconx), int(beacony)))

# Union all the range shapes of each beacon.
total_shape = [sensors[0].get_bounds()]
for sensor in sensors[1:]:
	operation = pyclipper.Pyclipper()
	operation.AddPaths(total_shape, pyclipper.PT_CLIP)
	operation.AddPath(sensor.get_bounds(), pyclipper.PT_SUBJECT)
	total_shape = operation.Execute(pyclipper.CT_UNION, pyclipper.PFT_NONZERO)

# Subtract that from the range in which we're finding the distract signal.
operation = pyclipper.Pyclipper()
operation.AddPath(((0, 0), (4000000, 0), (4000000, 4000000), (0, 4000000)), pyclipper.PT_SUBJECT)  # The sensor range.
operation.AddPaths(total_shape, pyclipper.PT_CLIP)
possible_places = operation.Execute(pyclipper.CT_DIFFERENCE)

assert len(possible_places) == 1  # There should be only one possible place.
distress_place = possible_places[0]
assert pyclipper.Area(distress_place) == 2.0  # It should be a diamond-shape with radius 1, centered around the coordinate of the distress beacon.
x = sum([vertex[0] for vertex in distress_place]) / len(distress_place)
y = sum([vertex[1] for vertex in distress_place]) / len(distress_place)
print("The tuning frequency of the distress beacon should be:", int(x) * 4000000 + int(y))