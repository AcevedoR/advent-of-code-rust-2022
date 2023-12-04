use std::fs::File;
use std::io::{BufReader, prelude::*};


fn advent2023_2(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();
        let normalized_game = temp.replace(";",",");
        let [_, game_draws]: [&str; 2] = normalized_game
            .split(": ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        let tokens = game_draws.split(", ");
        let mut min_red:u32=0;
        let mut min_green:u32=0;
        let mut min_blue:u32=0;


        for token in tokens {
            let [number_str, color]: [&str; 2] = token
                .split(" ")
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();
            let number = number_str.parse::<u32>().unwrap();
            match color {
                "red" => if number > min_red {min_red = number},
                "green" => if number > min_green {min_green = number},
                "blue" => if number > min_blue {min_blue = number},
                _ => panic!("wrong color {}", color),
            };
        }
        score = score + min_blue * min_green * min_red;

    }
    return score as i32;
}

#[cfg(test)]
mod tests {
    use crate::advent2023_2::{advent2023_2};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_2("./src/advent2023_2/input_example.txt"), 2286);
    }
}