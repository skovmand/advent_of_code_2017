use std::collections::HashMap;

type Point = (i32, i32);
type Memory = HashMap<Point, u32>;

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

const PUZZLE_INPUT: u32 = 368_078;

fn main() {
    let memory = init_memory();
    let populated_memory = populate_memory(memory, PUZZLE_INPUT);
    let (x, y): Point = populated_memory
        .iter()
        .find_map(|(&point, &value)| if value == PUZZLE_INPUT { Some(point) } else { None })
        .unwrap();

    println!("Coordinate is {}, {}", x, y);
    println!("Distance is {}", x.abs() + y.abs());
}

fn init_memory() -> Memory {
    let mut memory: Memory = HashMap::new();
    memory.insert((0, 0), 1);
    memory
}

fn populate_memory(mut coordinate_system: Memory, max_value: u32) -> Memory {
    let mut last_direction: Direction = Direction::East;
    let mut last_coordinate: Point = (0, 0);

    for next_value in 2..(max_value + 1) {
        // Calculate the next coordinate given the last coordinate and direction
        last_coordinate = calc_next_coordinate(&last_direction, &last_coordinate);

        match coordinate_system.insert(last_coordinate, next_value) {
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

// fn iterate_state(state: State) -> State {}

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
}
