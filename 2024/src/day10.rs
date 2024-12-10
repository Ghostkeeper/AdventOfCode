use std::collections::HashSet;
use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<i32>> {
	let mut rows = vec!();
	for line in input.split("\n") {
		rows.push(line.chars().map(|c| c as i32 - '0' as i32).collect_vec());
	}
	return rows;
}

fn find_peaks(grid: &Vec<Vec<i32>>, peaks: &mut HashSet<(usize, usize)>, pos: (usize, usize)) {
	let (x, y) = pos;
	let height = grid[y][x];
	if height == 9 {
		peaks.insert(pos);
		return;
	}
	for (dx, dy) in [(1i32, 0i32), (0, -1), (-1, 0), (0, 1)] {
		let nx = x.overflowing_add(dx as usize).0;
		let ny = y.overflowing_add(dy as usize).0;
		if nx < grid[y].len() && ny < grid.len() && grid[ny][nx] == height + 1 {
			find_peaks(grid, peaks, (nx, ny));
		}
	}
}

pub fn part1(input: String) -> u32 {
	let grid = parse(input);

	let mut sum = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] != 0 {
				continue; //Start at 0.
			}
			let mut peaks = HashSet::new();
			find_peaks(&grid, &mut peaks, (x, y));
			sum += peaks.len() as u32;
		}
	}

	return sum;
}

fn find_trails(grid: &Vec<Vec<i32>>, pos: (usize, usize)) -> u32 {
	let (x, y) = pos;
	let height = grid[y][x];
	if height == 9 {
		return 1;
	}
	let mut sum = 0;
	for (dx, dy) in [(1i32, 0i32), (0, -1), (-1, 0), (0, 1)] {
		let nx = x.overflowing_add(dx as usize).0;
		let ny = y.overflowing_add(dy as usize).0;
		if nx < grid[y].len() && ny < grid.len() && grid[ny][nx] == height + 1 {
			sum += find_trails(grid, (nx, ny));
		}
	}
	return sum;
}

pub fn part2(input: String) -> u32 {
	let grid = parse(input);

	let mut sum = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] != 0 {
				continue; //Start at 0.
			}
			sum += find_trails(&grid, (x, y));
		}
	}

	return sum;
}