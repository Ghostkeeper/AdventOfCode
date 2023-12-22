#[derive(Debug, Clone)]
struct Brick {
	x: u16,
	y: u16,
	z: u16,
	w: u16,
	h: u16,
	d: u16,
	supports: Vec<usize>,
	supported_by: Vec<usize>,
}

impl Brick {
	pub fn collides(&self, x: u16, y: u16, z: u16) -> bool {
		x >= self.x && y >= self.y && z >= self.z && x < self.x
			&& x < self.x + self.w && y < self.y + self.h && z < self.z + self.d
	}

	pub fn collides_brick(&self, other: &Brick) -> bool {
		let x_overlap = (other.x..(other.x + other.w)).contains(&self.x)
			|| (self.x..(self.x + self.w)).contains(&other.x);
		let y_overlap = (other.y..(other.y + other.h)).contains(&self.y)
			|| (self.y..(self.y + self.h)).contains(&other.y);
		let z_overlap = (other.z..(other.z + other.d)).contains(&self.z)
			|| (self.z..(self.z + self.d)).contains(&other.z);
		return x_overlap && y_overlap && z_overlap;
	}

	pub fn supported_without(&self, brick_id: usize) -> bool {
		self.supported_by.len() != 1 || self.supported_by[0] != brick_id
	}
}

fn parse(input: String) -> Vec<Brick> {
	let mut result = vec!();

	for line in input.split("\n") {
		let mut coords = line.split("~");
		let mut start = [0, 0, 0];
		for (i, c) in coords.next().unwrap().split(",").enumerate() {
			start[i] = c.parse::<u16>().unwrap();
		}
		let mut end = [0, 0, 0];
		for (i, c) in coords.next().unwrap().split(",").enumerate() {
			end[i] = c.parse::<u16>().unwrap();
		}
		result.push(Brick {
			x: start[0],
			y: start[1],
			z: start[2],
			w: end[0] - start[0] + 1,
			h: end[1] - start[1] + 1,
			d: end[2] - start[2] + 1,
			supports: vec!(),
			supported_by: vec!(),
		});
	}

	return result;
}

pub fn part1(input: String) -> u32 {
	let mut bricks = parse(input);
	bricks.sort_unstable_by(|b1, b2| b1.z.cmp(&b2.z)); //Sort by Z.

	//Drop the bricks.
	for (dropped_id, brick) in bricks.clone().iter().enumerate() {
		let mut dropped_brick = brick.clone();
		for z in (1..brick.z).rev() {
			dropped_brick.z = z;
			let mut resting = false;
			for collides_id in (0..dropped_id).rev() {
				if dropped_brick.collides_brick(&bricks[collides_id]) {
					bricks[dropped_id].z = z + 1;
					bricks[collides_id].supports.push(dropped_id);
					bricks[dropped_id].supported_by.push(collides_id);
					resting = true;
				}
			}
			if resting {
				break;
			}
			if z == 1 { //Fell all the way down without colliding.
				bricks[dropped_id].z = 1;
			}
		}
	}

	let mut disintegrate_count = 0;
	for (disintegrate_id, _brick) in bricks.iter().enumerate() {
		if bricks.iter().all(|b| b.supported_without(disintegrate_id)) {
			disintegrate_count += 1;
		}
	}
	return disintegrate_count;
}