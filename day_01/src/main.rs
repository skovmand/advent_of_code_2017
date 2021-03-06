use std::fs;

fn main() {
    let input = fs::read_to_string("input/day01.txt").expect("Unable to read file");

    let part_1_answer = inverse_capcha(input);
    println!("The answer to D1P1 is {}", part_1_answer);
}

pub fn inverse_capcha(input: String) -> u32 {
    let number_vec: Vec<u32> = create_number_vec(&input);
    let number_vec_offset_one = offset_number_vec(&number_vec);

    number_vec.iter().zip(number_vec_offset_one.iter())
        .map(|(x,y)| if x == y { x } else { &0 })
        .sum::<u32>()
}

fn create_number_vec(string_input: &String) -> Vec<u32> {
    string_input
        .split("")
        .filter(|c| c != &"")
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>()
}

fn offset_number_vec(original_vec: &Vec<u32>) -> Vec<u32> {
    let mut offset_number_vec: Vec<u32> = original_vec.clone();

    let first_element = offset_number_vec[0];
    offset_number_vec.push(first_element);
    offset_number_vec.remove(0);

    offset_number_vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d1_unit_tests() {
        assert_eq!(inverse_capcha(String::from("1122")), 3);
        assert_eq!(inverse_capcha(String::from("1111")), 4);
        assert_eq!(inverse_capcha(String::from("1234")), 0);
        assert_eq!(inverse_capcha(String::from("91212129")), 9);
    }
}
