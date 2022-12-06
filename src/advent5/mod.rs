use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent5_part_one(file_path: &str, is_part_one: bool) -> String {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for line in reader.lines() {
        let l = line.unwrap().parse::<String>().unwrap();
        if l.contains("[") {
            for i in 0..(l.len() % 4) {
                // for i in 0..(l.len() / 3 - 2) {
                let char_index = (i * 4) + 1;
                print!(" {}, {}   ", l.as_bytes()[char_index] as char, l.as_bytes()[char_index]);
                if stacks.get(i).is_none() {
                    stacks.push(VecDeque::new());
                }
                if l.as_bytes()[char_index] != 32 {
                    stacks[i].push_front(l.as_bytes()[char_index] as char);
                }
                println!(" stack-{} index:{}, {:?}", i, char_index, stacks);
            }
        } else if l.contains("move") {
            let mut split = l.split(" ");
            split.next();
            let quantity = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let from = split.next().unwrap().parse::<usize>().unwrap();
            split.next();
            let to = split.next().unwrap().parse::<usize>().unwrap();
            print!("move {} from {} to {}    ", quantity, from, to);

            if is_part_one {
                for _i in 0..quantity {
                    let to_move = stacks[from - 1].pop_back().unwrap();
                    stacks[to - 1].push_back(to_move);
                }
            } else {
                let mut temp_vec_to_move = VecDeque::new();
                for _i in 0..quantity {
                    temp_vec_to_move.push_front(stacks[from - 1].pop_back().unwrap());
                }
                for to_move in temp_vec_to_move {
                    stacks[to - 1].push_back(to_move);
                }
            }
        }
        println!("{:?}", stacks);
    }
    let mut res = String::from("");
    for mut stack in stacks {
        res.push(stack.pop_back().unwrap());
    }
    return res;
}

#[cfg(test)]
mod tests {
    use crate::advent5::advent5_part_one;

    #[test]
    fn ok_example_1() {
        assert_eq!(advent5_part_one("./src/advent5/input_example.txt", true), "CMZ");
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(advent5_part_one("./src/advent5/input_example.txt", false), "MCD");
    }
}