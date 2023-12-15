fn hash(sequence: &str) -> i32 {
	let mut hash = 0;
	for char in sequence.chars() {
		hash += char as i32;
		hash *= 17;
		hash %= 256;
	}
	return hash;
}

pub fn part1(input: String) -> i32 {
	let mut sum = 0;
	for sequence in input.split(",") {
		sum += hash(sequence);
	}
	return sum;
}

pub fn part2(input: String) -> i32 {
	const EMPTY_BOX: Vec<(&str, u8)> = vec!();
	let mut boxes = [EMPTY_BOX; 256];
	for sequence in input.split(",") {
		let label = sequence.split("-").next().unwrap().split("=").next().unwrap();
		let lightbox = hash(label) as usize;
		let operation = sequence.chars().nth(label.len()).unwrap();
		match operation {
			'-' => {
				let index = boxes[lightbox].iter().position(|l| l.0 == label);
				if index.is_some() {
					boxes[lightbox].remove(index.unwrap());
				}
			},
			'=' => {
				let focal_length = sequence.split("=").nth(1).unwrap().parse::<u8>().unwrap();
				let index = boxes[lightbox].iter().position(|l| l.0 == label);
				match index {
					Some(pos) => boxes[lightbox][pos].1 = focal_length,
					None => boxes[lightbox].push((label, focal_length)),
				}
			}
			_ => panic!("Unknown operation!"),
		}
	}

	let mut result = 0i32;
	for (boxid, box_) in boxes.iter().enumerate() {
		for (lensid, lens) in box_.iter().enumerate() {
			result += (boxid as i32 + 1) * (lensid as i32 + 1) * lens.1 as i32;
		}
	}
	return result;
}