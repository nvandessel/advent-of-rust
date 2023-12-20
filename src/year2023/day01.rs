use advent_of_rust::input_provider::Input;

pub struct Day1{
    input: Vec<String>,
}

impl Day1 {
    pub fn new() -> Self {
        match Input::read(2023, 1) {
            Ok(input) => Day1 { input },
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }

    pub fn solve_part1(&self) -> String {
        return format!("Part 1 : {:?}", self.input)
    }

    pub fn solve_part2(&self) -> String{
        return "Part 2".to_string();
    }
}