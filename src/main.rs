extern crate core;

use std::fs::File;
use std::io::{self, BufReader, prelude::*};

mod advent2;
mod advent3;
mod advent4;
mod challenge;
mod advent5;
mod advent6;
mod advent7;
mod advent8;
mod advent13;
mod advent22;
mod advent2023_1;
mod advent2023_2;
mod advent2023_3;
mod advent2023_4;

fn main() -> io::Result<()> {
    let file = File::open("./src/input.txt")?;
    let reader = BufReader::new(file);
    let mut i: i32 = 0;
    let mut temp: i32 = 0;
    let mut max: [i32; 3] = [0, 0, 0];

    for line in reader.lines() {
        if line.as_ref().unwrap().is_empty() {
            println!("    {:?} empty line at index: {}, max: {:?}, temp: {}", line, i, max, temp);
            check_and_assign_new_max(&mut temp, &mut max);
            temp = 0;
        } else {
            println!("    {:?} line: {}", line, line.as_ref().unwrap());
            let numberr = line?.parse::<i32>().unwrap();

            temp = temp + numberr;
            println!("    number: {}, temp: {}", numberr, temp);
        }
        i = i + 1;
    }
    check_and_assign_new_max(&mut temp, &mut max);
    println!("max: {:?}, sum: {:?}", max, max.into_iter().reduce(|a, b| a + b));

    Ok(())
}

fn check_and_assign_new_max(temp: &mut i32, max_array: &mut [i32; 3]) {
    for i in 0..max_array.len() {
        if temp > &mut max_array[i] {
            println!("found new max: {}", temp);
            let mut previous: i32 = *temp;
            for i2 in i..max_array.len() {
                let temp = max_array[i2];

                max_array[i2] = previous;

                previous = temp;
            }
            max_array.sort_by(|a, b| b.cmp(a));
            return;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::check_and_assign_new_max;

    #[test]
    fn check_and_assign_new_max_ok() {
        let mut new_value = 3;
        let mut array = [5, 2, 1];

        check_and_assign_new_max(&mut new_value, &mut array);

        assert_eq!(array, [5, 3, 2]);
    }

    #[test]
    fn check_and_assign_new_max_donothing_if_lower() {
        let mut new_value = 1;
        let mut array = [5, 3, 2];

        check_and_assign_new_max(&mut new_value, &mut array);

        assert_eq!(array, [5, 3, 2]);
    }
}