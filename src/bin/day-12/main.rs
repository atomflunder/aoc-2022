// Day 12 of Advent of Code 2022.
// https://adventofcode.com/2022/day/12

use std::mem::swap;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct Coordinates {
    x: usize,
    y: usize,
}

impl Coordinates {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    start: Coordinates,
    end: Coordinates,
    map: Vec<Vec<i32>>,
}

fn get_grid() -> Grid {
    let input = include_str!("hill.txt");

    let mut height_map: Vec<Vec<i32>> = Vec::new();
    let mut start_coords: (usize, usize) = (0, 0);
    let mut end_coords: (usize, usize) = (0, 0);

    let alphabet = "abcdefghijklmnopqrstuvwxyz";

    for (i, line) in input.lines().enumerate() {
        let mut current_line: Vec<i32> = Vec::new();

        for (j, c) in line.chars().enumerate() {
            let height = match c {
                'S' => {
                    start_coords = (i, j);
                    0
                }
                'E' => {
                    end_coords = (i, j);
                    25
                }
                _ => {
                    let index = alphabet.chars().position(|l| l == c).unwrap();
                    index
                }
            };
            current_line.push(height as i32);
        }

        height_map.push(current_line);
    }

    Grid {
        width: height_map[0].len(),
        height: height_map.len(),
        start: Coordinates {
            x: start_coords.0,
            y: start_coords.1,
        },
        end: Coordinates {
            x: end_coords.0,
            y: end_coords.1,
        },
        map: height_map,
    }
}

fn get_neighbors(coord: &Coordinates, grid: &Grid) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    if coord.x > 0 {
        positions.push((coord.x - 1, coord.y));
    }
    if coord.x < grid.height - 1 {
        positions.push((coord.x + 1, coord.y));
    }
    if coord.y > 0 {
        positions.push((coord.x, coord.y - 1));
    }
    if coord.y < grid.width - 1 {
        positions.push((coord.x, coord.y + 1));
    }
    positions
}

fn part_one() {
    let grid = get_grid();

    // Lucky for us, brute forcing backwards works well here.
    let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid.width]; grid.height];
    let mut positions = vec![grid.end];
    let mut next_positions = Vec::new();
    let mut step = -1;

    visited[grid.end.x][grid.end.y] = 0;

    loop {
        step += 1;
        for coord in positions.drain(..) {
            if coord == grid.start {
                println!("{}", step);
                return;
            }

            for (x, y) in get_neighbors(&coord, &grid) {
                if visited[x][y] == -1 && grid.map[coord.x][coord.y] - grid.map[x][y] <= 1 {
                    visited[x][y] = step;
                    next_positions.push(Coordinates::new(x, y));
                }
            }
        }
        swap(&mut next_positions, &mut positions);
    }
}

fn part_two() {
    let grid = get_grid();

    let mut visited: Vec<Vec<i32>> = vec![vec![-1; grid.width]; grid.height];
    let mut positions = vec![grid.end];
    let mut next_positions = Vec::new();
    let mut step = -1;

    visited[grid.end.x][grid.end.y] = 0;

    loop {
        step += 1;
        for coord in positions.drain(..) {
            // Instead of just the start, we have check if it is 0, so either start or 'a'.
            if grid.map[coord.x][coord.y] == 0 {
                println!("{}", step);
                return;
            }

            for (x, y) in get_neighbors(&coord, &grid) {
                if visited[x][y] == -1 && grid.map[coord.x][coord.y] - grid.map[x][y] <= 1 {
                    visited[x][y] = step;
                    next_positions.push(Coordinates::new(x, y));
                }
            }
        }
        swap(&mut next_positions, &mut positions);
    }
}

fn main() {
    part_one();
    part_two();
}
