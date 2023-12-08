extern crate itertools;

use std::fs::File;
use std::io::{BufReader, prelude::*};

use indexmap::IndexMap;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct LocationItem {
    destination: u64,
    source: u64,
    range_length: u64,
}

fn advent2023_5(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = u64::MAX;

    let mut lines = reader.lines();

    let mut location_maps: IndexMap<String, Vec<LocationItem>> = IndexMap::new();

    let binding = lines.next().unwrap().unwrap().replace("seeds: ", "");
    let seeds = binding.split(" ").map(|x| x.parse::<u64>().unwrap());
    lines.next();

    let mut next_seed_map_name: String = "".to_string();

    for line_opt in lines {
        let line = line_opt.unwrap();
        let chars = line.chars();
        let first_char_opt = chars.peekable().next();
        if let Some(first_char) = first_char_opt {
            if first_char.is_alphabetic() {
                // header
                let string = line.replace(" map:", "");
                next_seed_map_name = string;
            } else {
                let [destination, source, range_length]: [&str; 3] = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                let item = LocationItem {
                    destination: destination.parse::<u64>().unwrap(),
                    source: source.parse::<u64>().unwrap(),
                    range_length: range_length.parse::<u64>().unwrap(),
                };
                if let Some(m) = location_maps.get_mut(&next_seed_map_name) {
                    m.push(
                        item
                    );
                } else {
                    location_maps.insert(next_seed_map_name.to_string(), Vec::from([item]));
                }
            }
        } else {
            // empty line
        }
    }

    seeds.for_each(|seed| {
        let mut source = seed;

        location_maps.iter().for_each(|location_map| {
            // println!("\r\r{}", location_map.0);
            // for y in location_map.1 {
            //     print!("\t{:?} ", y);
            // }
            for location_item in location_map.1.iter() {
                let new_source = map_source_to_target(&source, &location_item);
                if new_source != source {
                    source = new_source;
                    break;
                }
            };
            // println!("\nmap {}", location_map.0);
            // println!("\tsource {} for seed: {}", source  , seed );
        });
        // println!("location {} for seed: {}", seed, source);
        if source < score {
            score = source;
        }
    });
    return score;
}

fn advent2023_5_2(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = u64::MAX;

    let mut lines = reader.lines();

    let mut location_maps: IndexMap<String, Vec<LocationItem>> = IndexMap::new();

    let binding = lines.next().unwrap().unwrap().replace("seeds: ", "");
    let vec = binding.split(" ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<_>>();
    let seeds_with_range = vec.chunks(2); // two by two
    let mut seeds = Vec::with_capacity(1_000_000);
    for seed_with_range in seeds_with_range {
        let seed = seed_with_range.get(0).unwrap();
        let range = seed_with_range.get(1).unwrap();
        for i in 0..*(range) {
            seeds.push(seed + i);
        }
    };
    lines.next();

    let mut next_seed_map_name: String = "".to_string();

    for line_opt in lines {
        let line = line_opt.unwrap();
        let chars = line.chars();
        let first_char_opt = chars.peekable().next();
        if let Some(first_char) = first_char_opt {
            if first_char.is_alphabetic() {
                // header
                let string = line.replace(" map:", "");
                next_seed_map_name = string;
            } else {
                let [destination, source, range_length]: [&str; 3] = line
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .try_into()
                    .unwrap();
                let item = LocationItem {
                    destination: destination.parse::<u64>().unwrap(),
                    source: source.parse::<u64>().unwrap(),
                    range_length: range_length.parse::<u64>().unwrap(),
                };
                if let Some(m) = location_maps.get_mut(&next_seed_map_name) {
                    m.push(
                        item
                    );
                } else {
                    location_maps.insert(next_seed_map_name.to_string(), Vec::from([item]));
                }
            }
        } else {
            // empty line
        }
    }
    // let mut xxx = 0;
    for seed in seeds {
        // if xxx > 1_000{
        //     break;
        // }
        // xxx +=1;
        let mut source = seed;

        location_maps.iter().for_each(|location_map| {
            // println!("\r\r{}", location_map.0);
            // for y in location_map.1 {
            //     print!("\t{:?} ", y);
            // }
            do_score(&mut source, location_map);
            // println!("\nmap {}", location_map.0);
            // println!("\tsource {} for seed: {}", source  , seed );
        });
        // println!("location {} for seed: {}", seed, source);
        if source < score {
            score = source;
        }
    };
    return score;
}

fn do_score(source: &mut u64, location_map: (&String, &Vec<LocationItem>)) {
    for location_item in location_map.1.iter() {
        let new_source = map_source_to_target(&source, &location_item);
        if new_source != *source {
            *source = new_source;
            break;
        }
    };
}

fn is_source_in_location_range(source: &u64, location_item: &LocationItem) -> bool {
    &location_item.source <= source && source < &(location_item.source + location_item.range_length)
}

fn map_source_to_target(source: &u64, location_item: &LocationItem) -> u64 {
    return if is_source_in_location_range(source, location_item) { location_item.destination + (source - location_item.source) } else { source.clone() };
}

#[cfg(test)]
mod tests {
    use crate::advent2023_5::{advent2023_5, advent2023_5_2, is_source_in_location_range, LocationItem, map_source_to_target};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_5("./src/advent2023_5/input_example.txt"), 35);
    }

    #[test]
    fn ok_example_1_part_two() {
        assert_eq!(advent2023_5_2("./src/advent2023_5/input.txt"), 46);
    }


    #[test]
    fn is_source_in_location_range_ok() {
        let map = LocationItem { destination: 52, source: 50, range_length: 48 };
        assert_eq!(is_source_in_location_range(&49, &map), false);
        assert_eq!(is_source_in_location_range(&50, &map), true);
        assert_eq!(is_source_in_location_range(&97, &map), true);
        assert_eq!(is_source_in_location_range(&98, &map), false);
    }

    #[test]
    fn map_source_to_target_ok() {
        let map = LocationItem { destination: 52, source: 50, range_length: 48 };
        assert_eq!(map_source_to_target(&50, &map), 52);
        assert_eq!(map_source_to_target(&53, &map), 55);
        assert_eq!(map_source_to_target(&10, &map), 10);

        let map2 = LocationItem { destination: 49, source: 53, range_length: 8 };
        assert_eq!(map_source_to_target(&53, &map2), 49);
    }
}