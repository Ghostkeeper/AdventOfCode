use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

enum Operator {
	OR,
	AND,
	XOR,
}

fn parse(input: String) -> (HashMap<String, bool>, Vec<(String, String, Operator, String)>) {
	let mut parts = input.split("\n\n");
	let states_part = parts.next().unwrap();
	let connections_part = parts.next().unwrap();

	let mut states = HashMap::new();
	for line in states_part.split("\n") {
		let mut state_parts = line.split(": ");
		let name = state_parts.next().unwrap();
		let state = state_parts.next().unwrap().parse::<u8>().unwrap() != 0;
		states.insert(name.to_string(), state);
	}

	let re = Regex::new(r"(?<left>[a-z0-9]+) (?<op>[A-Z]+) (?<right>[a-z0-9]+) -> (?<res>[a-z0-9]+)").unwrap();
	let connections = re.captures_iter(connections_part).map(|c| {
		let left = c.name("left").unwrap().as_str().to_string();
		let right = c.name("right").unwrap().as_str().to_string();
		let op = match c.name("op").unwrap().as_str() {
			"AND" => Operator::AND,
			"OR" => Operator::OR,
			"XOR" => Operator::XOR,
			_ => panic!("Unknown operator {}", c.name("op").unwrap().as_str()),
		};
		let res = c.name("res").unwrap().as_str().to_string();
		(left, right, op, res)
	}).collect_vec();

	(states, connections)
}

pub fn part1(input: String) -> u64 {
	let (mut states, connections) = parse(input);

	let mut changed = true;
	while changed {
		changed = false;
		for (left, right, operator, output) in &connections {
			if states.contains_key(left) && states.contains_key(right) && !states.contains_key(output) {
				let left_val = *states.get(left).unwrap();
				let right_val = *states.get(right).unwrap();
				let result = match operator {
					Operator::AND => left_val && right_val,
					Operator::OR => left_val || right_val,
					Operator::XOR => left_val ^ right_val,
				};
				states.insert(output.clone(), result);
				changed = true;
			}
		}
	}

	let mut result = 0;
	let mut i = 0;
	for (key, value) in states.into_iter().sorted() {
		if key.starts_with("z") {
			let this_bit = if value { 1 } else { 0 };
			result += this_bit << i;
			i += 1;
		}
	}

	return result;
}