use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, Debug)]
struct Node {
    module: char,
    output: Vec<String>,
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
            });
        } else {
            let module = name.chars().next().unwrap();
            let stripped_name = name[1..].to_string();
            nodes.insert(stripped_name, Node {
                module,
                output,
            });
        }
    }

    return nodes;
}

pub fn part1(input: String) -> u32 {
    let network = parse(input);
    println!("{:?}", network.values());
    return 0;
}