use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent2023_1(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();
        let first_digit: u32 = find_first_digit(temp.clone(), false);
        let last_digit: u32 = find_first_digit(temp.chars().rev().collect::<String>(), true);

        score = score + format!("{}{}", first_digit, last_digit).parse::<u32>().unwrap();

        println!("{} {}", first_digit, last_digit);
    }
    return score as i32;
}

fn find_first_digit(string: String, is_reversed: bool) -> u32 {
    let patterns: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let chars: Vec<char> = string.chars().collect();
    let mut to_match: VecDeque<char> = VecDeque::new();
    for char in chars {
        to_match.push_back(char);
        if to_match.len() > 5 {
            to_match.pop_front();
        }
        let str = to_match.iter().map(|c| c.to_string()).reduce(|a, b| format!("{}{}", a, b)).unwrap();
        println!("match {}  {}", is_reversed, str);
        if patterns.iter().any(|s| str.contains(&revv(s.to_string(), is_reversed))) {
            return if str.contains(&revv("one".to_string(), is_reversed)) { 1 } else if str.contains(&revv("two".to_string(), is_reversed)) { 2 } else if str.contains(&revv("three".to_string(), is_reversed)) { 3 } else if str.contains(&revv("four".to_string(), is_reversed)) { 4 } else if str.contains(&revv("five".to_string(), is_reversed)) { 5 } else if str.contains(&revv("six".to_string(), is_reversed)) { 6 } else if str.contains(&revv("seven".to_string(), is_reversed)) { 7 } else if str.contains(&revv("eight".to_string(), is_reversed)) { 8 } else if str.contains(&revv("nine".to_string(), is_reversed)) {
                9
            } else {
                0
            };
        }
        if let Some(digit) = char.to_digit(10) {
            return digit;
        }
    }
    return 0;
}

fn revv(s: String, reverse: bool) -> String {
    if reverse {
        return s.chars().rev().collect::<String>();
    }
    return s;
}


#[cfg(test)]
mod tests {
    use crate::advent2023_1::advent2023_1;

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_1("./src/advent2023_1/input.txt"), 281);
    }
}