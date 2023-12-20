use std::fs::File;
use std::{env, io};
use std::io::BufRead;

pub struct Input;

impl Input{
    pub fn read(year: i32, day: i32) -> io::Result<Vec<String>> {
        let cur_dir = match env::current_dir() {
            Ok(dir) => dir,
            Err(err) =>{
                eprintln!("Error getting current working directory: {}", err);
                return Err(err);
            }
        };

        let file_name = format!("inputs/year{year}/day{day}input.txt");
        let file_path = cur_dir.join(file_name);

        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Error opening the file: {}", err);
                return Err(err);
            }
        };

        let reader = io::BufReader::new(file);

        let mut lines = Vec::new();
        for line in reader.lines() {
            lines.push(line?);
        }

        return Ok(lines);
    }
}