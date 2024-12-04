#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
	let contents = fs::read_to_string("inputs/day4_1.txt").expect("File should exist.");
	let answer = day4::part2(contents);
	println!("ANSWER: {}", answer);
}
