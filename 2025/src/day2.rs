use crate::util;

fn parse(input: String) -> Vec<(u64, u64)> {
	let mut result = vec!();
	for range in input.split(",") {
		let mut parts = range.split("-");
		let lower_str = parts.next().unwrap();
		let upper_str = parts.next().unwrap();
		let lower = lower_str.parse::<u64>().unwrap();
		let upper = upper_str.parse::<u64>().unwrap();
		result.push((lower, upper));
	}
	result
}

pub fn part1(input: String) -> u64 {
	let mut result = 0;
	let ranges = parse(input);
	for (lower, upper) in ranges {
		for x in lower..(upper + 1) {
			if (x >= 10 && x <= 99 && x / 10 == x % 10) ||
					(x >= 1000 && x <= 9999 && x / 100 == x % 100) ||
					(x >= 100000 && x <= 999999 && x / 1000 == x % 1000) ||
					(x >= 10000000 && x <= 99999999 && x / 10000 == x % 10000) ||
					(x >= 1000000000 && x <= 9999999999 && x / 100000 == x % 100000) ||
					(x >= 100000000000 && x <= 999999999999 && x / 1000000 == x % 1000000) {
				result += x;
			}
		}
	}
	result
}

pub fn part2(input: String) -> u64 {
	let mut result = 0;
	let ranges = parse(input);
	for (lower, upper) in ranges {
		for x in lower..(upper + 1) {
			let num_digits = util::num_digits(x);
			'sizeloop: for pow in 1..num_digits {
				if num_digits % pow != 0 {
					continue;
				}
				let tens = 10u64.pow(pow as u32);
				let repeated_bit = x % tens;
				for pos in 0..num_digits / pow {
					let tens_lower = tens.pow(pos as u32);
					let tens_upper = tens.pow((pos + 1) as u32);
					if (x % tens_upper) / tens_lower != repeated_bit {
						continue 'sizeloop;
					}
				}
				//An invalid ID!
				result += x;
				break;
			}
		}
	}
	result
}