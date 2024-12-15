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

fn can_push(grid: &Vec<Vec<char>>, position: (i32, i32), step: (i32, i32)) -> bool {
	let push_to = ((position.0 + step.0) as usize, (position.1 + step.1) as usize);
	if grid[push_to.1][push_to.0] == '.' {
		return true;
	}
	if grid[push_to.1][push_to.0] == '#' {
		return false;
	}
	if grid[push_to.1][push_to.0] == '[' {
		if step.1 != 0 { //Vertical movement.
			return can_push(&grid, (push_to.0 as i32, push_to.1 as i32), step)
				&& can_push(&grid, (push_to.0 as i32 + 1, push_to.1 as i32), step);
		} else {
			return can_push(&grid, (position.0 + step.0 * 2, position.1), step);
		}
	}
	if grid[push_to.1][push_to.0] == ']' {
		if step.1 != 0 { //Vertical movement.
			return can_push(&grid, (push_to.0 as i32, push_to.1 as i32), step)
				&& can_push(&grid, (push_to.0 as i32 - 1, push_to.1 as i32), step);
		} else {
			return can_push(&grid, (position.0 + step.0 * 2, position.1), step);
		}
	}
	panic!();
}

fn push(grid: &mut Vec<Vec<char>>, position: &mut (i32, i32), step: (i32, i32)) {
	let push_to = ((position.0 + step.0) as usize, (position.1 + step.1) as usize);
	if grid[push_to.1][push_to.0] == '.' {
		grid[push_to.1][push_to.0] = grid[position.1 as usize][position.0 as usize];
		grid[position.1 as usize][position.0 as usize] = '.';
		position.0 += step.0;
		position.1 += step.1;
	} else if grid[push_to.1][push_to.0] == '#' {
		panic!("I thought we could push to this place!");
	} else if grid[push_to.1][push_to.0] == '[' {
		if step.1 != 0 { //Vertical movement.
			push(grid, &mut (position.0 + step.0, position.1 + step.1), step);
			push(grid, &mut (position.0 + step.0 + 1, position.1 + step.1), step);
			push(grid, position, step); //Try again.
		} else {
			push(grid, &mut (position.0 + step.0, position.1), step);
			push(grid, position, step); //Try again.
		}
	} else if grid[push_to.1][push_to.0] == ']' {
		if step.1 != 0 { //Vertical movement.
			push(grid, &mut (position.0 + step.0, position.1 + step.1), step);
			push(grid, &mut (position.0 + step.0 - 1, position.1 + step.1), step);
			push(grid, position, step); //Try again.
		} else {
			push(grid, &mut (position.0 + step.0, position.1), step);
			push(grid, position, step); //Try again.
		}
	}
}

pub fn part2(input: String) -> usize {
	let (mut grid, moves, mut robot) = parse(input);
	//Double up the grid.
	robot.0 *= 2;
	for y in 0..grid.len() {
		let mut new_row = vec!();
		for cell in &grid[y] {
			if *cell == 'O' {
				new_row.push('[');
				new_row.push(']');
			} else {
				new_row.push(*cell);
				new_row.push(*cell);
			}
		}
		grid[y] = new_row;
	}

	for step in moves {
		if can_push(&grid, robot.clone(), step) {
			push(&mut grid, &mut robot, step);
		}
	}

	let mut sum_gps = 0;
	for y in 0..grid.len() {
		for x in 0..grid[0].len() {
			if grid[y][x] == '[' {
				sum_gps += y * 100 + x;
			}
		}
	}
	return sum_gps;
}