use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::{BufReader, prelude::*};

use itertools::Itertools;

fn advent2023_7(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 1;
    let mut map: BTreeMap<u64, u32> = BTreeMap::new();

    let lines = reader.lines();
    lines.map(|line| {
        let l = line.unwrap();
        let [hand, bid]: [&str; 2] = l.split(" ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        return (hand.to_string(), bid.to_string());
    }).map(|(hand, bid)| (hand, bid.parse::<u32>().unwrap()))
        .for_each(|(hand, bid)| {
            map.insert(
                calculate_hand_strength(&hand),
                bid,
            );
        });
    // println!("{:?}", map);
    let mut rank = 1;
    for (_hand_strength, bid) in map {
        score += bid * rank;
        rank += 1;
    }
    return score as u64 - 1;
}

fn calculate_hand_strength(hand: &str) -> u64 {
    let mut x = 1;
    let mut res = 0;
    let char_occurrences = get_char_occurrence(hand);
    for occurrence in char_occurrences.iter()
        .map(|(_char, occurrence)| occurrence) {
        if occurrence == &5 {
            res += 60000000000;
            break;
        } else if occurrence == &4 {
            res += 50000000000;
            break;
        } else if occurrence == &3 {
            res += 30000000000;
        } else if occurrence == &2 {
            res += 10000000000;
        }
    }
    for char in hand.chars().rev() {
        let card_value = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("card not matched {}", char)
        };
        res += x * card_value;
        x = x * 100;
    }
    return res;
}

fn get_char_occurrence(s: &str) -> HashMap<char, u8> {
    return s
        .chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });
}


fn advent2023_7_2(file_path: &str) -> u64 {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut score = 1;
    let mut map: BTreeMap<u64, u32> = BTreeMap::new();
    let lines1 = reader.lines();

    let lines = lines1;
    lines.map(|line| {
        let l = line.unwrap();
        let [hand, bid]: [&str; 2] = l.split(" ")
            .collect::<Vec<&str>>()
            .try_into()
            .unwrap();
        return (hand.to_string(), bid.to_string());
    }).map(|(hand, bid)| (hand, bid.parse::<u32>().unwrap()))
        .for_each(|(hand, bid)| {
            map.insert(
                calculate_hand_strength_with_joker(&hand),
                bid,
            );
        });
    println!("{:?}", map);
    let mut rank = 1;
    for (_hand_strength, bid) in map {
        score += bid * rank;
        rank += 1;
    }
    return score as u64 - 1;
}

fn calculate_hand_strength_with_joker(hand: &str) -> u64 {
    let mut x = 1;
    let mut res: u64 = 0;
    let char_occurrences = get_char_occurrence(hand);
    let mut joker_opt = char_occurrences.get(&'J');
    let iter = char_occurrences.iter().sorted_by(|a, b| Ord::cmp(&b.1, &a.1));
    println!("{:?}", iter);
    for (current_char, occurrence) in iter {
        if current_char != &'J' {
            if occurrence == &5 {
                res += 60000000000;
                break;// TODO maybe replace all breaks
            } else if occurrence == &4 {
                if joker_opt.is_some() {
                    res += 60000000000;
                    joker_opt = None;
                } else {
                    res += 50000000000;
                }
                break;
            } else if occurrence == &3 {
                if joker_opt.is_some() {
                    if joker_opt.unwrap() == &1 {
                        res += 50000000000
                    } else if joker_opt.unwrap() == &2 {
                        res += 60000000000
                    } else { panic!() }
                    joker_opt = None;
                } else {
                    res += 30000000000;
                }
            } else if occurrence == &2 {
                // TODO check not sure
                if joker_opt.is_some() {
                    if joker_opt.unwrap() == &1 {
                        res += 30000000000;
                    } else if joker_opt.unwrap() == &2 {
                        res += 50000000000;
                    } else if joker_opt.unwrap() == &3 {
                        res += 60000000000;
                    } else {
                        panic!();
                    }
                    joker_opt = None;
                } else {
                    res += 10000000000;
                }
            } else if joker_opt.is_some() {
                if joker_opt.unwrap() == &4 {
                    res += 60000000000;
                } else if joker_opt.unwrap() == &3 {
                    res += 50000000000;
                } else if joker_opt.unwrap() == &2 {
                    res += 30000000000;
                } else if joker_opt.unwrap() == &1 {
                    res += 10000000000;
                } else { panic!() }
                joker_opt = None;
            }
        } else {
            if occurrence == &5 {
                res += 60000000000;
            }
        }
    }
    for char in hand.chars().rev() {
        let card_value = match char {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 1,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => panic!("card not matched {}", char)
        };
        res += x * card_value;
        x = x * 100;
    }
    return res;
}

