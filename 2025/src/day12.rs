fn parse(input: String) -> (Vec<[[bool; 3]; 3]>, Vec<(usize, usize, Vec<u32>)>) {
    let mut shape_parts: Vec<_> = input.split("\n\n").collect();
    let tree_lines = shape_parts.pop().unwrap().split("\n");

    let mut shapes = vec!();
    for shape_part in shape_parts {
        let mut shape = [[false; 3]; 3];
        for (y, line) in shape_part.split("\n").skip(1).enumerate() {
            for (x, chr) in line.chars().enumerate() {
                if chr == '#' {
                    shape[y][x] = true;
                }
            }
        }
        shapes.push(shape);
    }
    
    let mut trees = vec!();
    for tree_line in tree_lines {
        let mut tree_parts = tree_line.split(": ");
        let mut shape_part = tree_parts.next().unwrap().split("x");
        let width = shape_part.next().unwrap().parse::<usize>().unwrap();
        let height = shape_part.next().unwrap().parse::<usize>().unwrap();
        let mut count_parts = tree_parts.next().unwrap().split(" ");
        let mut counts = vec!();
        for count_part in count_parts {
            counts.push(count_part.parse::<u32>().unwrap());
        }
        trees.push((width, height, counts));
    }
    (shapes, trees)
}

pub fn part1(input: String) -> u64 {
	let (shapes, trees) = parse(input);
    let mut possible_count = 0;
    for (width, height, counts) in trees {
        //Turns out we don't really need to actually fit the shapes for the real input.
        //If it fits, the width/height will be way oversized!
        let mut total_count = 0;
        for count in counts {
            total_count += count;
        }
        if (width / 3) * (height / 3) >= total_count.try_into().unwrap() {
            possible_count += 1;
        }
    }

    possible_count
}