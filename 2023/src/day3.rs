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