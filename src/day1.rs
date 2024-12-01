use std::collections::HashMap;

use crate::helpers;

pub fn day1() {
    let file_string = helpers::file_to_string("src/inputs/day1P2.txt").unwrap();

    let mut left_list: Vec<String> = Vec::new();
    let mut right_list: Vec<String> = Vec::new();
    for line in file_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_list.push(parts[0].to_string());
            right_list.push(parts[1].to_string());
        }
    }

    part1(left_list.clone(), right_list.clone());
    part2(left_list, right_list);
}
fn part1(mut left_list: Vec<String>, mut right_list: Vec<String>) {
    left_list.sort();
    right_list.sort();

    let ans = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(x, y)| (x.parse::<i32>().unwrap() - y.parse::<i32>().unwrap()).abs())
        .sum::<i32>();
    println!("ans1 :{:?}", ans);
}

fn part2(mut left_list: Vec<String>, mut right_list: Vec<String>) {
    left_list.sort();
    right_list.sort();

    let mut map_right_list = HashMap::new();
    for value in right_list {
        *map_right_list.entry(value.to_string()).or_insert(0) += 1;
    }

    let mut ans2 = 0;

    for some in left_list.iter() {
        let count = map_right_list.get(some);

        if count.is_some() {
            ans2 += some.parse::<i32>().unwrap() * count.unwrap();
        }
    }

    println!("ans2: {:?}", ans2);
}
