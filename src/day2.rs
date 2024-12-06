use crate::helpers::{file_to_string, line_to_vec_ints};

fn is_safe_sequence(vec_int: &[i32]) -> bool {
    if vec_int.len() < 2 {
        return true;
    }
    let dec = vec_int[0] < vec_int[1];
    for i in 0..vec_int.len() - 1 {
        let diff = (vec_int[i] - vec_int[i + 1]).abs();
        if (dec && vec_int[i] >= vec_int[i + 1])
            || (!dec && vec_int[i] <= vec_int[i + 1])
            || diff > 3
        {
            return false;
        }
    }
    true
}

pub fn day2() -> Result<(), Box<dyn std::error::Error>> {
    let file_string = file_to_string("src/inputs/day2.txt")?;
    let mut checks = 0;

    for line in file_string.lines() {
        let vec_int = line_to_vec_ints(line)?;

        if is_safe_sequence(&vec_int) {
            checks += 1;
            continue;
        }

        let mut safe = false;
        for i in 0..vec_int.len() {
            let mut temp_vec = vec_int.clone();
            temp_vec.remove(i); // Remove the level at index `i`
            if is_safe_sequence(&temp_vec) {
                safe = true;
                break;
            }
        }

        if safe {
            checks += 1;
        }
    }

    // Output the result
    println!("{}", checks);

    Ok(())
}
