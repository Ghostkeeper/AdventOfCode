use std::collections::HashSet;

fn parse(input: String) -> (Vec<Vec<bool>>, (usize, usize)) {
	let mut grid = vec!();
	let mut start = (0, 0);
	for (y, line) in input.split("\n").enumerate() {
		grid.push(vec!());
		for (x, char) in line.chars().enumerate() {
			grid[y].push(char == '#');
			if char == 'S' {
				start = (x, y);
			}
		}
	}
	return (grid, start);
}

pub fn part1(input: String) -> usize {
	let (grid, start) = parse(input);

	let mut possible_places = HashSet::new();
	possible_places.insert(start);
	for step in 0..64 {
		for place in possible_places.clone() {
			possible_places.remove(&place);
			if place.0 > 0 && !grid[place.1][place.0 - 1] {
				possible_places.insert((place.0 - 1, place.1));
			}
			if place.0 < grid[0].len() - 1 && !grid[place.1][place.0 + 1] {
				possible_places.insert((place.0 + 1, place.1));
			}
			if place.1 > 0 && !grid[place.1 - 1][place.0] {
				possible_places.insert((place.0, place.1 - 1));
			}
			if place.1 < grid.len() - 1 && !grid[place.1 + 1][place.0] {
				possible_places.insert((place.0, place.1 + 1));
			}
		}
	}

	return possible_places.len();
}