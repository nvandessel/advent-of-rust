mod year2015;

pub trait Solution{
    fn solve_part1(&self) -> String;
    fn solve_part2(&self) -> String;
}

pub trait Year{
    fn get_days(&self) -> Vec<Box<dyn Solution>>;
}

fn main() {
    run_all_days(year2015::Year2015);
}

fn run(s: &dyn Solution){
    println!("{}",  s.solve_part1());
    println!("{}",  s.solve_part2());
}

fn run_all_days(year: impl Year){
    let days = year.get_days();
    for day in days {
        run(&*day);
    }
}


