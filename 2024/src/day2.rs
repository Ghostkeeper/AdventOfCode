fn parse(input: String) -> Vec<Vec<i32>> {
	let mut result = vec!();
	for line in input.split("\n") {
		let mut report = vec!();
		for level in line.split(" ") {
			report.push(level.parse::<i32>().unwrap());
		}
		result.push(report);
	}
	return result;
}

fn derivative(report: Vec<i32>) -> Vec<i32> {
	let mut result = vec!();
	let mut prev = report[0];
	for next in &report[1..] {
		result.push(next - prev);
		prev = *next;
	}
	return result;
}

pub fn part1(input: String) -> u32 {
	let reports = parse(input);
	let mut num_safe = 0;
	for report in reports {
		let report_derivative = derivative(report);
		if report_derivative[0] > 0 {
			if report_derivative.into_iter().all(|x| x > 0 && x >= 1 && x <= 3) {
				num_safe += 1;
			}
		} else if report_derivative[0] < 0 {
			if report_derivative.into_iter().all(|x| x < 0 && x <= -1 && x >= -3) {
				num_safe += 1;
			}
		}
	}
	return num_safe;
}

pub fn part2(input: String) -> u32 {
	let reports = parse(input);
	let mut num_safe = 0;
	for report in reports {
		for i in 0..report.len() {
			let mut dampened = report.clone();
			dampened.remove(i);
			let report_derivative = derivative(dampened);
			if report_derivative[0] > 0 {
				if report_derivative.into_iter().all(|x| x > 0 && x >= 1 && x <= 3) {
					num_safe += 1;
					break;
				}
			} else if report_derivative[0] < 0 {
				if report_derivative.into_iter().all(|x| x < 0 && x <= -1 && x >= -3) {
					num_safe += 1;
					break;
				}
			}
		}
	}
	return num_safe;
}