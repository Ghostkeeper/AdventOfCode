use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;

fn parse_seeds(input: String) -> Vec<i64> {
    let mut result = vec!();
    for line in input.split("\n") {
        if !line.starts_with("seeds: ") {
            continue;
        }
        for seed_id in line[7..].split_whitespace() {
            result.push(seed_id.parse::<i64>().unwrap());
        }
    }
    return result;
}

fn parse_seed_ranges(input: String) -> Vec<Range<i64>> {
    let mut result = vec!();
    for line in input.split("\n") {
        if !line.starts_with("seeds: ") {
            continue;
        }
        for (start, length) in line[7..].split_whitespace().tuples() {
            let start_int = start.parse::<i64>().unwrap();
            let length_int = length.parse::<i64>().unwrap();
            result.push(start_int..(start_int + length_int));
        }
    }
    return result;
}

fn parse_map(input: String, name: &str) -> HashMap<Range<i64>, i64> {
    let mut result = HashMap::new();
    for part in input.split("\n\n") {
        if !part.starts_with(name) {
            continue;
        }
        for line in part.split("\n") {
            if line.starts_with(name) {
                continue; //Skip the header.
            }
            let mut parts = line.split_whitespace();
            let dest_range = parts.next().unwrap().parse::<i64>().unwrap();
            let source_range = parts.next().unwrap().parse::<i64>().unwrap();
            let range_length = parts.next().unwrap().parse::<i64>().unwrap();
            result.insert(source_range..(source_range + range_length), dest_range - source_range);
        }
    }
    return result;
}

pub fn part1(input: String) {
    let seeds = parse_seeds(input.clone());
    let maps = [
        parse_map(input.clone(), "seed-to-soil"),
        parse_map(input.clone(), "soil-to-fertilizer"),
        parse_map(input.clone(), "fertilizer-to-water"),
        parse_map(input.clone(), "water-to-light"),
        parse_map(input.clone(), "light-to-temperature"),
        parse_map(input.clone(), "temperature-to-humidity"),
        parse_map(input.clone(), "humidity-to-location"),
    ];

    let mut lowest = i64::MAX;
    for seed in seeds {
        let mut current = seed;
        for map in maps.clone() {
            for range in map.keys() {
                if range.contains(&current) {
                    current += map[range];
                    break;
                }
            }
        }
        lowest = lowest.min(current);
    }
    println!("Lowest location ID: {}", lowest);
}

pub fn part2(input: String) {
    let seeds = parse_seed_ranges(input.clone());
    let maps = [
        parse_map(input.clone(), "seed-to-soil"),
        parse_map(input.clone(), "soil-to-fertilizer"),
        parse_map(input.clone(), "fertilizer-to-water"),
        parse_map(input.clone(), "water-to-light"),
        parse_map(input.clone(), "light-to-temperature"),
        parse_map(input.clone(), "temperature-to-humidity"),
        parse_map(input.clone(), "humidity-to-location"),
    ];

    let mut lowest = i64::MAX;
    for seed_range in seeds {
        println!("Starting on seed range: {} -> {}", seed_range.start, seed_range.end);
        for seed in seed_range {
            let mut current = seed;
            for map in maps.clone() {
                for range in map.keys() {
                    if range.contains(&current) {
                        current += map[range];
                        break;
                    }
                }
            }
            lowest = lowest.min(current);
        }
    }
    println!("Lowest location ID: {}", lowest);
}