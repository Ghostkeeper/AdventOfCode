use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::cmp::Ordering;

struct Machine {
    goal: u32,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<u32>,
}

fn parse(input: String) -> Vec<Machine> {
    let mut machines = vec!();
    for line in input.split("\n") {
        let parts: Vec<&str> = line.split(" ").collect();
        let goal_part = parts[0];
        let button_parts = &parts[1..parts.len() - 1];
        let joltage_part = parts[parts.len() - 1];

        let goal_str = &goal_part[1..goal_part.len() - 1];
        let mut goal = 0;
        for i in 0..goal_str.len() {
            if goal_str.chars().nth(i).unwrap() == '#' {
                goal += 1 << i;
            }
        }

        let mut buttons = vec!();
        for button_part in button_parts {
            let button_str = &button_part[1..button_part.len() - 1];
            let mut button = vec!();
            for button_effect in button_str.split(",") {
                button.push(button_effect.parse::<usize>().unwrap());
            }
            buttons.push(button);
        }

        let joltage_str = &joltage_part[1..joltage_part.len() - 1];
        let mut joltages = vec!();
        for joltage in joltage_str.split(",") {
            joltages.push(joltage.parse::<u32>().unwrap());
        }

        machines.push(Machine{goal: goal, buttons: buttons, joltages: joltages});
    }
    machines
}

//Graph nodes for A*.
#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    state: u32,
    path_len: u32,
    heuristic_cost: u32,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.path_len + other.heuristic_cost).cmp(&(self.path_len + self.heuristic_cost))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(a: u32, b: u32) -> u32 {
    //Bit twiddling hack from https://graphics.stanford.edu/~seander/bithacks.html#CountBitsSetParallel
    let mut v = (a ^ b) as u64;
    v = v - ((v >> 1) & 0x55555555);
    v = (v & 0x33333333) + ((v >> 2) & 0x33333333);
    (((v + (v >> 4) & 0xF0F0F0F) * 0x1010101) >> 24) as u32
}

fn press_buttons(machine: Machine) -> u32 {
    //An N-dimensional A* algorithm to get to the goal state.
    let mut open_set = BinaryHeap::new();
    open_set.push(Node{state: 0, path_len: 0, heuristic_cost: distance(0, machine.goal)});
    let mut path_lens = HashMap::new();
    path_lens.insert(0, 0);

    while let Some(current) = open_set.pop() {
        if current.state == machine.goal {
            return current.path_len;
        }
        
        //Get neighbouring states.
        for button in &machine.buttons {
            let mut neighbour_state = current.state;
            for i in button {
                neighbour_state ^= 1 << i;
            }
            let neighbour_path_len = path_lens[&current.state] + 1;
            if neighbour_path_len < *path_lens.get(&neighbour_state).unwrap_or(&u32::MAX) {
                path_lens.insert(neighbour_state, neighbour_path_len);
                open_set.push(Node {
                    state: neighbour_state,
                    path_len: neighbour_path_len,
                    heuristic_cost: distance(neighbour_state, machine.goal),
                });
            }
        }
    }

    u32::MAX
}

pub fn part1(input: String) -> u32 {
    let machines = parse(input);

    let mut total = 0;
    for machine in machines {
        total += press_buttons(machine);
    }
    total
}