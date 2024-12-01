use std::iter;

fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
	let mut left = vec!();
	let mut right = vec!();
	for line in input.split("\n") {
		let mut parts = line.split("   ");
		left.push(parts.next().unwrap().parse::<i32>().unwrap());
		right.push(parts.next().unwrap().parse::<i32>().unwrap());
	}
	return (left, right);
}

pub fn part1(input: String) -> i32 {
	let (mut left, mut right) = parse(input);
	left.sort();
	right.sort();

	let mut result = 0;
	for (l, r) in iter::zip(left, right) {
		result += (r - l).abs();
	}
	return result;
}