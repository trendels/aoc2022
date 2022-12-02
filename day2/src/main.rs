use std::fs;

#[derive(Debug, Clone, Copy)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Debug)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

fn read_choice(s: &str) -> Option<Choice> {
    match s {
        "A" | "X" => Some(Choice::Rock),
        "B" | "Y" => Some(Choice::Paper),
        "C" | "Z" => Some(Choice::Scissors),
        _ => None,
    }
}


fn play(their_choice: &Choice, our_choice: &Choice) -> Outcome {
    match (their_choice, our_choice) {
        (Choice::Rock, Choice::Paper) => Outcome::Win,
        (Choice::Rock, Choice::Scissors) => Outcome::Loss,
        (Choice::Paper, Choice::Rock) => Outcome::Loss,
        (Choice::Paper, Choice::Scissors) => Outcome::Win,
        (Choice::Scissors, Choice::Rock) => Outcome::Win,
        (Choice::Scissors, Choice::Paper) => Outcome::Loss,
        _ => Outcome::Draw
    }
}


fn read_outcome(s: &str) -> Option<Outcome> {
    match s {
        "X" => Some(Outcome::Loss),
        "Y" => Some(Outcome::Draw),
        "Z" => Some(Outcome::Win),
        _ => None,
    }
}

fn get_move(their_choice: &Choice, outcome: &Outcome) -> Choice {
    match (their_choice, outcome) {
        (Choice::Rock, Outcome::Win) => Choice::Paper,
        (Choice::Rock, Outcome::Loss) => Choice::Scissors,
        (Choice::Paper, Outcome::Win) => Choice::Scissors,
        (Choice::Paper, Outcome::Loss) => Choice::Rock,
        (Choice::Scissors, Outcome::Win) => Choice::Rock,
        (Choice::Scissors, Outcome::Loss) => Choice::Paper,
        _ => *their_choice,
    }
}

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read from input.txt");
    let mut score = 0;
    for line in content.lines() {
        let (first, second) = line.split_once(' ').unwrap();
        let their_choice = read_choice(first).unwrap();
        let our_choice = read_choice(second).unwrap();
        let outcome = play(&their_choice, &our_choice);
        score += our_choice as u32 + outcome as u32;
    }
    println!("part 1: {}", score);

    let mut score = 0;
    for line in content.lines() {
        let (first, second) = line.split_once(' ').unwrap();
        let their_choice = read_choice(first).unwrap();
        let outcome = read_outcome(second).unwrap();
        let our_choice = get_move(&their_choice, &outcome);
        score += our_choice as u32 + outcome as u32;
    }
    println!("part 2: {}", score);
}
