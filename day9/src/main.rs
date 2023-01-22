use std::collections::HashSet;
use std::fmt;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Move {
    dir: Direction,
    steps: u8,
}

#[derive(Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Board {
    rope: Vec<Point>,
    seen: HashSet<(i16, i16)>,
}

impl Board {
    fn new(len: usize) -> Self {
        let mut rope = vec![];
        for _ in 0..len {
            rope.push(Point { x: 0, y: 0 });
        }
        let mut seen = HashSet::new();
        seen.insert((0, 0));

        Self { rope, seen }
    }

    fn move_head(&mut self, m: &Move) {
        //println!("== {:?} {} ==", m.dir, m.steps);
        for _ in 0..m.steps {
            match m.dir {
                Direction::Up => self.rope[0].y += 1,
                Direction::Right => self.rope[0].x += 1,
                Direction::Down => self.rope[0].y -= 1,
                Direction::Left => self.rope[0].x -= 1,
            }
            //println!("{}", self);
            self.move_tail();
        }
    }

    fn move_tail(&mut self) {
        for i in 1..self.rope.len() {
            let dx = self.rope[i - 1].x - self.rope[i].x;
            let dy = self.rope[i - 1].y - self.rope[i].y;
            if dx.abs() > 1 || dy.abs() > 1 {
                self.rope[i].x += dx.signum();
                self.rope[i].y += dy.signum();
                if i == self.rope.len() - 1 {
                    self.seen.insert((self.rope[i].x, self.rope[i].y));
                }
                //println!("{}", self);
            }
        }
    }
}

// Only works for part 1
// https://doc.rust-lang.org/std/fmt/trait.Display.html
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for y in (self.rope[1].y - 2..self.rope[1].y + 3).rev() {
            for x in self.rope[1].x - 2..self.rope[1].x + 3 {
                if x == self.rope[0].x && y == self.rope[0].y {
                    s.push('H');
                } else if x == self.rope[1].x && y == self.rope[1].y {
                    s.push('T');
                } else {
                    s.push('.');
                }
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn read_input(input: &str) -> Option<Vec<Move>> {
    let mut moves = vec![];
    for line in input.lines() {
        let (direction, steps) = line.split_once(' ')?;
        let dir = match direction {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => return None,
        };
        moves.push(Move {
            dir,
            steps: steps.parse().ok()?,
        });
    }
    Some(moves)
}

fn main() {
    let input = include_str!("../input.txt");
    let moves = read_input(input).unwrap();

    let mut board = Board::new(2);
    for m in &moves {
        board.move_head(m);
    }
    let result = board.seen.len();
    println!("part 1: {result}");

    let mut board = Board::new(10);
    for m in &moves {
        board.move_head(m);
    }
    let result = board.seen.len();
    println!("part 2: {result}");
}
