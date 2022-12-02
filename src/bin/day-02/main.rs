// Day 2 of Advent of Code 2022.
// https://adventofcode.com/2021/day/2

fn part_one() {
    let rps = include_str!("rps.txt");

    let mut total_score = 0;

    for line in rps.lines() {
        let opponent = line.chars().next().unwrap();
        let me = line.chars().nth(2).unwrap();

        let points = match me {
            'X' => match opponent {
                'A' => 4,
                'B' => 1,
                'C' => 7,
                _ => 0,
            },
            'Y' => match opponent {
                'A' => 8,
                'B' => 5,
                'C' => 2,
                _ => 0,
            },
            'Z' => match opponent {
                'A' => 3,
                'B' => 9,
                'C' => 6,
                _ => 0,
            },
            _ => 0,
        };

        total_score += points;
    }

    println!("Total score Part 1: {}", total_score);
}

fn part_two() {
    let rps = include_str!("rps.txt");

    let mut total_score = 0;

    for line in rps.lines() {
        let opponent = line.chars().next().unwrap();
        let me = line.chars().nth(2).unwrap();

        let points = match me {
            'X' => match opponent {
                'A' => 3,
                'B' => 1,
                'C' => 2,
                _ => 0,
            },
            'Y' => match opponent {
                'A' => 4,
                'B' => 5,
                'C' => 6,
                _ => 0,
            },
            'Z' => match opponent {
                'A' => 8,
                'B' => 9,
                'C' => 7,
                _ => 0,
            },
            _ => 0,
        };

        total_score += points;
    }

    println!("Total score Part 2: {}", total_score);
}

fn main() {
    part_one();
    part_two();
}
