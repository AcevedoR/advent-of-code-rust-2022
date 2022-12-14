use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent8(file_path: &str) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();
    let mut forest: Vec<Vec<i8>> = Vec::new();

    let mut line_index: usize = 0;
    for line in lines {
        println!("{:?}", line);
        let tree_alley: Vec<i8> = line.unwrap().chars().map(|s| s.to_digit(10).unwrap() as i8).collect();
        let mut new_alley = Vec::new();
        for tree_height in tree_alley {
            new_alley.push(tree_height);
        }
        if forest.get(line_index).is_none() {
            forest.push(new_alley)
        }
        line_index += 1;
    }
    let visible_trees: Vec<(usize, usize)> = get_visible_trees(&forest);
    return visible_trees.len() as u32;
}

fn get_visible_trees(forest: &Vec<Vec<i8>>) -> Vec<(usize, usize)> {
    let mut visible_trees: Vec<(usize, usize)> = Vec::new();
    let mut previous: &i8 = &-1;
    let forest_lenght = forest.first().unwrap().len();

    for column_index in 0..forest.len() {
        let tree_alley = forest.get(column_index).unwrap();
        for line_index in 0..tree_alley.len() {
            let tree = tree_alley.get(line_index).unwrap();
            if tree > previous {
                previous = tree;
                if visible_trees.iter().all(|x| x != &(column_index, line_index)) {
                    visible_trees.push((column_index, line_index));
                }
            }
        }
        previous = &-1;
    }
    for column_index in 0..forest.len() {
        let tree_alley = forest.get(column_index).unwrap();
        for line_index in (0..tree_alley.len()).rev() {
            let tree = tree_alley.get(line_index).unwrap();
            if tree > previous {
                previous = tree;
                if visible_trees.iter().all(|x| x != &(column_index, line_index)) {
                    visible_trees.push((column_index, line_index));
                }
            }
        }
        previous = &-1;
    }


    for line_index in 0..forest_lenght {
        for column_index in 0..forest.len() {
            let tree = forest.get(column_index).unwrap().get(line_index).unwrap();
            if tree > previous {
                previous = tree;
                if visible_trees.iter().all(|x| x != &(column_index, line_index)) {
                    visible_trees.push((column_index, line_index));
                }
            }
        }
        previous = &-1;
    }
    for line_index in 0..forest_lenght {
        for column_index in (0..forest.len()).rev() {
            let tree = forest.get(column_index).unwrap().get(line_index).unwrap();
            if tree > previous {
                previous = tree;
                if visible_trees.iter().all(|x| x != &(column_index, line_index)) {
                    visible_trees.push((column_index, line_index));
                }
            }
        }
        previous = &-1;
    }
    return visible_trees;
}


#[cfg(test)]
mod tests {
    use crate::advent8::{advent8, get_visible_trees};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent8("./src/advent8/input_example.txt"), 21);
    }

    #[test]
    fn get_visible_trees_for_direction_test_ok_all() {
        let expected: Vec<(usize, usize)> = vec![(0, 0), (0, 1), (0, 2)];
        assert_eq!(get_visible_trees(&vec![vec![1, 3, 1]]), expected);
    }

    #[test]
    fn get_visible_trees_for_direction_test_ok_all_vertical() {
        let expected: Vec<(usize, usize)> = vec![(0, 0), (1, 0), (2, 0)];
        assert_eq!(get_visible_trees(&vec![vec![1], vec![3], vec![1]]), expected);
    }

    #[test]
    fn get_visible_trees_for_direction_test_ok_inside() {
        let expected: Vec<(usize, usize)> = vec![(0, 0), (1, 0), (1, 1), (2, 0), (0, 2), (1, 2), (2, 2), (0, 1), (2, 1)];
        assert_eq!(get_visible_trees(&vec![vec![1, 1, 1], vec![2, 9, 2], vec![3, 3, 3]]), expected);
    }

    #[test]
    fn get_visible_trees_for_direction_test_ok_outside() {
        let expected: Vec<(usize, usize)> = vec![(0, 0), (1, 0), (2, 0), (0, 2), (1, 2), (2, 2), (0, 1), (2, 1)];
        assert_eq!(get_visible_trees(&vec![vec![9, 9, 9], vec![9, 1, 9], vec![9, 9, 9]]), expected);
    }
}