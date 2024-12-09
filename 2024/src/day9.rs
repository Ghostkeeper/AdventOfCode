fn parse(input: String) -> Vec<i32> {
	let mut result = vec!();
	let mut is_empty_space = false; //Whether the next file should be empty or an actual file.
	let mut next_id = 0;
	for file in input.chars() {
		let length = file as u32 - '0' as u32;
		if is_empty_space {
			for _ in 0..length {
				result.push(-1);
			}
		} else {
			for _ in 0..length {
				result.push(next_id);
			}
			next_id += 1;
		}
		is_empty_space = !is_empty_space;
	}

	return result;
}

pub fn part1(input: String) -> u64 {
	let mut memory = parse(input);

	let mut i = 0;
	while i < memory.len() {
		if memory[i] == -1 { //Empty space we can fill.
			memory[i] = memory.pop().unwrap();
			while memory[memory.len() - 1] == -1 {
				memory.pop();
			}
		}
		i += 1;
	}

	let mut checksum = 0;
	for i in 0..memory.len() {
		checksum += memory[i] as u64 * i as u64;
	}

	return checksum;
}