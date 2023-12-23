use itertools::Itertools;
use rustc_hash::FxHashSet;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.chars().collect_vec());
	}
	return result;
}

fn find_endpoints(map: &Vec<Vec<char>>) -> ((usize, usize), (usize, usize)) {
	let mut start = (0, 0);
	for (x, cell) in map[0].iter().enumerate() {
		if *cell == '.' {
			start.0 = x;
			break;
		}
	}
	let mut end = (0, map.len() - 1);
	for (x, cell) in map[map.len() - 1].iter().enumerate() {
		if *cell == '.' {
			end.0 = x;
			break;
		}
	}
	return (start, end);
}

fn longest_path(map: &Vec<Vec<char>>, end: &(usize, usize), path_so_far: &Vec<(usize, usize)>) -> usize {
	let head = path_so_far.last().unwrap();
	if head == end {
		return path_so_far.len();
	}

	let mut longest = 0;
	//Try going right.
	let mut next = (head.0 + 1, head.1);
	if map[next.1][next.0] != '#' && ['>', '.'].contains(&map[head.1][head.0]) && !path_so_far.contains(&next) {
		let mut new_path = path_so_far.clone();
		new_path.push(next);
		longest = longest.max(longest_path(map, end, &new_path));
	}
	//Try going up.
	if head.1 > 0 {
		next = (head.0, head.1 - 1);
		if map[next.1][next.0] != '#' && ['^', '.'].contains(&map[head.1][head.0]) && !path_so_far.contains(&next) {
			let mut new_path = path_so_far.clone();
			new_path.push(next);
			longest = longest.max(longest_path(map, end, &new_path));
		}
	}
	//Try going left.
	next = (head.0 - 1, head.1);
	if map[next.1][next.0] != '#' && ['<', '.'].contains(&map[head.1][head.0]) && !path_so_far.contains(&next) {
		let mut new_path = path_so_far.clone();
		new_path.push(next);
		longest = longest.max(longest_path(map, end, &new_path));
	}
	//Try going down.
	next = (head.0, head.1 + 1);
	if head.1 < map.len() - 1 && map[next.1][next.0] != '#' && ['v', '.'].contains(&map[head.1][head.0]) && !path_so_far.contains(&next) {
		let mut new_path = path_so_far.clone();
		new_path.push(next);
		longest = longest.max(longest_path(map, end, &new_path));
	}
	return longest;
}

fn longest_path_2(map: &Vec<Vec<char>>, end: &(usize, usize), head: (usize, usize), path_so_far: &mut FxHashSet<(usize, usize)>) -> usize {
	if head == *end {
		return path_so_far.len();
	}

	let mut longest = 0;
	//Try going right.
	let mut next = (head.0 + 1, head.1);
	if map[next.1][next.0] != '#' && !path_so_far.contains(&next) {
		path_so_far.insert(next);
		longest = longest.max(longest_path_2(map, end, next, path_so_far));
		path_so_far.remove(&next);
	}
	//Try going up.
	if head.1 > 0 {
		next = (head.0, head.1 - 1);
		if map[next.1][next.0] != '#' && !path_so_far.contains(&next) {
			path_so_far.insert(next);
			longest = longest.max(longest_path_2(map, end, next, path_so_far));
			path_so_far.remove(&next);
		}
	}
	//Try going left.
	next = (head.0 - 1, head.1);
	if map[next.1][next.0] != '#' && !path_so_far.contains(&next) {
		path_so_far.insert(next);
		longest = longest.max(longest_path_2(map, end, next, path_so_far));
		path_so_far.remove(&next);
	}
	//Try going down.
	next = (head.0, head.1 + 1);
	if head.1 < map.len() - 1 && map[next.1][next.0] != '#' && !path_so_far.contains(&next) {
		path_so_far.insert(next);
		longest = longest.max(longest_path_2(map, end, next, path_so_far));
		path_so_far.remove(&next);
	}
	return longest;
}

pub fn part1(input: String) -> usize {
	let map = parse(input);
	let (start, end) = find_endpoints(&map);

	let path = vec![start];
	return longest_path(&map, &end, &path) - 1;
}

pub fn part2(input: String) -> usize {
	let map = parse(input);
	let (start, end) = find_endpoints(&map);

	let mut path = FxHashSet::default();
	path.insert(start);
	return longest_path_2(&map, &end, start, &mut path) - 1;
}