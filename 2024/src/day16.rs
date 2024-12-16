use pathfinding::prelude::dijkstra;

fn parse(input: String) -> (Vec<Vec<char>>, (i32, i32), (i32, i32)) {
	let mut rows = vec!();
	let mut start_pos = (0, 0);
	let mut goal_pos = (0, 0);
	let mut y = 0;
	for line in input.split("\n") {
		let mut row = vec!();
		let mut x = 0;
		for char in line.chars() {
			if char == 'S' {
				row.push('.');
				start_pos = (x, y);
			} else if char == 'E' {
				row.push('.');
				goal_pos = (x, y);
			} else {
				row.push(char);
			}
			x += 1;
		}
		rows.push(row);
		y += 1;
	}

	return (rows, start_pos, goal_pos);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct State(i32, i32, u8);

impl State {
	fn successors(&self, grid: &Vec<Vec<char>>) -> Vec<(State, usize)> {
		let &State(x, y, dir) = self;
		let dir_v = match dir {
			0 => (1, 0),
			1 => (0, -1),
			2 => (-1, 0),
			3 => (0, 1),
			_ => panic!("Unknown direction encoded"),
		};
		let straight = State(x + dir_v.0, y + dir_v.1, dir);
		let turn_left = State(x, y, (dir + 1) % 4);
		let turn_right = State(x, y, (dir + 3) % 4);

		if grid[straight.1 as usize][straight.0 as usize] == '.' {
			return vec![(straight, 1), (turn_left, 1000), (turn_right, 1000)];
		} else {
			return vec![(turn_left, 1000), (turn_right, 1000)];
		}
	}
}

pub fn part1(input: String) -> usize {
	let (grid, pos, goal) = parse(input);
	let result = dijkstra(&State(pos.0, pos.1, 0), |s| s.successors(&grid), |s| s.0 == goal.0 && s.1 == goal.1);
	let (_path, cost) = result.expect("No path found.");

	return cost;
}