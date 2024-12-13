use itertools::Itertools;
use regex::Regex;

fn parse(input: String) -> Vec<((f64, f64), (f64, f64), (f64, f64))> {
	let re = Regex::new(r"Button A: X(?<ax>[+-]\d+), Y(?<ay>[+-]\d+)\nButton B: X(?<bx>[+-]\d+), Y(?<by>[+-]\d+)\nPrize: X=(?<px>\d+), Y=(?<py>\d+)").unwrap();
	let machines = re.captures_iter(input.as_str()).map(|caps| {
		let ax = caps.name("ax").unwrap().as_str().parse::<f64>().unwrap();
		let ay = caps.name("ay").unwrap().as_str().parse::<f64>().unwrap();
		let bx = caps.name("bx").unwrap().as_str().parse::<f64>().unwrap();
		let by = caps.name("by").unwrap().as_str().parse::<f64>().unwrap();
		let px = caps.name("px").unwrap().as_str().parse::<f64>().unwrap();
		let py = caps.name("py").unwrap().as_str().parse::<f64>().unwrap();
		((ax, ay), (bx, by), (px, py))
	}).collect_vec();

	return machines;
}

pub fn part1(input: String) -> u32 {
	let machines = parse(input);

	let mut cost = 0;
	for (a, b, prize) in machines {
		let cross = a.0 * b.1 - b.0 * a.1;
		let num_a = (b.1 * prize.0 - b.0 * prize.1) / cross;
		let num_b = (a.0 * prize.1 - a.1 * prize.0) / cross;
		if num_a.round() == num_a && num_b.round() == num_b {
			cost += 3 * (num_a as u32) + (num_b as u32);
		}
	}

	return cost;
}