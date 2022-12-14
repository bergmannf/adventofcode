use mymacro;

mymacro::make_problem_import!(2);
mymacro::make_problem_import!(3);
mymacro::make_problem_import!(4);
mymacro::make_problem_import!(5);
mymacro::make_problem_import!(6);
mymacro::make_problem_import!(7);
mymacro::make_problem_import!(8);
mymacro::make_problem_import!(9);
mymacro::make_problem_import!(10);
mymacro::make_problem_import!(11);

pub trait Problem {
    fn load(&self) -> String;

    fn solve_part1(&self);

    fn solve_part2(&self);
}

fn help(name: String) {
    println!("./{0} <problem_number>", name);
}

fn main() {
    let program = std::env::args()
        .next()
        .expect("No program name passed - this is strange.");
    let problem = std::env::args().nth(1);
    match problem.as_deref() {
        Some("2") => run2::run(),
        Some("3") => run3::run(),
        Some("4") => run4::run(),
        Some("5") => run5::run(),
        Some("6") => run6::run(),
        Some("7") => run7::run(),
        Some("8") => run8::run(),
        Some("9") => run9::run(),
        Some("10") => run10::run(),
        Some("11") => run11::run(),
        Some(&_) => {}
        None => {
            run11::run();
            help(program);
        }
    };
}
