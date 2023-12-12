use std::cmp::max;

struct Game {
	id: u32,
	reds: u32,
	greens: u32,
	blues: u32,
}

fn parse(input: String) -> Vec<Game> {
	let mut result = vec!();
	for line in input.split("\n") {
		let id = line[5..].split(": ").next().unwrap().parse::<u32>().unwrap();
		let mut reds = 0;
		let mut greens = 0;
		let mut blues = 0;
		for part in line[5..].split(": ").last().unwrap().split("; ") {
			for grab in part.split(", ") {
				let mut segments = grab.split(" ");
				let number = segments.next().unwrap().parse::<u32>().unwrap();
				let colour = segments.next().unwrap();
				if colour == "red" {
					reds = max(reds, number);
				}
				if colour == "green" {
					greens = max(greens, number);
				}
				if colour == "blue" {
					blues = max(blues, number);
				}
			}
		}
		result.push(Game {
			id: id,
			reds: reds,
			greens: greens,
			blues: blues,
		});
	}
	return result;
}

pub fn part1(input: String) -> u32 {
	let games = parse(input);
	let mut sum = 0;
	for game in games {
		if game.reds <= 12 && game.greens <= 13 && game.blues <= 14 {
			sum += game.id;
		}
	}
	return sum;
}

pub fn part2(input: String) -> u32 {
	let games = parse(input);
	let mut sum = 0;
	for game in games {
		sum += game.reds * game.greens * game.blues;
	}
	return sum;
}