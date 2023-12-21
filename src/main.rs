mod year2015;


fn main() {
    year2015::run_all_days();
}

pub trait Solution{
    fn new() -> Self;
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}
