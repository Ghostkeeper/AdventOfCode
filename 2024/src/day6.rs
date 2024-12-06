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