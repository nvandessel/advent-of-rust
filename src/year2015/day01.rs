use advent_of_rust::input_provider::Input;
use crate::Solution;

pub struct Day1{
    input: Vec<String>,
}

impl Day1{
    pub fn new() -> Self {
        match Input::read(2015, 1) {
            Ok(input) => Day1 { input },
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }

    fn count_chars(&self, as_index: bool) -> i32{
        let mut count = 0;

        for s in &self.input {
            for (index, char) in s.char_indices() {
                match char {
                    c if c == '(' => count += 1,
                    c if c == ')' => count -= 1,
                    _ => {}
                }
                if as_index && count == -1 {
                    return (index + 1) as i32;
                }
            }
        }
        return count;
    }
}

impl Solution for Day1 {

    fn solve_part1(&self) -> String {
        return self.count_chars(false).to_string();
    }

    fn solve_part2(&self) -> String{
        return self.count_chars(true).to_string();
    }
}