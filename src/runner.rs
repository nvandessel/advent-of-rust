
pub struct Runner;

pub trait Solution{
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub trait Year{
    fn get_days(&self) -> Vec<Box<dyn Solution>>;
}

impl Runner{
    fn run(s: &Box<dyn Solution>){
        println!("{}",  s.solve_part1());
        println!("{}",  s.solve_part2());
    }

    pub fn run_all_days(year: impl Year){
        let days = year.get_days();
        for day in days {
            Runner::run(&day);
        }
    }
}