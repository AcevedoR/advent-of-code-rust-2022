use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent2(file_path: &str, scoring_function: fn(char, char) -> i32) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();
        let line_arr: Vec<char> = temp.chars().collect();
        let left = line_arr[0];
        let right = line_arr[2];

        let my_score: i32 = scoring_function(right, left);

        score += my_score;

        println!("{}", score);
    }
    return score;
}

fn score_round_1(my_input: char, enemy_input: char) -> i32 {
    let temp: i32 = my_input as i32 - 87;
    println!("{} {} {}", my_input, my_input as i32, temp);
    println!("{} {}", enemy_input, enemy_input as i32);

    if enemy_input as i32 == my_input as i32 - 23 {
        return temp + 3;
    }
    if enemy_input == 'A' && my_input == 'Y' {
        return temp + 6;
    }
    if enemy_input == 'A' && my_input == 'Z' {
        return temp;
    }
    if enemy_input == 'B' && my_input == 'Z' {
        return temp + 6;
    }
    if enemy_input == 'B' && my_input == 'X' {
        return temp;
    }
    if enemy_input == 'C' && my_input == 'X' {
        return temp + 6;
    }
    if enemy_input == 'C' && my_input == 'Y' {
        return temp;
    }
    panic!("wrong char");
}

fn score_round_part_two(my_input: char, enemy_input: char) -> i32 {
    if enemy_input == 'A' && my_input == 'X' {
        return 3;
    }
    if enemy_input == 'B' && my_input == 'X' {
        return 1;
    }
    if enemy_input == 'C' && my_input == 'X' {
        return 2;
    }


    if enemy_input == 'A' && my_input == 'Y' {
        return 1 + 3;
    }
    if enemy_input == 'B' && my_input == 'Y' {
        return 2 + 3;
    }
    if enemy_input == 'C' && my_input == 'Y' {
        return 3 + 3;
    }

    if enemy_input == 'A' && my_input == 'Z' {
        return 2 + 6;
    }
    if enemy_input == 'B' && my_input == 'Z' {
        return 3 + 6;
    }
    if enemy_input == 'C' && my_input == 'Z' {
        return 1 + 6;
    }

    panic!("wrong char");
}

#[cfg(test)]
mod tests {
    use crate::advent2::{advent2, score_round_1, score_round_part_two};

    #[test]
    fn ok_example_1() {
        assert_eq!(self::advent2("./src/advent2/input_example.txt", score_round_1), 15);
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(self::advent2("./src/advent2/input_example.txt", score_round_part_two), 12);
    }
}