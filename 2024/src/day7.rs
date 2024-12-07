use itertools::Itertools;

fn parse(input: String) -> Vec<(i64, Vec<i64>)> {
	let mut result = vec!();

	for line in input.split("\n") {
		let mut parts = line.split(": ");
		let answer = parts.next().unwrap().parse::<i64>().unwrap();
		let numbers = parts.next().unwrap().split(" ").map(|s| s.parse::<i64>().unwrap()).collect_vec();
		result.push((answer, numbers));
	}

	return result;
}

fn is_equatable(numbers: &Vec<i64>, computation: i64, position: usize, answer: i64) -> bool {
	let computation_mul = computation * numbers[position];
	let computation_add = computation + numbers[position];
	if position == numbers.len() - 1 {
		if computation_mul == answer {
			return true;
		} else if computation_add == answer {
			return true;
		} else {
			return false;
		}
	} else {
		return is_equatable(numbers, computation_mul, position + 1, answer) || is_equatable(numbers, computation_add, position + 1, answer);
	}
}

pub fn part1(input: String) -> i64 {
	let mut result = 0;
	let rules = parse(input);
	for (answer, numbers) in rules {
		let computation = numbers[0];
		if is_equatable(&numbers, computation, 1, answer) {
			result += answer;
		}
	}

	return result;
}