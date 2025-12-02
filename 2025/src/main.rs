#![allow(dead_code)]

use std::fs;
use std::time::Instant;
use tikv_jemallocator::Jemalloc;

mod day2;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
	let contents = fs::read_to_string("inputs/day2_1.txt").expect("File should exist.");
	let start = Instant::now();
	let answer = day2::part2(contents);
	let elapsed = start.elapsed();
	println!("ANSWER: {}", answer);
	println!("Elapsed time: {:.2?}", elapsed);
}
