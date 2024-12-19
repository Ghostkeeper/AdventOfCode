use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;

fn parse(input: String) -> (Vec<String>, Vec<String>) {
	let mut parts = input.split("\n\n");
	let patterns_part = parts.next().unwrap();
	let designs_part = parts.next().unwrap();

	let patterns = patterns_part.split(", ").map(|s| s.to_owned()).collect_vec();
	let designs = designs_part.split("\n").map(|s| s.to_owned()).collect_vec();

	(patterns, designs)
}

pub fn part1(input: String) -> u32 {
	let (patterns, designs) = parse(input);
	let re = Regex::new(&("^(".to_owned() + &patterns.join("|") + ")*$")).unwrap();
	designs.into_par_iter().map(|design| { re.is_match(&design) as u32 }).sum()
}

fn num_arrangements(design: &str, patterns: &Vec<&str>, start_index: usize) -> u64 {
	let mut result = 0;
	for &pattern in patterns {
		let end_index = start_index + pattern.len();
		if end_index > design.len() {
			continue; //This pattern is too long.
		}
		if design[start_index..(start_index + pattern.len())] != *pattern {
			continue; //Not a match.
		}
		//All characters match.
		if end_index == design.len() {
			result += 1;
		} else {
			result += num_arrangements(&design, &patterns, end_index);
		}
	}
	return result;
}

pub fn part2(input: String) -> u64 {
	let (patterns, designs) = parse(input);
	let patterns_ref = patterns.iter().map(AsRef::as_ref).collect();
	designs.into_par_iter().map(|design| {
		let arrangements = num_arrangements(&design, &patterns_ref, 0);
		println!("Design {} has {} arrangements", design, arrangements);
		arrangements
	}).sum()
}