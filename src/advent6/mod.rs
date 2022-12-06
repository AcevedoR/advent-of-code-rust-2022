use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::hash::Hash;
use std::io::{BufReader, prelude::*};

fn advent6_part_one(file_path: &str, is_part_one: bool) -> i32 {
    let marker_size = if is_part_one {
        4
    } else {
        14
    };

    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let line = reader.lines().next().unwrap().unwrap();

    let mut last_chars: VecDeque<char> = VecDeque::new();
    let mut i = 0;
    for character in line.chars() {
        last_chars.push_back(character);
        if last_chars.len() > marker_size {
            last_chars.pop_front();
        }
        println!("{}, {:?}", character, &last_chars);
        if last_chars.len() == marker_size {
            if has_unique_elements(&last_chars) {
                return i + 1;
            }
        }
        i += 1;
    }
    return 0;
}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use crate::advent6::advent6_part_one;

    #[test]
    fn ok_example_1() {
        assert_eq!(advent6_part_one("./src/advent6/input_example.txt", true), 7);
    }

    #[test]
    fn ok_example_part_two() {
        assert_eq!(advent6_part_one("./src/advent6/input_example.txt", false), 19);
    }
}