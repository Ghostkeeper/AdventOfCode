fn parse(input: String) -> Vec<(i64, i64, i64)> {
	let mut coordinates = vec!();
    for line in input.split("\n") {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<i64>().unwrap();
        let y = parts.next().unwrap().parse::<i64>().unwrap();
        let z = parts.next().unwrap().parse::<i64>().unwrap();
        coordinates.push((x, y, z));
    }
    coordinates
}

fn distance(v1: (i64, i64, i64), v2: (i64, i64, i64)) -> i64 {
    (v1.0 - v2.0) * (v1.0 - v2.0) + (v1.1 - v2.1) * (v1.1 - v2.1) + (v1.2 - v2.2) * (v1.2 - v2.2)
}

fn connect_closest(vertices: &Vec<(i64, i64, i64)>, connected: &mut Vec<Vec<usize>>) {
    let mut smallest_distance = i64::MAX;
    let mut smallest_i = 0;
    let mut smallest_j = 0;
    for i in 0..vertices.len() {
        for j in 0..i {
            if connected[i].contains(&j) {
                continue;
            }
            let dist = distance(vertices[i], vertices[j]);
            if dist < smallest_distance {
                smallest_distance = dist;
                smallest_i = i;
                smallest_j = j;
            }
        }
    }
    if smallest_i == 0 && smallest_j == 0 {
        panic!("Couldn't find anything to connect.")
    }
    connected[smallest_i].push(smallest_j);
    connected[smallest_j].push(smallest_i);
}

fn circuit_sizes(connected: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut circuits = vec!();
    let mut found = vec!();
    for _ in 0..connected.len() {
        found.push(false);
    }
    for i in 0..connected.len() {
        if found[i] {
            continue;
        }
        circuits.push(0);
        let circuit_id = circuits.len() - 1;
        let mut todo = vec!();
        todo.push(i);
        while !todo.is_empty() {
            let neighbour = todo.pop().unwrap();
            for j in 0..connected[neighbour].len() {
                let adjacent = connected[neighbour][j];
                if found[adjacent] {
                    continue;
                }
                circuits[circuit_id] += 1;
                found[adjacent] = true;
                todo.push(adjacent);
            }
        }
        if circuits[circuit_id] == 0 {
            circuits[circuit_id] += 1;
        }
    }
    circuits
}

pub fn part1(input: String) -> usize {
    let vertices = parse(input);
    let mut connected = vec!();
    for _ in 0..vertices.len() {
        connected.push(vec!());
    }
    for _ in 0..1000 {
        connect_closest(&vertices, &mut connected);
    }
    let mut sizes = circuit_sizes(&connected);
    sizes.sort_unstable();
    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
}