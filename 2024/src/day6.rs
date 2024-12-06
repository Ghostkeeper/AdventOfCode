use std::collections::HashSet;

fn parse(input: String) -> (Vec<Vec<char>>, (usize, usize), (i32, i32)) {
	let mut rows = vec!();
	let mut pos = (0, 0);
	let mut dir = (0, 0);
	let mut y = 0;
	for line in input.split("\n") {
		let mut row = vec!();
		let mut x = 0;
		for c in line.chars() {
			match c {
				'.' => row.push(c),
				'#' => row.push(c),
				'v' => { row.push('.'); pos = (x, y); dir = (0, 1) },
				'<' => { row.push('.'); pos = (x, y); dir = (-1, 0) },
				'^' => { row.push('.'); pos = (x, y); dir = (0, -1) },
				'>' => { row.push('.'); pos = (x, y); dir = (1, 0) },
				_ => panic!()
			}
			x += 1;
		}
		rows.push(row);
		y += 1;
	}
	return (rows, pos, dir);
}

pub fn part1(input: String) -> u32 {
	let (grid, mut pos, mut dir) = parse(input);
	let mut visited = HashSet::new();
	loop {
		visited.insert(pos);
		let new_pos = ((pos.0 as i32 + dir.0) as usize, ((pos.1 as i32) + dir.1) as usize);
		if new_pos.0 >= grid[0].len() || new_pos.1 >= grid[0].len() { //Out of bounds.
			return visited.len() as u32;
		} else if grid[new_pos.1][new_pos.0] == '#' { //Obstructed.
			dir = (-dir.1, dir.0); //Rotate right.
		} else { //Not obstructed.
			pos = new_pos;
		}
	}
}

pub fn part2(input: String) -> u32 {
	let mut result = 0;

	let (original_grid, start_pos, start_dir) = parse(input);
	for obs_y in 0..original_grid.len() {
		for obs_x in 0..original_grid[0].len() {
			let mut grid = original_grid.clone();
			if (obs_x, obs_y) == start_pos {
				continue; //Not allowed to obstruct the starting position.
			}
			let mut pos = start_pos;
			let mut dir = start_dir;
			grid[obs_y][obs_x] = '#';
			//Now let's simulate to see if we get in a loop.
			let mut visited = HashSet::new();
			loop {
				if visited.contains(&(pos, dir)) { //Been here before. This was a loop!
					result += 1;
					break;
				}
				visited.insert((pos, dir));
				let new_pos = ((pos.0 as i32 + dir.0) as usize, ((pos.1 as i32) + dir.1) as usize);
				if new_pos.0 >= grid[0].len() || new_pos.1 >= grid[0].len() { //Out of bounds. Didn't loop.
					break;
				} else if grid[new_pos.1][new_pos.0] == '#' { //Obstructed.
					dir = (-dir.1, dir.0); //Rotate right.
				} else { //Not obstructed.
					pos = new_pos;
				}
			}
		}
	}

	return result;
}