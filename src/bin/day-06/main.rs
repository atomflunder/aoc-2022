// Day 6 of Advent of Code 2022.
// https://adventofcode.com/2022/day/6

fn get_marker_at(num: usize) -> i32 {
    let signal = include_str!("signal.txt");

    let mut count = 0;
    let mut current = vec![' '; num];

    for c in signal.chars() {
        current.rotate_left(1);
        current[num - 1] = c;
        count += 1;

        let mut temp = current.to_vec();
        temp.sort();
        temp.dedup();
        if temp.len() == num && !current.contains(&' ') {
            break;
        }
    }

    count
}

fn part_one() {
    println!("{}", get_marker_at(4));
}

fn part_two() {
    println!("{}", get_marker_at(14));
}

fn main() {
    part_one();
    part_two();
}
