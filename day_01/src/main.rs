use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01.txt").expect("Unable to read file");

    let part_1_answer = inverse_capcha(input.clone());
    println!("The answer to D1P1 is {}", part_1_answer);

    let part_2_answer = inverse_circular_capcha(input);
    println!("The answer to D1P2 is {}", part_2_answer);
}

/* Solver for D1P1 */
pub fn inverse_capcha(input: String) -> u32 {
    let number_vec: Vec<u32> = parse_input(&input);
    let number_vec_offset_one = offset_vec_by_one(&number_vec);

    number_vec
        .iter()
        .zip(number_vec_offset_one.iter())
        .map(|(x, y)| if x == y { x } else { &0 })
        .sum::<u32>()
}

/* Solver for D1P2 */
pub fn inverse_circular_capcha(input: String) -> u32 {
    let number_vec: Vec<u32> = parse_input(&input);

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
fn parse_input(string_input: &String) -> Vec<u32> {
    string_input
        .split("")
        .filter(|c| c != &"")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1_unit_tests() {
        assert_eq!(inverse_capcha(String::from("1122")), 3);
        assert_eq!(inverse_capcha(String::from("1111")), 4);
        assert_eq!(inverse_capcha(String::from("1234")), 0);
        assert_eq!(inverse_capcha(String::from("91212129")), 9);
    }

    #[test]
    fn p2_unit_tests() {
        assert_eq!(inverse_circular_capcha(String::from("1212")), 6);
        assert_eq!(inverse_circular_capcha(String::from("1221")), 0);
        assert_eq!(inverse_circular_capcha(String::from("123425")), 4);
        assert_eq!(inverse_circular_capcha(String::from("123123")), 12);
        assert_eq!(inverse_circular_capcha(String::from("12131415")), 4);
    }
}
