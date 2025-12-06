#![allow(dead_code)]

use std::fs;
use std::time::Instant;
use tikv_jemallocator::Jemalloc;

mod day1;
mod day2;
mod day3;
mod day4;
mod util;
mod day5;
mod day6;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
	let contents = fs::read_to_string("inputs/day6_1.txt").expect("File should exist.");
	let start = Instant::now();
	let answer = day6::part1(contents);
	let elapsed = start.elapsed();
	println!("ANSWER: {}", answer);
	println!("Elapsed time: {:.2?}", elapsed);
}
