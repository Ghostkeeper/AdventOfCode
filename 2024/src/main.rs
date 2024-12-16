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
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
	let contents = fs::read_to_string("inputs/day16_1.txt").expect("File should exist.");
	let answer = day16::part1(contents);
	println!("ANSWER: {}", answer);
}
