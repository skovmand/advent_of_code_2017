use std::collections::HashMap;

type Point = (i32, i32);
type Memory = HashMap<Point, u32>;
type ValueFn = dyn Fn(&u32, &Point, &Memory) -> u32;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const PUZZLE_INPUT: u32 = 368_078;

fn main() {
    let distance = find_memory_location_with_puzzle_input(PUZZLE_INPUT);
    println!(
        "D3P1: The manhattan distance from access port to location with value {} is {}",
        PUZZLE_INPUT, distance
    );

    let (point, value) = find_first_value_exceeding_puzzle_input(PUZZLE_INPUT);
    println!(
        "D3P2: The first value written exceeding {} is {} at coordinate {:?}",
        PUZZLE_INPUT, value, point
    );
}

fn find_memory_location_with_puzzle_input(puzzle_input: u32) -> i32 {
    let memory = init_memory();
    let populated_memory = populate_memory(memory, puzzle_input, &add_one);

    let point = populated_memory
        .iter()
        .find_map(|(&point, &value)| if value == puzzle_input { Some(point) } else { None })
        .unwrap();

    point.0.abs() + point.1.abs()
}

fn find_first_value_exceeding_puzzle_input(puzzle_input: u32) -> (Point, u32) {
    let memory = init_memory();
    let populated_memory = populate_memory(memory, puzzle_input, &adjacent_sum);
    let point: Point = populated_memory
        .iter()
        .find_map(|(&point, &value)| if value >= PUZZLE_INPUT { Some(point) } else { None })
        .unwrap();
    let first_larger_value = populated_memory.get(&point).unwrap();

    (point, *first_larger_value)
}

fn init_memory() -> Memory {
    let mut memory: Memory = HashMap::new();
    memory.insert((0, 0), 1);
    memory
}

fn populate_memory(mut coordinate_system: Memory, max_value: u32, value_fn: &ValueFn) -> Memory {
    let mut last_direction: Direction = Direction::East;
    let mut last_coordinate: Point = (0, 0);
    let mut last_value = 1;

    while last_value < max_value {
        // Calculate the next coordinate given the last coordinate and direction
        last_coordinate = calc_next_coordinate(&last_direction, &last_coordinate);
        last_value = value_fn(&last_value, &last_coordinate, &coordinate_system);

        match coordinate_system.insert(last_coordinate, last_value) {
            None => (),
            Some(_) => panic!("Overwrote value in memory grid!"),
        };

        // Attempt to turn direction left
        let left_turned_direction = left_turn_direction(&last_direction);
        let next_coordinate_after_left_turn = calc_next_coordinate(&left_turned_direction, &last_coordinate);

        // If no value is written at the next coordinate after left turn, change direction
        if let None = coordinate_system.get(&next_coordinate_after_left_turn) {
            last_direction = left_turned_direction;
        }
    }

    coordinate_system
}

// Calculate the value as last value + 1 for D3P1
fn add_one(last_value: &u32, _: &Point, _: &Memory) -> u32 {
    *last_value + 1
}

// Sum the eight adjacent coordinates as value D3P2
fn adjacent_sum(_: &u32, point: &Point, memory: &Memory) -> u32 {
    let adjacent_points: Vec<Point> = calc_adjacent_points(&point);

    adjacent_points.iter().filter_map(|point| memory.get(point)).sum()
}

fn left_turn_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::East => Direction::North,
        Direction::North => Direction::West,
        Direction::West => Direction::South,
        Direction::South => Direction::East,
    }
}

fn calc_next_coordinate(direction: &Direction, point: &Point) -> Point {
    match (direction, point) {
        (Direction::East, (x, y)) => (x + 1, *y),
        (Direction::North, (x, y)) => (*x, y + 1),
        (Direction::West, (x, y)) => (x - 1, *y),
        (Direction::South, (x, y)) => (*x, y - 1),
    }
}

fn calc_adjacent_points((x, y): &Point) -> Vec<Point> {
    vec![
        (x + 1, *y),
        (x + 1, y + 1),
        (*x, y + 1),
        (x - 1, y + 1),
        (x - 1, *y),
        (x - 1, y - 1),
        (*x, y - 1),
        (x + 1, y - 1),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_left_turn_direction() {
        assert_eq!(left_turn_direction(&Direction::East), Direction::North);
        assert_eq!(left_turn_direction(&Direction::North), Direction::West);
        assert_eq!(left_turn_direction(&Direction::West), Direction::South);
        assert_eq!(left_turn_direction(&Direction::South), Direction::East);
    }

    #[test]
    fn test_next_coordinate() {
        assert_eq!(calc_next_coordinate(&Direction::East, &(0, 0)), (1, 0));
        assert_eq!(calc_next_coordinate(&Direction::North, &(0, 0)), (0, 1));
        assert_eq!(calc_next_coordinate(&Direction::West, &(0, 0)), (-1, 0));
        assert_eq!(calc_next_coordinate(&Direction::South, &(0, 0)), (0, -1));
    }

    #[test]
    fn test_calc_adjacent_points() {
        assert_eq!(
            calc_adjacent_points(&(1, 1)),
            vec![(2, 1), (2, 2), (1, 2), (0, 2), (0, 1), (0, 0), (1, 0), (2, 0)]
        );
        assert_eq!(
            calc_adjacent_points(&(-1, -1)),
            vec![(0, -1), (0, 0), (-1, 0), (-2, 0), (-2, -1), (-2, -2), (-1, -2), (0, -2)]
        );
    }

    #[test]
    fn d3p2_unit_test() {
        let mut memory: Memory = HashMap::new();
        memory.insert((0, 0), 1);
        memory.insert((1, 0), 1);
        memory.insert((1, 1), 2);
        memory.insert((0, 1), 4);
        memory.insert((-1, 1), 5);

        assert_eq!(adjacent_sum(&0, &(-1, 0), &memory), 10);
    }

    #[test]
    fn solves_d3() {
        let distance = find_memory_location_with_puzzle_input(PUZZLE_INPUT);
        assert_eq!(distance, 371);

        let (_, value) = find_first_value_exceeding_puzzle_input(PUZZLE_INPUT);
        assert_eq!(value, 369601);
    }
}
