// Day 1 of Advent of Code 2022.
// https://adventofcode.com/2022/day/1

fn part_one() {
    let calories = include_str!("cal.txt");

    let mut elves: Vec<usize> = Vec::new();

    let mut current_elf = 0;

    for line in calories.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<usize>().unwrap();
        }
    }

    println!("{:?}", elves.iter().max())
}

fn part_two() {
    let calories = include_str!("cal.txt");

    let mut elves: Vec<usize> = Vec::new();

    let mut current_elf = 0;

    for line in calories.lines() {
        if line.is_empty() {
            elves.push(current_elf);
            current_elf = 0;
        } else {
            current_elf += line.parse::<usize>().unwrap();
        }
    }

    elves.sort();
    elves.reverse();

    println!("{}", elves[0] + elves[1] + elves[2])
}

fn main() {
    part_one();
    part_two();
}
