use itertools::Itertools;

fn parse(input: String) -> Vec<u64> {
    input.split(" ").map(|x| x.parse::<u64>().unwrap()).collect_vec()
}

fn blink(stones: &mut Vec<u64>) {
    let num_stones = stones.len();
    for i in 0..num_stones {
        if stones[i] == 0 {
            stones[i] = 1;
        } else {
            let num_digits = stones[i].checked_ilog10().unwrap_or(0) + 1;
            if num_digits % 2 == 0 {
                let lower = stones[i] % 10_u64.pow(num_digits / 2);
                let upper = (stones[i] - lower) / 10_u64.pow(num_digits / 2);
                stones[i] = lower;
                stones.push(upper);
            } else {
                stones[i] *= 2024;
            }
        }
    }
}

pub fn part1(input: String) -> usize {
    let mut stones = parse(input);

    for _ in 0..25 {
        blink(&mut stones);
    }

    return stones.len();
}

pub fn part2(input: String) -> usize {
    let mut stones = parse(input);

    for _ in 0..75 {
        blink(&mut stones);
    }

    return stones.len();
}