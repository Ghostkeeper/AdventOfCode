use rayon::prelude::*;
use std::collections::HashMap;

fn parse(input: String) -> (Vec<String>, Vec<Vec<bool>>) {
	let mut name_to_pos = HashMap::new();
	let mut names = vec!();
	for line in input.split("\n") {
		let mut parts = line.split("-");
		let left = parts.next().unwrap();
		let right = parts.next().unwrap();
		if !name_to_pos.contains_key(left) {
			name_to_pos.insert(left, names.len());
			names.push(left.to_string());
		}
		if !name_to_pos.contains_key(right) {
			name_to_pos.insert(right, names.len());
			names.push(right.to_string());
		}
	}

	let mut connected = vec![vec![false; names.len()]; names.len()];
	for line in input.split("\n") {
		let mut parts = line.split("-");
		let left = *name_to_pos.get(parts.next().unwrap()).unwrap();
		let right = *name_to_pos.get(parts.next().unwrap()).unwrap();
		connected[left][right] = true;
		connected[right][left] = true;
	}

	return (names, connected);
}

pub fn part1(input: String) -> u64 {
	let (names, connected) = parse(input);
	let sum_3cliques = (0..names.len()).into_par_iter().map(|i| {
		let mut num_3cliques = 0;
		for j in (i + 1)..names.len() {
			for k in (j + 1)..names.len() {
				if connected[i][j] && connected[j][k] && connected[k][i] && (names[i].starts_with("t") || names[j].starts_with("t") || names[k].starts_with("t")) {
					num_3cliques += 1;
				}
			}
		}
		num_3cliques
	}).sum();

	return sum_3cliques;
}