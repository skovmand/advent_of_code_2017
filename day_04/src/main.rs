use std::collections::HashSet;
use std::iter::FromIterator;

const PUZZLE_INPUT: &str = include_str!("../../puzzle_inputs/day_04.txt");

fn main() {
    let passwords: Vec<&str> = parse_input(PUZZLE_INPUT);

    let valid_passwords = passwords.iter().filter(|pw| validate_no_same_words(pw)).count();
    println!("D4P1: The count of valid passwords is {}", valid_passwords);

    let valid_passwords = passwords.iter().filter(|pw| validate_no_anagrams(pw)).count();
    println!("D4P2: The count of passwords with no anagrams is {}", valid_passwords);
}

fn parse_input(string_input: &str) -> Vec<&str> {
    string_input.trim().split('\n').collect()
}

// Validate: Are any two words in the password similar?
fn validate_no_same_words(password: &str) -> bool {
    let words: Vec<&str> = password.split(' ').collect();
    let word_count = words.len();
    let set: HashSet<&str> = HashSet::from_iter(words);

    word_count == set.len()
}

// Validate: Are any two words anagrams?
fn validate_no_anagrams(password: &str) -> bool {
    let normalized_words: Vec<String> = password.split(' ').map(|word| to_normalized_word(word)).collect();
    let normalized_word_count = normalized_words.len();
    let set: HashSet<String> = HashSet::from_iter(normalized_words);

    normalized_word_count == set.len()
}

// Having no clue what to call this, but take all chars in a word,
// and sort them by char code, creating a way to identify anagrams
fn to_normalized_word(word: &str) -> String {
    let mut chars: Vec<char> = word.chars().collect();
    chars.sort_unstable();
    chars.iter().collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_no_same_words() {
        assert_eq!(validate_no_same_words("aa bb cc dd ee"), true);
        assert_eq!(validate_no_same_words("aa bb cc dd aa"), false);
        assert_eq!(validate_no_same_words("aa bb cc dd aaa"), true);
    }

    #[test]
    fn test_validate_no_anagrams() {
        assert_eq!(validate_no_anagrams("abcde fghij"), true);
        assert_eq!(validate_no_anagrams("abcde xyz ecdab"), false);
        assert_eq!(validate_no_anagrams("a ab abc abd abf abj"), true);
        assert_eq!(validate_no_anagrams("iiii oiii ooii oooi oooo"), true);
        assert_eq!(validate_no_anagrams("oiii ioii iioi iiio"), false);
    }

    #[test]
    fn test_to_normalized_word() {
        assert_eq!(to_normalized_word("zzffuuqqaa"), "aaffqquuzz");
        assert_eq!(to_normalized_word("mountaintop"), "aimnnoopttu");
    }

    #[test]
    fn solves_d4() {
        let passwords: Vec<&str> = parse_input(PUZZLE_INPUT);

        let valid_passwords_p1 = passwords.iter().filter(|pw| validate_no_same_words(pw)).count();
        assert_eq!(valid_passwords_p1, 455);

        let valid_passwords_p2 = passwords.iter().filter(|pw| validate_no_anagrams(pw)).count();
        assert_eq!(valid_passwords_p2, 186);
    }
}
