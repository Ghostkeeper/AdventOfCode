use std::collections::HashMap;

fn parse(input: String) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
	let mut parts = input.split("\n\n");
	let rules_part = parts.next().unwrap();
	let manuals_part = parts.next().unwrap();

	let mut rules = HashMap::new();
	for rule in rules_part.split("\n") {
		let mut sides = rule.split("|");
		let left = sides.next().unwrap().parse::<u32>().unwrap();
		let right = sides.next().unwrap().parse::<u32>().unwrap();
		if !rules.contains_key(&left) {
			rules.insert(left, vec!());
		}
		rules.get_mut(&left).unwrap().push(right);
	}

	let mut manuals = vec!();
	for manual in manuals_part.split("\n") {
		let mut manual_vec = vec!();
		for page in manual.split(",") {
			manual_vec.push(page.parse::<u32>().unwrap());
		}
		manuals.push(manual_vec);
	}
	return (rules, manuals);
}

pub fn part1(input: String) -> u32 {
	let (rules, manuals) = parse(input);

	let mut sum = 0;
	for manual in manuals {
		let mut allowed = true;
		'pageloop: for i in 0..manual.len() {
			if rules.contains_key(&manual[i]) {
				let higher = rules.get(&manual[i]).unwrap();
				for j in 0..i {
					if higher.contains(&manual[j]) {
						allowed = false;
						break 'pageloop;
					}
				}
			}
		}
		if allowed {
			sum += manual[manual.len() / 2];
		}
	}
	return sum;
}