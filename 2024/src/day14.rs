use std::fs;
use std::os::linux::fs::MetadataExt;
use image::*;
use itertools::Itertools;
use regex::Regex;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn parse(input: String) -> Vec<((i32, i32), (i32, i32))> {
	let re = Regex::new(r"p=(?<px>\d+),(?<py>\d+) v=(?<vx>-?\d+),(?<vy>-?\d+)").unwrap();
	let robots = re.captures_iter(input.as_str()).map(|caps| {
		let px = caps.name("px").unwrap().as_str().parse::<i32>().unwrap();
		let py = caps.name("py").unwrap().as_str().parse::<i32>().unwrap();
		let vx = caps.name("vx").unwrap().as_str().parse::<i32>().unwrap();
		let vy = caps.name("vy").unwrap().as_str().parse::<i32>().unwrap();
		((px, py), (vx, vy))
	}).collect_vec();

	return robots;
}

fn step(robots: &mut Vec<((i32, i32), (i32, i32))>) {
	for i in 0..robots.len() {
		let (mut pos, vel) = robots[i];
		pos.0 += vel.0;
		pos.1 += vel.1;
		while pos.0 < 0 {
			pos.0 += WIDTH;
		}
		while pos.0 >= WIDTH {
			pos.0 -= WIDTH;
		}
		while pos.1 < 0 {
			pos.1 += HEIGHT;
		}
		while pos.1 >= HEIGHT {
			pos.1 -= HEIGHT;
		}
		robots[i].0 = pos;
	}
}

fn score(robots: Vec<((i32, i32), (i32, i32))>) -> u64 {
	let mut upperleft = 0;
	let mut upperright = 0;
	let mut lowerleft = 0;
	let mut lowerright = 0;

	for robot in robots {
		if robot.0.0 < WIDTH / 2 {
			if robot.0.1 < HEIGHT / 2 {
				upperleft += 1;
			}
			else if robot.0.1 > HEIGHT / 2 {
				lowerleft += 1;
			}
		} else if robot.0.0 > WIDTH / 2 {
			if robot.0.1 < HEIGHT / 2 {
				upperright += 1;
			}
			else if robot.0.1 > HEIGHT / 2 {
				lowerright += 1;
			}
		}
	}

	return upperleft * upperright * lowerleft * lowerright;
}

fn print(robots: Vec<((i32, i32), (i32, i32))>) {
	for y in 0..HEIGHT {
		for x in 0..WIDTH {
			let mut num = 0;
			for robot in robots.clone() {
				if robot.0.0 == x && robot.0.1 == y {
					num += 1;
				}
			}
			if num == 0 {
				print!(" ");
			} else {
				print!("{}", num);
			}
		}
		println!();
	}
}

pub fn part1(input: String) -> u64 {
	let mut robots = parse(input);

	for _ in 0..100 {
		step(&mut robots);
	}

	return score(robots);
}

pub fn part2(input: String) -> i32 {
	let mut robots = parse(input);

	let mut i = 0;
	for _ in 0..(WIDTH * HEIGHT) {
		step(&mut robots);
		i += 1;
		let mut buffer = [0u8;(WIDTH * HEIGHT) as usize];
		let mut pix = 0;
		for y in 0..HEIGHT {
			for x in 0..WIDTH {
				for robot in robots.clone() {
					if robot.0.0 == x && robot.0.1 == y {
						buffer[pix] = 255;
						break;
					}
				}
				pix += 1;
			}
		}
		let filename = format!("step{}.png", i);
		save_buffer(filename, &buffer, WIDTH as u32, HEIGHT as u32, ExtendedColorType::L8).unwrap();
	}

	let mut best = 0;
	let mut lowest_size = 999999999;
	for i in 1..(WIDTH * HEIGHT) {
		let filename = format!("step{}.png", i);
		let fsize = fs::metadata(filename).unwrap().st_size();
		if fsize < lowest_size {
			lowest_size = fsize;
			best = i;
		}
	}

	return best;
}