# Advent of code 2022, by Ghostkeeper

import get_input_file

score = 0
for line in open(get_input_file.get_path(2, 1)):
    if line[2] == "X":
        score += 1  # Always get 1 point for selecting Rock
        if line[0] == "A":
            score += 3  # Tie
        elif line[0] == "B":
            pass  # I lose
        else:
            score += 6  # I win
    elif line[2] == "Y":
        score += 2  # Always get 2 points for selecting Paper
        if line[0] == "A":
            score += 6  # I win
        elif line[0] == "B":
            score += 3  # Tie
        else:
            pass  # I lose
    else:
        score += 3  # Always get 3 points for selecting Scissors
        if line[0] == "A":
            pass  # I lose
        elif line[0] == "B":
            score += 6  # I win
        else:
            score += 3  # Tie
    print(score)

print("At the end of the tournament, my score will be:", score)