use std::fs;

mod day1;
mod day2;
mod day3;

fn main() {
	let contents = fs::read_to_string("inputs/day3_1.txt").expect("File should exist.");
	day3::part2(contents);
}
