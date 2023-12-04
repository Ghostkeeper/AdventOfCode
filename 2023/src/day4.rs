use std::collections::HashSet;

pub fn part1(input: String) {
	let mut sum = 0;
	for line in input.split("\n") {
		let mut sides = line[10..].split(" | ");
		let left = sides.next().unwrap();
		let right = sides.next().unwrap();
		let mut left_nums = HashSet::new();
		for num in left.split_whitespace() {
			left_nums.insert(num.parse::<i32>().unwrap());
		}
		let mut right_nums = HashSet::new();
		for num in right.split_whitespace() {
			right_nums.insert(num.parse::<i32>().unwrap());
		}
		let in_common = left_nums.intersection(&right_nums).collect::<Vec<&i32>>();
		let amount_in_common = in_common.len();
		if amount_in_common > 0 {
			sum += 1 << (amount_in_common - 1);
		}
	}
	println!("{}", sum);
}