// 7 03 02 10 03 13
#[cfg(test)]
mod tests {
    use crate::advent2023_7::{advent2023_7, advent2023_7_2, calculate_hand_strength, calculate_hand_strength_with_joker};

    #[test]
    fn ok_example_1() {
        assert_eq!(advent2023_7("./src/advent2023_7/input_example.txt"), 6440);
    }

    #[test]
    fn calculate_hand_strength_ok() {
        assert_eq!(calculate_hand_strength(&"42T3K"), 0402100313);
        assert_eq!(calculate_hand_strength(&"QQQJA"), 31212121114);
        assert_eq!(calculate_hand_strength(&"KK677"), 21313060707);
    }

    #[test]
    fn ok_example_1_part_two() {
        assert_eq!(advent2023_7_2("./src/advent2023_7/input.txt"), 5905); // not 251873744 or 251899052 too high, and 250917800 too low
    }

    #[test]
    fn calculate_hand_strength_2_ok() {
        assert_eq!(calculate_hand_strength_with_joker(&"42T3K"), 0402100313);
        assert_eq!(calculate_hand_strength_with_joker(&"32455"), 10302040505);
        assert_eq!(calculate_hand_strength_with_joker(&"KK677"), 21313060707);
        assert_eq!(calculate_hand_strength_with_joker(&"32555"), 30302050505);
        assert_eq!(calculate_hand_strength_with_joker(&"22555"), 40202050505);
        assert_eq!(calculate_hand_strength_with_joker(&"25555"), 50205050505);
        assert_eq!(calculate_hand_strength_with_joker(&"55555"), 60505050505);

        //jokers
        assert_eq!(calculate_hand_strength_with_joker(&"KKJQQ"), 41313011212);
        assert_eq!(calculate_hand_strength_with_joker(&"QQQJA"), 51212120114);
        assert_eq!(calculate_hand_strength_with_joker(&"KTJJT"), 51310010110);
        assert_eq!(calculate_hand_strength_with_joker(&"T55J5"), 51005050105);
        assert_eq!(calculate_hand_strength_with_joker(&"QJJQ2"), 51201011202);
        assert_eq!(calculate_hand_strength_with_joker(&"22333"), 40202030303);
        assert_eq!(calculate_hand_strength_with_joker(&"2233J"), 40202030301);
        assert_eq!(calculate_hand_strength_with_joker(&"223JJ"), 50202030101);
        assert_eq!(calculate_hand_strength_with_joker(&"222JJ"), 60202020101);
        assert_eq!(calculate_hand_strength_with_joker(&"32JJJ"), 50302010101);
        assert_eq!(calculate_hand_strength_with_joker(&"22JJJ"), 60202010101);
        assert_eq!(calculate_hand_strength_with_joker(&"2JJJJ"), 60201010101);
        assert_eq!(calculate_hand_strength_with_joker(&"JJJJJ"), 60101010101);
        assert_eq!(calculate_hand_strength_with_joker(&"3245J"), 10302040501);
        assert_eq!(calculate_hand_strength_with_joker(&"22J34"), 30202010304);
        assert_eq!(calculate_hand_strength_with_joker(&"324JJ"), 30302040101);
        assert!(calculate_hand_strength_with_joker("QQQQ2") > calculate_hand_strength_with_joker("JKKK2"));
        assert!(calculate_hand_strength_with_joker("QQQJA") > calculate_hand_strength_with_joker("T55J5"));
        assert!(calculate_hand_strength_with_joker("KTJJT") > calculate_hand_strength_with_joker("QQQJA"));
        assert!(calculate_hand_strength_with_joker("23456") > calculate_hand_strength_with_joker("JJJJJ"));
        assert!(calculate_hand_strength_with_joker("2JJJJ") > calculate_hand_strength_with_joker("JJJJ2"));
        assert!(calculate_hand_strength_with_joker("2234J") > calculate_hand_strength_with_joker("9QK56"));
    }
}