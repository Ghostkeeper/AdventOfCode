use itertools::Itertools;
use std::collections::HashMap;
use regex::Regex;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
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

pub fn part2(input: String) -> String {
    let (states, connections) = parse(input);

    //Count the number of bits that this adder supports.
    let mut num_bits = 0;
    for wire in states.keys() {
        if wire.starts_with("x") {
            num_bits += 1;
        }
    }

    let mut is_valid = vec![true; connections.len()];
    let mut i = 0;
    for (left, right, operator, output) in &connections {
        //Check for some obvious errors to prune the search as much as we can.
		if output.starts_with("z") && *operator != Operator::XOR && *output != format!("z{}", num_bits) {
			//All outputs must be XOR (and cannot use AND and OR to construct XOR because there is no negation).
			is_valid[i] = false;
		} else if *operator == Operator::XOR && !left.starts_with("x") && !left.starts_with("y") && !right.starts_with("x") && !right.starts_with("y") && !output.starts_with("z") {
			//XORs can only be used for input and output. The rest of the operators are AND and OR.
			is_valid[i] = false;
		} else if *operator == Operator::XOR {
			//XOR must output into an AND of another XOR.
			for (other_left, other_right, other_operator, _) in &connections {
				if (*output == *other_left || *output == *other_right) && *other_operator == Operator::OR {
					is_valid[i] = false;
					break;
				}
			}
		} else if *operator == Operator::AND {
			//AND must output into OR.
			if (left == "x00" && right == "y00") || (left == "y00" && right == "x00") {
				//Except with bit 0 which must be an AND.
			} else {
				for (other_left, other_right, other_operator, _) in &connections {
					if (*output == *other_left || *output == *other_right) && *other_operator != Operator::OR {
						is_valid[i] = false;
						break;
					}
				}
			}
		}
        i += 1;
    }

	is_valid.iter().zip(connections).filter(|x| !*x.0).map(|x| x.1.3).sorted().join(",")
}