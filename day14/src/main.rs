use std::cmp::{max, min};
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(format!("Invalid point: {s}"))?;
        let x = x
            .parse::<u32>()
            .map_err(|_| format!("Invalid X coordinate: {x}"))?;
        let y = y
            .parse::<u32>()
            .map_err(|_| format!("Invalid Y coordinate: {y}"))?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
struct Map {
    source: Point,
    cells: HashMap<(u32, u32), char>,
    max_y: u32,
}

impl Map {
    fn move_grain(&self, p: &mut Point) -> bool {
        if !self.cells.contains_key(&(p.x, p.y + 1)) {
            p.y += 1;
        } else if !self.cells.contains_key(&(p.x - 1, p.y + 1)) {
            p.y += 1;
            p.x -= 1;
        } else if !self.cells.contains_key(&(p.x + 1, p.y + 1)) {
            p.y += 1;
            p.x += 1;
        } else {
            return false;
        }
        true
    }

    fn simulate_sand_part1(&mut self) -> Option<Point> {
        let mut p: Point = self.source;
        while p.y < self.max_y {
            if !self.move_grain(&mut p) {
                self.cells.insert((p.x, p.y), 'o');
                return Some(p);
            }
        }
        None
    }

    fn simulate_sand_part2(&mut self) -> Option<Point> {
        let mut p: Point = self.source;
        while p.y < self.max_y + 1 {
            if !self.move_grain(&mut p) {
                self.cells.insert((p.x, p.y), 'o');
                return Some(p);
            }
        }
        self.cells.insert((p.x, p.y), 'o');
        Some(p)
    }
}

fn read_input(input: &str) -> Option<Map> {
    let mut map = Map {
        source: Point { x: 500, y: 0 },
        cells: HashMap::new(),
        max_y: 0,
    };
    for line in input.lines() {
        let mut points = vec![];
        for coords in line.split(" -> ") {
            points.push(Point::from_str(coords).ok()?)
        }
        let mut p = &points[0];
        for next in points.iter().skip(1) {
            let x1 = min(p.x, next.x);
            let x2 = max(p.x, next.x);
            let y1 = min(p.y, next.y);
            let y2 = max(p.y, next.y);
            if y2 > map.max_y {
                map.max_y = y2;
            }
            for x in x1..x2 + 1 {
                map.cells.insert((x, p.y), '#');
            }
            for y in y1..y2 + 1 {
                map.cells.insert((p.x, y), '#');
            }
            p = next;
        }
    }
    Some(map)
}

fn main() {
    let input = include_str!("../input.txt");
    let mut map = read_input(input).unwrap();
    let mut result = 0;
    while map.simulate_sand_part1().is_some() {
        result += 1;
    }
    println!("part 1: {result}");

    let mut map = read_input(input).unwrap();
    let mut result = 0;
    while let Some(point) = map.simulate_sand_part2() {
        result += 1;
        if point == map.source {
            break;
        }
    }
    println!("part 2: {result}");
}
