use std::collections::HashMap;
use regex::Regex;

pub fn part1(input: String) {
	let mut sum = 0;
	let re = Regex::new(r"[^0-9]").unwrap();
	for line in input.split("\n") {
		let filtered = re.replace_all(line, "");
		let first = filtered.chars().next().unwrap() as i32 - 48;
		let last = filtered.chars().last().unwrap() as i32 - 48;
		sum += first * 10 + last;
	}
	println!("{}", sum);
}

pub fn part2(input: String) {
	let replacement = HashMap::from([
		// ("eleven", "11"),
		// ("twelve", "12"),
		// ("thirteen", "13"),
		// ("fourteen", "14"),
		// ("fifteen", "15"),
		// ("sixteen", "16"),
		// ("seventeen", "17"),
		// ("eighteen", "18"),
		// ("nineteen", "19"),
		// ("ten", "10"),
		// ("twenty", "20"),
		// ("thirty", "30"),
		// ("fourty", "40"),
		// ("fifty", "50"),
		// ("sixty", "60"),
		// ("seventy", "70"),
		// ("eighty", "80"),
		// ("ninety", "90"),
		// ("hundred", "100"),
		("one", 1),
		("1", 1),
		("two", 2),
		("2", 2),
		("three", 3),
		("3", 3),
		("four", 4),
		("4", 4),
		("five", 5),
		("5", 5),
		("six", 6),
		("6", 6),
		("seven", 7),
		("7", 7),
		("eight", 8),
		("8", 8),
		("nine", 9),
		("9", 9),
		// ("zero", "0"),
	]);
	let mut sum = 0;
	let re = Regex::new(r"[^0-9]").unwrap();
	for line in input.split("\n") {
		let mut first = 0;
		let mut last = 0;
		let mut i = 0 as i32;
		while i < line.len() as i32 {
			for (key, val) in replacement.iter() {
				if line[i as usize..].starts_with(key) {
					first = *val;
					i = line.len() as i32;
					break;
				}
			}
			i += 1;
		}
		i = line.len() as i32 - 1;
		while i >= 0 {
			for (key, val) in replacement.iter() {
				if line[i as usize..].starts_with(key) {
					last = *val;
					i = -1;
					break;
				}
			}
			i -= 1;
		}
		sum += first * 10 + last;
		println!("{}  {}", line, first * 10 + last);
	}
	println!("{}", sum);
}