use std::fs;

fn main() {
    let input = fs::read_to_string("input/day_02.txt").expect("Unable to read file");
    let spreadsheet = parse_spreadsheet(input);

    let part_1_answer = spreadsheet_checksum(&spreadsheet);
    println!("The answer to D2P1 is {}", part_1_answer);

    let part_2_answer = spreadsheet_checksum_by_division(&spreadsheet);
    println!("The answer to D2P2 is {}", part_2_answer);
}

/* D2P1 */
pub fn spreadsheet_checksum(spreadsheet: &Vec<Vec<u32>>) -> u32 {
    spreadsheet.iter().map(|line| row_checksum(line)).sum::<u32>()
}

/* D2P2 */
pub fn spreadsheet_checksum_by_division(spreadsheet: &Vec<Vec<u32>>) -> u32 {
    spreadsheet.iter().map(|line| row_checksum_division(line)).sum::<u32>()
}

/* Calculate the checksum of a single row using the first algorithm */
fn row_checksum(row: &Vec<u32>) -> u32 {
    let minimum = row.iter().min().unwrap();
    let maximum = row.iter().max().unwrap();

    maximum - minimum
}

/* Calculate the checksum of a single row using the second algorithm */
fn row_checksum_division(row: &Vec<u32>) -> u32 {
    let result: Option<(u32, u32)> = row.iter().enumerate().find_map(|(index, value)| {
        let mut row_without_value = row.clone();
        row_without_value.remove(index);
        find_divisible(*value, row_without_value)
    });

    match result {
        None => panic!("No result found!"),
        Some((dividend, divisor)) => dividend / divisor,
    }
}

fn find_divisible(dividend: u32, row: Vec<u32>) -> Option<(u32, u32)> {
    match row.iter().find(|divisor| dividend % *divisor == 0) {
        None => None,
        Some(divisor) => Some((dividend, *divisor)),
    }
}

fn parse_spreadsheet(spreadsheet_input: String) -> Vec<Vec<u32>> {
    spreadsheet_input
        .trim()
        .split("\n")
        .map(|s| parse_row(s))
        .collect::<Vec<Vec<u32>>>()
}

fn parse_row(row: &str) -> Vec<u32> {
    row.split_whitespace().map(|x| x.parse().unwrap()).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn row_checksum_test() {
        assert_eq!(row_checksum(&parse_row("5 1 9 5")), 8);
        assert_eq!(row_checksum(&parse_row("7 5 3")), 4);
        assert_eq!(row_checksum(&parse_row("2 4 6 8")), 6);
    }

    #[test]
    fn spreadsheet_checksum_test() {
        let spreadsheet_input = "5 1 9 5\n\
                           7 5 3\n\
                           2 4 6 8"
            .to_string();

        let spreadsheet = parse_spreadsheet(spreadsheet_input);

        assert_eq!(spreadsheet_checksum(&spreadsheet), 18);
    }

    #[test]
    fn row_checksum_division_test() {
        assert_eq!(row_checksum_division(&parse_row("5 9 2 8")), 4);
        assert_eq!(row_checksum_division(&parse_row("9 4 7 3")), 3);
        assert_eq!(row_checksum_division(&parse_row("3 8 6 5")), 2);
    }

    #[test]
    fn spreadsheet_checksum_by_division_test() {
        let spreadsheet_input = "5 9 2 8\n\
                            9 4 7 3\n\
                            3 8 6 5"
            .to_string();

        let spreadsheet = parse_spreadsheet(spreadsheet_input);

        assert_eq!(spreadsheet_checksum_by_division(&spreadsheet), 9);
    }
}
