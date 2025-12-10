use std::collections::HashMap;

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

fn dfs(state: &mut u32, machine: &Machine, path_len: usize, cache: &mut HashMap<(u32, usize), i64>) -> i64 {
    if *state == machine.goal {
        return 0;
    }
    if path_len == machine.buttons.len() {
        return -1;
    }

    let key = (*state, path_len);
    if let Some(c) = cache.get(&key) {
        return *c;
    }

    let mut result = i64::MAX - 1;
    for i in path_len..machine.buttons.len() {
        for &k in &machine.buttons[i] {
            *state ^= 1u32 << k;
        }
        let r = 1 + dfs(state, machine, i + 1, cache);
        if r > 0 {
            result = result.min(r);
        }
        //Restore.
        for &k in &machine.buttons[i] {
            *state ^= 1u32 << k;
        }
    }

    cache.insert(key, result);
    result
}

pub fn part1(input: String) -> i64 {
    let machines = parse(input);

    let mut total = 0;
    let mut state = 0;
    for machine in machines {
        total += dfs(&mut state, &machine, 0, &mut HashMap::new());
    }
    total
}