use std::collections::BinaryHeap;
use itertools::Itertools;
use regex::Regex;

fn parse(input: String) -> Vec<((i32, i32), (i32, i32), (i32, i32))> {
	let re = Regex::new(r"Button A: X(?<ax>[+-]\d+), Y(?<ay>[+-]\d+)\nButton B: X(?<bx>[+-]\d+), Y(?<by>[+-]\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
	let machines = re.captures_iter(input.as_str()).map(|caps| {
		let ax = caps.name("ax").unwrap().as_str().parse::<i32>().unwrap();
		let ay = caps.name("ay").unwrap().as_str().parse::<i32>().unwrap();
		let bx = caps.name("bx").unwrap().as_str().parse::<i32>().unwrap();
		let by = caps.name("by").unwrap().as_str().parse::<i32>().unwrap();
		let px = caps.name("px").unwrap().as_str().parse::<i32>().unwrap();
		let py = caps.name("py").unwrap().as_str().parse::<i32>().unwrap();
		((ax, ay), (bx, by), (px, py))
	}).collect_vec();

	return machines;
}

pub fn part1(input: String) -> u32 {
	let machines = parse(input);

	let minimum_coins = 0;
	for (a, b, prize) in machines {
		let mut todo = BinaryHeap::new();
		todo.push((0, 0));
	}

	return minimum_coins;
}