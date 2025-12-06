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