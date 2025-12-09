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