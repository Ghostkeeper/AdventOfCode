fn parse(input: String) -> Vec<Vec<Vec<bool>>> {
    let mut result = vec!();
    for field in input.split("\n\n") {
        result.push(vec!());
        for row in field.split("\n") {
            result.last_mut().unwrap().push(vec!());
            for char in row.chars() {
                let is_rock = char == '#';
                result.last_mut().unwrap().last_mut().unwrap().push(is_rock);
            }
        }
    }
    return result;
}

fn find_reflection(field: &Vec<Vec<bool>>, required_num_errors: i32) -> usize {
    for y in 1..field.len() {
        if test_reflection_horizontal(&field, y) == required_num_errors {
            return y * 100;
        }
    }
    for x in 1..field[0].len() {
        if test_reflection_vertical(&field, x) == required_num_errors {
            return x;
        }
    }
    return 0; //Meaning: No reflection found.
}

fn test_reflection_horizontal(field: &Vec<Vec<bool>>, reflect_y: usize) -> i32 {
    let mut num_errors = 0;
    for x in 0..field[0].len() {
        for dy in 0..(field.len() / 2) {
            let yplus = reflect_y + dy;
            let ymin = reflect_y as i32 - 1 - (dy as i32);
            if yplus >= field.len() || ymin < 0 {
                break; //Don't go out of bounds.
            }
            if field[yplus][x] != field[ymin as usize][x] {
                num_errors += 1;
            }
        }
    }
    return num_errors;
}

fn test_reflection_vertical(field: &Vec<Vec<bool>>, reflect_x: usize) -> i32 {
    let mut num_errors = 0;
    for y in 0..field.len() {
        for dx in 0..(field[0].len() / 2) {
            let xplus = reflect_x + dx;
            let xmin = reflect_x as i32 - 1 - (dx as i32);
            if xplus >= field[0].len() || xmin < 0 {
                break; //Don't go out of bounds.
            }
            if field[y][xplus] != field[y][xmin as usize] {
                num_errors += 1;
            }
        }
    }
    return num_errors;
}

pub fn part1(input: String) -> usize {
    let fields = parse(input);
    let mut sum = 0;
    for field in fields {
        sum += find_reflection(&field, 0);
    }
    return sum;
}

pub fn part2(input: String) -> usize {
    let fields = parse(input);
    let mut sum = 0;
    for mut field in fields {
        sum += find_reflection(&mut field, 1);
    }
    return sum;
}