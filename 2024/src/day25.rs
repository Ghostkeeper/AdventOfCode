use itertools::Itertools;

fn parse(input: String) -> (Vec<Vec<i32>>, Vec<Vec<i32>>) {
	let mut keys = vec!();
	let mut locks = vec!();
	for schema in input.split("\n\n") {
		let rows = schema.split("\n").collect_vec();
		let is_lock = rows[0].chars().next().unwrap() == '#';
		let mut columns = vec![-1; rows[0].len()];
		for row in rows {
			for i in 0..row.len() {
				if row.chars().nth(i).unwrap() == '#' {
					columns[i] += 1;
				}
			}
		}
		if is_lock {
			locks.push(columns);
		} else {
			keys.push(columns);
		}
	}

	(locks, keys)
}

pub fn part1(input: String) -> u64 {
	let (locks, keys) = parse(input);

	let mut num_combinations = 0;
	for lock in locks {
		'keyloop: for key in keys.iter() {
			for i in 0..lock.len() {
				if lock[i] + key[i] > 5 {
					continue 'keyloop;
				}
			}
			num_combinations += 1;
		}
	}
	return num_combinations;
}