fn parse(input: String) -> (Vec<i64>, Vec<i64>) {
	let mut time = vec!();
	let mut space = vec!();
	for line in input.split("\n") {
		if line.starts_with("Time:") {
			time = line[9..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
		}
		if line.starts_with("Distance:") {
			space = line[9..].split_whitespace().map(|s| s.parse::<i64>().unwrap()).collect();
		}
	}
	return (time, space);
}

fn dist(time: i64, hold: i64) -> i64 {
	return (time - hold) * hold;
}

pub fn part1(input: String) -> usize {
	let (time, space) = parse(input);

	let mut results = vec!(0; time.len());
	for i in 0..time.len() {
		let mut num_win = 0usize;
		for hold in 0..time[i] {
			if dist(time[i], hold) > space[i] {
				num_win += 1;
			}
		}
		results[i] = num_win;
	}
	let mut result = 1;
	for wins in results {
		result *= wins;
	}
	return result;
}

fn parse_dumb(input: String) -> (i64, i64) {
	let mut time = 0i64;
	let mut space = 0i64;
	for line in input.split("\n") {
		if line.starts_with("Time:") {
			time = line[9..].replace(" ", "").parse::<i64>().unwrap();
		}
		if line.starts_with("Distance:") {
			space = line[9..].replace(" ", "").parse::<i64>().unwrap();
		}
	}
	return (time, space);
}

pub fn part2(input: String) -> usize {
	let (time, space) = parse_dumb(input);

	let mut num_win = 0usize;
	for hold in 0..time {
		if dist(time, hold) > space {
			num_win += 1;
		}
	}
	return num_win;
}