use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
	let contents = fs::read_to_string("inputs/day7_1.txt").expect("File should exist.");
	day7::part1(contents);
}
