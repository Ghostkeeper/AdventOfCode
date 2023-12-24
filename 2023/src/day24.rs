use itertools::Itertools;
use regex::Regex;

fn parse(input: String) -> Vec<(i128, i128, i128, i128, i128, i128)> {
	let re = Regex::new(r"(?<x>\d+), (?<y>\d+), (?<z>\d+) @  ?(?<dx>-?\d+),  ?(?<dy>-?\d+),  ?(?<dz>-?\d+)").unwrap();
	let hails = re.captures_iter(input.as_str()).map(|caps| {
		let x = caps.name("x").unwrap().as_str().parse::<i128>().unwrap();
		let y = caps.name("y").unwrap().as_str().parse::<i128>().unwrap();
		let z = caps.name("z").unwrap().as_str().parse::<i128>().unwrap();
		let dx = caps.name("dx").unwrap().as_str().parse::<i128>().unwrap();
		let dy = caps.name("dy").unwrap().as_str().parse::<i128>().unwrap();
		let dz = caps.name("dz").unwrap().as_str().parse::<i128>().unwrap();
		(x, y, z, dx, dy, dz)
	}).collect_vec();

	return hails;
}

fn cross_product(a: (i128, i128), b: (i128, i128)) -> i128 {
	a.0 * b.1 - a.1 * b.0
}

fn round_divide(numerator: i128, denominator: i128) -> i128 {
	if (numerator < 0) ^ (denominator < 0) {
		(numerator - denominator / 2) / denominator
	} else {
		(numerator + denominator / 2) / denominator
	}
}

fn intersect_line_2d(line1: (i128, i128, i128, i128, i128, i128), line2: (i128, i128, i128, i128, i128, i128)) -> (i128, i128) {
	let divisor = cross_product((line1.3, line1.4), (line2.3, line2.4));
	if divisor == 0 { //They don't intersect.
		return (i128::MIN, i128::MIN);
	}
	let starts_delta = (line1.0 - line2.0, line1.1 - line2.1);
	let line1_parametric = cross_product((line2.3, line2.4), starts_delta);
	let line2_parametric = cross_product((line1.3, line1.4), starts_delta);
	return (line1.0 + round_divide(line1_parametric * line1.3, divisor), line1.1 + round_divide(line1_parametric * line1.4, divisor));
}

pub fn part1(input: String) -> usize {
	let hails = parse(input);
	println!("HAILS: {:?}", hails);

	let mut count = 0;
	for hail_id1 in 0..hails.len() {
		for hail_id2 in 0..hail_id1 {
			let intersection = intersect_line_2d(hails[hail_id1], hails[hail_id2]);
			println!("Lines {:?} x {:?} intersect at {:?}", hails[hail_id1], hails[hail_id2], intersection);
			if intersection.0 >= 7 && intersection.0 <= 27 && intersection.1 >= 7 && intersection.1 <= 27 {
				count += 1;
			}
		}
	}

	return count;
}