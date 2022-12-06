use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent4_part_one(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let l = line.unwrap().parse::<String>().unwrap();
        let mut split = l.split(",");
        let mut split_left = split.next().unwrap().split("-");
        let a1 = split_left.next().unwrap().parse::<i32>().unwrap();
        let b1 = split_left.next().unwrap().parse::<i32>().unwrap();
        let mut split_right = split.next().unwrap().split("-");
        let a2 = split_right.next().unwrap().parse::<i32>().unwrap();
        let b2 = split_right.next().unwrap().parse::<i32>().unwrap();

        if a1 >= a2 && b1 <= b2 {
            score += 1;
        } else if a2 >= a1 && b2 <= b1 {
            score += 1;
        }
        // println!("{}-{}, {}-{}", a1, b1, a2, b2);
    }
    return score;
}

fn advent4_part_two(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let l = line.unwrap().parse::<String>().unwrap();
        let mut split = l.split(",");
        let mut split_left = split.next().unwrap().split("-");
        let a1 = split_left.next().unwrap().parse::<i32>().unwrap();
        let b1 = split_left.next().unwrap().parse::<i32>().unwrap();
        let mut split_right = split.next().unwrap().split("-");
        let a2 = split_right.next().unwrap().parse::<i32>().unwrap();
        let b2 = split_right.next().unwrap().parse::<i32>().unwrap();

        if a1 <= b2 && a2 <= b1 {
            score += 1;
            println!("f    {}-{}, {}-{}", a1, b1, a2, b2);
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use crate::advent4::{advent4_part_one, advent4_part_two};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent4_part_one("./src/advent4/input_example.txt"), 2);
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(advent4_part_two("./src/advent4/input_example.txt"), 4);
    }
}