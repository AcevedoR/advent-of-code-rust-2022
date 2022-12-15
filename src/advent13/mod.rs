use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufReader, prelude::*};

use indextree::Arena;

fn part_one(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut count = 0;
    let mut previous_line = String::new();
    let mut line_index = 1;
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        if line.is_empty() {
            previous_line.clear();
        } else if !previous_line.is_empty() {
            let ressss = compare(
                Pair::new(previous_line.clone(), line.clone())
            );
            if ressss == 1 {
                count += line_index;
            }
            if ressss == 0 {
                panic!("fdsfdsfsddfs");
            }
            line_index += 1;
        } else {
            previous_line = line;
        }
    }
    return count;
}

fn part_two(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut inputs: Vec<String> = Vec::new();
    for line_res in reader.lines() {
        let line = line_res.unwrap();
        if !line.is_empty() {
            inputs.push(line);
        }
    }
    inputs.push("[[2]]".to_string());
    inputs.push("[[6]]".to_string());
    inputs.sort_by(|a, b| compare_pair(Pair::new(a.clone(), b.clone())));
    println!("\n\nLIST ");

    let mut first_indice_value :i32 = -1;
    let mut second_indice_value :i32 = -1;
    let mut i = 1;
    for input in inputs {
        println!("{:?}", input);
        if input == "[[2]]" {
            first_indice_value = i;
        }
        if input == "[[6]]" {
            second_indice_value = i;
        }
        if first_indice_value != -1 && second_indice_value != -1 {
            return (first_indice_value * second_indice_value) as u32;
        }
        i+=1;
    }
    panic!("something bad happend");
}

struct Pair {
    left: String,
    right: String,
}

