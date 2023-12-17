use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn parse(input: String) -> Vec<Vec<u32>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(vec!());
		for char in line.chars() {
			result.last_mut().unwrap().push(char as u32 - 48);
		}
	}
	return result;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
	x: usize,
	y: usize,
	straight_steps: usize,
	straight_dir: usize,
	cost: u32,
}
impl Ord for State {
	fn cmp(&self, other: &Self) -> Ordering {
		other.cost.cmp(&self.cost)
			.then_with(|| other.straight_steps.cmp(&self.straight_steps))
			.then_with(|| self.x.cmp(&other.x))
			.then_with(|| self.y.cmp(&other.y))
			.then_with(|| self.straight_dir.cmp(&other.straight_dir))
	}
}
impl PartialOrd for State {
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

pub fn part1(input: String) -> u32 {
	let grid = parse(input);
	let mut distance = vec!();
	for _ in 0..grid.len() {
		distance.push(vec!());
		for _ in 0..grid[0].len() {
			distance.last_mut().unwrap().push(vec!());
			for _ in 0..4 {
				distance.last_mut().unwrap().last_mut().unwrap().push(vec!());
				for _ in 0..4 {
					distance.last_mut().unwrap().last_mut().unwrap().last_mut().unwrap().push(u32::MAX);
				}
			}
		}
	}

	let mut queue = BinaryHeap::new();
	queue.push(State {
		x: 0,
		y: 0,
		straight_steps: 0,
		straight_dir: 0,
		cost: 0,
	});
	while let Some(state) = queue.pop() {
		if state.x == grid[0].len() - 1 && state.y == grid.len() - 1 {
			return state.cost; //This is the shortest path.
		}
		if state.cost > distance[state.y][state.x][state.straight_dir][state.straight_steps] {
			continue; //Already found a shorter path to this cell.
		}
		for (dir, delta) in [(1, 0), (0, -1), (-1, 0), (0, 1)].iter().enumerate() {
			if state.x as i32 + delta.0 < 0
				|| (state.x as i32 + delta.0) as usize >= grid[0].len()
				|| state.y as i32 + delta.1 < 0
				|| (state.y as i32 + delta.1) as usize >= grid.len() {
				continue; //Out of bounds.
			}
			if dir == (state.straight_dir + 2) % 4 {
				continue; //Can't reverse direction.
			}
			if dir == state.straight_dir && state.straight_steps >= 3 {
				continue; //Can't go straight for more than 3 steps.
			}
			let new_x = (state.x as i32 + delta.0) as usize;
			let new_y = (state.y as i32 + delta.1) as usize;
			let new_cost = state.cost + grid[new_y][new_x];
			let new_steps = if dir == state.straight_dir { state.straight_steps + 1 } else { 1 };
			if new_cost < distance[new_y][new_x][dir][new_steps] {
				let next = State {
					x: new_x,
					y: new_y,
					straight_steps: new_steps,
					straight_dir: dir,
					cost: new_cost,
				};
				queue.push(next);
				distance[new_y][new_x][dir][new_steps] = new_cost; //Found a new shortest path.
			}
		}
	}

	panic!("No path found!");
}