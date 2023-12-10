use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.chars().collect_vec());
	}
	return result;
}

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
	for y in 0..grid.len() {
		for x in 0..grid[y].len() {
			if grid[y][x] == 'S' {
				return (x, y);
			}
		}
	}
	panic!("Couldn't find start position!");
}

#[derive(PartialEq)]
enum Direction { EAST, NORTH, WEST, SOUTH }

fn follow(grid: &Vec<Vec<char>>, start: (usize, usize), start_direction: Direction) -> i32 {
	let mut position = start;
	let mut direction = start_direction;
	let mut num_steps = 0;

	//Keep stepping until we've found the start position (or hit a broken pipe).
	loop {
		num_steps += 1;
		if direction == Direction::EAST {
			position.0 += 1;
			direction = match grid[position.1][position.0] {
				'-' => Direction::EAST,
				'J' => Direction::NORTH,
				'7' => Direction::SOUTH,
				'S' => return num_steps,
				_ => panic!("Pipe doesn't connect!")
			}
		} else if direction == Direction::NORTH {
			position.1 -= 1;
			direction = match grid[position.1][position.0] {
				'|' => Direction::NORTH,
				'7' => Direction::WEST,
				'F' => Direction::EAST,
				'S' => return num_steps,
				_ => panic!("Pipe doesn't connect!")
			}
		} else if direction == Direction::WEST {
			position.0 -= 1;
			direction = match grid[position.1][position.0] {
				'-' => Direction::WEST,
				'F' => Direction::SOUTH,
				'L' => Direction::NORTH,
				'S' => return num_steps,
				_ => { panic!("Pipe doesn't connect!") }
			}
		} else {
			position.1 += 1;
			direction = match grid[position.1][position.0] {
				'|' => Direction::SOUTH,
				'J' => Direction::WEST,
				'L' => Direction::EAST,
				'S' => return num_steps,
				_ => panic!("Pipe doesn't connect!")
			}
		}
	}
}

pub fn part1(input: String) {
	let grid = parse(input);
	let start = find_start(&grid);
	let loop_size = follow(&grid, start, Direction::SOUTH);
	println!("{}", loop_size / 2);
}