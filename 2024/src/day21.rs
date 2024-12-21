use rayon::prelude::*;
use std::collections::{BinaryHeap, HashMap, HashSet};

fn parse(input: String) -> Vec<Vec<char>> {
	let mut codes = vec!();
	for line in input.split("\n") {
		codes.push(line.chars().collect());
	}
	return codes;
}

const NUMPAD: &[&[char]] = &[&['7', '8', '9'], &['4', '5', '6'], &['1', '2', '3'], &[' ', '0', 'A']];
const DIRPAD: &[&[char]] = &[&[' ', '^', 'A'], &['<', 'v', '>']];

fn use_dirpad(x: usize, y: usize, keypress: char, pad: &[&[char]]) -> (usize, usize, Option<char>) {
	match keypress {
		'<' => (x.wrapping_sub(1), y, None),
		'^' => (x, y.wrapping_sub(1), None),
		'>' => (x + 1, y, None),
		'v' => (x, y + 1, None),
		'A' => (x, y, Some(pad[y][x])),
		_ => panic!("Unknown pad key {}", keypress),
	}
}

fn cost(goal: char, previous_key: char, depth: usize, cache: &mut HashMap<(char, char, usize), usize>) -> usize {
	if depth == 0 {
		return 1;
	}
	if cache.contains_key(&(goal, previous_key, depth)) {
		return *cache.get(&(goal, previous_key, depth)).unwrap();
	}
	let start = match previous_key {
		'^' => (1, 0),
		'A' => (2, 0),
		'<' => (0, 1),
		'v' => (1, 1),
		'>' => (2, 1),
		_ => panic!("Unknown dirpad key {} has no position", previous_key),
	};
	let mut search_queue = BinaryHeap::new();
	search_queue.push((0, start, 'A', '?'));
	while !search_queue.is_empty() {
		let (priority, (x, y), prev, out) = search_queue.pop().unwrap();
		let dist = (-priority) as usize;
		if out == goal {
			cache.insert((goal, previous_key, depth), dist);
			return dist;
		}
		for key in ['A', '<', '^', '>', 'v'] {
			let (new_x, new_y, result_key) = use_dirpad(x, y, key, DIRPAD);
			if *DIRPAD.get(new_y).and_then(|row| row.get(new_x)).unwrap_or(&' ') == ' ' {
				continue;
			}
			if result_key.is_some() && result_key.unwrap() != goal {
				continue;
			}
			let distance_including = dist + cost(key, prev, depth - 1, cache);
			search_queue.push((-(distance_including as i64), (new_x, new_y), key, result_key.unwrap_or('?')));
		}
	}
	unreachable!()
}

fn numcode_cost(numcode: &Vec<char>, depth: usize) -> usize {
	let mut search_queue = BinaryHeap::new();
	search_queue.push((0, (2, 3), 'A', 0));
	let mut cache = HashMap::new();
	let mut seen = HashSet::new();
	while !search_queue.is_empty() {
		let (priority, (x, y), prev, codepos) = search_queue.pop().unwrap();
		let dist = (-priority) as usize;
		if codepos == numcode.len() {
			return dist;
		}
		let state = ((x, y), prev, codepos);
		if seen.contains(&state) {
			continue;
		}
		seen.insert(state);
		for key in ['A', '<', '^', '>', 'v'] {
			let (new_x, new_y, result_key) = use_dirpad(x, y, key, NUMPAD);
			if *NUMPAD.get(new_y).and_then(|row| row.get(new_x)).unwrap_or(&' ') == ' ' {
				continue;
			}
			let mut new_codepos = codepos;
			if result_key.is_some() {
				if result_key.unwrap() != numcode[codepos] {
					continue;
				}
				new_codepos += 1;
			}
			let distance_including = dist + cost(key, prev, depth, &mut cache);
			search_queue.push((-(distance_including as i64), (new_x, new_y), key, new_codepos));
		}
	}
	unreachable!()
}

pub fn part1(input: String) -> usize {
	let numcodes = parse(input);

	let mut sum = 0;
	for numcode in numcodes {
		let codecost = numcode_cost(&numcode, 2);
		let numeric_bit = numcode[..numcode.len() - 1].iter().collect::<String>();
		let numeric_number = numeric_bit.parse::<usize>().unwrap();
		let complexity = numeric_number * codecost;
		sum += complexity;
	}
	return sum;
}

pub fn part2(input: String) -> usize {
	let numcodes = parse(input);

	numcodes.into_par_iter().map(|numcode| {
		let codecost = numcode_cost(&numcode, 25);
		let numeric_bit = numcode[..numcode.len() - 1].iter().collect::<String>();
		let numeric_number = numeric_bit.parse::<usize>().unwrap();
		numeric_number * codecost
	}).sum()
}