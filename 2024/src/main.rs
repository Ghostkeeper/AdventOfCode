#![allow(dead_code)]

use std::fs;
use std::time::Instant;
use tikv_jemallocator::Jemalloc;

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
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
	let contents = fs::read_to_string("inputs/day22_1.txt").expect("File should exist.");
	let start = Instant::now();
	let answer = day22::part2(contents);
	let elapsed = start.elapsed();
	println!("ANSWER: {}", answer);
	println!("Elapsed time: {:.2?}", elapsed);
}
