const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_09.txt");

fn main() {
    let score = calculate_score(PUZZLE_INPUT);
    println!("Day 9 part 1: Score is {}", score.0);
    println!("Day 9 part 2: Char count is {}", score.1);
}

fn calculate_score(input: &str) -> (u32, u32) {
    let mut level = 0;
    let mut score = 0;
    let mut is_garbage: bool = false;
    let mut garbage_chars: u32 = 0;
    let mut stream = input.trim().chars();

    while let Some(c) = stream.next() {
        match c {
            '<' if !is_garbage => is_garbage = true,
            '>' => is_garbage = false,
            '{' if !is_garbage => {
                level += 1;
                score += level;
            }
            '}' if !is_garbage => level -= 1,
            '!' => {
                stream.next();
            }
            _ if is_garbage => garbage_chars += 1,
            _ => {}
        }
    }

    (score, garbage_chars)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calculates_group_scores() {
        assert_eq!(calculate_score("{}").0, 1);
        assert_eq!(calculate_score("{{{}}}").0, 6);
        assert_eq!(calculate_score("{{},{}}").0, 5);
        assert_eq!(calculate_score("{{{},{},{{}}}}").0, 16);
        assert_eq!(calculate_score("{<a>,<a>,<a>,<a>}").0, 1);
        assert_eq!(calculate_score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0, 9);
        assert_eq!(calculate_score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0, 9);
        assert_eq!(calculate_score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0, 3);
    }

    #[test]
    fn counts_characters() {
        assert_eq!(calculate_score("<>").1, 0);
        assert_eq!(calculate_score("<random characters>").1, 17);
        assert_eq!(calculate_score("<<<<>").1, 3);
        assert_eq!(calculate_score("<{!>}>").1, 2);
        assert_eq!(calculate_score("<!!>").1, 0);
        assert_eq!(calculate_score("<!!!>>").1, 0);
        assert_eq!(calculate_score(r#"<{o"i!a,<{i<a>"#).1, 10);
    }
}
