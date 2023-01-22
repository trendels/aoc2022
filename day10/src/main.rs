use std::fmt;
use std::io;
use std::{thread, time};

#[derive(Debug)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Cpu {
    x: i32,
    program: Vec<Instruction>,
    ip: usize,
    counter: i32,
}

impl Cpu {
    fn new(program: Vec<Instruction>) -> Self {
        Self {
            x: 1,
            program,
            ip: 0,
            counter: 0,
        }
    }

    fn cycle(&mut self) {
        if self.done() {
            return;
        }

        let instruction = &self.program[self.ip];
        if self.counter == 0 {
            self.counter = match instruction {
                Instruction::Noop => 1,
                Instruction::Addx(_) => 2,
            }
        }
        self.counter -= 1;
        if self.counter == 0 {
            self.x = match instruction {
                Instruction::Noop => self.x,
                Instruction::Addx(value) => self.x + value,
            };
            self.ip += 1;
        }
    }

    fn done(&self) -> bool {
        self.ip >= self.program.len()
    }
}

struct Crt {
    pixels: [[char; 40]; 6],
    h: usize,
    v: usize,
}

impl Crt {
    fn new() -> Self {
        Crt {
            pixels: [['.'; 40]; 6],
            h: 0,
            v: 0,
        }
    }

    fn cycle(&mut self, input: i32) {
        if (input - self.h as i32).abs() < 2 {
            self.pixels[self.v][self.h] = '#'
        } else {
            self.pixels[self.v][self.h] = '.'
        }
        self.h = (self.h + 1) % 40;
        if self.h == 0 {
            self.v = (self.v + 1) % 6;
        }
    }
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for y in 0..6 {
            for x in 0..40 {
                s.push(self.pixels[y][x]);
            }
            s.push('\n');
        }
        write!(f, "{}", s)
    }
}

fn read_input(input: &str) -> Option<Vec<Instruction>> {
    let mut instructions = vec![];
    for line in input.lines() {
        if line == "noop" {
            instructions.push(Instruction::Noop);
        } else if let Some(value) = line.strip_prefix("addx ") {
            instructions.push(Instruction::Addx(value.parse().ok()?));
        } else {
            return None;
        }
    }
    Some(instructions)
}

fn main() {
    let input = include_str!("../input.txt");
    let instructions = read_input(input).unwrap();
    let mut cpu = Cpu::new(instructions);
    let samples = [20, 60, 100, 140, 180, 220];
    let mut result = 0;
    for n in 1..221 {
        if samples.contains(&n) {
            result += n * cpu.x;
        }
        cpu.cycle();
    }
    println!("part 1: {result}");

    println!("\npress Enter for part 2");
    let mut answer = String::new();
    io::stdin().read_line(&mut answer).unwrap();

    // This uses ANSI escape sequences and sleep to produce a little animation of the screen being
    // drawn.
    // Rust does not support octal escape codes, but only hexadecimal ones, so you have to
    // start the escape sequences with \x1B instead of \033 as you see in other languages.
    // See https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
    let instructions = read_input(input).unwrap();
    let mut cpu = Cpu::new(instructions);
    let mut crt = Crt::new();
    let delay = time::Duration::from_millis(8);
    print!("\x1B[2J"); // Clear the screen
    while !cpu.done() {
        crt.cycle(cpu.x);
        cpu.cycle();
        print!("\x1B[0;0H"); // Move cursor to 0,0
        println!("{}", &crt);
        thread::sleep(delay);
    }
}
