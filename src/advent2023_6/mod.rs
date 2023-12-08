extern crate itertools;

use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent2023_6(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 1;
    let mut lines = reader.lines();
    let times = lines.next().unwrap().unwrap();
    let distances = lines.next().unwrap().unwrap();
// distance = x * (game_time - x)
    times.split_whitespace().skip(1)
        .zip(distances.split_whitespace().skip(1))
        .map(|(t, d)| {
            println!("{} {}", t, d);
            return (t.parse::<u32>().unwrap(), d.parse::<u32>().unwrap());
        })
        .for_each(|(t, d)| {
            let mut wins = 0;
            for x in 0..t {
                let possible_distance = x * (t - x);
                if possible_distance > d {
                    wins += 1;
                }
            }
            score = score * wins;
        });
    return score;
}

fn advent2023_6_2(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 1;
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let time = first_line.replace("Time:", "").replace(" ", "").parse::<u64>().unwrap();
    let distance = second_line.replace("Distance:", "").replace(" ", "").parse::<u64>().unwrap();
    let mut wins = 0;
    for x in 0..time {
        let possible_distance = x * (time - x);
        if possible_distance > distance {
            wins += 1;
        }
    }
    score = score * wins;
    return score;
}

#[cfg(test)]
mod tests {
    use crate::advent2023_6::{advent2023_6, advent2023_6_2};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_6("./src/advent2023_6/input_example.txt"), 288);
    }

    #[test]
    fn ok_example_1_part_two() {
        assert_eq!(advent2023_6_2("./src/advent2023_6/input_example.txt"), 71503);
    }
}