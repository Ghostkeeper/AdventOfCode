use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

fn main() {
	let contents = fs::read_to_string("inputs/day11_1.txt").expect("File should exist.");
	day11::part1(contents);
}