fn compare_pair(pair: Pair) -> Ordering {
    return if compare(pair) == 1 {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

impl Pair {
    pub fn new_from_str(left: &str, right: &str) -> Self {
        Pair {
            left: left.parse().unwrap(),
            right: right.parse().unwrap(),
        }
    }
    pub fn new(left: String, right: String) -> Self {
        Pair {
            left,
            right,
        }
    }
}

fn compare(pair: Pair) -> i8 {
    println!("Compare {} vs {}", pair.left.clone(), pair.right.clone());
    let left_elements: Vec<String> = split(pair.left.clone());
    let right_elements: Vec<String> = split(pair.right.clone());
    let highest = if right_elements.len() > left_elements.len() { right_elements.len() } else { left_elements.len() };
    for i in 0..highest {
        let right_opt = right_elements.get(i);
        let left_opt = left_elements.get(i);
        println!("    Compare {} vs {}", left_opt.unwrap_or(&"".to_string()), right_opt.unwrap_or(&"".to_string()));

        if right_opt.is_none() {
            return -1;
        }
        if left_opt.is_none() {
            return 1;
        }
        let right = right_opt.unwrap();
        let left = left_elements.get(i).unwrap();

        if left.starts_with("[") && right.starts_with("[") {
            let res = compare(Pair::new(
                left.clone(),
                right.clone(),
            ));
            if res != 0 {
                return res;
            }
        } else if !left.starts_with("[") && right.starts_with("[") {
            let res = compare(Pair::new(
                format!("[{}]", left.clone()),
                right.clone(),
            ));
            if res != 0 {
                return res;
            }
        } else if left.starts_with("[") && !right.starts_with("[") {
            let res = compare(Pair::new(
                left.clone(),
                format!("[{}]", right.clone()),
            ));
            if res != 0 {
                return res;
            }
        } else {
            if is_left_higher_than_right(left, right) {
                return -1;
            }
            if is_left_lower_than_right(left, right) {
                return 1;
            }
        }
    }
    return 0;
}

fn is_left_higher_than_right(left_element: &String, right_element: &String) -> bool {
    left_element.parse::<u32>().expect(&*format!("{} was not a digit", left_element)) > right_element.parse::<u32>().unwrap()
}

fn is_left_lower_than_right(left_element: &String, right_element: &String) -> bool {
    left_element.parse::<u32>().unwrap() < right_element.parse::<u32>().unwrap()
}

fn split(input: String) -> Vec<String> {
    if !input.starts_with("[") || !input.ends_with("]") {
        panic!("should be a list and was: {}", input);
    }
    let mut result = Vec::new();
    let mut input_inside_list = input.clone();
    input_inside_list.remove(0);
    input_inside_list.remove(input_inside_list.len() - 1);

    let mut opened_list_count = 0;
    let mut buffer = String::new();
    for i in 0..input_inside_list.chars().count() {
        let element = input_inside_list.chars().nth(i).unwrap();
        if opened_list_count == 0 && element == '[' {
            opened_list_count += 1;
        } else if opened_list_count == 1 {
            if element == ']' {
                let mut to_add = "[".to_owned();
                to_add.push_str(&*buffer.clone());
                to_add.push(']');
                result.push(to_add);
                buffer.clear();
                opened_list_count = 0;
            } else if element == '[' {
                opened_list_count += 1;
                buffer.push(element);
            } else {
                buffer.push(element);
            }
        } else if opened_list_count > 0 {
            if element == ']' {
                opened_list_count -= 1;
            } else if element == '[' {
                opened_list_count += 1;
            }
            buffer.push(element);
        } else {
            if element == ',' {
                // result.push(buffer.clone());
                // buffer.clear();
            } else {
                buffer.push(element);
                if input_inside_list.chars().nth(i + 1).is_none() || (input_inside_list.chars().nth(i + 1).is_some() && !input_inside_list.chars().nth(i + 1).unwrap().is_digit(10)) {
                    result.push(buffer.clone());
                    buffer.clear();
                }
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::advent13::{part_one, compare, Pair, split, part_two};

    #[test]
    fn ok_example_1() {
        assert_eq!(part_one("./src/advent13/input_example.txt"), 13);
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(part_two("./src/advent13/input_example.txt"), 140);
    }

    #[test]
    fn compare_ok() {
        assert_eq!(compare(Pair::new_from_str("[1,1,3,1,1]", "[1,1,5,1,1]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[1],[2,3,4]]", "[[1],4]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[1],[2,3,4]]", "[[1],4]")), 1);
        assert_eq!(compare(Pair::new_from_str("[9]", "[[8,7,6]]")), -1);
        assert_eq!(compare(Pair::new_from_str("[[4,4],4,4]", "[[4,4],4,4,4]")), 1);
        assert_eq!(compare(Pair::new_from_str("[7,7,7,7]", "[7,7,7]")), -1);
        assert_eq!(compare(Pair::new_from_str("[]", "[3]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[[]]]", "[[]]")), -1);
        assert_eq!(compare(Pair::new_from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]")), -1);

        assert_eq!(compare(Pair::new_from_str("[3,2]", "[1,5]")), -1);
        assert_eq!(compare(Pair::new_from_str("[7,7]", "[7,7,7]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[1,2]]", "[[3,4]]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[]]", "[]")), -1);
        assert_eq!(compare(Pair::new_from_str("[3]", "[]")), -1);
        assert_eq!(compare(Pair::new_from_str("[[5]]", "[[7],[2,[6,[2,4,5,6],2,9,[4]],7,6],[[]],[7,[[10,10,3,7],9,[9],8],5,[],10],[]]")), 1);

        assert_eq!(compare(Pair::new_from_str("[1,1,5,1,1]", "[1,1,3,1,1]")), -1);
        assert_eq!(compare(Pair::new_from_str("[1,[2,[3,[4,[5,6,0]]]],8,9]", "[1,[2,[3,[4,[5,6,7]]]],8,9]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[4,4],4,4,4]", "[[4,4],4,4]")), -1);
        assert_eq!(compare(Pair::new_from_str("[7,7,7]", "[7,7,7,7]")), 1);


        assert_eq!(compare(Pair::new_from_str("[[1], [1,2]]", "[[1,2],[1]]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[6,0,[3,1]]]", "[[6,0,[3,1]]]")), 0);
        assert_eq!(compare(Pair::new_from_str("[1,1]", "[1,1]")), 0);
        assert_eq!(compare(Pair::new_from_str("[[],[[[2,5],[3,3],9]],[]]", "[[6,10],[]]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[[[7,4],5,[6,6,0]],[1,[8,1,6],9],3,[[0,10],[0,3,8],[7,3,5],9],[]],[9,9,[[1,7,10,0,0],[2,1,1,6,4],[7,1],[],[7,7,5,9,5]]]]", "[]")), -1);
        assert_eq!(compare(Pair::new_from_str("[8,8,4,6]", "[8,8,4,6,10]")), 1);
        //
        //
        // // logan
        assert_eq!(compare(Pair::new_from_str("[[0,[[9,6,4,5]],[]]]", "[[10,9,[[5,7],4,[5,8,0,9,8],[0,10],[3,1]],4]]")), 1);
        assert_eq!(compare(Pair::new_from_str("[[10,[],[9,8,6,[2,7,0,8]],[],[[4,6,2,1,4],6,5]]]", "[[4,5,1,[[0,0],[6,10],4,5],10],[[2,4,6,1,[6,8]],[[2,0,10]],[[5,10],[6,4,5,6],[8,9,2]]],[[[],10,0,7,7],[[9,9,7,3,8]]],[[[7],3,8],[6,3],1,5]]")), -1);
    }

    #[test]
    fn split_ok() {
        // assert_eq!(split("[1,2]".to_string()), vec!["1", "2"]);
        // assert_eq!(split("[[1,2],3]".to_string()), vec!["[1,2]", "3"]);
        // assert_eq!(split("[1,[2],3]".to_string()), vec!["1", "[2]", "3"]);
        // assert_eq!(split("[[1,[2],3],4]".to_string()), vec!["[1,[2],3]", "4"]);
        // assert_eq!(split("[[4,4],4,4,4]".to_string()), vec!["[4,4]", "4", "4", "4"]);
        assert_eq!(split("[10,20]".to_string()), vec!["10", "20"]);
    }
}