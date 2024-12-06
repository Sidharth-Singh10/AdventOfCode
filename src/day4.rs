use crate::helpers::file_to_string;

const CHECK: &str = "XMAS";

pub fn day4() -> Result<(), Box<dyn std::error::Error>> {
    let file_string = file_to_string("src/inputs/day4.txt")?;

    let mut grid: Vec<Vec<char>> = vec![];
    for line in file_string.lines() {
        let row: Vec<char> = line.chars().collect();
        grid.push(row);
    }

    let mut count = 0;

    let directions = [
        (0, 1),   
        (1, 0),   
        (0, -1),  
        (-1, 0),  
        (1, 1),   
        (1, -1),  
        (-1, 1),  
        (-1, -1),
    ];

    for i in 0..grid.len() {
        for j in 0..grid[i].len() {

            for &(dx, dy) in &directions {
                if let Some(word) = extract_word(&grid, i, j, dx, dy, CHECK.len()) {
                    if word == CHECK {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Total occurrences of '{}': {}", CHECK, count);
    Ok(())
}

// Helper function to extract a word of given length in a specific direction
fn extract_word(
    grid: &Vec<Vec<char>>,
    start_x: usize,
    start_y: usize,
    dx: isize,
    dy: isize,
    length: usize,
) -> Option<String> {
    
    let mut word = String::new();
    let (mut x, mut y) = (start_x as isize, start_y as isize);

    for _ in 0..length {
     
        if x < 0 || y < 0 || x >= grid.len() as isize || y >= grid[0].len() as isize {
            return None;
        }
        word.push(grid[x as usize][y as usize]);
        x += dx;
        y += dy;
    }

    Some(word)
}
