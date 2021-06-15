const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_09.txt");

fn main() {
    let score = calculate_score(PUZZLE_INPUT);
    println!("Day 9 part 1: Score is {}", score);
}

fn calculate_score(input: &str) -> u32 {
    let mut level = 0;
    let mut score = 0;
    let mut is_garbage: bool = false;
    let mut stream = input.chars();

    while let Some(c) = stream.next() {
        match c {
            '<' => is_garbage = true,
            '>' => is_garbage = false,
            '{' => {
                if !is_garbage {
                    level += 1;
                    score += level;
                }
            }
            '}' => {
                if !is_garbage {
                    level -= 1;
                }
            }
            '!' => {
                stream.next();
            }
            _ => {}
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_group_scores() {
        assert_eq!(calculate_score("{}"), 1);
        assert_eq!(calculate_score("{{{}}}"), 6);
        assert_eq!(calculate_score("{{},{}}"), 5);
        assert_eq!(calculate_score("{{{},{},{{}}}}"), 16);
        assert_eq!(calculate_score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(calculate_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
        assert_eq!(calculate_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(calculate_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);
    }
}
