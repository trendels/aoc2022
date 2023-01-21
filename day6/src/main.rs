fn get_start_pos(input: &str, wsize: usize) -> Option<usize> {
    'marker: for i in 0..input.len() - wsize - 1 {
        for start in 0..wsize {
            // You can only slice, but not index strings in Rust. As a workaround, I use slices of
            // length 1 to extract characters.
            // A better way would probably be to treat the input as bytes here.
            let s1 = &input[i + start..i + start + 1];
            for end in start + 1..wsize {
                let s2 = &input[i + end..i + end + 1];
                if s1 == s2 {
                    continue 'marker;
                }
            }
        }
        return Some(i + wsize);
    }
    return None;
}

fn main() {
    let input = include_str!("../input.txt");

    let answer = get_start_pos(input, 4).unwrap();
    println!("part 1: {answer}");

    let answer = get_start_pos(input, 14).unwrap();
    println!("part 2: {answer}");
}
