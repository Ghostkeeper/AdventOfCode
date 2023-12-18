fn execute(command: &str, verts: &mut Vec<(i32, i32)>) {
    let mut parts = command.split_whitespace();
    let direction = parts.next().unwrap();
    let distance = parts.next().unwrap().parse::<i32>().unwrap();
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

fn area(verts: &Vec<(i32, i32)>) -> i64 {
    let mut sum = 0i64;
    for i in 0..verts.len() {
        sum += (verts[i].0 * verts[(i + 1) % verts.len()].1 - verts[i].1 * verts[(i + 1) % verts.len()].0) as i64;
    }
    return sum.abs() / 2;
}

fn length(verts: &Vec<(i32, i32)>) -> i64 {
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