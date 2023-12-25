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

fn intersect_line_2d(line1: (i128, i128, i128, i128, i128, i128), line2: (i128, i128, i128, i128, i128, i128)) -> (i128, i128) {
	let a1 = line1.4;
	let a2 = line2.4;
	let b1 = -line1.3;
	let b2 = -line2.3;
	let c1 = line1.4 * line1.0 - line1.3 * line1.1;
	let c2 = line2.4 * line2.0 - line2.3 * line2.1;
	if a1 * b2 == b1 * a2 {
		return (i128::MIN, i128::MIN); //Parallel.
	}
	let x = (b2 * c1 - b1 * c2) / (a1 * b2 - b1 * a2);
	let y = (a1 * c2 - a2 * c1) / (a1 * b2 - a2 * b1);
	if (x - line1.0 < 0) == (line1.3 < 0)
		&& (y - line1.1 < 0) == (line1.4 < 0)
		&& (x - line2.0 < 0) == (line2.3 < 0)
		&& (y - line2.1 < 0) == (line2.4 < 0) {
		return (x, y);
	} else {
		return (i128::MIN, i128::MIN); //In the past.
	}
}

pub fn part1(input: String) -> usize {
	let hails = parse(input);

	let mut count = 0;
	for hail_id1 in 0..hails.len() {
		for hail_id2 in 0..hail_id1 {
			let intersection = intersect_line_2d(hails[hail_id1], hails[hail_id2]);
			if intersection.0 >= 200000000000000 && intersection.0 <= 400000000000000 && intersection.1 >= 200000000000000 && intersection.1 <= 400000000000000 {
			//if intersection.0 >= 7 && intersection.0 <= 27 && intersection.1 >= 7 && intersection.1 <= 27 {
				count += 1;
			}
		}
	}

	return count;
}