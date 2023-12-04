use std::cmp::min;
use std::collections::HashMap;
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
        if normalized_winning_numbers.chars().next().unwrap() == ' ' {
            normalized_winning_numbers.remove(0usize);
        }
        let mut normalized_numbers_you_have = numbers_you_have_raw.replace("  ", " ");
        if normalized_numbers_you_have.chars().next().unwrap() == ' ' {
            normalized_numbers_you_have.remove(0usize);
        }
        println!("parsing: {} and : {}", normalized_winning_numbers, normalized_numbers_you_have);

        let winning_numbers: Vec<u32> = normalized_winning_numbers.split(" ")
            .map(|c| c.parse::<u32>().expect(&*format!("failed to parse number {}", c)))
            .collect();
        let numbers_you_have: Vec<u32> = normalized_numbers_you_have.split(" ").map(|c| c.parse::<u32>().expect(&*format!("failed to parse number {}", c))).collect();
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

#[derive(Clone, PartialEq, Eq, Hash)]
struct Game {
    input: String,
    id: u32,
}

fn advent2023_4_2(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut games: Vec<Game> = Vec::new();
    let mut distinct_games: Vec<Game> = Vec::new();
    let mut encountered_scratchcards: HashMap<u32, u32> = HashMap::new();

    let mut lines = reader.lines();
    while let Some(line) = lines.next() {
        let string = line
            .unwrap();
        let [mut id, input]: [&str; 2] = string
            .split(": ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let string1 = id.replace("   ", " ").replace("  ", " ");
        id = string1.split(" ").nth(1).unwrap();
        games.push(Game {
            input: input.to_string(),
            id: id.parse::<u32>().unwrap(),
        });
        distinct_games.push(Game {
            input: input.to_string(),
            id: id.parse::<u32>().unwrap(),
        });
    }
    games.reverse();
    while let Some(game) = games.pop() {
        let [winning_numbers_raw, numbers_you_have_raw]: [&str; 2] = game.input
            .split(" | ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let mut normalized_winning_numbers = winning_numbers_raw.replace("  ", " ");
        if normalized_winning_numbers.chars().next().unwrap() == ' ' {
            normalized_winning_numbers.remove(0usize);
        }
        let mut normalized_numbers_you_have = numbers_you_have_raw.replace("  ", " ");
        if normalized_numbers_you_have.chars().next().unwrap() == ' ' {
            normalized_numbers_you_have.remove(0usize);
        }
        println!("parsing: {} and : {}", normalized_winning_numbers, normalized_numbers_you_have);

        let winning_numbers: Vec<u32> = normalized_winning_numbers.split(" ")
            .map(|c| c.parse::<u32>().expect(&*format!("failed to parse number {}", c)))
            .collect();
        let numbers_you_have: Vec<u32> = normalized_numbers_you_have.split(" ").map(|c| c.parse::<u32>().expect(&*format!("failed to parse number {}", c))).collect();
        let mut gained_number_cards = 0;
        for number_you_have in &numbers_you_have {
            for winning_number in &winning_numbers {
                if number_you_have == winning_number {
                    gained_number_cards = gained_number_cards + 1;
                }
            }
        }
        increment_map(&mut encountered_scratchcards, &game);
        if gained_number_cards > 0 {
            println!("card {} is won with {}", game.id, gained_number_cards);
            if let Some(to_duplicate) = encountered_scratchcards.clone().get(&game.id) {
                for y in 0..to_duplicate.clone() {
                    println!("\t to duplicate {} > {}", to_duplicate.clone(), y);
                    for i in (game.id + 1)..(min((distinct_games.len() + 1) as u32, gained_number_cards + game.id + 1)) {
                        let found_game = distinct_games.iter().find(|g| g.id == i).expect(&*format!("game {} should exists", i));
                        increment_map(&mut encountered_scratchcards, found_game);
                        println!("\tadding {}", found_game.id.clone());
                    }
                }
            }
        }
    }
    println!("{:?}", encountered_scratchcards.clone());
    return encountered_scratchcards.iter().map(|x| x.1.clone()).reduce(|a, b| (a + b)).unwrap();
}

fn increment_map(encountered_scratchcards: &mut HashMap<u32, u32>, game: &Game) {
    encountered_scratchcards.insert(game.id, *encountered_scratchcards.get(&game.id).unwrap_or(&0) + 1);
}

#[cfg(test)]
mod tests {
    use crate::advent2023_4::{advent2023_4, advent2023_4_2};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_4("./src/advent2023_4/input_example.txt"), 13);
    }

    #[test]
    fn ok_example_1_part_two() {
        assert_eq!(advent2023_4_2("./src/advent2023_4/input_example.txt"), 30);
    }
}