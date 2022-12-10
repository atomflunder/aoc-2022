// Day 10 of Advent of Code 2022.
// https://adventofcode.com/2022/day/10

use std::fmt::Display;

struct Crt {
    pixels: [char; 40 * 6],
}

impl Crt {
    fn new() -> Self {
        Self {
            pixels: [' '; 40 * 6],
        }
    }
}

impl Display for Crt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, c) in self.pixels.iter().enumerate() {
            if i % 40 == 0 {
                writeln!(f)?;
            }
            write!(f, "{}", c)?;
        }
        writeln!(f)
    }
}

fn handle_turn(
    line: &str,
    cycle: &mut i32,
    x: &mut i32,
    signal_strengths: &mut Vec<i32>,
    crt_display: &mut Crt,
) {
    let args = line.split(' ').collect::<Vec<&str>>();

    if args[0] == "noop" {
        handle_cycle(*cycle, *x, signal_strengths, crt_display);
        *cycle += 1;
    } else if args[0] == "addx" {
        handle_cycle(*cycle, *x, signal_strengths, crt_display);
        *cycle += 1;
        handle_cycle(*cycle, *x, signal_strengths, crt_display);
        *cycle += 1;
        let y = args[1].parse::<i32>().unwrap();
        *x += y;
    }
}

fn handle_cycle(cycle: i32, x: i32, signal_strengths: &mut Vec<i32>, crt_display: &mut Crt) {
    add_signal(cycle, x, signal_strengths);
    draw_pixel(cycle, x, crt_display);
}

fn add_signal(cycle: i32, x: i32, signal_strengths: &mut Vec<i32>) {
    if [20, 60, 100, 140, 180, 220].contains(&cycle) {
        signal_strengths.push(cycle * x);
    }
}

fn draw_pixel(cycle: i32, x: i32, crt_display: &mut Crt) {
    let row = (cycle as f64 / 40.).floor() as i32;
    let sprite_center = x + row * 40;
    let sprite = [sprite_center - 1, sprite_center, sprite_center + 1];

    if sprite.contains(&cycle) {
        crt_display.pixels[cycle as usize] = '#';
    } else {
        crt_display.pixels[cycle as usize] = '.';
    }
}

// This did not make sense to split up into two parts so I just added the second to the first.
fn solution() {
    let instructions = include_str!("instructions.txt");

    let mut cycle = 0;
    let mut x = 1;
    let mut signal_strengths: Vec<i32> = Vec::new();

    let mut crt_display = Crt::new();

    for line in instructions.lines() {
        handle_turn(
            line,
            &mut cycle,
            &mut x,
            &mut signal_strengths,
            &mut crt_display,
        );
    }

    println!("Part 1: {}", signal_strengths.iter().sum::<i32>());
    println!("Part 2: {}", crt_display);
}

fn main() {
    solution();
}
