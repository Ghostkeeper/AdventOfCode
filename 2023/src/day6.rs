fn parse(input: String) -> (Vec<i32>, Vec<i32>) {
    let mut time = vec!();
    let mut space = vec!();
    for line in input.split("\n") {
        if line.starts_with("Time:") {
            time = line[9..].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        }
        if line.starts_with("Distance:") {
            space = line[9..].split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        }
    }
    return (time, space);
}

fn dist(time: i32, hold: i32) -> i32 {
    return (time - hold) * hold;
}

pub fn part1(input: String) {
    let (time, space) = parse(input);

    let mut results = vec!(0; time.len());
    for i in 0..time.len() {
        let mut num_win = 0usize;
        for hold in 0..time[i] {
            if dist(time[i], hold) > space[i] {
                num_win += 1;
            }
        }
        results[i] = num_win;
    }
    let mut result = 1;
    for wins in results {
        result *= wins;
    }
    println!("{}", result);
}