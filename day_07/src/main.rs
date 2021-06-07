#[macro_use]
extern crate lazy_static;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_07.txt");

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let programs: HashMap<String, Program> = parse_input(PUZZLE_INPUT);
    let root_program = match find_root_program(&programs) {
        Ok(program) => program,
        Err(reason) => panic!("Failed to find root program, {}", reason),
    };

    println!("D7P1: Root program is {}", &root_program);

    let correct_weight = find_correct_weight_at_leaf(root_program, &programs);
    println!("D7P2: Correct weight is {}", &correct_weight);
}

fn find_root_program(programs: &HashMap<String, Program>) -> Result<String, String> {
    let program_names: HashSet<String> = programs.values().map(|p| p.name.clone()).collect();
    let mut children_names: HashSet<String> = HashSet::new();

    for (_, program) in programs {
        if let Some(children) = &program.children {
            for child in children {
                children_names.insert(child.clone());
            }
        }
    }

    let difference: Vec<String> = program_names.difference(&children_names).cloned().collect();

    if difference.len() > 1 {
        Err(String::from("More than one root program found"))
    } else if difference.len() == 0 {
        Err(String::from("No root program found"))
    } else {
        Ok(difference[0].clone())
    }
}

fn find_correct_weight_at_leaf(root_node: String, programs: &HashMap<String, Program>) -> u32 {
    let mut odd_value: u32 = 0;
    let mut normal_value: u32 = 0;
    let mut next_node = root_node;

    while let Some(children) = programs.get(&next_node).unwrap().children.clone() {
        let names_with_weights: Vec<(String, u32)> = children
            .into_iter()
            .map(|c| (c.clone(), total_weight(&c, &programs)))
            .collect();
        if let Some((name, odd_weight, normal_weight)) = find_odd_child_name(names_with_weights) {
            odd_value = odd_weight;
            normal_value = normal_weight;
            next_node = name;
        } else {
            break;
        }
    }

    let difference: i32 = (odd_value - normal_value) as i32;
    let weight = programs.get(&next_node).unwrap().weight as i32;

    (weight - difference) as u32
}

fn parse_input(puzzle_input: &str) -> HashMap<String, Program> {
    let programs = puzzle_input
        .lines()
        .map(|input| Program::from(input))
        .collect::<Vec<Program>>();

    let mut program_map: HashMap<String, Program> = HashMap::new();

    for program in programs {
        program_map.insert(program.name.clone(), program);
    }

    program_map
}

// Calculate weight of node and all children. Not optimized.
fn total_weight(name: &str, programs: &HashMap<String, Program>) -> u32 {
    let program = programs.get(name).expect("Failed to find program");
    let weight = program.weight;

    match program.children.clone() {
        None => weight,
        Some(children) => {
            let children_weight: u32 = children.iter().map(|c| total_weight(c, &programs)).sum();
            weight + children_weight
        }
    }
}

fn find_odd_child_name(programs_with_weights: Vec<(String, u32)>) -> Option<(String, u32, u32)> {
    let mut frequencies: HashMap<u32, u32> = HashMap::new();

    // Count weight frequencies
    for (_, weight) in programs_with_weights.clone() {
        let counter = frequencies.entry(weight).or_insert(0);
        *counter += 1;
    }

    match frequencies.len() {
        1 => None,
        2 => {
            let (odd_weight, _) = frequencies
                .clone()
                .into_iter()
                .find(|(_, count)| *count == 1)
                .expect("Could not find one odd weight");

            let (normal_weight, _) = frequencies
                .into_iter()
                .find(|(_, count)| *count != 1)
                .expect("Could not find the normal weight");

            let (name, _) = programs_with_weights
                .into_iter()
                .find(|(_, prog_weight)| odd_weight == *prog_weight)
                .expect("Could not find program from weight");

            Some((name, odd_weight, normal_weight))
        }
        _ => panic!("Unexpected weights"),
    }
}

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: u32,
    children: Option<HashSet<String>>,
}

impl From<&str> for Program {
    fn from(input: &str) -> Program {
        lazy_static! {
            static ref SPLITTER: Regex = Regex::new(r"([a-z]+) \(([0-9]+)\)(?: -> )?(.+)?").unwrap();
        }

        let captures = SPLITTER.captures(input).unwrap();

        let name = match captures.get(1) {
            Some(m) => m.as_str().to_owned(),
            None => panic!("Name not matched in regex"),
        };

        let weight: u32 = match captures.get(2) {
            Some(m) => m.as_str().parse().expect("Could not convert weight to integer"),
            None => panic!("Weight not matched in regex"),
        };

        let children: Option<HashSet<String>> = match captures.get(3) {
            Some(m) => Some(m.as_str().split(", ").map(|x| x.to_owned()).collect()),
            None => None,
        };

        Program { name, weight, children }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "pbga (66)\n\
                             xhth (57)\n\
                             ebii (61)\n\
                             havc (66)\n\
                             ktlj (57)\n\
                             fwft (72) -> ktlj, cntj, xhth\n\
                             qoyq (66)\n\
                             padx (45) -> pbga, havc, qoyq\n\
                             tknk (41) -> ugml, padx, fwft\n\
                             jptl (61)\n\
                             ugml (68) -> gyxo, ebii, jptl\n\
                             gyxo (61)\n\
                             cntj (57)";

    #[test]
    fn finds_root_program() {
        let programs = parse_input(TEST_INPUT);
        let root_program = find_root_program(&programs);

        assert!(root_program.is_ok());
        assert_eq!(root_program.unwrap(), String::from("tknk"));
    }

    #[test]
    fn finds_program_weights() {
        let programs = parse_input(TEST_INPUT);
        assert_eq!(total_weight("ugml", &programs), 251);
        assert_eq!(total_weight("padx", &programs), 243);
        assert_eq!(total_weight("fwft", &programs), 243);
    }

    #[test]
    fn finds_normal_and_odd_program_weight() {
        let programs = parse_input(TEST_INPUT);

        // Get the three children
        let program: Program = programs.get("tknk").unwrap().to_owned();
        let children: HashSet<String> = program.children.unwrap();
        let children_with_weights: Vec<(String, u32)> = children
            .iter()
            .map(|c| (c.clone(), total_weight(c, &programs)))
            .collect();

        let odd_child: Option<(String, u32, u32)> = find_odd_child_name(children_with_weights);

        assert!(odd_child.is_some());
        assert_eq!(odd_child.unwrap(), (String::from("ugml"), 251, 243));
    }

    #[test]
    fn finds_correct_weight_at_leaf() {
        let programs = parse_input(TEST_INPUT);
        assert_eq!(find_correct_weight_at_leaf(String::from("tknk"), &programs), 60);
    }
}
