use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent3_part_one(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();

        let (first_half, second_half): (&str, &str) = temp.split_at(temp.len() / 2);

        let mut ascii: [u32; 127] = [0; 127];

        for item in first_half.chars() {
            ascii[item as usize] = 1;
        }
        for item in second_half.chars() {
            // println!("{}, {}", item, ascii[item as usize]);

            if ascii[item as usize] == 1 {
                ascii[item as usize] += 1;

                let temp: i32 = if item.is_lowercase() {
                    item as i32 - 96
                } else {
                    item as i32 - 38
                };
                println!("{}, {}  => {}", item, item as i32, temp);

                score += temp;
            }
        }
    }
    return score;
}

fn advent3_part_two(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut ascii: [u32; 127] = [0; 127];
    let mut group_index = 0;

    for line in reader.lines() {
        let temp = line.unwrap().parse::<String>().unwrap();

        println!("new line {}, {}", group_index, ascii['q' as usize]);

        for item in temp.chars() {
            if ascii[item as usize] == group_index {
                ascii[item as usize] += 1;
                // println!("    adding {}", item);
            }

            if group_index == 2 {
                if ascii[item as usize] == 3 {
                    let temp: i32 = if item.is_lowercase() {
                        item as i32 - 96
                    } else {
                        item as i32 - 38
                    };
                    println!("{}, {}  => {}", item, item as i32, temp);

                    score += temp;
                    ascii[item as usize] += 1;
                }
            }
        }

        if group_index == 2 {
            group_index = 0;
            for i in 0..ascii.len() {
                ascii[i] = 0;
            }
            println!("changing group");
        } else {
            group_index += 1;
        }
    }
    return score;
}

#[cfg(test)]
mod tests {
    use crate::advent3::{advent3_part_one, advent3_part_two};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent3_part_one("./src/advent3/input_example.txt"), 157);
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(advent3_part_two("./src/advent3/input_example.txt"), 70);
    }
}