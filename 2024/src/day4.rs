use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut rows = vec!();
	for line in input.split("\n") {
		rows.push(line.chars().collect_vec());
	}
	return rows;
}

pub fn part1(input: String) -> u32 {
	let grid = parse(input);
	let search = ['X', 'M', 'A', 'S'];
	let directions = [[1, 0], [1, 1], [0, 1], [-1, 1], [-1, 0], [-1, -1], [0, -1], [1, -1]];

	let mut count = 0;
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			for d in directions {
				for i in 0..search.len() {
					let dx = (x as i32) + d[0] * (i as i32);
					let dy = (y as i32) + d[1] * (i as i32);
					if dy < 0 || (dy as usize) >= grid.len() {
						break;
					}
					if dx < 0 || (dx as usize) >= grid[dy as usize].len() {
						break;
					}
					if grid[dy as usize][dx as usize] != search[i] {
						break;
					}
					if i == search.len() - 1 {
						count += 1; //Found it!
					}
				}
			}
		}
	}
	return count;
}

pub fn part2(input: String) -> u32 {
	let grid = parse(input);
	let directions: [[i32; 2]; 4] = [[1, 1], [-1, 1], [-1, -1], [1, -1]];

	let mut count = 0;
	for y in 1..grid.len() - 1 {
		for x in 1..grid[y].len() - 1 {
			if grid[y][x] != 'A' {
				continue;
			}
			let mut num_mas = 0;
			for d in directions {
				let mx = (x as i32 + d[0]) as usize;
				let my = (y as i32 + d[1]) as usize;
				let sx = (x as i32 - d[0]) as usize;
				let sy = (y as i32 - d[1]) as usize;
				if grid[my][mx] == 'M' && grid[sy][sx] == 'S' {
					num_mas += 1;
				}
			}
			if num_mas == 2 {
				count += 1;
			}
		}
	}
	return count;
}