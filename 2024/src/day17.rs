use itertools::Itertools;
use regex::Regex;

fn parse(input: String) -> (i64, i64, i64, Vec<u8>) {
	let re = Regex::new(r"Register A: (\d+)\nRegister B: (\d+)\nRegister C: (\d+)\n\nProgram: ((:?\d,)+\d)").unwrap();
	let caps = re.captures(input.as_str()).unwrap();
	let a = caps[1].parse::<i64>().unwrap();
	let b = caps[2].parse::<i64>().unwrap();
	let c = caps[3].parse::<i64>().unwrap();
	let program = &caps[4];
	let program_vec = program.split(",").map(|p| p.parse::<u8>().unwrap()).collect_vec();
	(a, b, c, program_vec)
}

fn combo(val: u8, a: i64, b: i64, c: i64) -> i64 {
	if val <= 3 {
		return val as i64;
	} else if val == 4 {
		return a;
	} else if val == 5 {
		return b;
	} else if val == 6 {
		return c;
	} else {
		panic!("Invalid combo operand {}", val);
	}
}

fn execute(program: Vec<u8>, mut a: i64, mut b: i64, mut c: i64) -> Vec<u8> {
	let mut instruction = 0;
	let mut outputs = vec!();
	while instruction < program.len() {
		let operand = program[instruction + 1];
		match program[instruction] {
			0 => a = a / 2_i64.pow(combo(operand, a, b, c) as u32),
			1 => b = b ^ (operand as i64),
			2 => b = combo(operand, a, b, c) % 8,
			3 => {
				if a != 0 {
					instruction = operand as usize;
					continue;
				}
			},
			4 => b ^= c,
			5 => outputs.push((combo(operand, a, b, c) % 8) as u8),
			6 => b = a / 2_i64.pow(combo(operand, a, b, c) as u32),
			7 => c = a / 2_i64.pow(combo(operand, a, b, c) as u32),
			_ => panic!("Unknown instruction"),
		}
		instruction += 2;
	}
	return outputs;
}

pub fn part1(input: String) -> String {
	let (a, b, c, program) = parse(input);
	let outputs = execute(program, a, b, c);
	return outputs.iter().map(|o| format!("{}", o)).join(",");
}