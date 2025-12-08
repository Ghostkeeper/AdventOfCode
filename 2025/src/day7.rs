fn parse(input: String) -> Vec<Vec<char>> {
	let mut grid = vec!();
	for line in input.split("\n") {
        grid.push(line.chars().collect());
	}
    grid
}

fn cast_down(grid: &mut Vec<Vec<char>>, y: usize) -> u32 {
    let mut result = 0;
    for x in 0..grid[y].len() {
        if grid[y][x] == 'S' || grid[y][x] == '|' {
            if grid[y + 1][x] == '^' {
                grid[y + 1][x - 1] = '|';
                grid[y + 1][x + 1] = '|';
                result += 1;
            } else {
                grid[y + 1][x] = '|';
            }
        }
    }
    result
}

pub fn part1(input: String) -> u32 {
    let mut grid = parse(input);

    let mut split_count = 0;
    for y in 0..grid.len() - 1 {
        split_count += cast_down(&mut grid, y);
    }
    split_count
}

fn split_down(grid: &mut Vec<Vec<char>>, beam_count: &mut Vec<Vec<u64>>, y: usize) {
    for x in 0..grid[y].len() {
        if grid[y][x] == 'S' {
            beam_count[y][x] = 1;
        }
    }
    for x in 0..grid[y].len() {
        if beam_count[y][x] > 0 {
            if grid[y + 1][x] == '^' {
                beam_count[y + 1][x - 1] += beam_count[y][x];
                beam_count[y + 1][x + 1] += beam_count[y][x];
            } else {
                beam_count[y + 1][x] += beam_count[y][x];
            }
        }
    }
}

pub fn part2(input: String) -> u64 {
    let mut grid = parse(input);
    let mut beam_count = vec!();
    for row in &grid {
        beam_count.push(vec![0u64; row.len()]);
    }

    for y in 0..grid.len() - 1 {
        split_down(&mut grid, &mut beam_count, y);
    }
    let mut total = 0;
    for x in 0..beam_count[beam_count.len() - 1].len() {
        total += beam_count[beam_count.len() - 1][x];
    }
    total
}