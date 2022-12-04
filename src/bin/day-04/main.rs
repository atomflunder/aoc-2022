// Day 4 of Advent of Code 2022.
// https://adventofcode.com/2022/day/4

fn part_one() {
    let sections = include_str!("sections.txt");

    let mut count = 0;

    for line in sections.lines() {
        let (first, second) = line.split_once(',').unwrap();

        let (first1, first2) = first.split_once('-').unwrap();
        let (second1, second2) = second.split_once('-').unwrap();

        let mut first_vec = vec![];
        let mut second_vec = vec![];

        for i in first1.parse::<usize>().unwrap()..=first2.parse::<usize>().unwrap() {
            first_vec.push(i);
        }

        for i in second1.parse::<usize>().unwrap()..=second2.parse::<usize>().unwrap() {
            second_vec.push(i);
        }

        if first_vec.iter().all(|i| second_vec.contains(i))
            || second_vec.iter().all(|i| first_vec.contains(i))
        {
            count += 1;
        }
    }

    println!("{}", count);
}

fn part_two() {
    let sections = include_str!("sections.txt");

    let mut count = 0;

    for line in sections.lines() {
        let (first, second) = line.split_once(',').unwrap();

        let (first1, first2) = first.split_once('-').unwrap();
        let (second1, second2) = second.split_once('-').unwrap();

        let mut first_vec = vec![];
        let mut second_vec = vec![];

        for i in first1.parse::<usize>().unwrap()..=first2.parse::<usize>().unwrap() {
            first_vec.push(i);
        }

        for i in second1.parse::<usize>().unwrap()..=second2.parse::<usize>().unwrap() {
            second_vec.push(i);
        }

        if first_vec.iter().any(|i| second_vec.contains(i))
            || second_vec.iter().any(|i| first_vec.contains(i))
        {
            count += 1;
        }
    }

    println!("{}", count);
}

fn main() {
    part_one();
    part_two();
}
