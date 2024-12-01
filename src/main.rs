use std::collections::HashMap;

mod helpers;

fn main() {

    //day 1//
    //Part 1 //
   let file_string =  helpers::file_to_string("src/inputs/day1P2.txt").unwrap();

   let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in file_string.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() == 2 {
            left_list.push(parts[0].to_string());
            right_list.push(parts[1].to_string());
        }
    }
    // println!("{:?}", left_list);
    // println!("{:?}", right_list);

    left_list.sort();
    right_list.sort();

    let mut ans = 0;

    for i  in 0..left_list.len() {
    
    ans += (left_list[i].parse::<i32>().unwrap() - right_list[i].parse::<i32>().unwrap()).abs(); 

    }
    println!("{:?}", ans);

    //Part 2//

    let mut map_right_list = HashMap::new();
    for value in right_list {
        *map_right_list.entry(value.to_string()).or_insert(0) += 1;
    }

    let mut ans2 = 0;

    for some in left_list.iter() {

        let count  = map_right_list.get(some);

        if count.is_some(){
            ans2 += some.parse::<i32>().unwrap() * count.unwrap();
        }

    }

    println!("ans2: {:?}", ans2);



}
