#[macro_use]
extern crate lazy_static;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_07.txt");

use regex::Regex;
use std::collections::HashSet;

fn main() {
    let programs: Vec<Program> = parse_input(PUZZLE_INPUT);
    let root_program = match find_root_program(programs) {
        Ok(program) => program,
        Err(reason) => panic!("Failed to find root program, {}", reason),
    };

    println!("D7P1: Root program is {}", root_program);
}

fn find_root_program(programs: Vec<Program>) -> Result<String, String> {
    let program_names: HashSet<String> = programs.iter().map(|p| p.name.clone()).collect();
    let mut children_names: HashSet<String> = HashSet::new();

    for program in programs {
        if let Some(children) = program.children {
            for child in children {
                children_names.insert(child);
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

fn parse_input(puzzle_input: &str) -> Vec<Program> {
    puzzle_input
        .trim()
        .split("\n")
        .map(|input| Program::from(input))
        .collect::<Vec<Program>>()
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_root_program() {
        let input = "pbga (66)\n\
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
                     cntj (57)\n";

        let parsed_input = parse_input(input);
        let root_program = find_root_program(parsed_input);
        assert!(root_program.is_ok());
        assert_eq!(root_program.unwrap(), String::from("tknk"));
    }
}
