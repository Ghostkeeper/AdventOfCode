use itertools::Itertools;
use std::collections::HashMap;

fn parse(input: String) -> Vec<Vec<char>> {
	let mut rows = vec!();
	for line in input.split("\n") {
		rows.push(line.chars().collect_vec());
	}
	return rows;
}

fn make_unique(grid: Vec<Vec<char>>) -> Vec<Vec<u32>> {
    //First copy the chars into a new grid that we'll edit.
    //All new cell IDs will be above 1000 so we'll know which cells have already been processed.
    let mut result = vec!();
    for row in grid {
        let mut new_row = vec!();
        for cell in row {
            new_row.push(cell as u32);
        }
        result.push(new_row);
    }

    let mut next_id = 1000;
    for y in 0..result.len() {
        for x in 0..result[y].len() {
            let here = result[y][x];
            if here >= 1000 {
                continue; //Already processed by another floodfill.
            }
            //Floodfill this cell.
            let mut neighbours = vec!();
            neighbours.push((x, y));
            while neighbours.len() > 0 {
                let (dx, dy) = neighbours.pop().unwrap();
                if dx > 0 && result[dy][dx - 1] == here {
                    neighbours.push((dx - 1, dy));
                }
                if dy > 0 && result[dy - 1][dx] == here {
                    neighbours.push((dx, dy - 1));
                }
                if dx < result[y].len() - 1 && result[dy][dx + 1] == here {
                    neighbours.push((dx + 1, dy));
                }
                if dy < result.len() - 1 && result[dy + 1][dx] == here {
                    neighbours.push((dx, dy + 1));
                }
                result[dy][dx] = next_id;
            }
            next_id += 1;
        }
    }

    return result;
}

pub fn part1(input: String) -> u64 {
    let grid = parse(input);
    let ugrid = make_unique(grid);

    let mut area = HashMap::new();
    let mut perimeter = HashMap::new();
    for y in 0..ugrid.len() {
        for x in 0..ugrid[y].len() {
            let here = ugrid[y][x];
            if !area.contains_key(&ugrid[y][x]) {
                area.insert(&ugrid[y][x], 0);
                perimeter.insert(&ugrid[y][x], 0);
            }
            *area.get_mut(&here).unwrap() += 1;
            *perimeter.get_mut(&here).unwrap() += 4;
        }
    }
    for y in 0..ugrid.len() {
        for x in 1..ugrid[y].len() {
            if ugrid[y][x] == ugrid[y][x - 1] {
                *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 2;
            }
        }
    }
    for y in 1..ugrid.len() {
        for x in 0..ugrid[y].len() {
            if ugrid[y][x] == ugrid[y - 1][x] {
                *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 2;
            }
        }
    }

    let mut result = 0;
    for (cell_id, cell_area) in area {
        let cell_perimeter = perimeter.get(cell_id).unwrap();
        result += cell_area * cell_perimeter;
    }
    return result;
}

pub fn part2(input: String) -> u64 {
    let grid = parse(input);
    let ugrid = make_unique(grid);

    let mut area = HashMap::new();
    let mut perimeter = HashMap::new();
    for y in 0..ugrid.len() {
        for x in 0..ugrid[y].len() {
            let here = ugrid[y][x];
            if !area.contains_key(&ugrid[y][x]) {
                area.insert(&ugrid[y][x], 0);
                perimeter.insert(&ugrid[y][x], 0);
            }
            *area.get_mut(&here).unwrap() += 1;
            *perimeter.get_mut(&here).unwrap() += 4;
        }
    }
    for y in 0..ugrid.len() {
        for x in 1..ugrid[y].len() {
            if ugrid[y][x] == ugrid[y][x - 1] {
                *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 2;
            } else {
                if y > 0 && ugrid[y - 1][x] == ugrid[y][x] && ugrid[y - 1][x] != ugrid[y - 1][x - 1] {
                    //Border extends up higher.
                    *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 1;
                }
                if y > 0 && ugrid[y][x - 1] == ugrid[y - 1][x - 1] && ugrid[y - 1][x] != ugrid[y - 1][x - 1] {
                    *perimeter.get_mut(&ugrid[y][x - 1]).unwrap() -= 1;
                }
            }
        }
        if y > 0 && ugrid[y - 1][0] == ugrid[y][0] {
            *perimeter.get_mut(&ugrid[y][0]).unwrap() -= 1;
        }
        if y > 0 && ugrid[y - 1][ugrid[0].len() - 1] == ugrid[y][ugrid[0].len() - 1] {
            *perimeter.get_mut(&ugrid[y][ugrid[0].len() - 1]).unwrap() -= 1;
        }
    }
    for y in 1..ugrid.len() {
        for x in 0..ugrid[y].len() {
            if ugrid[y][x] == ugrid[y - 1][x] {
                *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 2;
            } else {
                if x > 0 && ugrid[y][x - 1] == ugrid[y][x] && ugrid[y][x - 1] != ugrid[y - 1][x - 1] {
                    //Border extends further left.
                    *perimeter.get_mut(&ugrid[y][x]).unwrap() -= 1;
                }
                if x > 0 && ugrid[y - 1][x] == ugrid[y - 1][x - 1] && ugrid[y][x - 1] != ugrid[y - 1][x - 1] {
                    *perimeter.get_mut(&ugrid[y - 1][x]).unwrap() -= 1;
                }
            }
        }
    }
    for x in 1..ugrid[0].len() {
        if ugrid[0][x - 1] == ugrid[0][x] {
            *perimeter.get_mut(&ugrid[0][x]).unwrap() -= 1;
        }
        if ugrid[ugrid.len() - 1][x - 1] == ugrid[ugrid.len() - 1][x] {
            *perimeter.get_mut(&ugrid[ugrid.len() - 1][x]).unwrap() -= 1;
        }
    }

    let mut result = 0;
    for (cell_id, cell_area) in area {
        let cell_perimeter = perimeter.get(cell_id).unwrap();
        result += cell_area * cell_perimeter;
    }
    return result;
}