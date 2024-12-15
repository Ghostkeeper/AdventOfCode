fn parse(input: String) -> (Vec<Vec<char>>, Vec<(i32, i32)>, (i32, i32)) {
	let mut parts = input.split("\n\n");
	let gridpart = parts.next().unwrap();
	let movespart = parts.next().unwrap();

	let mut rows = vec!();
	let mut y = 0;
	let mut robot = (0, 0);
	for line in gridpart.split("\n") {
		let mut x = 0;
		let mut row = vec!();
		for cell in line.chars() {
			if cell == '@' {
				robot.0 = x;
				robot.1 = y;
				row.push('.');
			} else {
				row.push(cell);
			}
			x += 1;
		}
		rows.push(row);
		y += 1;
	}

	let mut moves = vec!();
	for step in movespart.replace("\n", "").chars() {
		match step {
			'<' => moves.push((-1, 0)),
			'^' => moves.push((0, -1)),
			'>' => moves.push((1, 0)),
			'v' => moves.push((0, 1)),
			_ => panic!(),
		}
	}

	return (rows, moves, robot);
}

fn print(grid: &Vec<Vec<char>>, robot: &(i32, i32)) {
	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if (robot.0 as usize) == x && (robot.1 as usize) == y {
				print!("@");
			} else {
				print!("{}", grid[y][x]);
			}
		}
		println!();
	}
}

pub fn part1(input: String) -> usize {
	let (mut grid, moves, mut robot) = parse(input);

	for step in moves {
		let mut free_pos = robot.clone();
		while (free_pos.0 + step.0) >= 0 && ((free_pos.0 + step.0) as usize) < grid[0].len() && (free_pos.1 + step.1) >= 0 && ((free_pos.1 + step.1) as usize) < grid.len()
				&& grid[(free_pos.1 + step.1) as usize][(free_pos.0 + step.0) as usize] == 'O' {
			free_pos.0 += step.0;
			free_pos.1 += step.1;
		}
		free_pos.0 += step.0;
		free_pos.1 += step.1;
		if free_pos.0 < 0 || (free_pos.0 as usize) > grid[0].len() || free_pos.1 < 0 || (free_pos.1 as usize) > grid.len() {
			//Out of bounds. So there is no free pos.
			continue; //Don't move the robot.
		}
		if grid[free_pos.1 as usize][free_pos.0 as usize] == '#' {
			//Hit against a wall. Don't move the robot.
			continue;
		}
		let first_step = (robot.0 + step.0, robot.1 + step.1);
		grid[free_pos.1 as usize][free_pos.0 as usize] = grid[first_step.1 as usize][first_step.0 as usize]; //Move the row of boxes.
		grid[first_step.1 as usize][first_step.0 as usize] = '.';
		robot = first_step; //Move the robot itself.
	}

	let mut sum_gps = 0;
	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if grid[y][x] == 'O' {
				sum_gps += y * 100 + x;
			}
		}
	}
	return sum_gps;
}