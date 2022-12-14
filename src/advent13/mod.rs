use std::fs::File;
use std::io::{BufReader, prelude::*};

use indextree::Arena;

fn advent13(file_path: &str, is_part_one: bool) -> u32 {
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
            if compare(
                Pair::new(previous_line.clone(), line.clone())
            ) {
                count += line_index;
            }
            line_index += 1;
        } else {
            previous_line = line;
        }
    }
    return count;
}

struct Pair {
    left: String,
    right: String,
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

fn compare(pair: Pair) -> bool {
    println!("Compare {} vs {}", pair.left.clone(), pair.right.clone());
    let left_elements: Vec<String> = split(pair.left.clone());
    let right_elements: Vec<String> = split(pair.right.clone());
    let highest = if right_elements.len() > left_elements.len() { right_elements.len() } else { left_elements.len() };
    for i in 0..highest {
        let right_opt = right_elements.get(i);
        let left_opt = left_elements.get(i);
        println!("    Compare {} vs {}", left_opt.unwrap_or(&"".to_string()), right_opt.unwrap_or(&"".to_string()));

        if right_opt.is_none() {
            return false;
        }
        if left_opt.is_none() {
            return true;
        }
        let right = right_opt.unwrap();
        let left = left_elements.get(i).unwrap();

        if left.starts_with("[") && right.starts_with("[") {
            // [[4,4]   ,4    ,4   ,4   ]
            return compare(Pair::new(
                left.clone(),
                right.clone(),
            ));
        } else if !left.starts_with("[") && right.starts_with("[") {
            return compare(Pair::new(
                format!("[{}]", left.clone()),
                right.clone(),
            ));
        } else if left.starts_with("[") && !right.starts_with("[") {
            return compare(Pair::new(
                left.clone(),
                format!("[{}]", right.clone()),
            ));
        } else {}
        if is_left_higher_than_right(left, right) {
            return false;
        }
        if is_left_lower_than_right(left, right) {
            return true;
        }
    }
    return true;
}

fn is_left_higher_than_right(left_element: &String, right_element: &String) -> bool {
    left_element.parse::<u32>().unwrap() > right_element.parse::<u32>().unwrap()
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
    for element in input_inside_list.chars() {
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
            if element != ',' {
                result.push(element.to_string())
            }
        }
    }

    return result;
}

#[cfg(test)]
mod tests {
    use crate::advent13::{advent13, compare, Pair, split};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent13("./src/advent13/input_example.txt", true), 13);
    }


    #[test]
    fn compare_ok() {
        assert_eq!(compare(Pair::new_from_str("[1,1,3,1,1]", "[1,1,5,1,1]")), true);
        assert_eq!(compare(Pair::new_from_str("[[1],[2,3,4]]", "[[1],4]")), true);

        assert_eq!(compare(Pair::new_from_str("[3,2]", "[1,5]")), false);
        assert_eq!(compare(Pair::new_from_str("[7,7,7,7]", "[7,7,7]")), false);
        assert_eq!(compare(Pair::new_from_str("[7,7]", "[7,7,7]")), true);
        assert_eq!(compare(Pair::new_from_str("[[1,2]]", "[[3,4]]")), true);
        assert_eq!(compare(Pair::new_from_str("[9]", "[[8,7,6]]")), false);
        assert_eq!(compare(Pair::new_from_str("[[4,4],4,4]", "[[4,4],4,4,4]")), true);
        assert_eq!(compare(Pair::new_from_str("[]", "[3]")), true);
        assert_eq!(compare(Pair::new_from_str("[[[]]]", "[[]]")), false);
        assert_eq!(compare(Pair::new_from_str("[[]]", "[]")), false);
        assert_eq!(compare(Pair::new_from_str("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]")), false);
        assert_eq!(compare(Pair::new_from_str("[3]", "[]")), false);
        assert_eq!(compare(Pair::new_from_str("[[5]]", "[[7],[2,[6,[2,4,5,6],2,9,[4]],7,6],[[]],[7,[[10,10,3,7],9,[9],8],5,[],10],[]]")), true);
        // assert_eq!(compare(Pair::new_from_str("[[1], [1,2]]", "[[1,2],[1]]")), false);
    }

    #[test]
    fn split_ok() {
        assert_eq!(split("[1,2]".to_string()), vec!["1", "2"]);
        assert_eq!(split("[1,2]".to_string()), vec!["1", "2"]);
        assert_eq!(split("[[1,2],3]".to_string()), vec!["[1,2]", "3"]);
        assert_eq!(split("[1,[2],3]".to_string()), vec!["1", "[2]", "3"]);
        assert_eq!(split("[[1,[2],3],4]".to_string()), vec!["[1,[2],3]", "4"]);
        assert_eq!(split("[[4,4],4,4,4]".to_string()), vec!["[4,4]", "4", "4", "4"]);
    }
}