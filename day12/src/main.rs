// This was relatively straightforward thanks to the BinaryHeap implementation in the Rust
// stdlib for implementing Dijkstra's algorithm.
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt;

type Point = (usize, usize);

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<char>>,
    start: Point,
    end: Point,
    width: usize,
    height: usize,
}

impl Map {
    fn get_neighbours(&self, p: Point) -> Vec<(Point, char)> {
        let mut result = vec![];
        // Lots of conversions needed here between `usize` (for indexing into a vec)
        // and i32 (for calculations with possibly negative results).
        let width: i32 = self.width.try_into().unwrap();
        let height: i32 = self.height.try_into().unwrap();
        let x: i32 = p.0.try_into().unwrap();
        let y: i32 = p.1.try_into().unwrap();
        // I typed this expression in not expecting it to be valid syntx. To my surprise, it is!
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < width && ny >= 0 && ny < height {
                let n: Point = (nx.try_into().unwrap(), ny.try_into().unwrap());
                result.push((n, self.get_value(n)));
            }
        }
        result
    }

    fn get_value(&self, p: Point) -> char {
        self.tiles[p.1][p.0]
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        for row in &self.tiles {
            for tile in row {
                s.push(*tile);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn find_shortest_path(
    map: &Map,
    start: Point,
    end: Point,
    end_value: char,
) -> Result<Vec<Point>, &str> {
    let mut dist: HashMap<Point, i32> = HashMap::new();
    let mut prev: HashMap<Point, Point> = HashMap::new();
    let mut queue = BinaryHeap::new();
    let mut value;
    dist.insert(start, 0);
    // BinaryHeap is a max-heap. We we want a min-heap. Wrapping all items in Reverse reverses the
    // sort order.
    queue.push(Reverse((0, start)));

    fn build_path(prev: &HashMap<Point, Point>, node: Point) -> Vec<Point> {
        let mut path = vec![];
        let mut node = node;
        path.push(node);
        while let Some(next) = prev.get(&node) {
            path.push(*next);
            node = *next;
        }
        path
    }

    while let Some(Reverse((p, node))) = queue.pop() {
        value = map.get_value(node);
        if node == end || value == end_value {
            return Ok(build_path(&prev, node));
        }
        let d = *dist.get(&(node)).unwrap();
        if p != d {
            continue;
        }
        for (next, next_value) in map.get_neighbours(node) {
            // Flipped the condition around for the part two,
            // where we need to search in the opposite direction
            // (which also works for part 1).
            // `as i32` used here because of lazyness.
            if (value as i32 - next_value as i32) <= 1 {
                let alt = d + 1;
                if alt < *dist.get(&next).unwrap_or(&i32::MAX) {
                    dist.insert(next, alt);
                    prev.insert(next, node);
                    queue.push(Reverse((alt, next)));
                }
            }
        }
    }

    Err("failed to find path")
}

fn read_input(input: &str) -> Map {
    let mut tiles = vec![];
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let mut row = vec![];
        for (x, c) in line.chars().enumerate() {
            if c == 'S' {
                start = (x, y);
                row.push('a');
            } else if c == 'E' {
                end = (x, y);
                row.push('z');
            } else {
                row.push(c);
            }
        }
        tiles.push(row);
    }
    let height = tiles.len();
    let width = tiles[0].len();

    Map {
        tiles,
        start,
        end,
        width,
        height,
    }
}

// `&[...]` is short for `&Vec<...>`.
// This was a suggestion from Clippy (run `cargo clippy`).
fn print_path(map: &Map, path: &[(usize, usize)]) {
    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            let c = if (x, y) == map.start {
                'S'
            } else if (x, y) == map.end {
                'E'
            } else {
                *tile
            };
            if path.contains(&(x, y)) {
                print!("\x1B[2m");
                print!("{}", c);
                print!("\x1B[0m");
            } else {
                print!("{}", c);
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let input = include_str!("../input.txt");
    let map = read_input(input);
    let path = find_shortest_path(&map, map.end, map.start, 'E').unwrap();
    print_path(&map, &path);
    println!("part 1: {}", path.len() - 1);

    println!();

    let path = find_shortest_path(&map, map.end, map.start, 'a').unwrap();
    print_path(&map, &path);
    println!("part 2: {}", path.len() - 1);
}
