use itertools::Itertools;

fn parse(input: String) -> (Vec<Vec<char>>, Vec<Vec<i32>>) {
	let mut fields = vec!();
	let mut sequences = vec!();

	for line in input.split("\n") {
		let mut parts = line.split_whitespace();
		fields.push(parts.next().unwrap().chars().collect_vec());
		sequences.push(parts.next().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect_vec());
	}
	return (fields, sequences);
}

fn is_valid(field: &Vec<char>, sequence: &Vec<i32>) -> bool {
	let mut pos = 0usize;
	for streak in sequence {
		//Find when the streak starts.
		while pos < field.len() && field[pos] == '.' {
			pos += 1;
		}
		//Find this streak's length.
		let mut length = 0;
		while pos < field.len() && field[pos] == '#' {
			length += 1;
			pos += 1;
		}
		if length != *streak {
			return false; //Streak is too short.
		}
	}
	//Find if there are any more streaks after the last.
	while pos < field.len() {
		if field[pos] == '#' {
			return false; //Too many streaks.
		}
		pos += 1;
	}
	return true;
}

fn num_valid(field: &Vec<char>, sequence: &Vec<i32>) -> usize {
	let num_unknown = field.iter().filter(|c| **c == '?').count();
	let mut valid = 0usize;
	for bitfield in 0usize..(1 << num_unknown) {
		let mut filled_in = field.clone();
		let mut bit = 0;
		for (pos, char) in field.iter().enumerate() {
			if *char == '?' {
				if bitfield & (1 << bit) > 0 {
					filled_in[pos] = '#';
				} else {
					filled_in[pos] = '.';
				}
				bit += 1;
			}
		}
		if is_valid(&filled_in, sequence) {
			valid += 1;
		}
	}
	return valid;
}

pub fn part1(input: String) {
	let (fields, sequences) = parse(input);
	let mut sum_arrangements = 0;
	for i in 0..fields.len() {
		let num = num_valid(&fields[i], &sequences[i]);
		sum_arrangements += num;
	}
	println!("{}", sum_arrangements)
}