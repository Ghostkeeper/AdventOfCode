# Advent of code 2022, by Ghostkeeper

import get_input_file
import rock_paper_scissors

score = 0
for line in open(get_input_file.get_path(2, 1)):
	my_symbol = rock_paper_scissors.my_choice(line[0], line[2])
	if my_symbol == "A":
		score += 1  # Always get 1 point for selecting Rock
	elif my_symbol == "B":
		score += 2  # Always get 2 points for selecting Paper
	else:
		score += 3  # Always get 3 points for selecting Scissors
	score += rock_paper_scissors.who_wins(line[0], my_symbol) * 3

print("At the end of the tournament, my score will be:", score)