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

pub fn part2(input: String) -> u64 {
	let memory = parse(input);

	//A bit stupid, but let's transform this back into a RLE datastructure so we can re-use the parse() function...
	let mut pieces = vec!();
	let mut previous_id = -1;
	let mut length: usize = 0;
	for i in 0..memory.len() {
		if memory[i] == previous_id {
			length += 1;
			continue;
		}
		pieces.push((previous_id, length));
		previous_id = memory[i];
		length = 1;
	}
	pieces.push((previous_id, length));

	let mut i = pieces.len() - 1;
	'pieces_loop: while i > 0 { //Won't hit pieces[0], but that one can never move anyway.
		if pieces[i].0 == -1 {
			i -= 1;
			continue;
		}
		let (id, length) = pieces[i];
		for destination in 0..i {
			if pieces[destination].0 == -1 && pieces[destination].1 >= length {
				//Move the piece. It fits here!
				pieces[destination].1 -= length;
				pieces.insert(destination, (id, length));
				pieces[i + 1].0 = -1;
				continue 'pieces_loop;
				//Don't decrease i because all items have shifted by 1!
			}
		}
		//Couldn't find a better place for this piece. Leave it there.
		i -= 1;
	}

	let mut checksum = 0;
	let mut pos = 0;
	for (id, length) in pieces {
		if id == -1 {
			pos += length;
			continue;
		}
		for j in pos..(pos + length) {
			checksum += j as u64 * id as u64;
		}
		pos += length;
	}
	return checksum;
}