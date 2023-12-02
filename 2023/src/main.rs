use std::fs;

mod day1;

fn main() {
	let contents = fs::read_to_string("inputs/day1_1.txt").expect("File should exist.");
	day1::part2(contents);
}
