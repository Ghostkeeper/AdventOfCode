use std::collections::HashMap;

fn parse(input: String) -> (Vec<Vec<usize>>, usize, usize, usize, usize) {
	let mut you = usize::MAX;
	let mut svr = usize::MAX;
	let mut fft = usize::MAX;
	let mut dac = usize::MAX;
	let mut vertices = vec!();
	let mut labels = HashMap::new();
	for line in input.split("\n") {
		let mut parts = line.split(": ");
		let from = parts.next().unwrap();
		if from == "you" {
			you = vertices.len();
		} else if from == "svr" {
			svr = vertices.len();
		} else if from == "fft" {
			fft = vertices.len();
		} else if from == "dac" {
			dac = vertices.len();
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

	(vertices, you, svr, fft, dac)
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

fn num_paths(vertices_top: &Vec<usize>, edges: &Vec<Vec<usize>>, from: usize, to: usize) -> u64 {
	let start_i = vertices_top.iter().position(|&x| x == from).unwrap();
	let mut num_path_to = vec![0; edges.len()];
	num_path_to[vertices_top[start_i]] = 1;
	for i in start_i..vertices_top.len() {
		for neighbour in &edges[vertices_top[i]] {
			num_path_to[*neighbour] += num_path_to[vertices_top[i]];
		}
	}

	num_path_to[to]
}

pub fn part1(input: String) -> u64 {
	let (edges, start, _, _, _) = parse(input);
	let vertices_top = topological_sort(&edges);
	num_paths(&vertices_top, &edges, start, edges.len() - 1)
}

pub fn part2(input: String) -> u64 {
	let (edges, _, svr, fft, dac) = parse(input);
	let vertices_top = topological_sort(&edges);
	let out = edges.len() - 1;
	let svr_fft = num_paths(&vertices_top, &edges, svr, fft);
	let svr_dac = num_paths(&vertices_top, &edges, svr, dac);
	let fft_dac = num_paths(&vertices_top, &edges, fft, dac);
	let dac_fft = num_paths(&vertices_top, &edges, dac, fft);
	let fft_out = num_paths(&vertices_top, &edges, fft, out);
	let dac_out = num_paths(&vertices_top, &edges, dac, out);

	svr_fft * fft_dac * dac_out + svr_dac * dac_fft * fft_out
}