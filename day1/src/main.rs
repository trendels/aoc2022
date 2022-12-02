use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").expect("Failed to read form input.txt");
    let chunks = content.split("\n\n").collect::<Vec<&str>>();

    let max = chunks.iter().map(
        |c| c.trim().split("\n").map(
            |s| s.parse::<i32>().unwrap()
        ).sum::<i32>()
    ).max().unwrap();

    println!("part1: {}", max);

    let mut totals = chunks.iter().map(
        |c| c.trim().split("\n").map(
            |s| s.parse::<i32>().unwrap()
        ).sum::<i32>()
    ).collect::<Vec<i32>>();

    totals.sort_by(|a, b| b.cmp(a));
    let sum_top3 = totals[..3].iter().sum::<i32>();

    println!("part2: {}", sum_top3);
}
