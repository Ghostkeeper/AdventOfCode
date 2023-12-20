fn execute(command: &str, verts: &mut Vec<(i64, i64)>) {
	let mut parts = command.split_whitespace();
	let direction = parts.next().unwrap();
	let distance = parts.next().unwrap().parse::<i64>().unwrap();
	let delta = match direction {
		"R" => (1, 0),
		"U" => (0, -1),
		"L" => (-1, 0),
		"D" => (0, 1),
		_ => panic!("Unknown direction!"),
	};
	let prev = verts.last().unwrap();
	let new = (prev.0 + delta.0 * distance, prev.1 + delta.1 * distance);
	verts.push(new);
}

fn execute2(command: &str, verts: &mut Vec<(i64, i64)>) {
	let parts = command.split_whitespace();
	let part = parts.last().unwrap();
	let distance = i64::from_str_radix(&part[2..(part.len() - 2)], 16).unwrap();
	let direction = part.chars().nth(part.len() - 2).unwrap() as i32 - 48;
	let delta = match direction {
		0 => (1, 0),
		1 => (0, 1),
		2 => (-1, 0),
		3 => (0, -1),
		_ => panic!("Unknown direction integer!"),
	};
	let prev = verts.last().unwrap();
	let new = (prev.0 + delta.0 * distance, prev.1 + delta.1 * distance);
	verts.push(new);
}

fn area(verts: &Vec<(i64, i64)>) -> i64 {
	let mut sum = 0i64;
	for i in 0..verts.len() {
		sum += (verts[i].0 * verts[(i + 1) % verts.len()].1 - verts[i].1 * verts[(i + 1) % verts.len()].0) as i64;
	}
	return sum.abs() / 2;
}

fn length(verts: &Vec<(i64, i64)>) -> i64 {
	let mut sum = 0i64;
	for i in 0..verts.len() {
		//Just do Manhattan distance. There are no diagonals anyway.
		sum += (verts[i].0 - verts[(i + 1) % verts.len()].0).abs() as i64;
		sum += (verts[i].1 - verts[(i + 1) % verts.len()].1).abs() as i64;
	}
	return sum;
}

pub fn part1(input: String) -> i64 {
	let mut verts = vec![(0, 0)];
	for command in input.split("\n") {
		execute(command, &mut verts);
	}
	return area(&verts) + length(&verts) / 2 + 1;
}

pub fn part2(input: String) -> i64 {
	let mut verts = vec![(0, 0)];
	for command in input.split("\n") {
		execute2(command, &mut verts);
	}
	return area(&verts) + length(&verts) / 2 + 1;
}