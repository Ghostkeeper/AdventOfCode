pub fn part1(input: String) -> i32 {
	let mut sum = 0;
	for sequence in input.split(",") {
		let mut hash = 0;
		for char in sequence.chars() {
			hash += char as i32;
			hash *= 17;
			hash %= 256;
		}
		println!("-- {}", hash);
		sum += hash;
	}
	return sum;
}