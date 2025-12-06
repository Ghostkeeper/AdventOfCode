fn parse(input: String) -> Vec<(Vec<u64>, char)> {
	let mut results = vec!();
	for line in input.split("\n") {
		for (i, column) in line.split_whitespace().enumerate() {
			while results.len() <= i {
				results.push((vec!(), '_'));
			}
			let parsed = column.parse::<u64>();
			if parsed.is_ok() {
				results[i].0.push(column.parse::<u64>().unwrap());
			} else {
				results[i].1 = column.chars().next().unwrap();
			}
		}
	}
	results
}

fn multiplication(input: &Vec<u64>) -> u64 {
	let mut result = 1;
	for x in input {
		result *= x;
	}
	result
}

pub fn part1(input: String) -> u64 {
    let table = parse(input);
	let mut total = 0;

	for exercise in table {
		let result = match exercise.1 {
			'*' => multiplication(&exercise.0),
			'+' => exercise.0.iter().sum(),
			_ => panic!(),
		};
		total += result;
	}

    total
}

fn parse_part2(input: String) -> Vec<(Vec<u64>, char)> {
	let mut results = vec!();
	let mut lines = input.split("\n").collect::<Vec<&str>>();
	lines.pop();
	for i in 0..lines[0].len() {
		let operator = lines.last().unwrap().chars().nth(i).unwrap();
		if operator != ' ' {
			results.push((vec!(), operator));
		}
		let mut number = "".to_owned();
		for line in lines.iter().take(lines.len() - 1) {
			number.push(line.chars().nth(i).unwrap());
		}
		if number.trim() == "" {
			continue;
		}
		results.last_mut().unwrap().0.push(number.trim().parse::<u64>().unwrap());
	}
	results
}

pub fn part2(input: String) -> u64 {
    let table = parse_part2(input);
	let mut total = 0;

	for exercise in table {
		let result = match exercise.1 {
			'*' => multiplication(&exercise.0),
			'+' => exercise.0.iter().sum(),
			_ => panic!(),
		};
		total += result;
	}

    total
}