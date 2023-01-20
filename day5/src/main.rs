type Stack = Vec<char>;

#[derive(Debug)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn read_input(input: &str) -> Option<(Vec<Stack>, Vec<Move>)> {
    let mut stack_lines: Vec<&str> = input.lines().take_while(|l| *l != "").collect();
    let moves_lines: Vec<&str> = input.lines().skip_while(|l| *l != "").collect();

    let n_stacks = stack_lines.pop()?.split_whitespace().count();

    let mut stacks = Vec::new();
    for _ in 0..n_stacks {
        stacks.push(Stack::new());
    }
    for line in stack_lines {
        let chars: Vec<char> = line.chars().collect();
        for s in 0..n_stacks {
            let pos = 1 + s * 4;
            if chars[pos] != ' ' {
                stacks[s].insert(0, chars[pos])
            }
        }
    }

    let mut moves = Vec::new();
    for line in moves_lines {
        if line == "" {
            continue;
        }
        let items: Vec<&str> = line.split_whitespace().collect();
        let m = Move {
            count: items[1].parse().ok()?,
            from: items[3].parse().ok()?,
            to: items[5].parse().ok()?,
        };
        moves.push(m);
    }
    return Some((stacks, moves));
}

fn main() {
    let input = include_str!("../input.txt");

    let (mut stacks, moves) = read_input(input).unwrap();
    for m in moves {
        for _ in 0..m.count {
            let item: char = stacks[m.from - 1].pop().unwrap();
            stacks[m.to - 1].push(item);
        }
    }

    let mut answer = String::new();
    for s in stacks {
        answer.push(*s.last().unwrap());
    }
    println!("part 1: {answer}");

    let (mut stacks, moves) = read_input(input).unwrap();
    for m in moves {
        let mut crates = Vec::new();
        for _ in 0..m.count {
            crates.insert(0, stacks[m.from - 1].pop().unwrap());
        }
        for c in crates {
            stacks[m.to - 1].push(c);
        }
    }

    let mut answer = String::new();
    for s in stacks {
        answer.push(*s.last().unwrap());
    }
    println!("part 2: {answer}");
}
