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