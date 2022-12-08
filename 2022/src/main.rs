use mymacro;

mymacro::make_problem_import!(2);
mymacro::make_problem_import!(3);
mymacro::make_problem_import!(4);
mymacro::make_problem_import!(5);
mymacro::make_problem_import!(6);

use problem_7::run as run7;

mod problem_7 {
    pub mod run;
}

fn help(name: String) {
    println!("./{0} <problem_number>", name);
}

fn main() {
    run7::run();
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
        "7" => run7::run(),
        _ => {
            help(program);
            None
        }
    };
}
