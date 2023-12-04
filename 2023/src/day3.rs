use std::collections::HashMap;

fn to_grid(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(vec!());
		let line_nr = result.len() - 1;
		for c in line.chars() {
			result.get_mut(line_nr).unwrap().push(c);
		}
	}
	return result;
}

pub fn is_symbol(c: &char) -> bool {
	!c.is_digit(10) && *c != '.'
}

pub fn part1(input: String) {
	let mut sum = 0;
	let grid = to_grid(input);
	for (line_nr, line) in grid.iter().enumerate() {
		let mut part_nr = 0;
		let mut register = false;
		for (i, c) in line.iter().enumerate() {
			if c.is_digit(10) {
				if i > 0 {
					if line_nr > 0 {
						register |= is_symbol(&grid[line_nr - 1][i - 1]);
					}
					register |= is_symbol(&grid[line_nr][i - 1]);
					if line_nr < grid.len() - 1 {
						register |= is_symbol(&grid[line_nr + 1][i - 1]);
					}
				}
				if line_nr > 0 {
					register |= is_symbol(&grid[line_nr - 1][i]);
				}
				if line_nr < grid.len() - 1 {
					register |= is_symbol(&grid[line_nr + 1][i]);
				}
				if i < grid[line_nr].len() - 1 {
					if line_nr > 0 {
						register |= is_symbol(&grid[line_nr - 1][i + 1]);
					}
					register |= is_symbol(&grid[line_nr][i + 1]);
					if line_nr < grid.len() - 1 {
						register |= is_symbol(&grid[line_nr + 1][i + 1]);
					}
				}
				part_nr = part_nr * 10 + (*c as i32) - 48;
			} else {
				if register {
					sum += part_nr;
				}
				part_nr = 0;
				register = false;
			}
		}
		if register {
			sum += part_nr;
		}
	}
	println!("{}", sum);
}

fn register_gear(partial_gears: &mut HashMap<(i32, i32), i32>, sum: &mut i32, x: i32, y: i32, part_nr: i32) {
	if partial_gears.contains_key(&(x, y)) {
		*sum += partial_gears.get(&(x, y)).unwrap() * part_nr;
		partial_gears.remove(&(x, y));
	} else {
		partial_gears.insert((x, y), part_nr);
	}
}

pub fn part2(input: String) {
	let mut sum = 0;
	let mut partial_gears: HashMap<(i32, i32), i32> = HashMap::new();
	let grid = to_grid(input);
	for (line_nr, line) in grid.iter().enumerate() {
		let mut part_nr = 0;
		let mut x: i32 = -1;
		let mut y: i32 = -1;
		for (i, c) in line.iter().enumerate() {
			if c.is_digit(10) {
				if i > 0 {
					if line_nr > 0 {
						if grid[line_nr - 1][i - 1] == '*' {
							x = (line_nr - 1) as i32;
							y = (i - 1) as i32;
						}
					}
					if grid[line_nr][i - 1] == '*' {
						x = line_nr as i32;
						y = (i - 1) as i32;
					}
					if line_nr < grid.len() - 1 {
						if grid[line_nr + 1][i - 1] == '*' {
							x = (line_nr + 1) as i32;
							y = (i - 1) as i32;
						}
					}
				}
				if line_nr > 0 {
					if grid[line_nr - 1][i] == '*' {
						x = (line_nr - 1) as i32;
						y = i as i32;
					}
				}
				if line_nr < grid.len() - 1 {
					if grid[line_nr + 1][i] == '*' {
						x = (line_nr + 1) as i32;
						y = i as i32;
					}
				}
				if i < grid[line_nr].len() - 1 {
					if line_nr > 0 {
						if grid[line_nr - 1][i + 1] == '*' {
							x = (line_nr - 1) as i32;
							y = (i + 1) as i32;
						}
					}
					if grid[line_nr][i + 1] == '*' {
						x = line_nr as i32;
						y = (i + 1) as i32;
					}
					if line_nr < grid.len() - 1 {
						if grid[line_nr + 1][i + 1] == '*' {
							x = (line_nr + 1) as i32;
							y = (i + 1) as i32;
						}
					}
				}
				part_nr = part_nr * 10 + (*c as i32) - 48;
			} else {
				if x != -1 {
					register_gear(&mut partial_gears, &mut sum, x, y, part_nr);
				}
				x = -1;
				y = -1;
				part_nr = 0;
			}
		}
		if x != -1 {
			register_gear(&mut partial_gears, &mut sum, x, y, part_nr);
		}
	}
	println!("{}", sum);
}