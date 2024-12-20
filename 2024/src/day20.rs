use pathfinding::prelude::dijkstra;
use std::collections::HashMap;

fn parse(input: String) -> (Vec<Vec<char>>, (usize, usize), (usize, usize)) {
	let mut rows = vec!();
	let mut start_pos = (0, 0);
	let mut goal_pos = (0, 0);
	let mut y = 0;
	for line in input.split("\n") {
		let mut row = vec!();
		let mut x = 0;
		for char in line.chars() {
			if char == 'S' {
				row.push('.');
				start_pos = (x, y);
			} else if char == 'E' {
				row.push('.');
				goal_pos = (x, y);
			} else {
				row.push(char);
			}
			x += 1;
		}
		rows.push(row);
		y += 1;
	}

	return (rows, start_pos, goal_pos);
}

pub fn part1(input: String) -> usize {
    let (grid, start, end) = parse(input);
    let no_cheating = dijkstra(&start, |&(x, y)| {
			let mut neighbours = vec!();
			if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
			if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
			if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
			if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
			neighbours.into_iter().map(|p| (p, 1))
		}, |&pos| pos == end).unwrap();
	let no_cheating_time = no_cheating.0.len() - 1;

    let mut saves_time = HashMap::new();
	for cheat_start in &no_cheating.0 {
        let first_half = dijkstra(&start, |&(x, y)| {
            let mut neighbours = vec!();
            if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
            if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
            if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
            if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
            neighbours.into_iter().map(|p| (p, 1))
        }, |&pos| pos == *cheat_start).unwrap();

        let mut possible_cheat_ends = vec!();
        if cheat_start.0 > 2 && grid[cheat_start.1][cheat_start.0 - 1] == '#' { possible_cheat_ends.push((cheat_start.0 - 1, cheat_start.1)); }
        if cheat_start.0 < grid[0].len() - 2 && grid[cheat_start.1][cheat_start.0 + 1] == '#' { possible_cheat_ends.push((cheat_start.0 + 1, cheat_start.1)); }
        if cheat_start.1 > 2 && grid[cheat_start.1 - 1][cheat_start.0] == '#' { possible_cheat_ends.push((cheat_start.0, cheat_start.1 - 1)); }
        if cheat_start.1 < grid.len() - 2 && grid[cheat_start.1 + 1][cheat_start.0] == '#' { possible_cheat_ends.push((cheat_start.0, cheat_start.1 + 1)); }
        for cheat_end in possible_cheat_ends {
            let second_half = dijkstra(&cheat_end, |&(x, y)| {
                let mut neighbours = vec!();
                if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
                if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
                if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
                if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
                neighbours.into_iter().map(|p| (p, 1))
            }, |&pos| pos == end);
            if second_half.is_some() {
                let cheating_time = first_half.0.len() - 1 + second_half.unwrap().0.len();
                if no_cheating_time as i32 - cheating_time as i32 >= 100 {
                    saves_time.insert((cheat_start, cheat_end), no_cheating_time - cheating_time);
                }
            }
        }
    }

    return saves_time.len();
}

pub fn part2(input: String) -> usize {
    let (grid, start, end) = parse(input);
    let no_cheating = dijkstra(&start, |&(x, y)| {
			let mut neighbours = vec!();
			if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
			if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
			if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
			if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
			neighbours.into_iter().map(|p| (p, 1))
		}, |&pos| pos == end).unwrap();
	let no_cheating_time = no_cheating.0.len() - 1;

    let mut saves_time = HashMap::new();
	for cheat_start in &no_cheating.0 {
        let first_half = dijkstra(&start, |&(x, y)| {
            let mut neighbours = vec!();
            if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
            if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
            if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
            if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
            neighbours.into_iter().map(|p| (p, 1))
        }, |&pos| pos == *cheat_start).unwrap();

        let (sx, sy) = *cheat_start;
        let mut possible_cheat_ends = vec!();
        for dx in -20i32..20i32 {
            let ex = sx as i32 + dx;
            if ex < 1 || ex >= grid[0].len() as i32 - 1 {
                continue; //Don't go into the border.
            }
            for dy in -20..20 {
                let ey = sy as i32 + dy;
                if ey < 1 || ey >= grid.len() as i32 - 1 {
                    continue; //Don't go into the border.
                }
                let cheat_time = dx.abs() + dy.abs();
                if cheat_time > 20 {
                    continue; //More than 20 picoseconds is not allowed.
                }
                if grid[ey as usize][ex as usize] != '.' {
                    continue; //Must end on an open spot.
                }
                possible_cheat_ends.push((ex as usize, ey as usize));
            }
        }
        for cheat_end in possible_cheat_ends {
            let second_half = dijkstra(&cheat_end, |&(x, y)| {
                let mut neighbours = vec!();
                if grid[y][x - 1] == '.' { neighbours.push((x - 1, y)); }
                if grid[y][x + 1] == '.' { neighbours.push((x + 1, y)); }
                if grid[y - 1][x] == '.' { neighbours.push((x, y - 1)); }
                if grid[y + 1][x] == '.' { neighbours.push((x, y + 1)); }
                neighbours.into_iter().map(|p| (p, 1))
            }, |&pos| pos == end);
            if second_half.is_some() {
                let cheat_time = ((cheat_start.0 as i32 - cheat_end.0 as i32).abs() + (cheat_start.1 as i32 - cheat_end.1 as i32).abs()) as usize;
                let cheating_time = first_half.0.len() - 1 + cheat_time + second_half.unwrap().0.len() - 1;
                if no_cheating_time as i32 - cheating_time as i32 >= 50 {
                    let savings = no_cheating_time - cheating_time;
                    saves_time.insert((cheat_start, cheat_end), savings);
                }
            }
        }
    }

    return saves_time.len();
}