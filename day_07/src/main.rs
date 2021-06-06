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

    println!("D7P1: Root program is {}", root_program);

    let children_weights = find_program_with_unbalanced_children(&programs).expect("Could not find unbalanced node");

    println!("D7P2: Unbalanced children are: {:?}", children_weights);
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

fn parse_input(puzzle_input: &str) -> HashMap<String, Program> {
    let programs = puzzle_input
        .trim()
        .split("\n")
        .map(|input| Program::from(input))
        .collect::<Vec<Program>>();

    let mut program_map: HashMap<String, Program> = HashMap::new();

    for program in programs {
        program_map.insert(program.name.clone(), program);
    }

    program_map
}

fn find_program_with_unbalanced_children(programs: &HashMap<String, Program>) -> Option<Vec<u16>> {
    // Get programs with children only:
    for program in programs.values().cloned().filter(|p| p.children.is_some()) {
        let children = program.children.clone().unwrap();

        let children_weights: Vec<u16> = children
            .iter()
            .map(|c| programs.get(c).expect("Child program not found"))
            .map(|p| p.weight(&programs))
            .collect();

        let first_weight = children_weights[0];
        let all_weights_same = children_weights.iter().all(|&w| w == first_weight);

        if !all_weights_same {
            return Some(children_weights);
        }
    }

    None
}

#[derive(Clone, Debug)]
struct Program {
    name: String,
    weight: u16,
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

        let weight: u16 = match captures.get(2) {
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

impl Program {
    fn weight(&self, programs: &HashMap<String, Program>) -> u16 {
        let children_weights: u16 = match self.children.clone() {
            None => 0,
            Some(children) => children
                .iter()
                .map(|c| programs.get(c).expect("Child not present in programs!"))
                .map(|p| p.weight)
                .sum(),
        };

        self.weight + children_weights
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

        let ugml = programs.get("ugml").expect("Could not get program");
        assert_eq!(ugml.weight(&programs), 251);

        let program = programs.get("padx").expect("Could not get program");
        assert_eq!(program.weight(&programs), 243);

        let program = programs.get("fwft").expect("Could not get program");
        assert_eq!(program.weight(&programs), 243);
    }

    #[test]
    fn finds_program_with_unbalanced_children() {
        let programs = parse_input(TEST_INPUT);
        let children_weights: Vec<u16> =
            find_program_with_unbalanced_children(&programs).expect("Found no unbalanced program");

        assert_eq!(children_weights, vec![1,2,3]);
    }
}
