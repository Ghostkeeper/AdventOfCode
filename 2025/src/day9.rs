fn parse(input: String) -> Vec<(i64, i64)> {
	let mut coordinates = vec!();
    for line in input.split("\n") {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        coordinates.push((x, y));
    }
    coordinates
}

fn area(v1: &(i64, i64), v2: &(i64, i64)) -> i64 {
	((v1.0 - v2.0).abs() + 1) * ((v1.1 - v2.1).abs() + 1)
}

pub fn part1(input: String) -> i64 {
    let vertices = parse(input);
	let mut biggest_area = 0;
	for i in 0..vertices.len() {
		for j in 0..i {
			biggest_area = biggest_area.max(area(&vertices[i], &vertices[j]));
		}
	}

	biggest_area
}

pub fn part2(input: String) -> i64 {
	let vertices = parse(input);
	let mut biggest_area = 0;

	for i in 0..vertices.len() {
		'rectfind: for j in 0..i {
			let v1 = vertices[i];
			let v2 = vertices[j];
			let minx = v1.0.min(v2.0);
			let miny = v1.1.min(v2.1);
			let maxx = v1.0.max(v2.0);
			let maxy = v1.1.max(v2.1);
			for k in 0..vertices.len() {
				let edge1 = vertices[k];
				let edge2 = vertices[(k + 1) % vertices.len()];
				if !(edge1.0 <= minx && edge2.0 <= minx) && !(edge1.1 <= miny && edge2.1 <= miny) && !(edge1.0 >= maxx && edge2.0 >= maxx) && !(edge1.1 >= maxy && edge2.1 >= maxy) {
					//Intersects with the rectangle. Not allowed then!
					continue 'rectfind;
				}
			}
			//Nothing intersects. So this is allowed!
			biggest_area = biggest_area.max(area(&v1, &v2));
		}
	}

	biggest_area
}