fn parse(input: String) -> (Vec<(u64, u64)>, Vec<u64>) {
	let mut halves = input.split("\n\n");
	let first_half = halves.next().unwrap();
	let second_half = halves.next().unwrap();

	let mut ranges = vec!();
	for line in first_half.split("\n") {
		let mut ends = line.split("-");
		let start = ends.next().unwrap().parse::<u64>().unwrap();
		let end = ends.next().unwrap().parse::<u64>().unwrap();
		ranges.push((start, end));
	}

	let mut items = vec!();
	for line in second_half.split("\n") {
		items.push(line.trim().parse::<u64>().unwrap());
	}

	(ranges, items)
}

pub fn part1(input: String) -> u64 {
    let (ranges, items) = parse(input);
	let mut fresh = 0;

	for item in items {
		for range in &ranges {
			if item >= range.0 && item <= range.1 {
				fresh += 1;
				break;
			}
		}
	}

    fresh
}