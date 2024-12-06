#![allow(dead_code)]

use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main() {
	let contents = fs::read_to_string("inputs/day6_1.txt").expect("File should exist.");
	let answer = day6::part1(contents);
	println!("ANSWER: {}", answer);
}
