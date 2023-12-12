use std::collections::HashSet;

pub fn part1(input: String) -> i32 {
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
	return sum;
}

pub fn part2(input: String) -> usize {
	let mut lines = input.split("\n").collect::<Vec<&str>>();
	let mut line_id = 0;
	while line_id < lines.len() {
		let line = lines[line_id];
		let card_id = line[5..8].split_whitespace().collect::<Vec<&str>>()[0].parse::<usize>().unwrap();
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
		for copied_id in 0..amount_in_common {
			lines.push(lines[card_id + copied_id]);
		}
		line_id += 1;
		if lines.len() % 10000 == 0 {
			println!("so far: {}", lines.len());
		}
	}
	return lines.len();
}