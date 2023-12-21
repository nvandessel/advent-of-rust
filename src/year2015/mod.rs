use crate::{Solution, Year};

mod day01;
mod day02;

pub struct Year2015;

impl Year for Year2015 {
    fn get_days(&self) -> Vec<Box<dyn Solution>> {
        vec![
            Box::new(day01::Day1::new()),
            Box::new(day02::Day2::new()),
        ]
    }
}