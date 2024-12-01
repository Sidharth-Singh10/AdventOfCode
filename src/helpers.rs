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
