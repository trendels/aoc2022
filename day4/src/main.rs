// I refactored out these functions after solving the puzzle, and tried to write
// them without using unwrap(). I learned that you can use the `?` early-return
// operator with Option as well as Result, which is neat.
fn parse_range(s: &str) -> Option<(u32, u32)> {
    let (lo, hi) = s.split_once('-')?;
    let range = (
        // Use ok() to convert from Result (from parse()) to Option (returned by this function).
        lo.parse::<u32>().ok()?,
        hi.parse::<u32>().ok()?,
    );
    Some(range)
}

fn parse_line(line: &str) -> Option<((u32, u32), (u32, u32))> {
    let (s1, s2) = line.split_once(',')?;
    let r1 = parse_range(s1)?;
    let r2 = parse_range(s2)?;
    Some((r1, r2))
}

fn main() {
    let input = include_str!("../input.txt");

    let mut count = 0;
    for line in input.lines() {
        let (r1, r2) = parse_line(line).unwrap();
        if (r1.0 >= r2.0 && r1.1 <= r2.1) || (r2.0 >= r1.0 && r2.1 <= r1.1) {
            count += 1;
        }
    }
    println!("part 1: {count}");

    let mut count = 0;
    for line in input.lines() {
        let (r1, r2) = parse_line(line).unwrap();
        if (r1.0 <= r2.1) && (r1.1 >= r2.0) {
            count += 1;
        }
    }
    println!("part 2: {count}");
}
