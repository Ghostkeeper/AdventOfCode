use itertools::Itertools;
use rayon::prelude::*;

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

fn blink_n(stone: u64, n: u8) -> usize {
    if n == 0 {
        return 1;
    }
    if stone == 0 {
        return blink_n(1, n - 1);
    }
    let num_digits = stone.checked_ilog10().unwrap_or(0) + 1;
    if num_digits % 2 == 0 {
        let lower = stone % 10_u64.pow(num_digits / 2);
        let upper = (stone - lower) / 10_u64.pow(num_digits / 2);
        return blink_n(lower, n - 1) + blink_n(upper, n - 1);
    }
    return blink_n(stone * 2024, n - 1);
}

pub fn part2(input: String) -> usize {
    let mut stones = parse(input);
    for _ in 0..4 { //A few layers fully calculated to allow creating more threads.
        blink(&mut stones);
    }

    let results: Vec<usize> = stones.into_par_iter().map(|stone| blink_n(stone, 71)).collect();
    let mut result = 0;
    for stone in results {
        result += stone;
    }
    return result;
}