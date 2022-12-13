use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Monkey {
    inspections: usize,
    items: VecDeque<u128>,
    operation: fn(i: u128) -> u128,
    test: fn(i: u128) -> usize,
}

fn problem_one(mut monkeys: Vec<Monkey>, iterations: usize, divisor: impl Fn(u128) -> u128) {
    for _ in 0..iterations {
        for j in 0..monkeys.len() {
            while let Some(item) = monkeys[j].items.pop_front() {
                let new_worry = divisor((monkeys[j].operation)(item));
                let target_monkey = (monkeys[j].test)(new_worry);
                monkeys[target_monkey].items.push_back(new_worry);
                monkeys[j].inspections += 1;
            }
        }
    }
    for m in &monkeys {
        println!("Monkey debug: {:?}", m);
    }
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect::<_>();
    inspections.sort();
    println!("{:?}", inspections);
    inspections.reverse();
    let monkey_business = inspections[..2].iter().fold(1, |a, b| a * b);
    println!("Monkey business: {0}", monkey_business);
}

pub fn run() -> Option<()> {
    let monkeys = vec![
        Monkey {
            inspections: 0,
            items: VecDeque::from([72, 97]),
            operation: |x| x * 13,
            test: |x| if x % 19 == 0 { 5 } else { 6 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([55, 70, 90, 74, 95]),
            operation: |x| x * x,
            test: |x| if x % 7 == 0 { 5 } else { 0 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([74, 97, 66, 57]),
            operation: |x| x + 6,
            test: |x| if x % 17 == 0 { 1 } else { 0 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([86, 54, 53]),
            operation: |x| x + 2,
            test: |x| if x % 13 == 0 { 1 } else { 2 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([50, 65, 78, 50, 62, 99]),
            operation: |x| x + 3,
            test: |x| if x % 11 == 0 { 3 } else { 7 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([90]),
            operation: |x| x + 4,
            test: |x| if x % 2 == 0 { 4 } else { 6 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([88, 92, 63, 94, 96, 82, 53, 53]),
            operation: |x| x + 8,
            test: |x| if x % 5 == 0 { 4 } else { 7 },
        },
        Monkey {
            inspections: 0,
            items: VecDeque::from([70, 60, 71, 69, 77, 70, 98]),
            operation: |x| x * 7,
            test: |x| if x % 3 == 0 { 2 } else { 3 },
        },
    ];
    let m2 = monkeys.clone();
    problem_one(monkeys, 20, |x| x / 3);
    let divisors = vec![3, 5, 2, 11, 13, 17, 7, 19];
    let lcm = divisors.iter().product::<u128>();
    let f = |x| x % lcm;
    problem_one(m2, 10000, f);
    None
}
