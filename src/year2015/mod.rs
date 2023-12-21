use crate::Solution;

pub mod day01;
pub mod day02;

fn run(s: impl Solution){
    println!("{}",  s.solve_part1());
    println!("{}",  s.solve_part2());
}

pub fn run_all_days(){
    let day1 = day01::Day1::new();
    let day2 = day02::Day2::new();

    run(day1);
    run(day2);
}

