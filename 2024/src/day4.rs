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