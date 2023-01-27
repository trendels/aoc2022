// For this day I used the external 'regex' crate to parse the input using regular expressions,
// instead of only relying on split/strip_prefix etc.
// Coming from Python, having to anticipate how large your numbers can get during calculations and
// sizing your integer variables appropriately (u32, u64 etc.) was new to me.
// Also, I was using `as u32` etc. for doing conversions between differnet sizes before, which is
// dangerous, because when converting to a smaller size values will silently overflow. In general,
// it is better to use the `::from` function from the `From` trait, which I started doing here. It
// is only implemented for save conversions, or `TryFrom` if a conversion can fail.

use regex::Regex;

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(u32),
    Square,
}

#[derive(Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    divisor: u32,
    target: (usize, usize),
    inspected: u32,
}

fn read_input(input: &str) -> Option<Vec<Monkey>> {
    let mut monkeys = vec![];
    let start_re = Regex::new(r"Starting items: (\d+(?:, \d+)*)").unwrap();
    let op_re = Regex::new(r"Operation: new = old (.) (old|\d+)").unwrap();
    let test_re = Regex::new(r"Test: divisible by (\d+)").unwrap();
    let branch_re = Regex::new(r"If (true|false): throw to monkey (\d+)").unwrap();
    for chunk in input.split("\n\n") {
        let mut monkey = Monkey {
            items: vec![],
            operation: Operation::Add(0),
            divisor: 0,
            target: (0, 0),
            inspected: 0,
        };
        for line in chunk.lines() {
            if let Some(c) = start_re.captures(line) {
                monkey.items = c[1]
                    .split(',')
                    .map(|s| s.trim().parse::<u64>().unwrap())
                    .collect();
            }
            if let Some(c) = op_re.captures(line) {
                monkey.operation = match (&c[1], &c[2]) {
                    ("*", "old") => Operation::Square,
                    ("*", value) => Operation::Multiply(value.parse().unwrap()),
                    ("+", value) => Operation::Add(value.parse().unwrap()),
                    _ => return None,
                };
            }
            if let Some(c) = test_re.captures(line) {
                monkey.divisor = c[1].parse().unwrap();
            }
            if let Some(c) = branch_re.captures(line) {
                if &c[1] == "true" {
                    monkey.target.0 = c[2].parse().unwrap();
                } else {
                    monkey.target.1 = c[2].parse().unwrap();
                }
            }
        }
        monkeys.push(monkey);
    }
    Some(monkeys)
}

fn play_round(monkeys: &mut Vec<Monkey>, divide_by_tree: bool) {
    // The trick to solving part two is that `x mod a*b` is divisible by `a` if `x` was divisible
    // by `a`.
    let divisor: u64 = monkeys.iter().map(|m| u64::from(m.divisor)).product();
    for i in 0..monkeys.len() {
        let m = &mut monkeys[i];
        let mut thrown = vec![];
        for i in m.items.drain(..) {
            m.inspected += 1;
            let mut level = match m.operation {
                Operation::Add(value) => i + u64::from(value),
                Operation::Multiply(value) => i * u64::from(value),
                Operation::Square => i * i,
            };
            if divide_by_tree {
                level /= 3;
            } else {
                level %= divisor;
            }
            let target = if level % u64::from(m.divisor) == 0 {
                m.target.0
            } else {
                m.target.1
            };
            thrown.push((target, level));
        }
        for (i, item) in thrown {
            monkeys[i].items.push(item);
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut monkeys = read_input(input).unwrap();
    for _ in 0..20 {
        play_round(&mut monkeys, true);
    }
    let mut counts = monkeys.iter().map(|m| m.inspected).collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    let result = counts[..2].iter().product::<u32>();
    println!("part 1: {result}");

    let mut monkeys = read_input(input).unwrap();
    for _ in 0..10_000 {
        play_round(&mut monkeys, false);
    }
    let mut counts = monkeys
        .iter()
        .map(|m| u64::from(m.inspected))
        .collect::<Vec<_>>();
    counts.sort_by(|a, b| b.cmp(a));
    let result = counts[..2].iter().product::<u64>();
    println!("part 2: {result}");
}
