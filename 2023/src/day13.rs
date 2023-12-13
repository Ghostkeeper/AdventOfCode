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

fn find_reflection(field: &Vec<Vec<bool>>) -> usize {
    for y in 1..field.len() {
        if test_reflection_horizontal(&field, y) {
            return y * 100;
        }
    }
    for x in 1..field[0].len() {
        if test_reflection_vertical(&field, x) {
            return x;
        }
    }
    panic!("No reflection found!");
}

fn test_reflection_horizontal(field: &Vec<Vec<bool>>, reflect_y: usize) -> bool {
    for x in 0..field[0].len() {
        for dy in 0..(field.len() / 2) {
            let yplus = reflect_y + dy;
            let ymin = reflect_y as i32 - 1 - (dy as i32);
            if yplus >= field.len() || ymin < 0 {
                break; //Don't go out of bounds.
            }
            if field[yplus][x] != field[ymin as usize][x] {
                return false;
            }
        }
    }
    return true;
}

fn test_reflection_vertical(field: &Vec<Vec<bool>>, reflect_x: usize) -> bool {
    for y in 0..field.len() {
        for dx in 0..(field[0].len() / 2) {
            let xplus = reflect_x + dx;
            let xmin = reflect_x as i32 - 1 - (dx as i32);
            if xplus >= field[0].len() || xmin < 0 {
                break; //Don't go out of bounds.
            }
            if field[y][xplus] != field[y][xmin as usize] {
                return false;
            }
        }
    }
    return true;
}

pub fn part1(input: String) -> usize {
    let fields = parse(input);
    let mut sum = 0;
    for field in fields {
        sum += find_reflection(&field);
    }
    return sum;
}