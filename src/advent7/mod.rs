use std::fs::File;
use std::io::{BufReader, prelude::*};

use indextree::{Arena, NodeId};

fn advent7(file_path: &str, is_part_one: bool) -> u32 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let lines = reader.lines();

    let mut total_used_space = 0;
    let mut file_system_arena = Arena::new();
    // see https://dev.to/deciduously/no-more-tears-no-more-knots-arena-allocated-trees-in-rust-44k6

    let mut current_file = file_system_arena.new_node(
        Filee {
            name: "/".parse().unwrap(),
            size: 0,
        }
    );
    let file_system_root = current_file;

    for line in lines.skip(1) {
        println!("{:?}", line);

        let line_elements: Vec<String> = line.unwrap().split(" ").map(|s| s.to_string()).collect();
        if line_elements[0] == "$" {
            if line_elements[1] == "cd" {
                current_file = cd(&mut file_system_arena, &mut current_file, &line_elements);
            }
        } else {
            let size = if line_elements[0] == "dir" {
                0
            } else {
                line_elements[0].parse().unwrap()
            };
            let new_child = file_system_arena.new_node(Filee {
                name: line_elements[1].to_string(),
                size,
            });
            current_file.append(new_child, &mut file_system_arena);
            total_used_space += size;
        }
    }

    let res = sum_size(&file_system_arena, file_system_root, total_used_space);
    println!("res: {:?}", res);
    return if is_part_one {
        res.1
    } else {
        res.2
    }
}

fn cd(file_system_arena: &mut Arena<Filee>, current_file: &mut NodeId, line_elements: &Vec<String>) -> NodeId {
    return if line_elements[2] == ".." {
        current_file.ancestors(&file_system_arena).skip(1).next().unwrap()
    } else {
        current_file.children(&file_system_arena)
            .find(|&r| file_system_arena.get(r).unwrap().get().name == line_elements[2]).unwrap()
    };
}


fn sum_size(file_system_arena: &Arena<Filee>, file: NodeId, total_used_space : u32) -> (u32, u32, u32) {
    let actual_file = file_system_arena.get(file).unwrap().get();
    let mut sum = actual_file.size;
    let mut global_sum = 0;
    let mut lowest_big_dir = 70000000;

    for child in file.children(file_system_arena) {
        let (temp_sum, temp_global_sum, temp_lowest_big_dir) = sum_size(file_system_arena, child, total_used_space);
        sum += temp_sum;
        global_sum += temp_global_sum;
        if temp_lowest_big_dir < lowest_big_dir {
            lowest_big_dir = temp_lowest_big_dir;
        }
    }
    println!("{}    {}    sum: {}", actual_file.name, actual_file.size, sum);

    if sum < 100000 {
        if actual_file.size == 0 {
            global_sum += sum;
        }
    }
    if actual_file.size == 0 {
        let free_space = 70000000 - total_used_space;
        if (30000000 <= free_space + sum) && sum < lowest_big_dir {
            lowest_big_dir = sum;
        }
    }

    return (sum, global_sum, lowest_big_dir);
}

struct Filee {
    name: String,
    size: u32,
}

#[cfg(test)]
mod tests {
    use crate::advent7::advent7;

    #[test]
    fn ok_example_1() {
        assert_eq!(advent7("./src/advent7/input_example.txt", true), 95437);
    }

    #[test]
    fn ok_part_two() {
        assert_eq!(advent7("./src/advent7/input_example.txt", false), 24933642);
    }
}