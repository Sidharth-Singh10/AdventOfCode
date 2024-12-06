// Learned Regex

use crate::helpers::file_to_string;
use regex::Regex;

pub fn day3() -> Result<(), Box<dyn std::error::Error>> {
    // Finding mul(X,Y)
    let mut pattern = r"mul\([0-9]{1,3},[0-9]{1,3}\)";

    let file_string = file_to_string("src/inputs/day3.txt")?;

    let mut re = Regex::new(pattern).expect("Invalid regex");

    let vec_string: Vec<String> = re
        .find_iter(file_string.as_str())
        .map(|mat| (mat.as_str().to_string()))
        .collect();

    // Storing the summation of mul(X,Y)
    let mut sum = 0;

    for mul in vec_string {
        pattern = r"[0-9]{1,3},[0-9]{1,3}";
        re = Regex::new(pattern).expect("Invalid regex");

        let digits_str = re
            .find(mul.as_str())
            .map(|mat| mat.as_str().to_string())
            .unwrap();

        let digits: Vec<&str> = digits_str.split(',').collect();

        sum += digits[0].parse::<i32>()? * digits[1].parse::<i32>()?;
    }

    println!("part1ans: {}", sum);

    part2()?;

    Ok(())
}

fn part2() -> Result<(), Box<dyn std::error::Error>> {
    // Finding mul(X,Y) , do() and don't()
    let mut pattern = r"((mul\([0-9]{1,3},[0-9]{1,3}\))|(do\(\))|(don't\(\)))";

    let file_string = file_to_string("src/inputs/day3.txt")?;

    let mut re = Regex::new(pattern).expect("Invalid regex");

    let vec_string: Vec<String> = re
        .find_iter(file_string.as_str())
        .map(|mat| (mat.as_str().to_string()))
        .collect();

    // A Vector to store valid mul(X,Y)
    let mut final_vec: Vec<String> = Vec::new();

    let mut take = true;

    for item in vec_string {
        if item == "do()" {
            take = true;
        } else if item == "don't()" {
            take = false;
        } else {
            if take {
                final_vec.push(item);
            }
        }
    }

    // Storing the summation of mul(X,Y)
    let mut sum = 0;

    for mul in final_vec {
        pattern = r"[0-9]{1,3},[0-9]{1,3}";
        re = Regex::new(pattern).expect("Invalid regex");

        let digits_str = re
            .find(mul.as_str())
            .map(|mat| mat.as_str().to_string())
            .unwrap();

        let digits: Vec<&str> = digits_str.split(',').collect();

        sum += digits[0].parse::<i32>()? * digits[1].parse::<i32>()?;
    }
    println!("part2ans: {}", sum);

    Ok(())
}
