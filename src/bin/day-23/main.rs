// Day 23 of Advent of Code 2022.
// https://adventofcode.com/2022/day/23

use std::collections::{HashMap, HashSet};

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct Elf {
    x: isize,
    y: isize,
}

impl Elf {
    fn move_in_direction(&self, dir: Direction) -> Self {
        match dir {
            Direction::North => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::NorthEast => Self {
                x: self.x + 1,
                y: self.y - 1,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
            },
            Direction::SouthEast => Self {
                x: self.x + 1,
                y: self.y + 1,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::SouthWest => Self {
                x: self.x - 1,
                y: self.y + 1,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::NorthWest => Self {
                x: self.x - 1,
                y: self.y - 1,
            },
        }
    }

    fn get_neighbors_for_direction(&self, dir: Direction) -> Vec<Self> {
        let (diag1, diag2) = dir.get_adjacent();
        vec![
            self.move_in_direction(diag1),
            self.move_in_direction(dir),
            self.move_in_direction(diag2),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

impl Direction {
    fn get_adjacent(&self) -> (Self, Self) {
        match self {
            Direction::North => (Direction::NorthWest, Direction::NorthEast),
            Direction::East => (Direction::NorthEast, Direction::SouthEast),
            Direction::South => (Direction::SouthEast, Direction::SouthWest),
            Direction::West => (Direction::SouthWest, Direction::NorthWest),
            _ => unreachable!(),
        }
    }

    fn get_cardinal() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::West,
            Direction::East,
        ]
    }

    fn get_all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
            Direction::NorthWest,
            Direction::NorthEast,
            Direction::SouthWest,
            Direction::SouthEast,
        ]
    }
}

fn get_elves(input: &str) -> HashSet<Elf> {
    let mut positions = HashSet::default();
    for (i, line) in input.lines().enumerate() {
        for (j, c) in line.as_bytes().iter().enumerate() {
            match c {
                b'#' => positions.insert(Elf {
                    x: j as isize,
                    y: i as isize,
                }),
                b'.' => continue,
                _ => unreachable!(),
            };
        }
    }
    positions
}

fn part_one() {
    let input = include_str!("elves.txt");
    let mut elves = get_elves(input);
    let mut directions = Direction::get_cardinal();

    for _ in 0..10 {
        let mut proposed_positions: HashMap<Elf, Elf> = HashMap::default();
        let mut end_positions: HashMap<Elf, u32> = HashMap::default();

        for p in elves.iter() {
            if Direction::get_all()
                .iter()
                .any(|d| elves.contains(&p.move_in_direction(*d)))
            {
                for card in directions.iter() {
                    if p.get_neighbors_for_direction(*card)
                        .iter()
                        .all(|e| !elves.contains(e))
                    {
                        let end_pos = p.move_in_direction(*card);
                        proposed_positions.insert(*p, end_pos);
                        *end_positions.entry(end_pos).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        for (start, end) in proposed_positions.iter() {
            if end_positions[end] == 1 {
                elves.remove(start);
                elves.insert(*end);
            }
        }
        directions.rotate_left(1);
    }

    let max_x = elves.iter().map(|e| e.x).max().unwrap();
    let max_y = elves.iter().map(|e| e.y).max().unwrap();
    let min_x = elves.iter().map(|e| e.x).min().unwrap();
    let min_y = elves.iter().map(|e| e.y).min().unwrap();

    let solution = ((max_x + 1 - min_x) as usize * (max_y + 1 - min_y) as usize) - elves.len();

    println!("{}", solution);
}

fn part_two() {
    let input = include_str!("elves.txt");
    let mut elves = get_elves(input);
    let mut directions = Direction::get_cardinal();

    let mut round_counter = 0;
    'elfloop: loop {
        round_counter += 1;
        let mut proposed_positions: HashMap<Elf, Elf> = HashMap::default();
        let mut end_positions: HashMap<Elf, u32> = HashMap::default();

        for p in elves.iter() {
            if Direction::get_all()
                .iter()
                .any(|d| elves.contains(&p.move_in_direction(*d)))
            {
                for card in directions.iter() {
                    if p.get_neighbors_for_direction(*card)
                        .iter()
                        .all(|e| !elves.contains(e))
                    {
                        let end_pos = p.move_in_direction(*card);
                        proposed_positions.insert(*p, end_pos);
                        *end_positions.entry(end_pos).or_insert(0) += 1;
                        break;
                    }
                }
            }
        }

        if proposed_positions.is_empty() {
            println!("{}", round_counter);
            break 'elfloop;
        }

        for (start, end) in proposed_positions.iter() {
            if end_positions[end] == 1 {
                elves.remove(start);
                elves.insert(*end);
            }
        }
        directions.rotate_left(1);
    }
}

fn main() {
    part_one();
    part_two();
}
