# Advent of code 2022, by Ghostkeeper

import get_input_file
import rock_paper_scissors

score = 0
for line in open(get_input_file.get_path(2, 1)):
	if line[2] == "X":
		score += 1  # Always get 1 point for selecting Rock
	elif line[2] == "Y":
		score += 2  # Always get 2 points for selecting Paper
	else:
		score += 3  # Always get 3 points for selecting Scissors
	score += rock_paper_scissors.who_wins(line[0], line[2]) * 3

print("At the end of the tournament, my score will be:", score)