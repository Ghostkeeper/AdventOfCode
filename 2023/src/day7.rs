use std::cmp::Ordering;
use itertools::Itertools;

enum Combination {
    HighCard = 0,
    OnePair = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}
static ORDER: [char; 13] = ['2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'];

fn parse(input: String) -> Vec<([char; 5], usize)>{
    let mut result = vec!();
    for line in input.split("\n") {
        let mut parts = line.split_whitespace();
        let hand = parts.next().unwrap();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        result.push((hand.chars().collect_tuple::<(char, char, char, char, char)>().unwrap().into(), bid));
    }
    return result;
}

fn get_combination(hand: [char; 5]) -> Combination {
    let mut histogram = [0; 13];
    for card in hand {
        histogram[ORDER.iter().position(|c| *c == card).unwrap()] += 1;
    }
    histogram.sort();
    if histogram[12] == 5 {
        return Combination::FiveOfAKind;
    } else if histogram[12] == 4 {
        return Combination::FourOfAKind;
    } else if histogram[12] == 3 && histogram[11] == 2 {
        return Combination::FullHouse;
    } else if histogram[12] == 3 && histogram[11] == 1 {
        return Combination::ThreeOfAKind;
    } else if histogram[12] == 2 && histogram[11] == 2 {
        return Combination::TwoPair;
    } else if histogram[12] == 2 && histogram[11] == 1 {
        return Combination::OnePair;
    } else {
        return Combination::HighCard;
    }
}

fn comparison(left_hand: &([char; 5], usize), right_hand: &([char; 5], usize)) -> Ordering {
    let left_combi = get_combination(left_hand.0) as i32;
    let right_combi = get_combination(right_hand.0) as i32;
    if left_combi < right_combi {
        return Ordering::Less;
    }
    if left_combi > right_combi {
        return Ordering::Greater;
    }
    let left_ids = left_hand.0.map(|c| ORDER.iter().position(|c2| *c2 == c).unwrap());
    let right_ids = right_hand.0.map(|c| ORDER.iter().position(|c2| *c2 == c).unwrap());
    return left_ids.cmp(&right_ids);
}

pub fn part1(input: String) {
    let mut hands = parse(input);
    hands.sort_by(comparison);
    let mut result = 0;
    for (rank, hand) in hands.iter().enumerate() {
        result += (rank + 1) * hand.1;
    }
    println!("{}", result);
}