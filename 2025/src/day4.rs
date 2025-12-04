fn parse(input: String) -> Vec<Vec<char>> {
	let mut result = vec!();
	for line in input.split("\n") {
        result.push(line.chars().collect());
	}
	result
}

fn consume(grid: &mut Vec<Vec<char>>) -> u32 {
    let original_grid = grid.clone();
    let mut eaten = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            if grid[y][x] != '@' {
                continue;
            }
            let start_y = (y as i32 - 1).max(0) as usize;
            let start_x = (x as i32 - 1).max(0) as usize;
            let end_y = (y + 2).min(grid.len());
            let end_x = (x + 2).min(grid[y].len());
            let mut count = 0;
            for dy in start_y..end_y {
                for dx in start_x..end_x {
                    if original_grid[dy][dx] == '@' {
                        count += 1;
                    }
                }
            }
            if count < 5 { //Less than 4, but including the roll itself.
                grid[y][x] = 'x';
                eaten += 1;
            }
        }
    }
    eaten
}

pub fn part1(input: String) -> u32 {
    let mut grid = parse(input);
	let result = consume(&mut grid);
    return result;
}