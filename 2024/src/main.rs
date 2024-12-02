#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;

fn main() {
	let contents = fs::read_to_string("inputs/day2_1.txt").expect("File should exist.");
	let answer = day2::part2(contents);
	println!("ANSWER: {}", answer);
}
