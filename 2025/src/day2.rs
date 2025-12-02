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
		println!("{} - {}", lower, upper);
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