#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

fn main() {
	let contents = fs::read_to_string("inputs/day8_1.txt").expect("File should exist.");
	let answer = day8::part2(contents);
	println!("ANSWER: {}", answer);
}
