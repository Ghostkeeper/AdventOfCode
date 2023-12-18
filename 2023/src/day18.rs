use std::collections::HashSet;

fn execute(grid: &mut HashSet<(i32, i32)>, pos: &mut (i32, i32), command: &str) {
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
    for _ in 0..distance {
        pos.0 += delta.0;
        pos.1 += delta.1;
        grid.insert(*pos);
    }
}

fn floodfill(grid: &mut HashSet<(i32, i32)>) {
    let mut max = (0, 0);
    let mut min = (0, 0);
    for (x, y) in grid.iter() {
        max.0 = max.0.max(*x);
        max.1 = max.1.max(*y);
        min.0 = min.0.min(*x);
        min.1 = min.1.min(*y);
    }
    max.0 += 1; //Add a border so that we're sure that the outside is outside.
    max.1 += 1;
    min.0 -= 1;
    min.1 -= 1;
    let mut outside = HashSet::new();
    let mut todo = vec![(0, 0)];
    while !todo.is_empty() {
        let next = todo.pop().unwrap();
        let candidates = [
            (next.0 + 1, next.1),
            (next.0, next.1 - 1),
            (next.0 - 1, next.1),
            (next.0, next.1 + 1),
        ];
        for candidate in candidates {
            if candidate.0 < min.0 || candidate.0 > max.0 || candidate.1 < min.1 || candidate.1 > max.1 {
                continue; //Out of bounds.
            }
            if grid.contains(&candidate) {
                continue; //Part of the border. Don't cross!
            }
            if outside.contains(&candidate) {
                continue; //Already processed this one.
            }
            outside.insert(candidate);
            todo.push(candidate);
        }
    }
    //Anything that's not outside is inside.
    //The border will again be re-added, but since it's a set that doesn't matter.
    for x in min.0..max.0 {
        for y in min.1..max.1 {
            if !outside.contains(&(x, y)) {
                grid.insert((x, y));
            }
        }
    }
}

pub fn part1(input: String) -> usize {
    let mut grid = HashSet::new();
    let mut pos = (0, 0);
    for command in input.split("\n") {
        execute(&mut grid, &mut pos, command);
    }
    floodfill(&mut grid);
    return grid.len();
}