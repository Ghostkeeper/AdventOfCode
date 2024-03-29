use itertools::Itertools;
use num::abs;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.chars().collect_vec());
	}
	return result;
}

fn find_empty(universe: &Vec<Vec<char>>) -> (Vec<i64>, Vec<i64>) {
	let mut empty_rows = vec!();
	let mut empty_columns = vec!();

	for row in 0..universe.len() {
		if universe[row].iter().all(|c| *c == '.') {
			empty_rows.push(row as i64);
		}
	}
	'outer: for column in 0..universe[0].len() {
		for row in 0..universe.len() {
			if universe[row][column] == '#' {
				continue 'outer;
			}
		}
		empty_columns.push(column as i64);
	}
	return (empty_rows, empty_columns);
}

fn find_galaxies(universe: &Vec<Vec<char>>, empty_rows: Vec<i64>, empty_columns: Vec<i64>, expansion_factor: i64) -> Vec<(i64, i64)> {
	let mut result = vec!();
	let mut expanded_y = 0i64;
	for (y, row) in universe.iter().enumerate() {
		if empty_rows.contains(&(y as i64)) {
			expanded_y += expansion_factor;
		} else {
			expanded_y += 1;
		}
		let mut expanded_x = 0i64;
		for (x, cell) in row.iter().enumerate() {
			if empty_columns.contains(&(x as i64)) {
				expanded_x += expansion_factor;
			} else {
				expanded_x += 1;
			}
			if *cell == '#' {
				result.push((expanded_x, expanded_y));
			}
		}
	}
	return result;
}

pub fn part1(input: String) -> i64 {
	let universe = parse(input);
	let (empty_rows, empty_columns) = find_empty(&universe);
	let galaxies = find_galaxies(&universe, empty_rows, empty_columns, 2);

	let mut sum = 0;
	for galaxy1_id in 0..galaxies.len() {
		for galaxy2_id in 0..galaxy1_id {
			let galaxy1 = galaxies[galaxy1_id];
			let galaxy2 = galaxies[galaxy2_id];
			let distance = abs(galaxy1.0 - galaxy2.0) + abs(galaxy1.1 - galaxy2.1);
			sum += distance;
		}
	}
	return sum;
}

pub fn part2(input: String) -> i64 {
	let universe = parse(input);
	let (empty_rows, empty_columns) = find_empty(&universe);
	let galaxies = find_galaxies(&universe, empty_rows, empty_columns, 1000000);

	let mut sum = 0i64;
	for galaxy1_id in 0..galaxies.len() {
		for galaxy2_id in 0..galaxy1_id {
			let galaxy1 = galaxies[galaxy1_id];
			let galaxy2 = galaxies[galaxy2_id];
			let distance = abs(galaxy1.0 - galaxy2.0) + abs(galaxy1.1 - galaxy2.1);
			sum += distance;
		}
	}
	return sum;
}