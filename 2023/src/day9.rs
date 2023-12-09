use itertools::Itertools;

fn parse(input: String) -> Vec<Vec<i32>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect_vec());
	}
	return result;
}

fn extrapolate(sequence: Vec<i32>) -> i32 {
	if sequence.iter().all(|x| *x == 0) {
		return 0;
	}

	let mut derivative = vec!();
	for i in 0..(sequence.len() - 1) {
		derivative.push(sequence[i + 1] - sequence[i]);
	}

	let next_derivative = extrapolate(derivative);
	return sequence[sequence.len() - 1] + next_derivative;
}

fn extrapolate_back(sequence: Vec<i32>) -> i32 {
	if sequence.iter().all(|x| *x == 0) {
		return 0;
	}

	let mut derivative = vec!();
	for i in 0..(sequence.len() - 1) {
		derivative.push(sequence[i + 1] - sequence[i]);
	}

	let prev_derivative = extrapolate_back(derivative);
	return sequence[0] - prev_derivative;
}

pub fn part1(input: String) {
	let sequences = parse(input);
	let mut sum = 0;
	for sequence in sequences {
		sum += extrapolate(sequence);
	}
	println!("{}", sum);
}

pub fn part2(input: String) {
	let sequences = parse(input);
	let mut sum = 0;
	for sequence in sequences {
		sum += extrapolate_back(sequence);
	}
	println!("{}", sum);
}