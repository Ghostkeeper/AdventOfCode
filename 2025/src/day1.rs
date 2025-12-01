fn parse(input: String) -> Vec<i32> {
	let mut result = vec!();
	for line in input.split("\n") {
		if line.chars().next().unwrap() == 'L' {
			result.push(0 - line[1..].parse::<i32>().unwrap());
		} else {
			result.push(line[1..].parse::<i32>().unwrap());
		}
	}
	result
}

pub fn part1(input: String) -> u32 {
	let mut result = 0;
	let steps = parse(input);
	let mut dial = 50;
	for step in steps {
		dial += step;
		if dial % 100 == 0 {
			result += 1;
		}
	}
	result
}

pub fn part2(input: String) -> u32 {
	let mut result = 0;
	let steps = parse(input);
	let mut dial = 50;
	for step in steps {
		let direction = step.signum();
		let magnitude = step.abs();
		for _ in 0..magnitude {
			dial += direction;
			if dial % 100 == 0 {
				result += 1;
			}
		}
	}
	result
}