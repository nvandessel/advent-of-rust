mod year2023;

use year2023::day01::Day1;

fn main() {
    let day1 = Day1::new();

    println!("{}", day1.solve_part1());
    println!("{}", day1.solve_part2())
}
