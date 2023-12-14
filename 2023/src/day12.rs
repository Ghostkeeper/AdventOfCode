use std::collections::HashMap;
use itertools::Itertools;

fn parse(input: String) -> (Vec<String>, Vec<Vec<i32>>) {
	let mut fields = vec!();
	let mut sequences = vec!();

	for line in input.split("\n") {
		let mut parts = line.split_whitespace();
		fields.push(parts.next().unwrap().to_string());
		sequences.push(parts.next().unwrap().split(",").map(|n| n.parse::<i32>().unwrap()).collect_vec());
	}
	return (fields, sequences);
}

fn num_valid(field: &[u8], sequence: &Vec<i32>, pos: usize, next: char, cache: &mut HashMap<(Vec<i32>, usize, char), usize>) -> usize {
	let key = (sequence.clone(), pos, next);
	if cache.contains_key(&key) {
		return *cache.get(&key).unwrap();
	}
	if sequence.len() == 0 {
		if pos >= field.len() {
			cache.insert(key, 1);
			return 1; //Both sequence and field are matched, so this could be a possibility.
		}
		if field[pos] == '#' as u8 {
			cache.insert(key, 0);
			return 0; //No sequence left, but there are # left.
		} else {
			let num = num_valid(field, sequence, pos + 1, '?', cache); //Turn all ? into ., and hope there's only . left.
			cache.insert(key, num);
			return num;
		}
	}
	if pos >= field.len() {
		cache.insert(key, 0);
		return 0; //Still have sequence left, but no field left.
	}
	if next == '.' && field[pos] == '#' as u8 {
		cache.insert(key, 0);
		return 0; //Must start with ..
	}
	if next == '#' && field[pos] == '.' as u8 {
		cache.insert(key, 0);
		return 0; //Must start with #.
	}
	match field[pos] as char {
		'.' => {
			let num = num_valid(field, sequence, pos + 1, '?', cache);
			cache.insert(key, num);
			return num;
		},
		'#' => {
			let mut modified_sequence = sequence.clone();
			modified_sequence[0] -= 1;
			if modified_sequence[0] == 0 {
				modified_sequence.remove(0);
				let num = num_valid(field, &modified_sequence, pos + 1, '.', cache);
				cache.insert(key, num);
				return num;
			} else {
				let num = num_valid(field, &modified_sequence, pos + 1, '#', cache);
				cache.insert(key, num);
				return num;
			}
		},
		'?' => {
			let mut possibilities = 0;
			if next != '#' {
				possibilities += num_valid(field, sequence, pos + 1, '?', cache);
			}
			if next != '.' {
				let mut modified_sequence = sequence.clone();
				modified_sequence[0] -= 1;
				if modified_sequence[0] == 0 {
					modified_sequence.remove(0);
					possibilities += num_valid(field, &modified_sequence, pos + 1, '.', cache);
				} else {
					possibilities += num_valid(field, &modified_sequence, pos + 1, '#', cache);
				}
			}
			cache.insert(key, possibilities);
			return possibilities;
		},
		_ => panic!("Unknown character in field."),
	}
}

pub fn part1(input: String) -> usize {
	let (fields, sequences) = parse(input);
	let mut sum_arrangements = 0;
	for i in 0..fields.len() {
		let mut cache: HashMap<(Vec<i32>, usize, char), usize> = HashMap::new();
		let num = num_valid(fields[i].as_bytes(), &sequences[i], 0, '?', &mut cache);
		sum_arrangements += num;
	}
	return sum_arrangements;
}

pub fn part2(input: String) -> usize {
	let (fields, sequences) = parse(input);
	let mut sum_arrangements = 0;
	for i in 0..fields.len() {
		let mut multiplied_field = fields[i].clone();
		let mut multiplied_sequence: Vec<i32> = sequences[i].clone();
		for _ in 0..4 {
			multiplied_field += "?";
			multiplied_field += &fields[i];
			multiplied_sequence.append(&mut sequences[i].clone());
		}
		let mut cache: HashMap<(Vec<i32>, usize, char), usize> = HashMap::new();
		let num = num_valid(multiplied_field.as_bytes(), &multiplied_sequence, 0, '?', &mut cache);
		sum_arrangements += num;
	}
	return sum_arrangements;
}