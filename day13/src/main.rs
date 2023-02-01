use std::cmp::Ordering;

#[derive(Debug)]
struct Pair {
    left: String,
    right: String,
}

fn read_input(input: &str) -> Option<Vec<Pair>> {
    let mut pairs = vec![];
    for chunk in input.trim().split("\n\n") {
        let lines = chunk.split_once('\n')?;
        pairs.push(Pair {
            left: lines.0.to_string(),
            right: lines.1.to_string(),
        });
    }
    Some(pairs)
}

fn read_input_part2(input: &str) -> Vec<String> {
    input
        .lines()
        .filter(|x| !x.is_empty())
        .map(|x| x.to_string())
        .collect()
}

fn list_items(list: &str) -> Vec<&str> {
    let mut result = vec![];
    let mut start = 0;
    let mut depth = 0;
    for (i, c) in list.chars().enumerate() {
        match c {
            '[' => {
                depth += 1;
                if depth == 1 {
                    start = i + 1
                }
            }
            ']' => {
                if depth == 1 && start < i {
                    result.push(&list[start..i]);
                }
                depth -= 1;
            }
            ',' => {
                if depth == 1 {
                    result.push(&list[start..i]);
                    start = i + 1; // Skip comma
                }
            }
            _ => {}
        }
    }
    result
}

#[test]
fn test_list_items() {
    assert_eq!(list_items("[]"), Vec::<String>::new());
    assert_eq!(list_items("[1]"), vec!["1"]);
    assert_eq!(list_items("[1,2]"), vec!["1", "2"]);
    assert_eq!(list_items("[1,[2]]"), vec!["1", "[2]"]);
    assert_eq!(list_items("[[1],2]"), vec!["[1]", "2"]);
    assert_eq!(list_items("[[]]"), vec!["[]"]);
}

fn compare_lists(left: &str, right: &str) -> Ordering {
    let left_list = list_items(left);
    let right_list = list_items(right);
    let mut result: Ordering;

    for (left_item, right_item) in left_list.iter().zip(right_list.iter()) {
        if left_item.starts_with('[') && right_item.starts_with('[') {
            result = compare_lists(left_item, right_item);
        } else if left_item.starts_with('[') {
            result = compare_lists(left_item, format!("[{right_item}]").as_str());
        } else if right_item.starts_with('[') {
            result = compare_lists(format!("[{left_item}]").as_str(), right_item);
        } else {
            result = left_item
                .parse::<u32>()
                .unwrap()
                .cmp(&right_item.parse::<u32>().unwrap());
        }
        if result != Ordering::Equal {
            return result;
        }
    }

    left_list.len().cmp(&right_list.len())
}

#[test]
fn test_compare_lists() {
    assert_eq!(compare_lists("[1,1,3,1,1]", "[1,1,5,1,1]"), Ordering::Less);
    assert_eq!(compare_lists("[[1],[2,3,4]]", "[[1],4]"), Ordering::Less);
    assert_eq!(compare_lists("[9]", "[[8,7,6]]"), Ordering::Greater);
    assert_eq!(
        compare_lists("[[4,4],4,4]", "[[4,4],4,4,4]"),
        Ordering::Less
    );
    assert_eq!(compare_lists("[7,7,7,7]", "[7,7,7]"), Ordering::Greater);
    assert_eq!(compare_lists("[]", "[3]"), Ordering::Less);
    assert_eq!(compare_lists("[[[]]]", "[[]]"), Ordering::Greater);
    assert_eq!(
        compare_lists("[1,[2,[3,[4,[5,6,7]]]],8,9]", "[1,[2,[3,[4,[5,6,0]]]],8,9]"),
        Ordering::Greater
    );
}

fn main() {
    let input = include_str!("../input.txt");
    let pairs = read_input(input).unwrap();
    let result: usize = pairs
        .iter()
        .enumerate()
        .filter(|(_, p)| compare_lists(&p.left, &p.right) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum();
    println!("part 1: {result}");

    let mut packets = read_input_part2(input);
    packets.push(String::from("[[2]]"));
    packets.push(String::from("[[6]]"));
    packets.sort_by(|a, b| compare_lists(a, b));
    let pos1 = packets.iter().position(|x| x == "[[2]]").unwrap() + 1;
    let pos2 = packets.iter().position(|x| x == "[[6]]").unwrap() + 1;
    let result = pos1 * pos2;
    println!("part 2: {result}");
}
