use itertools::Itertools;
use rayon::prelude::*;

fn parse(input: String) -> Vec<i32> {
	input.split("\n").map(|line| line.parse::<i32>().unwrap()).collect_vec()
}

fn pseudorandom(mut n: i32) -> i32 {
	n = ((n << 6) ^ n) & 0xFFFFFF;
	n = ((n >> 5) ^ n) & 0xFFFFFF;
	n = ((n << 11) ^ n) & 0xFFFFFF;
	n
}

pub fn part1(input: String) -> u64 {
	let secrets = parse(input);
	let mut sum = 0u64;
	for mut secret in secrets {
		for _ in 0..2000 {
			secret = pseudorandom(secret);
		}
		sum += secret as u64;
	}
	sum
}

pub fn part2(input: String) -> i32 {
	let secrets = parse(input);

	let max_profit_multithread = (-9..10).into_par_iter().map(|d1| {
		let mut max_profit = i32::MIN;
		for d2 in -9..10 {
			for d3 in -9..10 {
				for d4 in -9..10 {
					if d1 + d2 + d3 + d4 < -9 {
						continue;
					}
					let mut profit = 0;
					for mut secret in secrets.clone() {
						let mut diff1;
						let (mut diff2, mut diff3, mut diff4) = (100, 100, 100);
						for _ in 0..2000 {
							let previous = secret;
							secret = pseudorandom(secret);
							diff1 = diff2;
							diff2 = diff3;
							diff3 = diff4;
							diff4 = (secret % 10) - (previous % 10);
							if d1 == diff1 && d2 == diff2 && d3 == diff3 && d4 == diff4 {
								//Sell!
								profit += secret % 10;
								break;
							}
						}
					}
					if profit > max_profit {
						max_profit = profit;
					}
				}
			}
		}
		max_profit
	}).max().unwrap();

	max_profit_multithread
}