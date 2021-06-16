#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate anyhow;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_08.txt");

use anyhow::Context;
use regex::Regex;
use std::{collections::HashMap, convert::TryFrom};

fn main() -> anyhow::Result<()> {
    let (max_register_value, max_value) = calculate(PUZZLE_INPUT)?;

    println!("D8P1 result is {}", max_register_value);
    println!("D8P2 result is {}", max_value);

    Ok(())
}

fn calculate(input: &str) -> anyhow::Result<(i32, i32)> {
    let instructions = parse_input(input).context("Could not parse input")?;
    let (registers, max_value) = apply_instructions(instructions);
    let max_register_value = maximum_value(&registers).ok_or_else(|| anyhow!("No max value found"))?;

    Ok((max_register_value.to_owned(), max_value))
}

type Instructions = Vec<Instruction>;

fn parse_input(input: &str) -> anyhow::Result<Instructions> {
    input
        .lines()
        .map(Instruction::try_from)
        .collect::<Result<Vec<Instruction>, _>>()
}

fn apply_instructions(instructions: Instructions) -> (HashMap<String, i32>, i32) {
    let mut registers: HashMap<String, i32> = HashMap::new();
    let mut highest_value = 0;

    for i in instructions {
        if condition_true(&i.condition, &registers) {
            let result_value = apply_instruction(&i, &mut registers);

            if result_value > highest_value {
                highest_value = result_value
            };
        }
    }

    (registers, highest_value)
}

fn condition_true(c: &Condition, registers: &HashMap<String, i32>) -> bool {
    let value = registers.get(&c.register).or(Some(&0)).unwrap().to_owned();

    match c.operator {
        Operator::EqualTo => value == c.value,
        Operator::GreaterThan => value > c.value,
        Operator::GreaterThanOrEqualTo => value >= c.value,
        Operator::LessThan => value < c.value,
        Operator::LessThanOrEqualTo => value <= c.value,
        Operator::NotEqualTo => value != c.value,
    }
}

fn apply_instruction(i: &Instruction, registers: &mut HashMap<String, i32>) -> i32 {
    let value = registers.get(&i.target).or(Some(&0)).unwrap().to_owned();

    let updated_value = match i.operation {
        Operation::Dec => value - i.value,
        Operation::Inc => value + i.value,
    };

    registers.insert(i.target.clone(), updated_value);

    updated_value
}

fn maximum_value(registers: &HashMap<String, i32>) -> Option<&i32> {
    registers.values().max()
}

#[derive(Debug)]
enum Operation {
    Inc,
    Dec,
}

#[derive(Debug)]
struct Condition {
    register: String,
    operator: Operator,
    value: i32,
}

#[derive(Debug)]
struct Instruction {
    condition: Condition,
    operation: Operation,
    target: String,
    value: i32,
}

#[derive(Debug)]
enum Operator {
    EqualTo,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    NotEqualTo,
}

impl TryFrom<&str> for Instruction {
    type Error = anyhow::Error;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        lazy_static! {
            static ref SPLITTER: Regex =
                Regex::new(r"([a-z]+) (dec|inc) ([-0-9]+) if ([a-z]+) (>=|<=|<|>|==|!=) ([-0-9]+)").unwrap();
        }

        let captures = SPLITTER
            .captures(input)
            .ok_or_else(|| anyhow!("Failed to match regex groups in input"))?;

        // Note: We unwrap the outer Option (since we matched it above 👆)
        //       but use the Try operator on the inner
        let target = captures.get(1).map(|x| x.as_str()).unwrap().to_owned();
        let operation = captures.get(2).map(|x| x.as_str()).map(Operation::try_from).unwrap()?;
        let i_value = captures.get(3).map(|x| x.as_str().parse::<i32>()).unwrap()?;

        // Condition fields
        let register = captures.get(4).map(|x| x.as_str()).unwrap().to_owned();
        let operator = captures.get(5).map(|x| x.as_str()).map(Operator::try_from).unwrap()?;
        let value = captures.get(6).map(|x| x.as_str().parse::<i32>()).unwrap()?;

        Ok(Instruction {
            target,
            operation,
            value: i_value,
            condition: Condition {
                register,
                operator,
                value,
            },
        })
    }
}

impl TryFrom<&str> for Operation {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "dec" => Ok(Operation::Dec),
            "inc" => Ok(Operation::Inc),
            _ => Err(anyhow!("Invalid operation, must be inc or dec")),
        }
    }
}

impl TryFrom<&str> for Operator {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            ">" => Ok(Operator::GreaterThan),
            ">=" => Ok(Operator::GreaterThanOrEqualTo),
            "<" => Ok(Operator::LessThan),
            "<=" => Ok(Operator::LessThanOrEqualTo),
            "==" => Ok(Operator::EqualTo),
            "!=" => Ok(Operator::NotEqualTo),
            _ => Err(anyhow!("Unknown operator in condition")),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn unit_test() {
        let input = indoc! {"
            b inc 5 if a > 1
            a inc 1 if b < 5
            c dec -10 if a >= 1
            c inc -20 if c == 10
        "};

        let (max_value, max_register_value) = calculate(input).unwrap();

        // Part 1
        assert_eq!(max_value, 1);

        // Part 2
        assert_eq!(max_register_value, 10);
    }

    #[test]
    fn solves_d8() {
        let (max_value, max_register_value) = calculate(PUZZLE_INPUT).unwrap();
        assert_eq!(max_value, 7296);
        assert_eq!(max_register_value, 8186);
    }
}
