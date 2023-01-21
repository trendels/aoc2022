use std::collections::HashSet;

struct Map {
    cells: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

fn read_input(input: &str) -> Map {
    let mut cells = vec![];
    for line in input.lines() {
        let mut row: Vec<u8> = vec![];
        for c in line.chars() {
            row.push(c.to_string().parse().unwrap());
        }
        cells.push(row);
    }
    let height = cells.len();
    let width = cells[0].len();

    Map {
        cells,
        width,
        height,
    }
}

fn count_visible_trees(map: &Map) -> usize {
    let total = 2 * map.width + 2 * map.height - 4;
    let mut seen = HashSet::new();
    let mut max_height;
    for y in 1..map.height - 1 {
        max_height = map.cells[y][0];
        // looking from the left
        for x in 1..map.width - 1 {
            if map.cells[y][x] > max_height {
                seen.insert((x, y));
                max_height = map.cells[y][x];
            }
        }
        max_height = map.cells[y][map.width - 1];
        // looking from the right
        for x in (1..map.width - 1).rev() {
            if map.cells[y][x] > max_height {
                seen.insert((x, y));
                max_height = map.cells[y][x];
            }
        }
    }
    for x in 1..map.width - 1 {
        max_height = map.cells[0][x];
        // looking from the top
        for y in 1..map.height - 1 {
            if map.cells[y][x] > max_height {
                seen.insert((x, y));
                max_height = map.cells[y][x];
            }
        }
        max_height = map.cells[map.height - 1][x];
        // looking from the bottom
        for y in (1..map.height - 1).rev() {
            if map.cells[y][x] > max_height {
                seen.insert((x, y));
                max_height = map.cells[y][x];
            }
        }
    }
    total + seen.len()
}

fn get_score(map: &Map, x: usize, y: usize) -> u32 {
    let value = map.cells[y][x];
    let mut score_r = 0;
    let mut score_l = 0;
    let mut score_d = 0;
    let mut score_u = 0;
    // look to the right
    for x2 in x + 1..map.width {
        score_r += 1;
        if map.cells[y][x2] >= value {
            break;
        }
    }
    // look to the left
    for x2 in (0..x).rev() {
        score_l += 1;
        if map.cells[y][x2] >= value {
            break;
        }
    }
    // look down
    for y2 in y + 1..map.height {
        score_d += 1;
        if map.cells[y2][x] >= value {
            break;
        }
    }
    // look up
    for y2 in (0..y).rev() {
        score_u += 1;
        if map.cells[y2][x] >= value {
            break;
        }
    }
    score_r * score_l * score_d * score_u
}

fn get_best_scenic_score(map: &Map) -> u32 {
    let mut best_score = 0;
    for y in 0..map.height {
        for x in 0..map.width {
            let score = get_score(map, x, y);
            if score > best_score {
                best_score = score
            }
        }
    }
    best_score
}

fn main() {
    let input = include_str!("../input.txt");
    let map = read_input(input);

    let result = count_visible_trees(&map);
    println!("part 1: {result}");

    let result = get_best_scenic_score(&map);
    println!("part 2: {result}");
}
