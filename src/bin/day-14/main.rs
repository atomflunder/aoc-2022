// Day 14 of Advent of Code 2022.
// https://adventofcode.com/2022/day/14

use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Clone, Copy, Hash, PartialEq, Eq)]
struct FullTile {
    x: usize,
    y: usize,
}

fn parse_rocks(input: &str) -> HashSet<FullTile> {
    let mut full_tiles: HashSet<FullTile> = HashSet::new();

    for line in input.lines() {
        let edges: Vec<(usize, usize)> = line
            .split(" -> ")
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                (x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();

        for pair in edges.windows(2) {
            if let [(x1, y1), (x2, y2)] = pair {
                let (min_x, max_x) = (min(x1, x2), max(x1, x2));
                let (min_y, max_y) = (min(y1, y2), max(y1, y2));

                for x in *min_x..=*max_x {
                    for y in *min_y..=*max_y {
                        full_tiles.insert(FullTile::new(x, y));
                    }
                }
            }
        }
    }
    full_tiles
}

impl FullTile {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn fall(&self, fill: &mut HashSet<FullTile>, limit: usize, part_one: bool) -> bool {
        let mut current_pos = *self;

        loop {
            let start = current_pos;

            for tile in [
                FullTile::new(current_pos.x, current_pos.y + 1),
                FullTile::new(current_pos.x - 1, current_pos.y + 1),
                FullTile::new(current_pos.x + 1, current_pos.y + 1),
            ] {
                if !fill.contains(&tile) {
                    current_pos = tile;
                    break;
                }
            }

            if start == current_pos {
                break;
            }

            if part_one {
                if current_pos.y >= limit {
                    return false;
                }
            } else if current_pos.y == limit + 1 {
                break;
            }
        }

        fill.insert(current_pos);

        if part_one {
            true
        } else {
            current_pos.y != self.y
        }
    }
}

fn part_one() {
    let file = include_str!("cave.txt");

    let cave_walls = parse_rocks(file);
    let sand = FullTile::new(500, 0);

    let mut full_tiles = cave_walls.clone();
    let limit = full_tiles.iter().fold(0, |acc, sand| max(acc, sand.y));
    while sand.fall(&mut full_tiles, limit, true) {}

    println!("{:?}", full_tiles.len() - cave_walls.len());
}

fn part_two() {
    let file = include_str!("cave.txt");

    let cave_walls = parse_rocks(file);
    let sand = FullTile::new(500, 0);

    let mut full_tiles = cave_walls.clone();
    let limit = full_tiles.iter().fold(0, |acc, sand| max(acc, sand.y));
    while sand.fall(&mut full_tiles, limit, false) {}

    println!("{:?}", full_tiles.len() - cave_walls.len());
}

fn main() {
    part_one();
    part_two();
}
