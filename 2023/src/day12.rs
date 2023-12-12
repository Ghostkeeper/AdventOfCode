use itertools::Itertools;

fn parse(input: String) -> (Vec<String>, Vec<Vec<i32>>) {
	let mut fields = vec!();
	let mut sequences = vec!();

	for line in input.split("\n") {
		let mut parts = line.split_whitespace();
		fields.push(parts.next().unwrap().to_string());
		sequences.push(parts.next().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect_vec());
	}
	return (fields, sequences);
}

fn is_valid(field: &[u8], sequence: &Vec<i32>) -> bool {
	let mut pos = 0usize;
	for streak in sequence {
		//Find when the streak starts.
		while pos < field.len() && field[pos] == '.' as u8 {
			pos += 1;
		}
		//Find this streak's length.
		let mut length = 0;
		while pos < field.len() && field[pos] == '#' as u8 {
			length += 1;
			pos += 1;
		}
		if length != *streak {
			return false; //Streak is too short.
		}
	}
	//Find if there are any more streaks after the last.
	while pos < field.len() {
		if field[pos] == '#' as u8 {
			return false; //Too many streaks.
		}
		pos += 1;
	}
	return true;
}

fn num_valid(field: &[u8], sequence: &Vec<i32>) -> usize {
	let num_unknown = field.iter().filter(|c| **c == '?' as u8).count();
	let num_damaged_known = field.iter().filter(|c| **c == '#' as u8).count();
	let num_damaged = sequence.iter().sum::<i32>() as u32;
	let num_damaged_unknown: u32 = num_damaged - (num_damaged_known as u32);
	let mut valid = 0usize;
	let mut bitfield = 2usize.pow(num_damaged_unknown) - 1;
	while bitfield <= 2usize.pow(num_damaged_unknown) - 1 << (num_unknown - num_damaged_unknown as usize) {
		let mut filled_in = String::new();
		let mut bit = 0;
		for c in field {
			if *c == '?' as u8 {
				if bitfield & (1 << bit) > 0 {
					filled_in.push('#');
				} else {
					filled_in.push('.');
				}
				bit += 1;
			} else {
				filled_in.push(*c as char);
			}
		}
		if is_valid(&filled_in.as_bytes(), sequence) {
			valid += 1;
		}

		//From https://graphics.stanford.edu/~seander/bithacks.html#NextBitPermutation
		let t = bitfield | (bitfield - 1);
		bitfield = (t + 1) | (((!t & -(!t as i32) as usize) - 1) >> (bitfield.trailing_zeros() + 1));
	}
	return valid;
}

pub fn part1(input: String) {
	let (fields, sequences) = parse(input);
	let mut sum_arrangements = 0;
	for i in 0..fields.len() {
		let num = num_valid(fields[i].as_bytes(), &sequences[i]);
		sum_arrangements += num;
	}
	println!("{}", sum_arrangements);
}

pub fn part2(input: String) {
	let (fields, sequences) = parse(input);
	let mut sum_arrangements = 0;
	for i in 0..fields.len() {
		let original_num = num_valid(fields[i].as_bytes(), &sequences[i]);
		if original_num == 1 {
			sum_arrangements += 1; //Then the multiplied field is also 1.
			continue;
		}
		let mut multiplied_field = String::new();
		let mut multiplied_sequence: Vec<i32> = vec!();
		for _ in 0..5 {
			multiplied_field += &fields[i];
			multiplied_sequence.append(&mut sequences[i].clone());
		}
		let num = num_valid(multiplied_field.as_bytes(), &multiplied_sequence);
		sum_arrangements += num;
	}
	println!("{}", sum_arrangements);
}