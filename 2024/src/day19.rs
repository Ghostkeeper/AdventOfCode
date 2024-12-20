use itertools::Itertools;
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;

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

fn num_arrangements(cache: &mut HashMap<String, u64>, design: &str, patterns: &Vec<&str>, start_index: usize) -> u64 {
	let remaining = design[start_index..].to_string();
	if cache.contains_key(&remaining) {
		return *cache.get(&remaining).unwrap();
	}
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
			result += num_arrangements(cache, &design, &patterns, end_index);
		}
	}
	cache.insert(remaining, result);
	return result;
}

pub fn part2(input: String) -> u64 {
	let (patterns, designs) = parse(input);
	let patterns_ref = patterns.iter().map(AsRef::as_ref).collect();
	let mut cache = HashMap::new();
	let mut sum = 0;
	for design in designs {
		let arrangements = num_arrangements(&mut cache, &design, &patterns_ref, 0);
		sum += arrangements;
	}
	return sum;
}