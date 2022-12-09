// Day 9 of Advent of Code 2022.
// https://adventofcode.com/2022/day/9

use std::collections::HashSet;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Movement {
    instruction: (Direction, i32),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pos: (i32, i32),
}

fn get_move_instruction(str: &str) -> Movement {
    let (dir, amount_str) = str.split_once(' ').unwrap();
    let amount = amount_str.parse::<i32>().unwrap();

    let direction = match dir {
        "U" => Direction::Up,
        "D" => Direction::Down,
        "L" => Direction::Left,
        "R" => Direction::Right,
        _ => panic!("Invalid direction."),
    };

    Movement {
        instruction: (direction, amount),
    }
}

impl Point {
    fn is_adjacent(&self, other: &Point) -> bool {
        let zero = self.pos.0 - other.pos.0;
        let one = self.pos.1 - other.pos.1;

        (-1..=1).contains(&zero) && (-1..=1).contains(&one)
    }

    fn move_head_coords(&mut self, dir: &Direction) {
        match dir {
            Direction::Up => self.pos.0 -= 1,
            Direction::Down => self.pos.0 += 1,
            Direction::Left => self.pos.1 -= 1,
            Direction::Right => self.pos.1 += 1,
        }
    }

    fn move_tail_coords(&mut self, other: &Point) {
        if self.pos != other.pos && !self.is_adjacent(other) {
            let possible_moves = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (1, 1),
                (-1, -1),
                (-1, 1),
                (1, -1),
            ];

            for (zero, one) in possible_moves {
                let neighbor = Point {
                    pos: (other.pos.0 + zero, other.pos.1 + one),
                };

                if self.is_adjacent(&neighbor) {
                    *self = neighbor;
                    break;
                }
            }
        }
    }
}

fn get_visited_tiles(movements: &Vec<Movement>, rope_length: usize) -> HashSet<Point> {
    let mut points: Vec<Point> = vec![Point { pos: (0, 0) }; rope_length];
    let mut visited_tiles = HashSet::new();

    visited_tiles.insert(Point { pos: (0, 0) });

    for movement in movements {
        for _ in 0..movement.instruction.1 {
            // First one is the head.
            points[0].move_head_coords(&movement.instruction.0);

            // Then come the followers.
            for i in 1..rope_length {
                let before_point = points[i - 1];
                points[i].move_tail_coords(&before_point);
            }

            // A HashSet ignores duplicate entries.
            visited_tiles.insert(points[rope_length - 1]);
        }
    }

    visited_tiles
}

fn part_one() {
    let movement = include_str!("rope.txt");
    let mut movements: Vec<Movement> = Vec::new();

    for line in movement.lines() {
        movements.push(get_move_instruction(line));
    }

    // Had to redesign everything for part two anyways,
    // so I just deleted all that and used the new code that worked for length 10.
    // Looks cleaner.
    let tiles = get_visited_tiles(&movements, 2);

    println!("{}", tiles.len());
}
fn part_two() {
    let movement = include_str!("rope.txt");
    let mut movements: Vec<Movement> = Vec::new();

    for line in movement.lines() {
        movements.push(get_move_instruction(line));
    }

    let tiles = get_visited_tiles(&movements, 10);

    println!("{}", tiles.len());
}

fn main() {
    part_one();
    part_two();
}
