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