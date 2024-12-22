use itertools::Itertools;

fn parse(input: String) -> Vec<u64> {
	input.split("\n").map(|line| line.parse::<u64>().unwrap()).collect_vec()
}

fn pseudorandom(mut n: u64) -> u64 {
	n = ((n << 6) ^ n) & 0xFFFFFF;
	n = ((n >> 5) ^ n) & 0xFFFFFF;
	n = ((n << 11) ^ n) & 0xFFFFFF;
	n
}

pub fn part1(input: String) -> u64 {
	let secrets = parse(input);
	let mut sum = 0;
	for mut secret in secrets {
		for _ in 0..2000 {
			secret = pseudorandom(secret);
		}
		sum += secret;
	}
	sum
}