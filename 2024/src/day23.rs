use rayon::prelude::*;
use std::collections::HashMap;
use itertools::Itertools;

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

pub fn part2(input: String) -> String {
	let (names, connected) = parse(input);

	let mut cliques = vec!();
	//Find all 3-cliques to start with.
	for i in 0..names.len() {
		for j in (i + 1)..names.len() {
			for k in (j + 1)..names.len() {
				if connected[i][j] && connected[j][k] && connected[k][i] {
					cliques.push(vec![i, j, k]);
				}
			}
		}
	}
	loop {
		let mut bigger_cliques = vec!();
		for clique in cliques.iter_mut() {
			'recruit: for i in 0..names.len() {
				for member in clique.iter() {
					if !connected[*member][i] {
						continue 'recruit; //i is not part of a bigger clique.
					}
				}
				//i is part of a bigger clique!
				clique.push(i);
				bigger_cliques.push(clique.iter().map(|i| *i).sorted().collect_vec());
			}
		}
		if bigger_cliques.len() == 0 {
			break;
		}
		cliques = bigger_cliques;
	}
	let code = cliques[0].iter().map(|i| names[*i].clone()).sorted().join(",");
	return code;
}