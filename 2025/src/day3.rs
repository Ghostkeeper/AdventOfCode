fn parse(input: String) -> Vec<Vec<u8>> {
	let mut result = vec!();
	for line in input.split("\n") {
		result.push(line.chars().map(|c| c as u8 - '0' as u8).collect());
	}
	result
}

pub fn part1(input: String) -> u32 {
	let mut result = 0;
	let banks = parse(input);
	for bank in banks {
		let bank_minus_last = &bank[0..bank.len() - 1];
		let tens = bank_minus_last.iter().max().unwrap();
		let index_of_tens = bank_minus_last.iter().position(|x| x == tens).unwrap();
		let bank_from_tens = &bank[(index_of_tens + 1)..];
		let ones = bank_from_tens.iter().max().unwrap();
		result += *tens as u32 * 10 + *ones as u32;
	}
	result
}

pub fn part2(input: String) -> u64 {
	let mut result = 0;
	let banks = parse(input);
	for bank in banks {
		let mut current_bank = &bank[..];
		let mut joltage = 0;
		for i in 0..12 {
			let bank_minus_end = &current_bank[0..current_bank.len() - (11 - i)];
			let digit = bank_minus_end.iter().max().unwrap();
			let index_of_digit = bank_minus_end.iter().position(|x| x == digit).unwrap();
			current_bank = &current_bank[(index_of_digit + 1)..];
			joltage += *digit as u64 * 10u64.pow(11 - i as u32);
		}
		result += joltage;
	}
	result
}