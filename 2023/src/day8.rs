use itertools::Itertools;

fn parse_graph(input: String) -> Vec<(String, usize, usize)> {
	let mut labels = vec!();
	for line in input.split("\n") {
		if line.len() < 5 {
			continue;
		}
		if line.chars().nth(4).unwrap() != '=' {
			continue;
		}
		let label = line.split(" = ").next().unwrap();
		labels.push(label);
	}

	let mut result = vec!();
	for line in input.split("\n") {
		if line.len() < 5 {
			continue;
		}
		if line.chars().nth(4).unwrap() != '=' {
			continue;
		}
		let mut assignment = line.split(" = ");
		let label = assignment.next().unwrap();
		let mut directions = assignment.next().unwrap().strip_prefix("(").unwrap().strip_suffix(")").unwrap().split(", ");
		let left = directions.next().unwrap();
		let right = directions.next().unwrap();
		result.push((
			label.to_string(),
			labels.iter().position(|l| *l == left).expect("Couldn't find left"),
			labels.iter().position(|l| *l == right).expect("Couldn't find right"),
		));
	}
	return result;
}

pub fn part1(input: String) {
	let graph = parse_graph(input.clone());
	let directions = input.split("\n").next().unwrap().chars().collect_vec();
	let mut position = graph.iter().position(|node| node.0 == "AAA").expect("There needs to be an AAA node.");
	let destination = graph.iter().position(|node| node.0 == "ZZZ").expect("There needs to be a ZZZ node.");
	let mut steps = 0;
	let mut instruction = 0;
	while position != destination {
		if directions[instruction] == 'L' {
			position = graph[position].1;
		} else if directions[instruction] == 'R' {
			position = graph[position].2;
		} else {
			panic!("Unknown direction.");
		}
		instruction = (instruction + 1) % directions.len();
		steps += 1;
	}
	println!("Steps: {}", steps);
}

pub fn part2(input: String) {
	let graph = parse_graph(input.clone());
	let directions = input.split("\n").next().unwrap().chars().collect_vec();
	let starts = graph.iter().positions(|node| node.0.ends_with("A")).collect_vec();
	let mut cycle_length = vec!();
	for start in starts {
		let mut steps = 0;
		let mut pos = start;
		while !graph[pos].0.ends_with("Z") {
			let direction = directions[steps % directions.len()];
			if direction == 'L' {
				pos = graph[pos].1;
			} else if direction == 'R' {
				pos = graph[pos].2;
			} else { panic!("Unknown direction."); }
			steps += 1;
		}
		cycle_length.push(steps);
	}

	let mut result: u64 = 1;
	for cycle in cycle_length {
		result = num::integer::lcm(cycle as u64, result);
	}
	println!("{}", result);
}