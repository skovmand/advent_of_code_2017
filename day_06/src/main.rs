use std::collections::HashMap;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_06.txt");

type MemoryBank = u32;

fn main() {
    let memory_banks: Vec<MemoryBank> = parse_input(PUZZLE_INPUT);
    let (d6p1_answer, d6p2_answer) = find_identical_redistribution_cycle(memory_banks);

    println!("D6P1: Identical cycle is {}", d6p1_answer);
    println!("D6P2: Loop size is {}", d6p2_answer);
}

fn parse_input(input: &str) -> Vec<MemoryBank> {
    input.split_whitespace().map(|s| s.parse().unwrap()).collect()
}

// Returns the cycle count where a value repeats for the first time as well as the loop size
fn find_identical_redistribution_cycle(mut memory_banks: Vec<MemoryBank>) -> (u32, u32) {
    let mut redistribution_cycle_count: u32 = 0;
    let mut memory_bank_map: HashMap<Vec<u32>, u32> = HashMap::new();

    loop {
        redistribution_cycle_count += 1;
        let max_index = max_memory_bank_index(&memory_banks);
        memory_banks = redistribute(memory_banks.clone(), max_index);

        match memory_bank_map.get(&memory_banks) {
            Some(_) => break,
            None => memory_bank_map.insert(memory_banks.clone(), redistribution_cycle_count),
        };
    }

    let first_occurence = memory_bank_map.get(&memory_banks).unwrap().to_owned();

    (redistribution_cycle_count, redistribution_cycle_count - first_occurence)
}

fn max_memory_bank_index(memory_banks: &[MemoryBank]) -> usize {
    let max_element: &u32 = memory_banks.iter().max().unwrap();

    memory_banks.iter().position(|elem| elem == max_element).unwrap()
}

fn redistribute(mut memory_banks: Vec<MemoryBank>, mut mem_index: usize) -> Vec<MemoryBank> {
    let length = memory_banks.len();
    let mut pool = memory_banks[mem_index];
    memory_banks[mem_index] = 0;

    while pool > 0 {
        mem_index = (mem_index + 1) % length;
        memory_banks[mem_index] += 1;
        pool -= 1;
    }

    memory_banks
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_memory_bank_index() {
        let memory_banks: Vec<MemoryBank> = vec![0, 2, 7, 0];
        assert_eq!(max_memory_bank_index(&memory_banks), 2);
    }

    #[test]
    fn test_max_memory_bank_index_returns_first_match() {
        let memory_banks: Vec<MemoryBank> = vec![0, 2, 1, 2];
        assert_eq!(max_memory_bank_index(&memory_banks), 1);
    }

    #[test]
    fn test_redistribute() {
        let memory_banks: Vec<MemoryBank> = vec![0, 2, 7, 0];
        assert_eq!(redistribute(memory_banks, 2), vec![2, 4, 1, 2]);
    }

    #[test]
    fn test_d6p1_and_2_example() {
        let memory_banks: Vec<MemoryBank> = vec![0, 2, 7, 0];
        assert_eq!(find_identical_redistribution_cycle(memory_banks), (5, 4));
    }

    #[test]
    fn solves_d6() {
        let memory_banks: Vec<MemoryBank> = parse_input(PUZZLE_INPUT);
        let (d6p1_answer, d6p2_answer) = find_identical_redistribution_cycle(memory_banks);

        assert_eq!(d6p1_answer, 7864);
        assert_eq!(d6p2_answer, 1695);
    }
}
