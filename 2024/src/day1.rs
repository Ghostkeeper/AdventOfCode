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

pub fn part2(input: String) -> i32 {
	let (left, right) = parse(input);
	//They probably want me to sort the lists and iterate over it linearly.
	//But with the power of Rust we can do it quadratically. Like a real programmer.

	let mut similarity_score = 0;
	for l in left {
		let occurrences = right.iter().filter(|&r| *r == l).count() as i32;
		similarity_score += l * occurrences;
	}
	return similarity_score;
}