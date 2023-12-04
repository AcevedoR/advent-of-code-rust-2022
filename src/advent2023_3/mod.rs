use std::cmp::{max, min};
use std::fs::File;
use std::io::{BufReader, prelude::*};

fn advent2023_3(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut numbers_to_check = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut line_number = 0;

    for line in reader.lines() {
        let mut line_in_grid: Vec<char> = Vec::new();
        let mut temp = line.unwrap().parse::<String>().unwrap();
        temp.push('.');
        let mut number_beginning = 0;
        let mut number_ending;
        let mut in_a_number = false;
        let mut number: Vec<char> = Vec::new();

        for i in 0..temp.len() {
            let current = temp.chars().nth(i).unwrap();
            line_in_grid.push(current);

            if let Some(number_part) = current.to_digit(10) {
                if !in_a_number {
                    number_beginning = i;
                }
                number.push(char::from_digit(number_part, 10).unwrap());
                in_a_number = true;
            } else {
                if in_a_number {
                    // ending number
                    in_a_number = false;
                    number_ending = i - 1;
                    numbers_to_check.push(Number {
                        number_beginning,
                        number_ending,
                        line: line_number,
                        value: number.iter().map(|x| x.to_string())
                            .reduce(|a, b| format!("{}{}", a, b)).unwrap().parse::<u32>().unwrap(),
                    });
                    number.clear();
                }
            }
        }
        grid.push(line_in_grid);
        line_number = line_number + 1;
    }
    for lines in grid.clone() {
        for charr in lines {
            print!("{}", charr);
        }
        println!();
    }
    for number in numbers_to_check {
        if is_eligible(&number, grid.clone()) {
            println!("number {} is eligible", number.value);
            score = score + number.value;
        }
    }

    return score as i32;
}

fn is_eligible(number: &Number, grid: Vec<Vec<char>>) -> bool {
    let grid_width = grid.get(0).unwrap().len();

    if number.number_beginning > 0 && is_a_symbol(grid.get(number.line).unwrap().get(number.number_beginning - 1).unwrap()) {
        return true;
    }
    if number.number_ending < grid_width && is_a_symbol(grid.get(number.line).unwrap().get(number.number_ending + 1).unwrap()) {
        return true;
    }
    if number.line > 0 {
        let y = number.line - 1;
        for x in max(0, number.number_beginning as i32 - 1) as usize..min(grid_width, number.number_ending + 2) {
            if is_a_symbol(grid.get(y).unwrap().get(x).unwrap()) {
                return true;
            }
        }
    }
    if number.line < grid.len() - 1 {
        let y = number.line + 1;
        for x in max(0, number.number_beginning as i32 - 1) as usize..min(grid_width, number.number_ending + 2) {
            if is_a_symbol(grid.get(y).unwrap().get(x).unwrap()) {
                return true;
            }
        }
    }

    return false;
}

fn is_a_symbol(c: &char) -> bool {
    !c.is_digit(10) && c != &'.'
}


fn advent2023_3_2(file_path: &str) -> i32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut numbers_to_check = Vec::new();
    let mut engines = Vec::new();
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut line_number = 0;

    for line in reader.lines() {
        let mut line_in_grid: Vec<char> = Vec::new();
        let mut temp = line.unwrap().parse::<String>().unwrap();
        temp.push('.');
        let mut number_beginning = 0;
        let mut number_ending;
        let mut in_a_number = false;
        let mut number: Vec<char> = Vec::new();

        for i in 0..temp.len() {
            let current = temp.chars().nth(i).unwrap();
            line_in_grid.push(current);

            if current == '*' {
                engines.push(Engine { x: i, y: line_number });
            }

            if let Some(number_part) = current.to_digit(10) {
                if !in_a_number {
                    number_beginning = i;
                }
                number.push(char::from_digit(number_part, 10).unwrap());
                in_a_number = true;
            } else {
                if in_a_number {
                    // ending number
                    in_a_number = false;
                    number_ending = i - 1;
                    numbers_to_check.push(Number {
                        number_beginning,
                        number_ending,
                        line: line_number,
                        value: number.iter().map(|x| x.to_string())
                            .reduce(|a, b| format!("{}{}", a, b)).unwrap().parse::<u32>().unwrap(),
                    });
                    number.clear();
                }
            }
        }
        grid.push(line_in_grid);
        line_number = line_number + 1;
    }
    for lines in grid.clone() {
        for charr in lines {
            print!("{}", charr);
        }
        println!();
    }
    for engine in engines {
        score = score + gear_ratio(engine, &numbers_to_check);
    }

    return score as i32;
}

fn gear_ratio(engine: Engine, numbers: &Vec<Number>) -> u32 {
    let mut matching_numbers = Vec::new();
    for number in numbers {
        for number_x in number.number_beginning..number.number_ending + 1 {
            if engine.y - 1 <= number.line && number.line <= engine.y + 1
                && engine.x - 1 <= number_x && number_x <= engine.x + 1 {
                matching_numbers.push(number);
                break;
            }
        }
    }
    if matching_numbers.len() == 2 {
        println!("gears = {:?} {:?}", matching_numbers.get(0).unwrap(), matching_numbers.get(1).unwrap());
        return matching_numbers.get(0).unwrap().value*matching_numbers.get(1).unwrap().value;
    }
    return 0;
}

struct Engine {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Number {
    number_beginning: usize,
    number_ending: usize,
    line: usize,
    value: u32,
}

#[cfg(test)]
mod tests {
    use crate::advent2023_3::{advent2023_3, advent2023_3_2};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_3("./src/advent2023_3/input_example.txt"), 4361);
    }

    #[test]
    fn ok_part_2_example_1() {
        assert_eq!(advent2023_3_2("./src/advent2023_3/input.txt"), 467835);
    }
}