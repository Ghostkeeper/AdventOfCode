use std::collections::HashSet;
use std::hash::BuildHasherDefault;
use nohash_hasher::NoHashHasher;
use rayon::prelude::*;

type HashSetNoHash<V> = HashSet<V, BuildHasherDefault<NoHashHasher<usize>>>;

#[derive(Debug)]
struct Node {
	neighbours: Vec<usize>,
}

fn parse(input: String) -> (Vec<Node>, Vec<(usize, usize)>) {
	let mut nodes = vec!();
	let mut names = vec!();
	for line in input.split("\n") {
		let name = line.split_at(3).0;
		if !names.contains(&name) {
			names.push(name);
			nodes.push(Node {
				neighbours: vec!(),
			});
		}
		for neighbour_name in line.split_at(5).1.split_whitespace() {
			if !names.contains(&neighbour_name) {
				names.push(neighbour_name);
				nodes.push(Node {
					neighbours: vec!(),
				});
			}
		}
	}

	let mut edges = vec!();
	for line in input.split("\n") {
		let name = line.split_at(3).0;
		let my_pos = names.iter().position(|n| *n == name).unwrap();
		let neighbour_names = line.split_at(5).1;
		for neighbour_name in neighbour_names.split_whitespace() {
			let neighbour_pos = names.iter().position(|n| *n == neighbour_name).unwrap();
			nodes[my_pos].neighbours.push(neighbour_pos);
			nodes[neighbour_pos].neighbours.push(my_pos);
			if my_pos < neighbour_pos { //Always the lower index first, to be canonical.
				edges.push((my_pos, neighbour_pos));
			} else {
				edges.push((neighbour_pos, my_pos));
			}
		}
	}

	return (nodes, edges);
}

fn cut(nodes: &Vec<Node>, cut_edges: [(usize, usize); 3]) -> usize {
	let mut seen = HashSetNoHash::default();
	let mut front = HashSetNoHash::default();
	front.insert(0);
	while !front.is_empty() {
		let node = *front.iter().next().unwrap();
		front.remove(&node);
		seen.insert(node);
		for neighbour in nodes[node].neighbours.iter() {
			if node < *neighbour {
				if cut_edges.contains(&(node, *neighbour)) { //This edge is "cut", ignore it.
					continue;
				}
			} else {
				if cut_edges.contains(&(*neighbour, node)) {
					continue;
				}
			}
			if !seen.contains(neighbour) {
				front.insert(*neighbour);
			}
		}
	}
	return seen.len();
}

pub fn part1(input: String) -> usize {
	let (nodes, edges) = parse(input);

	println!("Edges len: {}", edges.len());
	let n = edges.len() as u64;
	(0..edges.len()).into_par_iter().for_each(|i| {
		println!("{}", i);
		for j in 0..i {
			for k in 0..j {
				let cut_edges = [edges[i], edges[j], edges[k]];
				let part_size = cut(&nodes, cut_edges);
				if part_size < nodes.len() {
					println!("Part size: {} out of {}", part_size, nodes.len());
					println!("Cutting edges: {:?}", cut_edges);
					println!("Result: {}", part_size * (nodes.len() - part_size));
				}
			}
		}
	});
	panic!("No 3-cut found!");
}