use problem_2::run as run2;
use problem_3::run as run3;
use problem_4::run as run4;
use problem_5::run as run5;
use problem_6::run as run6;

mod problem_2 {
    pub mod run;
}
mod problem_3 {
    pub mod run;
}
mod problem_4 {
    pub mod run;
}
mod problem_5 {
    pub mod run;
}
mod problem_6 {
    pub mod run;
}

fn help(name: String) {
    println!("./{0} <problem_number>", name);
}

fn main() {
    let program = std::env::args()
        .next()
        .expect("No program name passed - this is strange.");
    let problem = std::env::args()
        .nth(1)
        .expect("No problem number was given");
    match problem.as_str() {
        "2" => run2::run(),
        "3" => run3::run(),
        "4" => run4::run(),
        "5" => run5::run(),
        "6" => run6::run(),
        _ => {
            help(program);
            None
        }
    };
}
