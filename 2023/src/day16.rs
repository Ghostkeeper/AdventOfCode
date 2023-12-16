use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.chars().collect_vec());
	}
	return result;
}

fn dir_to_vec(dir: usize) -> (i32, i32) {
	match dir % 4 {
		0 => (1, 0),
		1 => (0, -1),
		2 => (-1, 0),
		3 => (0, 1),
		_ => panic!("Modulo 4 can't be more than 4?"),
	}
}

fn in_bounds(head: (usize, usize, usize), delta: (i32, i32), grid: &Vec<Vec<char>>) -> bool {
	head.0 as i32 + delta.0 >= 0
		&& ((head.0 as i32 + delta.0) as usize) < grid[0].len()
		&& head.1 as i32 + delta.1 >= 0
		&& ((head.1 as i32 + delta.1) as usize) < grid.len()
}

pub fn part1(input: String) -> u32 {
	let grid = parse(input);
	let mut is_lit = vec!();
	for line in grid.iter() {
		is_lit.push(vec!());
		for _ in line {
			is_lit.last_mut().unwrap().push([false, false, false, false]);
		}
	}
	let mut beam_heads = vec!((0, 0, 0));
	while beam_heads.len() > 0 {
		let head = beam_heads.pop().unwrap();
		if is_lit[head.1][head.0][head.2] { //Was already lit. Don't go into infinite loops.
			continue;
		}
		is_lit[head.1][head.0][head.2] = true;
		match grid[head.1][head.0] {
			'.' => {
				let delta = dir_to_vec(head.2);
				if in_bounds(head, delta, &grid) {
					//Continue straight.
					beam_heads.push(((head.0 as i32 + delta.0) as usize, (head.1 as i32 + delta.1) as usize, head.2));
				}
			},
			'/' => {
				let new_dir = match head.2 {
					0 => 1,
					1 => 0,
					2 => 3,
					3 => 2,
					_ => panic!("Invalid direction!"),
				};
				let delta = dir_to_vec(new_dir);
				if in_bounds(head, delta, &grid) {
					beam_heads.push(((head.0 as i32 + delta.0) as usize, (head.1 as i32 + delta.1) as usize, new_dir));
				}
			},
			'\\' => {
				let new_dir = match head.2 {
					0 => 3,
					1 => 2,
					2 => 1,
					3 => 0,
					_ => panic!("Invalid direction!"),
				};
				let delta = dir_to_vec(new_dir);
				if in_bounds(head, delta, &grid) {
					beam_heads.push(((head.0 as i32 + delta.0) as usize, (head.1 as i32 + delta.1) as usize, new_dir));
				}
			},
			'-' => {
				if head.2 == 0 || head.2 == 2 {
					//Continue straight.
					let delta = dir_to_vec(head.2);
					if in_bounds(head, delta, &grid) {
						beam_heads.push(((head.0 as i32 + delta.0) as usize, (head.1 as i32 + delta.1) as usize, head.2));
					}
				} else {
					//Split into two.
					let west = dir_to_vec(2);
					if in_bounds(head, west, &grid) {
						beam_heads.push(((head.0 as i32 + west.0) as usize, (head.1 as i32 + west.1) as usize, 2));
					}
					let east = dir_to_vec(0);
					if in_bounds(head, east, &grid) {
						beam_heads.push(((head.0 as i32 + east.0) as usize, (head.1 as i32 + east.1) as usize, 0));
					}
				}
			}
			'|' => {
				if head.2 == 1 || head.2 == 3 {
					//Continue straight.
					let delta = dir_to_vec(head.2);
					if in_bounds(head, delta, &grid) {
						beam_heads.push(((head.0 as i32 + delta.0) as usize, (head.1 as i32 + delta.1) as usize, head.2));
					}
				} else {
					//Split into two.
					let north = dir_to_vec(1);
					if in_bounds(head, north, &grid) {
						beam_heads.push(((head.0 as i32 + north.0) as usize, (head.1 as i32 + north.1) as usize, 1));
					}
					let south = dir_to_vec(3);
					if in_bounds(head, south, &grid) {
						beam_heads.push(((head.0 as i32 + south.0) as usize, (head.1 as i32 + south.1) as usize, 3));
					}
				}
			}
			_ => panic!("Unknown grid character!"),
		}
	}
	//Now count how many grid cells we've lit up.
	let mut count = 0;
	for line in is_lit {
		for cell in line {
			if cell.iter().any(|c| *c) {
				count += 1;
			}
		}
	}
	return count;
}