const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_05.txt");

type Program = Vec<i32>;
type OffsetCalcFn = dyn Fn(&i32) -> i32;

fn main() {
    let program: Program = parse_input(PUZZLE_INPUT);

    let step: u32 = execute_program(program.clone(), &add_one_to_offset);
    println!("D5P1: Program exited after step #{}", step);

    let step: u32 = execute_program(program, &add_or_subtract_offset);
    println!("D5P2: Program exited after step #{}", step);
}

fn parse_input(input: &str) -> Program {
    input.trim().split("\n").map(|x| x.parse().unwrap()).collect()
}

fn execute_program(mut program: Program, offset_fn: &OffsetCalcFn) -> u32 {
    let mut pointer: usize = 0;
    let mut step: u32 = 0;

    while pointer <= program.len() - 1 {
        let offset = program[pointer];
        program[pointer] = offset_fn(&offset);
        pointer = calc_next_pointer(&pointer, &offset).unwrap();
        step = step + 1;
    }

    step
}

// Calculate the next offset by adding 1 to the previous offset (D5P1)
fn add_one_to_offset(offset: &i32) -> i32 {
    offset + 1
}

// Calculate the next offset by subtracting 1 if the offset >= 3, otherwise add 1 (D5P2)
fn add_or_subtract_offset(offset: &i32) -> i32 {
    if *offset >= 3 {
        offset - 1
    } else {
        offset + 1
    }
}

// Calculate the next pointer value. Since a Vec is indexed by usize, we need
// to do some work to add an i32 to it, since things could get wild
fn calc_next_pointer(pointer: &usize, offset: &i32) -> Option<usize> {
    if offset.is_negative() {
        pointer.checked_sub(offset.wrapping_abs() as u32 as usize)
    } else {
        pointer.checked_add(*offset as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d5p1_example() {
        let program: Program = vec![0, 3, 0, 1, -3];
        assert_eq!(execute_program(program, &add_one_to_offset), 5);
    }

    #[test]
    fn d5p2_example() {
        let program: Program = vec![0, 3, 0, 1, -3];
        assert_eq!(execute_program(program, &add_or_subtract_offset), 10);
    }
}
