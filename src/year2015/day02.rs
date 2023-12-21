use advent_of_rust::input_provider::Input;
use crate::Solution;

use itertools::sorted;

pub struct Day2{
    input: Vec<String>,
}

impl Day2{
    fn extract_dimensions(&self) -> Vec<(usize, usize, usize)>{
        self.input
            .iter()
            .map(|s| {
                let parts: Vec<&str> = s.split('x').collect();

                if parts.len() != 3 {
                    std::process::exit(-1);
                }

                return (
                    parts[0].parse().unwrap(),
                    parts[1].parse().unwrap(),
                    parts[2].parse().unwrap(),
                );
            }).collect()
    }

    fn surface_area(&self, l: usize, w: usize, h:usize) -> usize{
        return 2*l*w + 2*w*h + 2*h*l;
    }

    fn smallest_sides(&self, l: usize, w: usize, h:usize) -> (usize, usize){
        let sides = [l, w, h];
        let mut sorted_sides = sorted(&sides);

        let min1 = *sorted_sides.next().unwrap();
        let min2 = *sorted_sides.next().unwrap();

        (min1, min2)
    }
}

impl Solution for Day2 {
    fn new() -> Self {
        match Input::read(2015, 2) {
            Ok(input) => Day2 { input },
            Err(err) => {
                eprintln!("Error reading input: {}", err);
                std::process::exit(1);
            }
        }
    }

    fn solve_part1(&self) -> String {
        let dimensions = self.extract_dimensions();
        let result: usize = dimensions.iter()
            .map(|(l, w, h)| {
                let (min1, _min2) = self.smallest_sides(*l, *w, *h);
                return self.surface_area(*l, *w, *h) + min1;
            })
            .sum();
        return result.to_string();
    }

    fn solve_part2(&self) -> String{
        let dimensions = self.extract_dimensions();
        let result: usize = dimensions.iter()
            .map(|(l, w, h)| {
                let (min1, min2) = self.smallest_sides(*l, *w, *h);
                let ribbon = min1+min1+min2+min2;
                let bow = l*w*h;
                return ribbon + bow;
            })
            .sum();
        return result.to_string();
    }
}