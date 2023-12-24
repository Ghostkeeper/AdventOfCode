use geo::Line;
use intersect2d::{intersect, Intersection};
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

pub fn part1(input: String) -> usize {
	let hails = parse(input);
	println!("HAILS: {:?}", hails);

	let mut count = 0;
	for hail_id1 in 0..hails.len() {
		for hail_id2 in 0..hail_id1 {
			let line1 = Line::<f64> {
				start: (hails[hail_id1].0, hails[hail_id1].1).into(),
				end: (hails[hail_id1].0 + hails[hail_id1].3, hails[hail_id1].1 + hails[hail_id1].4).into(),
			};
			let line2 = Line::<f64> {
				start: (hails[hail_id2].0, hails[hail_id2].1).into(),
				end: (hails[hail_id2].0 + hails[hail_id2].3, hails[hail_id2].1 + hails[hail_id2].4).into(),
			};
			let rv = intersect(&line1, &line2);
			match rv {
				Some(Intersection::Intersection(a)) => println!("Intersection {:?} and {:?}: {:?}", line1, line2, a),
				_ => println!("These lines don't intersect."),
			}
			// if intersection.0 >= 7 && intersection.0 <= 27 && intersection.1 >= 7 && intersection.1 <= 27 {
			// 	count += 1;
			// }
		}
	}

	return count;
}