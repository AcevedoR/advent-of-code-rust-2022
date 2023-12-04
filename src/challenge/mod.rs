use std::collections::HashMap;

pub fn largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut map: HashMap<i32, i32> = HashMap::new();
    let mut max: i32 = 0;
    let mut previous: i32 = 0;
    let mut to_remove_heights: Vec<i32> = Vec::new();

    for height in heights {
        for entry in &map {
            if height < previous && entry.0 < &previous && entry.0 > &height {
                to_remove_heights.push(*entry.0);
                println!("mark to remove {} with value {}", entry.0, entry.1);
                if entry.1 > &max {
                    max = *entry.1;
                }
            }
        }
        for to_remove_height in &to_remove_heights {
            map.remove(&to_remove_height);
        }
        for i in 1..(height + 1) {
            println!("height {}, {}", &i, map.contains_key(&i));

            if map.contains_key(&i) {
                println!("update height {}", &i);

                map.insert(i, map.get(&i).unwrap() + 1);
            } else {
                println!("create height {}", &i);
                map.insert(i, 1);
            }
        }

        println!("{:?}", map);
        println!();
        previous = height;
    }

    return max;
}


#[cfg(test)]
mod tests {
    use crate::challenge::largest_rectangle_area;

    #[test]
    fn ok_example_1() {
        assert_eq!(largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    }

    #[test]
    fn ok_example_2() {
        assert_eq!(largest_rectangle_area(vec![3, 3, 3]), 9);
    }
}