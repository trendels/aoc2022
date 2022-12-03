use std::collections::HashSet;

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - 38
    } else {
        c as u32 - 96
    }
}

fn main() {
    // I learned this neat trick from looking at other people's AoC entries:
    // You can include a string at compile time.
    let input = include_str!("../input.txt");

    let mut score = 0;
    for line in input.lines() {
        let (front, back) = line.split_at(line.len()/2);
        'line: for c1 in front.chars() {
            for c2 in back.chars() {
                if c1 == c2 {
                    score += priority(c1);
                    break 'line;
                }
            }
        }
    }

    println!("part 1: {score}");

    // Let's use sets for part 2 instead of nested loops.

    let mut score = 0;
    // Before we can use chunks() (a method of slice), we need to
    // turn the lines iterator back into a slice.
    // We need to specify a type for collect(), but instead of `<Vec<&str>>`
    // we can just say `<Vec<_>>`, the inner type will be inferred.
    for group in input.lines().collect::<Vec<_>>().chunks(3) {
        // We can also put the type annotation on the variable.
        // This is equivalent to
        //  let set1 = group[0].chars().collect::<HashSet<char>>();
        let set1: HashSet<char> = group[0].chars().collect();
        let set2: HashSet<char> = group[1].chars().collect();
        let set3: HashSet<char> = group[2].chars().collect();
        // intersection() does not return a new set, but an iterator over references
        // to set items. So we have to use copied() to get copies of the items
        // (turn `&str` back into `str`) and construct an intermediate set from that.
        let tmp: HashSet<char> = set1.intersection(&set2).copied().collect();
        let badge = set3.intersection(&tmp).next().unwrap();
        score += priority(*badge);
    }

    println!("part 2: {score}");
}
