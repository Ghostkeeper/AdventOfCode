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

fn connect_closest(vertices: &Vec<(i64, i64, i64)>, connected: &mut Vec<Vec<usize>>, dists: &mut Vec<(i64, usize, usize)>) -> i64 {
    let (_dist, i, j) = dists.pop().expect("Ran out of vertices to connect.");
    connected[i].push(j);
    connected[j].push(i);
    vertices[i].0 * vertices[j].0
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

fn is_connected(connected: &Vec<Vec<usize>>) -> bool {
    let mut found = vec!();
    for _ in 0..connected.len() {
        found.push(false);
    }

    let mut todo = vec!();
    todo.push(0);
    while !todo.is_empty() {
        let neighbour = todo.pop().unwrap();
        for j in 0..connected[neighbour].len() {
            let adjacent = connected[neighbour][j];
            if found[adjacent] {
                continue;
            }
            found[adjacent] = true;
            todo.push(adjacent);
        }
    }
    for is_found in found {
        if !is_found {
            return false;
        }
    }
    return true;
}

fn distances(vertices: &Vec<(i64, i64, i64)>) -> Vec<(i64, usize, usize)> {
    let mut result = vec!();
    for i in 0..vertices.len() {
        for j in 0..i {
            result.push((distance(vertices[i], vertices[j]), i, j));
        }
    }
    result.sort_unstable_by_key(|d| d.0);
    result.reverse();
    result
}

pub fn part1(input: String) -> usize {
    let vertices = parse(input);
    let mut dists = distances(&vertices);
    let mut connected = vec!();
    for _ in 0..vertices.len() {
        connected.push(vec!());
    }
    for _ in 0..1000 {
        connect_closest(&vertices, &mut connected, &mut dists);
    }
    let mut sizes = circuit_sizes(&connected);
    sizes.sort_unstable();
    sizes[sizes.len() - 1] * sizes[sizes.len() - 2] * sizes[sizes.len() - 3]
}

pub fn part2(input: String) -> i64 {
    let vertices = parse(input);
    let mut dists = distances(&vertices);
    let mut connected = vec!();
    for _ in 0..vertices.len() {
        connected.push(vec!());
    }

    let mut answer = 0;
    while !is_connected(&connected) {
        answer = connect_closest(&vertices, &mut connected, &mut dists);
    }
    answer
}