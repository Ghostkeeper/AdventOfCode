use std::collections::HashMap;

fn parse(input: String) -> (HashMap<String, Vec<(char, char, u32, String)>>, Vec<HashMap<char, u32>>) {
	let mut workflows = HashMap::new();
	let mut parts = vec!();

	let mut chapters = input.split("\n\n");
	for workflow_str in chapters.next().unwrap().split("\n") {
		let mut name_and_rest = workflow_str.split("{");
		let name = name_and_rest.next().unwrap().to_string();
		let mut this_flows = vec!();
		let rest = name_and_rest.next().unwrap();
		for rule in rest[0..rest.len() - 1].split(",") {
			if rule.contains(":") {
				let mut condition_and_result = rule.split(":");
				let condition_str = condition_and_result.next().unwrap();
				let result = condition_and_result.next().unwrap();
				let condition_var = condition_str.chars().nth(0).unwrap();
				let condition_operator = condition_str.chars().nth(1).unwrap();
				let comparison = condition_str[2..].parse::<u32>().unwrap();
				this_flows.push((condition_var, condition_operator, comparison, result.to_string()));
			} else {
				this_flows.push(('x', '>', 0, rule.to_string())); //>0 is a tautology for my input.
			}
		}
		workflows.insert(name, this_flows);
	}

	for part_str in chapters.next().unwrap().split("\n") {
		let mut part = HashMap::new();
		for property_str in part_str[1..part_str.len() - 1].split(",") {
			let property = property_str.chars().next().unwrap();
			let value = property_str[2..].parse::<u32>().unwrap();
			part.insert(property, value);
		}
		parts.push(part);
	}

	return (workflows, parts);
}

pub fn part1(input: String) -> u32 {
	let (workflows, parts) = parse(input);

	let mut accepted = vec!();
	for part in parts {
		let mut workflow = workflows.get("in").unwrap();
		'flow_loop: loop {
			for rule in workflow {
				if rule.1 == '<' {
					if *part.get(&rule.0).unwrap() < rule.2 {
						if rule.3 == "A" {
							accepted.push(part.clone());
							break 'flow_loop;
						} else if rule.3 == "R" {
							break 'flow_loop;
						} else {
							workflow = workflows.get(rule.3.as_str()).unwrap();
							break;
						}
					}
				} else if rule.1 == '>' {
					if *part.get(&rule.0).unwrap() > rule.2 {
						if rule.3 == "A" {
							accepted.push(part.clone());
							break 'flow_loop;
						} else if rule.3 == "R" {
							break 'flow_loop;
						} else {
							workflow = workflows.get(rule.3.as_str()).unwrap();
							break;
						}
					}
				} else {
					panic!("Unknown comparison character!");
				}
			}
		}
	}

	let mut sum = 0;
	for part in accepted {
		sum += part.get(&'x').unwrap();
		sum += part.get(&'m').unwrap();
		sum += part.get(&'a').unwrap();
		sum += part.get(&'s').unwrap();
	}
	return sum;
}

fn num_accepted(workflows: &HashMap<String, Vec<(char, char, u32, String)>>, current: String, ranges: HashMap<char, (u32, u32)>) -> u128 {
	if current == "A" { //The whole range is accepted.
		return (ranges[&'x'].1 - ranges[&'x'].0 + 1) as u128
			* (ranges[&'m'].1 - ranges[&'m'].0 + 1) as u128
			* (ranges[&'a'].1 - ranges[&'a'].0 + 1) as u128
			* (ranges[&'s'].1 - ranges[&'s'].0 + 1) as u128;
	} else if current == "R" { //The whole range is rejected.
		return 0;
	}

	let mut accepted = 0;
	let workflow = workflows.get(&current).unwrap();
	let mut ranges_false = ranges.clone();
	for rule in workflow[0..workflow.len() - 1].iter() {
		let mut ranges_true = ranges_false.clone();
		let (rule_var, rule_operand, rule_value, destination) = rule;
		if *rule_operand == '<' {
			ranges_true.get_mut(rule_var).unwrap().1 = rule_value - 1;
			ranges_false.get_mut(rule_var).unwrap().0 = *rule_value;
		} else if *rule_operand == '>' {
			ranges_true.get_mut(rule_var).unwrap().0 = rule_value + 1;
			ranges_false.get_mut(rule_var).unwrap().1 = *rule_value;
		} else {
			panic!("Unknown operand!");
		}
		accepted += num_accepted(workflows, destination.clone(), ranges_true);
	}
	accepted += num_accepted(workflows, workflow.last().unwrap().3.clone(), ranges_false);

	return accepted;
}

pub fn part2(input: String) -> u128 {
	let (workflows, _) = parse(input);
	let mut total_range = HashMap::new();
	total_range.insert('x', (1, 4000));
	total_range.insert('m', (1, 4000));
	total_range.insert('a', (1, 4000));
	total_range.insert('s', (1, 4000));
	return num_accepted(&workflows, "in".to_string(), total_range);
}