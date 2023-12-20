use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

fn parse(input: String) -> (Vec<Vec<bool>>, Vec<(i32, i32)>) {
	let mut rocks = vec!();
	let mut grid = vec!();
	for (y, line) in input.split("\n").enumerate() {
		let mut gridline = vec!();
		for (x, char) in line.chars().enumerate() {
			if char == '#' {
				gridline.push(true);
			} else {
				gridline.push(false);
				if char == 'O' {
					rocks.push((x as i32, y as i32));
				}
			}
		}
		grid.push(gridline);
	}
	return (grid, rocks);
}

fn tilt(grid: &Vec<Vec<bool>>, rocks: &mut Vec<(i32, i32)>, direction: (i32, i32)) {
	rocks.sort_by_key(|rock| (rock.0 * -1 * direction.0, rock.1 * -1 * direction.1));
	let mut blocked = vec!();
	for rock in rocks.iter_mut() {
		if true
			&& rock.1 + direction.1 >= 0 //In bounds North side.
			&& rock.1 + direction.1 < grid.len() as i32 //In bounds South side.
			&& rock.0 + direction.0 >= 0 //In bounds West side.
			&& rock.0 + direction.0 < grid[0].len() as i32 //In bounds East side.
			&& !grid[(rock.1 + direction.1) as usize][(rock.0 + direction.0) as usize] //Not hitting a square rock.
			&& !blocked.contains(&(rock.0 + direction.0, rock.1 + direction.1)) { //Not hitting a round rock.
			rock.0 += direction.0;
			rock.1 += direction.1;
		} else {
			blocked.push(*rock);
		}
	}
}

pub fn part1(input: String) -> i32 {
	let (grid, mut rocks) = parse(input);
	for _ in 0..grid.len() {
		tilt(&grid, &mut rocks, (0, -1));
	}
	let mut sum = 0;
	for rock in rocks {
		sum += grid.len() as i32 - rock.1;
	}
	return sum;
}

fn rumble(grid: &Vec<Vec<bool>>, rocks: &mut Vec<(i32, i32)>) {
	for _ in 0..grid.len() { //Tilt North.
		tilt(&grid, rocks, (0, -1));
	}
	for _ in 0..grid[0].len() { //Tilt West.
		tilt(&grid, rocks, (-1, 0));
	}
	for _ in 0..grid.len() { //Tilt South.
		tilt(&grid, rocks, (0, 1));
	}
	for _ in 0..grid[0].len() { //Tilt East.
		tilt(&grid, rocks, (1, 0));
	}
}

pub fn part2(input: String) -> i32 {
	let (grid, mut rocks) = parse(input);
	let mut hashes = vec!();
	for rumbling in 0..1000000000 {
		rumble(&grid, &mut rocks);
		let mut hasher = DefaultHasher::new();
		rocks.hash(&mut hasher);
		let hash = hasher.finish();
		let position = hashes.iter().position(|&h| h == hash);
		if position.is_some() {
			let cycle_start = position.unwrap();
			let cycle_length = rumbling - cycle_start;
			let leftover = (1000000000 - cycle_start) % cycle_length - 1;
			for _ in 0..leftover {
				rumble(&grid, &mut rocks);
			}
			let mut sum = 0;
			for rock in rocks {
				sum += grid.len() as i32 - rock.1;
			}
			return sum;
		}
		hashes.push(hash);
	}
	let mut sum = 0;
	for rock in rocks {
		sum += grid.len() as i32 - rock.1;
	}
	return sum;
}