fn parse(input: String) -> Vec<(i64, i64)> {
	let mut result = vec!();
	for range in input.split(",") {
		let mut parts = range.split("-");
		let lower_str = parts.next().unwrap();
		let upper_str = parts.next().unwrap();
		let lower = lower_str.parse::<i64>().unwrap();
		let upper = upper_str.parse::<i64>().unwrap();
		result.push((lower, upper));
	}
	result
}

pub fn part1(input: String) -> i64 {
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

pub fn part2(input: String) -> i64 {
	let mut result = 0;
	let ranges = parse(input);
	for (lower, upper) in ranges {
		for x in lower..(upper + 1) {
			let num_digits = match x {
				0..=9 => 1,
				10..=99 => 2,
				100..=999 => 3,
				1000..=9999 => 4,
				10000..=99999 => 5,
				100000..=999999 => 6,
				1000000..=9999999 => 7,
				10000000..=99999999 => 8,
				100000000..=999999999 => 9,
				1000000000..=9999999999 => 10,
				_ => panic!()
			};
			'sizeloop: for pow in 1..num_digits {
				if num_digits % pow != 0 {
					continue;
				}
				let tens = 10i64.pow(pow);
				let repeated_bit = x % tens;
				for pos in 0..num_digits / pow {
					let tens_lower = tens.pow(pos);
					let tens_upper = tens.pow(pos + 1);
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