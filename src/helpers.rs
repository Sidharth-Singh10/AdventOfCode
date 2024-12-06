use std::{
    fs::File,
    io::{self, BufReader, Read},
};
pub fn file_to_string(filename: &str) -> Result<String, io::Error> {
    let file = File::open(filename)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn line_to_vec_ints(str_line: &str) -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    str_line
        .split_whitespace()
        .map(|s| s.parse::<i32>())
        .collect::<Result<Vec<i32>, _>>()
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)
}
