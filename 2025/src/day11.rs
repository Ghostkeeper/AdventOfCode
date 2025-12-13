use std::collections::HashMap;

fn parse(input: String) -> (Vec<Vec<usize>>, usize) {
	let mut start = usize::MAX;
	let mut vertices = vec!();
	let mut labels = HashMap::new();
	for line in input.split("\n") {
		let mut parts = line.split(": ");
		let from = parts.next().unwrap();
		if from == "you" {
			start = vertices.len();
		}
		labels.insert(from, vertices.len());
		vertices.push(vec!());
	}
	let num_verts = vertices.len();
	for (i, line) in input.split("\n").enumerate() {
		let mut parts = line.split(": ");
		let to = parts.nth(1).unwrap();
		for target in to.split(" ") {
			if target == "out" {
				vertices[i].push(num_verts);
			} else {
				vertices[i].push(labels[target]);
			}
		}
	}
	vertices.push(vec!());

	(vertices, start)
}

fn topological_sort(edges: &Vec<Vec<usize>>) -> Vec<usize> {
	//Khan's algorithm for topological sorting.
	let mut result = vec!();
	let mut indegree = vec![0; edges.len()];
	for vertex in edges {
		for neighbour in vertex {
			indegree[*neighbour] += 1;
		}
	}
	let mut todo = vec!();
	for (i, indeg) in indegree.iter().enumerate() {
		if *indeg == 0 {
			todo.push(i);
		}
	}
	while !todo.is_empty() {
		let vertex = todo.pop().unwrap();
		for neighbour in &edges[vertex] {
			indegree[*neighbour] -= 1;
			if indegree[*neighbour] == 0 {
				todo.push(*neighbour);
			}
		}
		result.push(vertex);
	}
	result
}

pub fn part1(input: String) -> u32 {
	let (edges, start) = parse(input);
	let vertices_top = topological_sort(&edges);
	let start_i = vertices_top.iter().position(|&x| x == start).unwrap();
	let mut num_path_to = vec![0; edges.len()];
	num_path_to[vertices_top[start_i]] = 1;
	for i in start_i..vertices_top.len() {
		for neighbour in &edges[vertices_top[i]] {
			num_path_to[*neighbour] += num_path_to[vertices_top[i]];
		}
	}

	num_path_to[vertices_top.len() - 1]
}