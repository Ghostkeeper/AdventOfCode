use std::collections::{HashMap, HashSet};
use itertools::Itertools;

fn parse(input: String) -> HashMap<char, Vec<(i32, i32)>> {
	let mut antennas = HashMap::new();
	let mut rows = vec!();
	for line in input.split("\n") {
		rows.push(line.chars().collect_vec());
	}
	for y in 0..rows.len() {
		for x in 0..rows[y].len() {
			if rows[y][x] == '.' {
				continue;
			}
			let frequency = rows[y][x];
			if !antennas.contains_key(&frequency) {
				antennas.insert(frequency, vec!());
			}
			antennas.get_mut(&frequency).unwrap().push((x as i32, y as i32));
		}
	}
	return antennas;
}

pub fn part1(input: String) -> u32 {
	let antennas = parse(input.clone());
	let height = input.split("\n").collect_vec().len() as i32;
	let width = input.split("\n").next().unwrap().len() as i32;

	let mut antinodes = HashSet::new();
	for frequency in antennas.keys() {
		let this_antennas = antennas.get(frequency).unwrap();
		for i in 0..this_antennas.len() {
			for j in (i + 1)..this_antennas.len() {
				let diff = (this_antennas[j].0 - this_antennas[i].0, this_antennas[j].1 - this_antennas[i].1);
				let pos1 = (this_antennas[i].0 - diff.0, this_antennas[i].1 - diff.1);
				let pos2 = (this_antennas[j].0 + diff.0, this_antennas[j].1 + diff.1);
				if pos1.0 >= 0 && pos1.0 < width && pos1.1 >= 0 && pos1.1 < height {
					// println!("pos {:?} freq {} from {:?} and {:?}, diff {:?}", pos1, frequency, this_antennas[i], this_antennas[j], diff);
					antinodes.insert(pos1);
				}
				if pos2.0 >= 0 && pos2.0 < width && pos2.1 >= 0 && pos2.1 < height {
					// println!("pos {:?} freq {}", pos2, frequency);
					antinodes.insert(pos2);
				}
			}
		}
	}

	// println!("{:?}", antinodes);
	return antinodes.len() as u32;
}