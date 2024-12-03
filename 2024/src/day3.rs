use itertools::Itertools;
use regex::Regex;

pub fn part1(input: String) -> u32 {
	let re = Regex::new(r"mul\((?<a>\d{1,3}),(?<b>\d{1,3})\)").unwrap();
	let operands = re.captures_iter(input.as_str()).map(|caps| {
		let a = caps.name("a").unwrap().as_str().parse::<u32>().unwrap();
		let b = caps.name("b").unwrap().as_str().parse::<u32>().unwrap();
		(a, b)
	}).collect_vec();

	let mut sum = 0;
	for (a, b) in operands {
		sum += a * b;
	}
	return sum;
}