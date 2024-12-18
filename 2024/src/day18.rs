use itertools::Itertools;
use pathfinding::prelude::dijkstra;

fn parse(input: String) -> Vec<(usize, usize)> {
    input.split("\n").map(|line| {
        let mut parts = line.split(",");
        let x = parts.next().unwrap().parse::<usize>().unwrap();
        let y = parts.next().unwrap().parse::<usize>().unwrap();
        (x, y)
    }).collect_vec()
}

pub fn part1(input: String) -> usize {
    let corrupting_locations = parse(input);
    let mut grid = [[' '; 71]; 71];
    for &(x, y) in &corrupting_locations[0..1024] {
        grid[y][x] = '#';
    }

    let shortest_path = dijkstra(&(0,0), |&(x, y)| {
        let mut neighbours = vec!();
        if x > 0 && grid[y][x - 1] == ' ' { neighbours.push((x - 1, y)); }
        if x < grid[0].len() - 1 && grid[y][x + 1] == ' ' { neighbours.push((x + 1, y)); }
        if y > 0 && grid[y - 1][x] == ' ' { neighbours.push((x, y - 1)); }
        if y < grid.len() - 1 && grid[y + 1][x] == ' ' { neighbours.push((x, y + 1)); }
        neighbours.into_iter().map(|p| (p, 1))
    }, |&(x, y)| x == 70 && y == 70).expect("There is no path possible.");

    return shortest_path.0.len() - 1;
}