use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent2023_4(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();
        let [winning_numbers_raw, numbers_you_have_raw]: [&str; 2] = temp
            .split(": ")
            .nth(1)
            .unwrap()
            .split(" | ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let mut normalized_winning_numbers = winning_numbers_raw.replace("  ", " ");
        if normalized_winning_numbers.chars().next().unwrap() == ' '{
            normalized_winning_numbers.remove(0usize);
        }
        let mut normalized_numbers_you_have = numbers_you_have_raw.replace("  ", " ");
        if normalized_numbers_you_have.chars().next().unwrap() == ' '{
            normalized_numbers_you_have.remove(0usize);
        }
        println!("parsing: {} and : {}", normalized_winning_numbers, normalized_numbers_you_have);

        let winning_numbers: Vec<i32> = normalized_winning_numbers.split(" ")
            .map(|c| c.parse::<i32>().expect(&*format!("failed to parse number {}", c)))
            .collect();
        let numbers_you_have: Vec<i32> = normalized_numbers_you_have.split(" ").map(|c| c.parse::<i32>().expect(&*format!("failed to parse number {}", c))).collect();
        let mut game_score = 0;
        for number_you_have in &numbers_you_have {
            for winning_number in &winning_numbers {
                if number_you_have == winning_number {
                    if game_score == 0 {
                        game_score = 1;
                    } else {
                        game_score = game_score * 2;
                    }
                }
            }
        }
        score = score + game_score;
    }

    return score;
}

#[cfg(test)]
mod tests {
    use crate::advent2023_4::advent2023_4;

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_4("./src/advent2023_4/input_example.txt"), 13);
    }
}