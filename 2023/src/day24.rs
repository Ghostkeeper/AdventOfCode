use itertools::Itertools;
use regex::Regex;

fn parse(input: String) -> Vec<(f64, f64, f64, f64, f64, f64)> {
	let re = Regex::new(r"(?<x>\d+), (?<y>\d+), (?<z>\d+) @  ?(?<dx>-?\d+),  ?(?<dy>-?\d+),  ?(?<dz>-?\d+)").unwrap();
	let hails = re.captures_iter(input.as_str()).map(|caps| {
		let x = caps.name("x").unwrap().as_str().parse::<f64>().unwrap();
		let y = caps.name("y").unwrap().as_str().parse::<f64>().unwrap();
		let z = caps.name("z").unwrap().as_str().parse::<f64>().unwrap();
		let dx = caps.name("dx").unwrap().as_str().parse::<f64>().unwrap();
		let dy = caps.name("dy").unwrap().as_str().parse::<f64>().unwrap();
		let dz = caps.name("dz").unwrap().as_str().parse::<f64>().unwrap();
		(x, y, z, dx, dy, dz)
	}).collect_vec();

	return hails;
}

fn intersect_line_2d(line1: (f64, f64, f64, f64, f64, f64), line2: (f64, f64, f64, f64, f64, f64)) -> (f64, f64) {
	let a1 = line1.4;
	let a2 = line2.4;
	let b1 = -line1.3;
	let b2 = -line2.3;
	let c1 = line1.4 * line1.0 - line1.3 * line1.1;
	let c2 = line2.4 * line2.0 - line2.3 * line2.1;
	if a1 * b2 == b1 * a2 {
		return (f64::MIN, f64::MIN); //Parallel.
	}
	let x = (b2 * c1 - b1 * c2) / (a1 * b2 - b1 * a2);
	let y = (a1 * c2 - a2 * c1) / (a1 * b2 - a2 * b1);
	if (x - line1.0 < 0.0) == (line1.3 < 0.0)
		&& (y - line1.1 < 0.0) == (line1.4 < 0.0)
		&& (x - line2.0 < 0.0) == (line2.3 < 0.0)
		&& (y - line2.1 < 0.0) == (line2.4 < 0.0) {
		return (x, y);
	} else {
		return (f64::MIN, f64::MIN); //In the past.
	}
}

pub fn part1(input: String) -> usize {
	let hails = parse(input);

	let mut count = 0;
	for hail_id1 in 0..hails.len() {
		for hail_id2 in 0..hail_id1 {
			let intersection = intersect_line_2d(hails[hail_id1], hails[hail_id2]);
			//if intersection.0 >= 200000000000000.0 && intersection.0 <= 400000000000000.0 && intersection.1 >= 200000000000000.0 && intersection.1 <= 400000000000000.0 {
			if intersection.0 >= 7.0 && intersection.0 <= 27.0 && intersection.1 >= 7.0 && intersection.1 <= 27.0 {
				count += 1;
			}
		}
	}

	return count;
}

pub fn part2(input: String) -> u64 {
	let hails = parse(input);

	for dx_int in -500..500 {
		for dy_int in -500..500 {
			'zloop: for dz_int in -500..500 {
				let dx = dx_int as f64;
				let dy = dy_int as f64;
				let dz = dz_int as f64;

				let t2_numerator = hails[1].1 - hails[0].1 - ((hails[0].4 - dy) * (hails[1].0 - hails[0].0)) / (hails[0].3 - dx);
				let t2_denominator = dy - hails[1].4 - ((hails[0].4 - dy) * (dx - hails[1].3)) / (hails[0].3 - dx);
				let t2 = t2_numerator / t2_denominator;
				if !t2.is_finite() {
					continue;
				}
				if t2 < 0.0 {
					continue; //Would hit hails[1] in the past.
				}
				if (t2.round() - t2).abs() > 0.0001 { //Not integer time.
					continue;
				}
				let t1 = (hails[1].0 - hails[0].0 - t2 * (dx - hails[1].3)) / (hails[0].3 - dx);
				if !t1.is_finite() {
					continue;
				}
				if t1 < 0.0 {
					continue; //Would hit hails[0] in the past.
				}
				if (t1.round() - t1).abs() > 0.0001 { //Not integer time.
					continue;
				}
				let px = hails[0].0 - t1 * (dx - hails[0].3);
				if !px.is_finite() {
					continue;
				}
				if (px.round() - px).abs() > 0.0001 { //Integer time => integer position!
					continue;
				}
				let py = hails[0].1 - t1 * (dy - hails[0].4);
				if !py.is_finite() {
					continue;
				}
				if (py.round() - py).abs() > 0.0001 {
					continue;
				}
				let pz = hails[0].2 - t1 * (dz - hails[0].5);
				if !pz.is_finite() {
					continue;
				}
				if (pz.round() - pz).abs() > 0.0001 {
					continue;
				}

				if (pz + t2 * (dz - hails[1].5) - hails[1].2).abs() > 0.0001 {
					continue; //Hitting hails[0] won't result in a hit for hails[1].
				}

				for i in 2..hails.len() {
					let t3 = (hails[i].0 - px) / (dx - hails[i].3);
					if (py + t3 * dy - (hails[i].1 + t3 * hails[i].4)).abs() > 0.0001 {
						continue 'zloop; //Never hits this third hail.
					}
					if (pz + t3 * dz - (hails[i].2 + t3 * hails[i].5)).abs() > 0.0001 {
						continue 'zloop; //Never hits this third hail.
					}
				}
				//Hits all hails!
				return px as u64 + py as u64 + pz as u64;
			}
		}
	}

	return 0;
}