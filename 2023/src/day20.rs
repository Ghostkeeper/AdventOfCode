use std::collections::{HashMap, VecDeque};
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Node {
	module: char,
	output: Vec<String>,
	flipflop_on: bool,
	conjunction_store: HashMap<String, bool>,
}

fn parse(input: String) -> HashMap<String, Node> {
	let mut nodes: HashMap<String, Node> = HashMap::new();

	for line in input.split("\n") {
		let mut name_and_output = line.split(" -> ");
		let name = name_and_output.next().unwrap().to_string();
		let output_str = name_and_output.next().unwrap();
		let output = output_str
			.split(", ")
			.map(|n| n.to_string())
			.collect_vec();
		if name == "broadcaster" {
			nodes.insert(name, Node {
				module: 'b',
				output,
				flipflop_on: false,
				conjunction_store: HashMap::new(),
			});
		} else {
			let module = name.chars().next().unwrap();
			let stripped_name = name[1..].to_string();
			nodes.insert(stripped_name, Node {
				module,
				output,
				flipflop_on: false,
				conjunction_store: HashMap::new(),
			});
		}
	}
	nodes.insert("output".to_string(), Node {
		module: 'b',
		output: vec!(),
		flipflop_on: false,
		conjunction_store: HashMap::new(),
	});

	//Fill the conjunction stores.
	let names = nodes.keys().map(|n| n.clone()).collect_vec();
	for name in names {
		for target in nodes.get(&name).unwrap().output.clone() {
			match nodes.get_mut(&target) {
				Some(somenode) => { somenode.conjunction_store.insert(name.clone(), false); },
				None => println!("Can't find node {}", target),
			}
		}
	}

	return nodes;
}

struct Output {
	lows: u32,
	highs: u32,
}

fn do_pulse(nodes: &mut HashMap<String, Node>, queue: &mut VecDeque<(String, bool, String)>, node_name: &String, is_high: bool, source: String, output: &mut Output) {
	if is_high {
		output.highs += 1;
	} else {
		output.lows += 1;
	}
	if !nodes.contains_key(node_name) {
		return; //Don't do any pulse I guess.
	}
	let module = nodes.get(node_name).unwrap().module;
	match module {
		'b' => { //Broadcaster.
			for target in nodes.get(node_name).unwrap().output.clone().iter() {
				queue.push_back((node_name.clone(), is_high, target.clone()));
			}
		},
		'%' => {
			if !is_high {
				let original_flip = nodes.get(node_name).unwrap().flipflop_on;
				nodes.get_mut(node_name).unwrap().flipflop_on = !original_flip;
				for target in nodes.get(node_name).unwrap().output.clone().iter() {
					queue.push_back((node_name.clone(), !original_flip, target.clone()));
				}
			}
		},
		'&' => {
			nodes.get_mut(node_name).unwrap().conjunction_store.insert(source.clone(), is_high);
			let send_low = nodes.get(node_name).unwrap().conjunction_store.values().all(|b| *b);
			for target in nodes.get(node_name).unwrap().output.clone().iter() {
				queue.push_back((node_name.clone(), !send_low, target.clone()));
			}
		}
		_ => panic!("Unknown node module type."),
	}
}

pub fn part1(input: String) -> u32 {
	let mut network = parse(input);
	let mut output = Output {
		lows: 0,
		highs: 0,
	};
	let mut queue: VecDeque<(String, bool, String)> = VecDeque::new();
	for _ in 0..1000 {
		queue.push_back(("button".to_string(), false, "broadcaster".to_string()));
		while !queue.is_empty() {
			let (source, is_high, target) = queue.pop_front().unwrap();
			do_pulse(&mut network, &mut queue, &target, is_high, source, &mut output);
		}
	}
	return output.highs * output.lows;
}