use itertools::Itertools;
use regex::{Regex, RegexBuilder};

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

pub fn part2(input: String) -> u32 {
	let ignore_re = RegexBuilder::new(r"don't\(\).*?do\(\)").multi_line(true).dot_matches_new_line(true).build().unwrap();
	//Edge case: If input ends after don't() then the last part wouldn't get erased unless we add another do().
	let filtered = String::from(ignore_re.replace_all((input + "do()").as_str(), ""));
	return part1(filtered);
}