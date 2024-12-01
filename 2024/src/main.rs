#![allow(dead_code)]

use std::fs;

mod day1;

fn main() {
	let contents = fs::read_to_string("inputs/day1_1.txt").expect("File should exist.");
	let answer = day1::part1(contents);
	println!("ANSWER: {}", answer);
}
