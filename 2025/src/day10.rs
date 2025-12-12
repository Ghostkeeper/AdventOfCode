use rayon::prelude::*;
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

fn dfs2(target_joltages: &Vec<u32>, buttons_allowed: u32, buttons: &Vec<Vec<usize>>) -> u32 {
    if target_joltages.iter().all(|j| *j == 0) {
        return 0;
    }

    let (lowfreq_button, &lowfreq) = target_joltages.iter().enumerate().filter(|&(_, &joltage)| joltage > 0).min_by_key(|&(i, &joltage)| {
        (buttons.iter().enumerate().filter(|&(j, b)| {
            buttons_allowed & (1 << j) != 0 && b.contains(&i)
        }).count(), -(joltage as i32))
    }).unwrap();

    let matching_buttons = buttons.iter().enumerate().filter(|&(i, button)| buttons_allowed & (1 << i) != 0 && button.contains(&lowfreq_button)).collect::<Vec<_>>();
    let mut result = u32::MAX;

    if !matching_buttons.is_empty() {
        let mut new_buttons_allowed = buttons_allowed;
        for (i, _) in &matching_buttons {
            new_buttons_allowed &= !(1 << i);
        }

        //Try all allowed buttons.
        let mut new_joltage = target_joltages.clone();
        let mut counts = vec![0; matching_buttons.len()];
        counts[matching_buttons.len() - 1] = lowfreq;
        loop {
            let mut has_button = true;
            new_joltage.copy_from_slice(target_joltages);
            'buttons: for (button_index, &count) in counts.iter().enumerate() {
                if count == 0 {
                    continue;
                }
                for &joltage_index in matching_buttons[button_index].1 {
                    if new_joltage[joltage_index] >= count {
                        new_joltage[joltage_index] -= count;
                    } else {
                        has_button = false;
                        break 'buttons;
                    }
                }
            }
            if has_button {
                let try_result = dfs2(&new_joltage, new_buttons_allowed, buttons);
                if try_result != u32::MAX {
                    result = result.min(lowfreq + try_result);
                }
            }

            let button_index = counts.iter().rposition(|&count| count != 0).unwrap();
            if button_index == 0 {
                break; //No more buttons to try.
            }
            let count = counts[button_index];
            counts[button_index - 1] += 1;
            counts[button_index] = 0;
            counts[matching_buttons.len() - 1] = count - 1;
        }
    }

    result
}

pub fn part2(input: String) -> u32 {
    let machines = parse(input);
    machines.into_par_iter().map(|mut machine| {
        //Sort the buttons by how commonly they affect each joltage.
        let mut joltage_affected_count = vec![0; machine.joltages.len()];
        for button in &machine.buttons {
            for button_effect in button {
                joltage_affected_count[*button_effect] += 1;
            }
        }
        let mut button_damage = vec!();
        let mut sort_indices = vec!();
        for button in &machine.buttons {
            let mut damage = 0;
            for button_effect in button {
                damage += joltage_affected_count[*button_effect];
            }
            button_damage.push(damage);
            sort_indices.push(sort_indices.len());
        }
        sort_indices.sort_unstable_by_key(|i| button_damage[*i]);
        machine.buttons = sort_indices.iter().map(|&i| machine.buttons[i].clone()).collect();
        let result = dfs2(&machine.joltages, (1 << machine.buttons.len()) - 1, &machine.buttons);
        result
    }).sum::<u32>()
}