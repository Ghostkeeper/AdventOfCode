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