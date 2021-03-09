const PUZZLE_INPUT: &str = include_str!("../input/day_01.txt");

fn main() {
    let puzzle_input: Vec<u32> = parse_input(PUZZLE_INPUT);
    let part_1_answer = inverse_capcha(&puzzle_input);
    println!("The answer to D1P1 is {}", part_1_answer);

    let part_2_answer = inverse_circular_capcha(&puzzle_input);
    println!("The answer to D1P2 is {}", part_2_answer);
}

/* Solver for D1P1 */
pub fn inverse_capcha(number_vec: &Vec<u32>) -> u32 {
    let number_vec_offset_one = offset_vec_by_one(&number_vec);

    number_vec
        .iter()
        .zip(number_vec_offset_one.iter())
        .map(|(x, y)| if x == y { x } else { &0 })
        .sum::<u32>()
}

/* Solver for D1P2 */
pub fn inverse_circular_capcha(number_vec: &Vec<u32>) -> u32 {
    number_vec
        .iter()
        .enumerate()
        .map(|(index, elem)| if elem == opposite(&number_vec, index) { elem } else { &0 })
        .sum::<u32>()
}

/* Get the opposite value in the circle given a vector index */
fn opposite(number_vec: &Vec<u32>, index: usize) -> &u32 {
    let number_vec_length = number_vec.len();
    if number_vec_length % 2 != 0 {
        panic!("The vector must have an even length");
    };

    let calculated_index = (index + (number_vec_length / 2)) % number_vec_length;
    number_vec.get(calculated_index).unwrap()
}

/* Given a vector of u32s, offset it by one by moving the first element to the last position */
fn offset_vec_by_one(original_vec: &Vec<u32>) -> Vec<u32> {
    let mut offset_vec_by_one: Vec<u32> = original_vec.clone();

    let first_element = offset_vec_by_one[0];
    offset_vec_by_one.push(first_element);
    offset_vec_by_one.remove(0);

    offset_vec_by_one
}

/* Create a vector of u32s from a string input */
fn parse_input(string_input: &str) -> Vec<u32> {
    string_input
        .chars()
        .filter_map(|c| c.to_digit(10))
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_unit_tests() {
        [("1122", 3), ("1111", 4), ("1234", 0), ("91212129", 9)]
            .iter()
            .for_each(|(input, expected)| {
                let puzzle_input = parse_input(*input);
                assert_eq!(inverse_capcha(&puzzle_input), *expected);
            })
    }

    #[test]
    fn solves_p1() {
        let puzzle_input = parse_input(PUZZLE_INPUT);
        assert_eq!(inverse_capcha(&puzzle_input), 997);
    }

    #[test]
    fn p2_unit_tests() {
        [("1212", 6), ("1221", 0), ("123425", 4), ("123123", 12), ("12131415", 4)]
            .iter()
            .for_each(|(input, expected)| {
                let puzzle_input = parse_input(*input);
                assert_eq!(inverse_circular_capcha(&puzzle_input), *expected);
            })
    }

    #[test]
    fn solves_p2() {
        let puzzle_input = parse_input(PUZZLE_INPUT);
        assert_eq!(inverse_circular_capcha(&puzzle_input), 1358);
    }
}
