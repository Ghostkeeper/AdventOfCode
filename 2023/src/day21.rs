use std::collections::HashSet;

fn parse(input: String) -> (Vec<Vec<bool>>, (i32, i32)) {
	let mut grid = vec!();
	let mut start = (0, 0);
	for (y, line) in input.split("\n").enumerate() {
		grid.push(vec!());
		for (x, char) in line.chars().enumerate() {
			grid[y].push(char == '#');
			if char == 'S' {
				start = (x as i32, y as i32);
			}
		}
	}
	return (grid, start);
}

fn iterate(possible_places: &mut HashSet<(i32, i32)>, grid: &Vec<Vec<bool>>) {
	for place in possible_places.clone() {
		possible_places.remove(&place);
		if place.0 - 1 >= 0 && place.0 - 1 < grid[0].len() as i32 && place.1 >= 0 && place.1 < grid.len() as i32 && !grid[place.1 as usize][(place.0 - 1) as usize] {
			possible_places.insert((place.0 - 1, place.1));
		}
		if place.0 + 1 >= 0 && place.0 + 1 < grid[0].len() as i32 && place.1 >= 0 && place.1 < grid.len() as i32 && !grid[place.1 as usize][(place.0 + 1) as usize] {
			possible_places.insert((place.0 + 1, place.1));
		}
		if place.0 >= 0 && place.0 < grid[0].len() as i32 && place.1 - 1 >= 0 && place.1 - 1 < grid.len() as i32 && !grid[(place.1 - 1) as usize][place.0 as usize] {
			possible_places.insert((place.0, place.1 - 1));
		}
		if place.0 >= 0 && place.0 < grid[0].len() as i32 && place.1 + 1 >= 0 && place.1 + 1 < grid.len() as i32 && !grid[(place.1 + 1) as usize][place.0 as usize] {
			possible_places.insert((place.0, place.1 + 1));
		}
	}
}

pub fn part1(input: String) -> usize {
	let (grid, start) = parse(input);

	let mut possible_places = HashSet::new();
	possible_places.insert(start);
	for _ in 0..64 {
		iterate(&mut possible_places, &grid);
	}

	return possible_places.len();
}

pub fn part2(input: String) -> usize {
	let (grid, start) = parse(input);

	//Count how many places can be reached when starting in the middle of a tile.
	let mut possible_places = HashSet::new();
	possible_places.insert(start);
	for _ in 0..grid.len() {
		iterate(&mut possible_places, &grid);
	}
	let odd_count = possible_places.len();
	iterate(&mut possible_places, &grid);
	let even_count = possible_places.len();

	//Count how many places can be reached when starting from a corner of a tile.
	possible_places.clear();
	possible_places.insert(((grid.len() - 1) as i32, grid.len() as i32));
	possible_places.insert((0, grid.len() as i32));
	possible_places.insert((grid.len() as i32, 0));
	possible_places.insert((-1, 0));
	for _ in 0..grid.len() / 2 + 1 {
		iterate(&mut possible_places, &grid);
	}
	let corners_odd_count = possible_places.len() - 4;
	iterate(&mut possible_places, &grid);
	let corners_even_count = possible_places.len();

	let num_tiles = (26501365 - (grid.len() / 2)) / grid.len();
	return (num_tiles + 1) * (num_tiles + 1) * odd_count + num_tiles * num_tiles * even_count - (num_tiles + 1) * corners_odd_count + num_tiles * corners_even_count + 256;
}