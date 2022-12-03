// Day 3 of Advent of Code 2022.
// https://adventofcode.com/2022/day/3

use std::collections::HashMap;

fn get_hashmap() -> HashMap<char, i32> {
    let lowercase_letters: Vec<char> = vec![
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let uppercase_letters: Vec<char> = lowercase_letters
        .iter()
        .map(|c| c.to_ascii_uppercase())
        .collect();

    let mut priority_hashmap = HashMap::new();

    let mut index = 1;

    for i in lowercase_letters {
        priority_hashmap.insert(i, index);
        index += 1;
    }

    for i in uppercase_letters {
        priority_hashmap.insert(i, index);
        index += 1;
    }

    priority_hashmap
}

fn part_one(priority_hashmap: HashMap<char, i32>) {
    let all_rucksacks = include_str!("rucksack.txt");

    let mut sum_priorities = 0;

    for rucksack in all_rucksacks.lines() {
        let (first_half, second_half) = rucksack.split_at(rucksack.len() / 2);

        // Kind of overkill to search for every occurrence.
        let common_chars: Vec<char> = first_half
            .chars()
            .filter(|c| second_half.contains(*c))
            .collect();

        sum_priorities += priority_hashmap.get(&common_chars[0]).unwrap();
    }

    println!("Sum of priorities: {}", sum_priorities);
}

fn part_two(priority_hashmap: HashMap<char, i32>) {
    let all_rucksacks = include_str!("rucksack.txt");

    // Kind of the same, except we split the rucksacks into groups of 3
    // and then compare them instead of splitting each rucksack in half.
    let binding = all_rucksacks.lines().collect::<Vec<&str>>();
    let groups: Vec<&[&str]> = binding.chunks(3).collect();

    let mut sum_priorities = 0;

    for group in groups {
        if let [first, second, third] = group {
            let common_chars: Vec<char> = first
                .chars()
                .filter(|c| second.contains(*c) && third.contains(*c))
                .collect();

            sum_priorities += priority_hashmap.get(&common_chars[0]).unwrap();
        }
    }

    println!("Sum of badges: {}", sum_priorities);
}

fn main() {
    let h = get_hashmap();
    part_one(h.clone());
    part_two(h);
}
